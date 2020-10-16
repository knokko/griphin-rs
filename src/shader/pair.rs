use crate::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Represents a pair of a *VertexShader* and *FragmentShader* that can be used to
/// create a *GraphicsPipeline*.
/// 
/// To create a *ShaderPair*, use one of the *link* functions of this struct
/// (currently, only the *link_by_attribute_names* function is available).
/// 
/// Upon creating a *ShaderPair*, some validation checks will be done to ensure that
/// the vertex shader *matches* the fragment shader (for instance, the fragment
/// shader should have a *VertexInput* variable for every *FragmentOutput* variable
/// of the vertex shader).
/// 
/// Also, the *External* shader variables of the vertex shader and the fragment
/// shader are collected upon creating a *ShaderPair*. This is done at this point
/// because all serious Griphin implementations will need this information anyway.
pub struct ShaderPair {
    vertex_shader: Arc<dyn VertexShader>,
    fragment_shader: Arc<dyn FragmentShader>,

    external_variables: Vec<ExternalShaderVariable>,
}

impl ShaderPair {
    fn new<E: Error>(
        vertex_shader: &Arc<dyn VertexShader>,
        fragment_shader: &Arc<dyn FragmentShader>,
    ) -> Result<Self, ShaderLinkError<E>> {
        let mut external_variables = Vec::new();
        for variable in vertex_shader.get_variables() {
            match variable.get_variable_type() {
                VertexShaderVariableType::External(ext) => {
                    external_variables.push(ExternalShaderVariable::new(
                        variable.get_name(),
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

                    let vertex_variable = external_variables.iter().find(
                        |candidate| candidate.get_name() == variable.get_name()
                    );

                    match vertex_variable {
                        Some(found_it) => {
                            if found_it.get_data_type() != variable.get_data_type() {
                                return Err(ShaderLinkError::general(
                                    vertex_shader.as_ref(), fragment_shader.as_ref(), 
                                    ShaderExternalVariableMismatch::new(
                                        variable.get_name(), 
                                        found_it.get_data_type(), 
                                        variable.get_data_type()
                                    )
                                ));
                            }
                        }, None => {
                            external_variables.push(ExternalShaderVariable::new(
                                variable.get_name(),
                                variable.get_data_type(),
                                ext,
                            ));
                        }
                    };

                    
                }
                _ => {}
            };
        }
        external_variables.shrink_to_fit();
        Ok(Self {
            vertex_shader: Arc::clone(vertex_shader),
            fragment_shader: Arc::clone(fragment_shader),
            external_variables,
        })
    }

    /// Gets a reference to the *VertexShader* of this *ShaderPair*.
    pub fn get_vertex_shader(&self) -> &Arc<dyn VertexShader> {
        &self.vertex_shader
    }

    /// Gets a reference to the *FragmentShader* of this *ShaderPair*.
    pub fn get_fragment_shader(&self) -> &Arc<dyn FragmentShader> {
        &self.fragment_shader
    }

    /// Gets a reference to the *Vec* holding the collected *External* variables of
    /// the shaders. If you consider the external variables of both shaders as a
    /// set, this will give the union of the two sets.
    pub fn get_external_variables(&self) -> &Vec<ExternalShaderVariable> {
        &self.external_variables
    }
}

/// This error indicates that a *VertexShader* couldn't be linked to a certain
/// *FragmentShader*. The reason why this linking fails could be because the
/// shaders have a *ShaderExternalVariableMismatch*, or because of some other
/// reason that is specific to the way the shaders were attempted to be linked.
#[derive(Debug)]
pub struct ShaderLinkError<E: Error> {
    vertex_name: String,
    fragment_name: String,

    error: ShaderLinkErrorType<E>,
}

/// This enum has the 2 possible classes of reasons two shaders can't be linked 
/// into a *ShaderPair*: 
/// (1) They have mismatching external variables (a *General* Error)
/// (2) An error that is *Specific* to the linking method occurred
#[derive(Debug)]
pub enum ShaderLinkErrorType<E: Error> {

    /// A general linking error occurred. Currently, this can only be a
    /// *ShaderExternalVariableMismatch*
    General(ShaderExternalVariableMismatch),

    /// An error occurred that is specific to the way the shaders were
    /// attempted to be linked. See the documentation of that method for
    /// more information.
    Specific(E)
}

impl<E: Error> Display for ShaderLinkErrorType<E> {

    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::General(general) => general.fmt(f),
            Self::Specific(specific) => Display::fmt(specific, f)
        }
    }
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
    fn specific(
        vertex_shader: &dyn VertexShader,
        fragment_shader: &dyn FragmentShader,
        error: E,
    ) -> Self {
        Self {
            vertex_name: vertex_shader.get_debug_name().to_string(),
            fragment_name: fragment_shader.get_debug_name().to_string(),
            error: ShaderLinkErrorType::Specific(error),
        }
    }

