use crate::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub struct ShaderPair {
    vertex_shader: Arc<dyn VertexShader>,
    fragment_shader: Arc<dyn FragmentShader>,

    external_variables: Vec<ExternalShaderVariable>,
}

impl ShaderPair {
    pub fn new(
        vertex_shader: &Arc<dyn VertexShader>,
        fragment_shader: &Arc<dyn FragmentShader>,
    ) -> Self {
        let mut external_variables = Vec::new();
        for variable in vertex_shader.get_variables() {
            match variable.get_variable_type() {
                VertexShaderVariableType::External(ext) => {
                    external_variables.push(ExternalShaderVariable::new(
                        variable.get_name().to_string(),
                        variable.get_data_type(),
                        ext,
                    ))
                }
                _ => {}
            };
        }
        for variable in fragment_shader.get_variables() {
            match variable.get_variable_type() {
                FragmentShaderVariableType::External(ext) => {
                    external_variables.push(ExternalShaderVariable::new(
                        variable.get_name().to_string(),
                        variable.get_data_type(),
                        ext,
                    ))
                }
                _ => {}
            };
        }
        external_variables.shrink_to_fit();
        Self {
            vertex_shader: Arc::clone(vertex_shader),
            fragment_shader: Arc::clone(fragment_shader),
            external_variables,
        }
    }

    pub fn get_vertex_shader(&self) -> &Arc<dyn VertexShader> {
        &self.vertex_shader
    }

    pub fn get_fragment_shader(&self) -> &Arc<dyn FragmentShader> {
        &self.fragment_shader
    }

    pub fn get_external_variables(&self) -> &Vec<ExternalShaderVariable> {
        &self.external_variables
    }
}

#[derive(Debug)]
pub struct ShaderLinkError<E: Error> {
    vertex_name: String,
    fragment_name: String,

    error: E,
}

impl<E: Error> Display for ShaderLinkError<E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "An error occurred while linking {} to {}: {}",
            self.vertex_name, self.fragment_name, self.error
        )
    }
}

impl<E: Error> Error for ShaderLinkError<E> {}

impl<E: Error> ShaderLinkError<E> {
    pub fn new(
        vertex_shader: &dyn VertexShader,
        fragment_shader: &dyn FragmentShader,
        error: E,
    ) -> Self {
        Self {
            vertex_name: vertex_shader.get_debug_name().to_string(),
            fragment_name: fragment_shader.get_debug_name().to_string(),
            error,
        }
    }
}
#[derive(Debug)]
pub enum ShaderNameLinkError {
    TypeMismatch {
        vertex_output: VertexShaderVariable,
        fragment_input: FragmentShaderVariable,
    },
    MissingFragmentInput {
        vertex_output_name: String,
    },
    MissingVertexOutput {
        fragment_input_name: String,
    },
}

impl Display for ShaderNameLinkError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::TypeMismatch{vertex_output, fragment_input} => write!(f, "Type mismatch: The vertex shader has output {:?} and the fragment shader has input {:?}", 
                        vertex_output, fragment_input
                    ),
            Self::MissingFragmentInput{vertex_output_name} => write!(f, "The vertex shader has output {:?}, which doesn't match any of the inputs of the fragment shader",
                vertex_output_name
            ),
            Self::MissingVertexOutput{fragment_input_name} => write!(f, "The fragment shader has input {:?}, which doesn't match any of the outputs of the vertex shader",
                fragment_input_name
            )
        }
    }
}

impl Error for ShaderNameLinkError {}

impl ShaderPair {
    fn match_shader_variables_types(
        variables1: &Vec<VertexShaderVariable>,
        variables2: &Vec<FragmentShaderVariable>,
        filter1: impl Fn(&VertexShaderVariable) -> bool,
        filter2: impl Fn(&FragmentShaderVariable) -> bool,
    ) -> Result<(), (VertexShaderVariable, FragmentShaderVariable)> {
        for var1 in variables1 {
            if filter1(var1) {
                for var2 in variables2 {
                    if filter2(var2) {
                        if var1.get_name() == var2.get_name()
                            && var1.get_data_type() != var2.get_data_type()
                        {
                            return Err((var1.clone(), var2.clone()));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn match_shader_variable_names(
        required_names: &Vec<String>,
        available_names: &Vec<String>,
    ) -> Result<(), String> {
        for required in required_names {
            let mut found = false;
            for available in available_names {
                if available == required {
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(required.clone());
            }
        }

        Ok(())
    }

    pub fn link_by_attribute_names(
        vertex_shader: &Arc<dyn VertexShader>,
        fragment_shader: &Arc<dyn FragmentShader>,
    ) -> Result<Self, ShaderLinkError<ShaderNameLinkError>> {
        let maybe_type_mismatch = Self::match_shader_variables_types(
            vertex_shader.get_variables(),
            fragment_shader.get_variables(),
            |var| var.get_variable_type() == VertexShaderVariableType::FragmentOutput,
            |var| var.get_variable_type() == FragmentShaderVariableType::VertexInput,
        );
        if maybe_type_mismatch.is_err() {
            let type_mismatch = maybe_type_mismatch.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::TypeMismatch {
                    vertex_output: type_mismatch.0,
                    fragment_input: type_mismatch.1,
                },
            ));
        }

        let vertex_output_names = vertex_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == VertexShaderVariableType::FragmentOutput)
            .map(|var| var.get_name().to_string())
            .collect();
        let fragment_input_names = fragment_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == FragmentShaderVariableType::VertexInput)
            .map(|var| var.get_name().to_string())
            .collect();
        let maybe_miss_fragment =
            Self::match_shader_variable_names(&vertex_output_names, &fragment_input_names);
        let maybe_miss_vertex =
            Self::match_shader_variable_names(&fragment_input_names, &vertex_output_names);

        if maybe_miss_fragment.is_err() {
            let unmatched_vertex_output = maybe_miss_fragment.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingFragmentInput {
                    vertex_output_name: unmatched_vertex_output,
                },
            ));
        }

        if maybe_miss_vertex.is_err() {
            let unmatched_fragment_input = maybe_miss_vertex.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingVertexOutput {
                    fragment_input_name: unmatched_fragment_input,
                },
            ));
        }

        Ok(ShaderPair::new(vertex_shader, fragment_shader))
    }
}
