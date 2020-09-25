use std::any::Any;
use std::fmt::Debug;

mod library;
mod manager;
mod pair;
mod variable;

pub use library::*;
pub use manager::*;
pub use pair::*;
pub use variable::*;

/// Represents a vertex shader (a very well-known part of the graphics pipeline). 
/// 
/// Vertex shaders are useless in isolution, but they can be used to create a
/// *ShaderPair* in combination with a *FragmentShader*. The same
/// *VertexShader* can be used to create multiple *ShaderPair*s.
pub trait VertexShader: Debug + Send + Sync + 'static {
    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;

    /// Gets the debug name that was used to create this vertex shader.
    fn get_debug_name(&self) -> &str;

    /// Gets a *Vec* containing all shader variables of this vertex shader
    /// (inputs, uniform variables, outputs...). These will be the same
    /// as the *variables* passed to the *create_vertex_shader* call that
    /// created this *VertexShader*.
    fn get_variables(&self) -> &Vec<VertexShaderVariable>;
}

// TODO Document once the documentation of *VertexShader* is stable.
pub trait FragmentShader: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;

    fn get_debug_name(&self) -> &str;

    fn get_variables(&self) -> &Vec<FragmentShaderVariable>;
}