    fn general(
        vertex_shader: &dyn VertexShader,
        fragment_shader: &dyn FragmentShader,
        error: ShaderExternalVariableMismatch,
    ) -> Self {
        Self {
            vertex_name: vertex_shader.get_debug_name().to_string(),
            fragment_name: fragment_shader.get_debug_name().to_string(),
            error: ShaderLinkErrorType::General(error),
        }
    }

    /// Gets the *debug_name* of the *VertexShader* that wasn't linked successfully.
    pub fn get_vertex_name(&self) -> &str {
        &self.vertex_name
    }

    /// Gets the *debug_name* of the *FragmentShader* that wasn't linked successfully.
    pub fn get_fragment_name(&self) -> &str {
        &self.fragment_name
    }

    /// Gets the error that occurred while trying to link the vertex shader to the
    /// fragment shader.
    pub fn get_error(&self) -> &ShaderLinkErrorType<E> {
        &self.error
    }
}

/// This error indicates that an attempt was made to link some vertex shader to some
/// fragment shader by the names of their inputs and outputs, but that this failed.
/// 
/// There are multiple reasons such an attempt could fail, and each option of this
/// enum describes one such reason. See the options of this enum for more 
/// information.
#[derive(Debug)]
pub enum ShaderNameLinkError {
    /// The vertex shader has an output with the same name as an input of the
    /// fragment shader, but they have different types. For instance, the vertex
    /// shader has an output with name "x" and type *float*, but the fragment
    /// shader has an input with name "x" and type *int*.
    TypeMismatch {
        vertex_output: VertexShaderVariable,
        fragment_input: FragmentShaderVariable,
    },
    /// The vertex shader has an output variable (that has a name), but the fragment 
    /// shader doesn't have an input variable with that same name.
    MissingFragmentInput {
        vertex_output_name: String,
    },
    /// The fragment shader has an input variable (that has a name), but the vertex
    /// shader doesn't have an output variable with that same name.
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

/// This error struct indicates that an attempt was made to link a vertex shader
/// to a fragment shader, but this failed because both shaders have an external
/// variable with the same name, but with a different type.
#[derive(Debug)]
pub struct ShaderExternalVariableMismatch {

    name: String,
    vertex_type: DataType,
    fragment_type: DataType
}

impl ShaderExternalVariableMismatch {

    fn new(name: &str, vertex_type: DataType, fragment_type: DataType) -> Self {
        Self { name: name.to_string(), vertex_type, fragment_type }
    }

    /// Gets the name of the conflicting external variables
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets the type of the variable of the vertex shader
    pub fn get_vertex_type(&self) -> DataType {
        self.vertex_type
    }

    /// Gets the type of the variable of the fragment shader
    pub fn get_fragment_type(&self) -> DataType {
        self.fragment_type
    }
}

impl Display for ShaderExternalVariableMismatch {

    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, 
            "Both the vertex and fragment shader have an external variable with name {}, 
            but the data type of the vertex shader is {:?} 
            and the data type of the fragment shader is {:?}", 
            self.name, self.vertex_type, self.fragment_type
        )
    }
}

