use crate::*;

/// The *variable* type of a variable of a vertex shader. This type indicates
/// the purpose of a variable (is it an input? or an output? and where should
/// the value of the variable come from or go to?).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexShaderVariableType {

    /// This variable is an input variable, and it will get its value from the
    /// per-vertex data of the model. 
    VertexInput,

    /// This variable is an input variable, and it will gets its value from the
    /// per-instance data of the model.
    InstancedInput,

    /// This variable is an input variable, and its value will be supplied
    /// 'externally' (for instance a uniform variable).
    External(ExternalShaderVariableType),

    /// This variable is an output variable, and its value will be passed on
    /// to the fragment shader. The value the fragment shader will receive
    /// will be a smooth (interpolated) mix of the output values of each of
    /// the vertices the fragment belongs to. The closer each vertex is to
    /// the fragment, the closer the mixed value will be to the value of
    /// that vertex.
    SmoothFragmentOutput,

    /// This variable is an output variable, and its value will be passed on
    /// to the fragment shader. The value the fragment shader will receive will
    /// be the output value of the *provoking* vertex (usually the first vertex
    /// of the line or triangle). The output values of the other vertices will
    /// be ignored.
    FlatFragmentOutput
}

/// The *variable* type of a variable of a fragment shader. This type indicates
/// the purpose of a variable (is it an input? or an output? and where should
/// the value of the variable come from or go to?).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FragmentShaderVariableType {

    /// This variable is an input variable, and its value will come from a
    /// corresponding output variable of the vertex shader. It will be a mix
    /// of the output values of the responsible vertices for the fragment.
    SmoothVertexInput,

    /// This variable is an input variable, and its value will come from a
    /// corresponding output variable of the vertex shader. It will be the
    /// output value of 1 of the responsible vertices for the fragment (and
    /// it will ignore the other responsible vertices).
    FlatVertexInput,

    /// This variable is an input variable, and its value will be supplied
    /// 'externally' (for instance a uniform variable).
    External(ExternalShaderVariableType),

    /// This variable is an output variable, and its value will be written onto
    /// a color grid.
    ColorOutput,

    /// This variable is an output variable, and its value will be written onto
    /// a depth stencil grid.
    DepthStencilOutput,
}

/// Represents a *variable* type of a shader variable (can be either a vertex shader
/// or a fragment shader) that comes from an 'external' source (so not from the
/// models vertex data or from another shader). See the options of this enum for
/// more information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalShaderVariableType {

    /// This variable is an input variable, and the value can be set right before
    /// drawing. This value is global and thus shared for all vertices and fragments
    /// that are drawn at the same time.
    UniformInput,

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
