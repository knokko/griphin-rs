use crate::*;

use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

pub trait ShaderManager: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;

    fn create_shader(&self, debug_name: &str, main_function: &str, variables: ShaderVariables, 
        libraries: Vec<Arc<dyn ShaderLibrary>>) -> Arc<dyn Shader>;

    fn create_library(&self, debug_name: &str, functions: &str) -> Arc<dyn ShaderLibrary>;
}

pub trait Shader: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;

    fn get_debug_name(&self) -> &str;

    fn get_variables(&self) -> &ShaderVariables;
}

pub trait ShaderLibrary: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;

    fn make_global(&self);
}

#[derive(Clone, Debug)]
pub struct ShaderVariable {

    name: String,
    data_type: DataType
}

impl ShaderVariable {

    pub fn new(name: String, data_type: DataType) -> ShaderVariable {
        ShaderVariable {name, data_type}
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> DataType {
        self.data_type
    }
}

pub struct ShaderVariables {

    inputs: Vec<ShaderVariable>,
    instanced_inputs: Vec<ShaderVariable>,
    outputs: Vec<ShaderVariable>,
    uniforms: Vec<ShaderVariable>
}

impl ShaderVariables {

    pub fn new<I>(inputs: I, instanced_inputs: I, outputs: I, uniforms: I) -> ShaderVariables 
    where I: IntoIterator<Item=ShaderVariable>, {
        ShaderVariables {
            inputs: inputs.into_iter().collect(),
            instanced_inputs: instanced_inputs.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
            uniforms: uniforms.into_iter().collect()
        }
    }

    pub fn get_inputs(&self) -> &Vec<ShaderVariable> {
        &self.inputs
    }

    pub fn get_instanced_inputs(&self) -> &Vec<ShaderVariable> {
        &self.instanced_inputs
    }

    pub fn get_outputs(&self) -> &Vec<ShaderVariable> {
        &self.outputs
    }

    pub fn get_uniforms(&self) -> &Vec<ShaderVariable> {
        &self.uniforms
    }
}

pub struct ShaderPair {

    vertex_shader: Arc<dyn Shader>,
    fragment_shader: Arc<dyn Shader>
}

#[derive(Debug)]
pub struct ShaderLinkError<E: Error> {

    vertex_name: String,
    fragment_name: String,

    error: E
}

impl<E: Error> Display for ShaderLinkError<E> {

    fn fmt(&self, f:&mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "An error occurred while linking {} to {}: {}", self.vertex_name, self.fragment_name, self.error)
    }
}

impl<E: Error> Error for ShaderLinkError<E> {}

impl<E: Error> ShaderLinkError<E> {

    pub fn new(vertex_shader: &dyn Shader, fragment_shader: &dyn Shader, error: E) -> Self {
        Self {
            vertex_name: vertex_shader.get_debug_name().to_string(),
            fragment_name: fragment_shader.get_debug_name().to_string(),
            error
        }
    }
}
#[derive(Debug)]
pub enum ShaderNameLinkError {

    TypeMismatch{vertex_output: ShaderVariable, fragment_input: ShaderVariable},
    MissingFragmentInput{vertex_output_name: String},
    MissingVertexOutput{fragment_input_name: String}
}

impl Display for ShaderNameLinkError {

    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::TypeMismatch{vertex_output, fragment_input} => 
                    write!(f, 
                        "Type mismatch: The vertex shader has output {:?} and the fragment shader has input {:?}", 
                        vertex_output, fragment_input
                    ),
            Self::MissingFragmentInput{vertex_output_name} => write!(f, 
                "The vertex shader has output {:?}, which doesn't match any of the inputs of the fragment shader",
                vertex_output_name
            ),
            Self::MissingVertexOutput{fragment_input_name} => write!(f,
                "The fragment shader has input {:?}, which doesn't match any of the outputs of the vertex shader",
                fragment_input_name
            )
        }
    }
}

impl Error for ShaderNameLinkError {}

impl ShaderPair {

    fn match_shader_variables_types(variables1: &Vec<ShaderVariable>, variables2: &Vec<ShaderVariable>) -> Result<(), (ShaderVariable, ShaderVariable)> {
        for var1 in variables1 {
            for var2 in variables2 {
                if var1.get_name() == var2.get_name() && var1.get_type() != var2.get_type() {
                    return Err((var1.clone(), var2.clone()));
                }
            }
        }

        Ok(())
    }

    fn match_shader_variable_names(required_names: &Vec<String>, available_names: &Vec<String>) -> Result<(), String> {

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

    fn extract_names(variables: &Vec<ShaderVariable>) -> Vec<String> {
        variables.into_iter().map(|variable| variable.get_name().to_string()).collect()
    }

    pub fn link_by_attribute_names(vertex_shader: &Arc<dyn Shader>, fragment_shader: &Arc<dyn Shader>) -> Result<Self, ShaderLinkError<ShaderNameLinkError>> {

        let maybe_type_mismatch = Self::match_shader_variables_types(
            vertex_shader.get_variables().get_outputs(),
            fragment_shader.get_variables().get_inputs()
        );
        if maybe_type_mismatch.is_err() {
            let type_mismatch = maybe_type_mismatch.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(), 
                fragment_shader.as_ref(), 
                ShaderNameLinkError::TypeMismatch{
                    vertex_output: type_mismatch.0, 
                    fragment_input: type_mismatch.1
                }
            ));
        }

        let vertex_output_names = Self::extract_names(vertex_shader.get_variables().get_outputs());
        let fragment_input_names = Self::extract_names(fragment_shader.get_variables().get_inputs());

        let maybe_miss_fragment = Self::match_shader_variable_names(&vertex_output_names, &fragment_input_names);
        let maybe_miss_vertex = Self::match_shader_variable_names(&fragment_input_names, &vertex_output_names);

        if maybe_miss_fragment.is_err() {
            let unmatched_vertex_output = maybe_miss_fragment.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingFragmentInput{vertex_output_name: unmatched_vertex_output}
            ));
        }

        if maybe_miss_vertex.is_err() {
            let unmatched_fragment_input = maybe_miss_vertex.unwrap_err();
            return Err(ShaderLinkError::new(
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
                ShaderNameLinkError::MissingVertexOutput{fragment_input_name: unmatched_fragment_input}
            ));
        }

        Ok(ShaderPair {
            vertex_shader: Arc::clone(vertex_shader), 
            fragment_shader: Arc::clone(fragment_shader)
        })
    }
}