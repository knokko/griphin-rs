use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderVariableType {

    Input,
    UniformInput,
    InstancedInput,
    TextureInput,
    ColorGridInput,
    DepthStencilGridInput,
    Output
}

#[derive(Clone, Debug)]
pub struct ShaderVariable {
    name: String,
    data_type: DataType,
    variable_type: ShaderVariableType
}

impl ShaderVariable {
    pub fn new(name: String, data_type: DataType, variable_type: ShaderVariableType) -> ShaderVariable {
        ShaderVariable { name, data_type, variable_type }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    pub fn get_variable_type(&self) -> ShaderVariableType {
        self.variable_type
    }
}
