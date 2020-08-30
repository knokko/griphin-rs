use crate::*;

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