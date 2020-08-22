use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait ShaderManager: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;

    fn create_shader(&self, main_function: &str, libraries: Vec<Arc<dyn ShaderLibrary>>) -> Arc<dyn Shader>;

    fn create_library(&self, functions: &str) -> Arc<dyn ShaderLibrary>;
}

pub trait Shader: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;
}

pub trait ShaderLibrary: Debug + Send + Sync + 'static {

    fn as_any(&self) -> &dyn Any;

    fn make_global(&self);
}

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
    outputs: Vec<ShaderVariable>,
    uniforms: Vec<ShaderVariable>
}

impl ShaderVariables {

    pub fn new<I>(inputs: I, outputs: I, uniforms: I) -> ShaderVariables 
    where I: IntoIterator<Item=ShaderVariable>, {
        ShaderVariables {
            inputs: inputs.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
            uniforms: uniforms.into_iter().collect()
        }
    }
}