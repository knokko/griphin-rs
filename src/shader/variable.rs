use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexShaderVariableType {
    ModelInput,
    External(ExternalShaderVariableType),
    FragmentOutput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FragmentShaderVariableType {
    VertexInput,
    External(ExternalShaderVariableType),
    ColorOutput,
    DepthStencilOutput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalShaderVariableType {
    UniformInput,
    InstancedInput,
    TextureInput,
    ColorGridInput,
    DepthStencilGridInput,
}

#[derive(Clone, Debug)]
pub struct VertexShaderVariable {
    name: String,
    data_type: DataType,
    variable_type: VertexShaderVariableType,
}

impl VertexShaderVariable {
    pub fn new(name: &str, data_type: DataType, variable_type: VertexShaderVariableType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            variable_type,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    pub fn get_variable_type(&self) -> VertexShaderVariableType {
        self.variable_type
    }
}

#[derive(Clone, Debug)]
pub struct FragmentShaderVariable {
    name: String,
    data_type: DataType,
    variable_type: FragmentShaderVariableType,
}

impl FragmentShaderVariable {
    pub fn new(
        name: String,
        data_type: DataType,
        variable_type: FragmentShaderVariableType,
    ) -> Self {
        Self {
            name,
            data_type,
            variable_type,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    pub fn get_variable_type(&self) -> FragmentShaderVariableType {
        self.variable_type
    }
}

#[derive(Clone, Debug)]
pub struct ExternalShaderVariable {
    name: String,
    data_type: DataType,
    variable_type: ExternalShaderVariableType,
}

impl ExternalShaderVariable {
    pub fn new(
        name: String,
        data_type: DataType,
        variable_type: ExternalShaderVariableType,
    ) -> Self {
        Self {
            name,
            data_type,
            variable_type,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    pub fn get_variable_type(&self) -> ExternalShaderVariableType {
        self.variable_type
    }
}