impl Error for ShaderExternalVariableMismatch {}

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

    /// Links a given *VertexShader* with a given *FragmentShader* by the names of
    /// their output and input variables.
    /// 
    /// Every variable of the vertex shader with type *FragmentOutput* will be
    /// linked with the variable of the fragment shader with type *VertexInput*
    /// that has the same name as that variable of the vertex shader. 
    /// If the data types of the variables differ (for instance *int* vs *float*), 
    /// an error will be returned. 
    /// If the fragment shader doesn't have such a variable, an error will be 
    /// returned.
    /// (And a similar error will be returned if the fragment shader has a
    /// *VertexInput* variable, but the vertex shader doesn't have a matching
    /// *FragmentOutput* variable.)
    /// 
    /// Furthermore, an error will be returned if the vertex and fragment shader
    /// have an external variable with the same name, but with different *DataType*.
    pub fn link_by_attribute_names(
        vertex_shader: &Arc<dyn VertexShader>,
        fragment_shader: &Arc<dyn FragmentShader>,
    ) -> Result<Self, ShaderLinkError<ShaderNameLinkError>> {
        let maybe_flat_type_mismatch = Self::match_shader_variables_types(
            vertex_shader.get_variables(),
            fragment_shader.get_variables(),
            |var| var.get_variable_type() == VertexShaderVariableType::FlatFragmentOutput,
            |var| var.get_variable_type() == FragmentShaderVariableType::FlatVertexInput,
        );
        let maybe_smooth_type_mismatch = Self::match_shader_variables_types(
            vertex_shader.get_variables(),
            fragment_shader.get_variables(),
            |var| var.get_variable_type() == VertexShaderVariableType::SmoothFragmentOutput,
            |var| var.get_variable_type() == FragmentShaderVariableType::SmoothVertexInput,
        );
        if maybe_flat_type_mismatch.is_err() {
            let type_mismatch = maybe_flat_type_mismatch.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::TypeMismatch {
                    vertex_output: type_mismatch.0,
                    fragment_input: type_mismatch.1,
                },
            ));
        }
        if maybe_smooth_type_mismatch.is_err() {
            let type_mismatch = maybe_smooth_type_mismatch.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::TypeMismatch {
                    vertex_output: type_mismatch.0,
                    fragment_input: type_mismatch.1,
                },
            ));
        }

        let flat_vertex_output_names = vertex_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == VertexShaderVariableType::FlatFragmentOutput)
            .map(|var| var.get_name().to_string())
            .collect();
        let smooth_vertex_output_names = vertex_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == VertexShaderVariableType::SmoothFragmentOutput)
            .map(|var| var.get_name().to_string())
            .collect();
        let flat_fragment_input_names = fragment_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == FragmentShaderVariableType::FlatVertexInput)
            .map(|var| var.get_name().to_string())
            .collect();
        let smooth_fragment_input_names = fragment_shader
            .get_variables()
            .into_iter()
            .filter(|var| var.get_variable_type() == FragmentShaderVariableType::SmoothVertexInput)
            .map(|var| var.get_name().to_string())
            .collect();
        let maybe_miss_fragment_flat =
            Self::match_shader_variable_names(&flat_vertex_output_names, &flat_fragment_input_names);
        let maybe_miss_fragment_smooth =
            Self::match_shader_variable_names(&smooth_vertex_output_names, &smooth_fragment_input_names);
        let maybe_miss_vertex_flat =
            Self::match_shader_variable_names(&flat_fragment_input_names, &flat_vertex_output_names);
        let maybe_miss_vertex_smooth =
            Self::match_shader_variable_names(&smooth_fragment_input_names, &smooth_vertex_output_names);

        if maybe_miss_fragment_flat.is_err() {
            let unmatched_vertex_output = maybe_miss_fragment_flat.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingFragmentInput {
                    vertex_output_name: unmatched_vertex_output,
                },
            ));
        }
        if maybe_miss_fragment_smooth.is_err() {
            let unmatched_vertex_output = maybe_miss_fragment_smooth.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingFragmentInput {
                    vertex_output_name: unmatched_vertex_output,
                },
            ));
        }

        if maybe_miss_vertex_flat.is_err() {
            let unmatched_fragment_input = maybe_miss_vertex_flat.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingVertexOutput {
                    fragment_input_name: unmatched_fragment_input,
                },
            ));
        }
        if maybe_miss_vertex_smooth.is_err() {
            let unmatched_fragment_input = maybe_miss_vertex_smooth.unwrap_err();
            return Err(ShaderLinkError::specific(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingVertexOutput {
                    fragment_input_name: unmatched_fragment_input,
                },
            ));
        }

        ShaderPair::new(vertex_shader, fragment_shader)
    }
}
