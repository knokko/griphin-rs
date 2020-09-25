use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// Every Griphin *Instance* has a *ShaderManager* that can be accessed using
/// its *get_shader_manager* method. The shader manager is the entrypoint from
/// where you can create (objects related to) shaders.
pub trait ShaderManager: Debug + Send + Sync + 'static {

    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;

    /// Creates a new *VertexShader* with the given functions, variables, and
    /// libraries.
    /// 
    /// The *main_function_body* and *variables* are the most important parameters
    /// (and properties) for a vertex shader. Every (vertex) shader has a main
    /// function with no parameters that doesn't return anything. Because the
    /// *headers* of that function will be the same for any shader, you only need
    /// to specify its *body* as parameter. This body should be written in GLSL
    /// and you should note that Rust has excellent support for multi-line strings!
    /// 
    /// The *variables* determine which inputs, outputs, uniforms... the shader
    /// has. These variables will be available in the main function body and the
    /// other functions of this shader. The Griphin implementation will make sure
    /// that all these variables will appear at the top of the final shader code.
    /// 
    /// The *other_functions* parameter can be used to add more functions than
    /// just the main function. These functions can be used as helper functions
    /// by the main function. You can write all these functions as (multi-line)
    /// string and pass them to this parameter. Unlike the *main_function_body*,
    /// you also need to write the headers of these functions. If you don't need
    /// any other functions, you can just pass an empty string.
    /// 
    /// The *libraries* can be used to let this shader depend on given
    /// *ShaderLibrary*s. The main function and other functions will be able to
    /// call any functions defined in any of the shader libraries.
    /// 
    /// Finally, there is the *debug_name* parameter. This name will be used by
    /// the Griphin implementation when reporting warnings or errors about the
    /// shader. You are encouraged to give every vertex shader a distinct and
    /// intuitive debug name.
    /// 
    /// ## Example
    /// ```
    /// use griphin::*;
    /// use std::sync::Arc;
    /// 
    /// fn create_simple_vertex_shader(manager: &dyn ShaderManager) -> Arc<dyn VertexShader> {
    ///     let main_function_body = "
    /// outputPosition = finalMatrix() * vec4(position, 1.0);
    /// ";
    ///     let other_functions = "
    /// mat4 finalMatrix() {
    ///     return projectionMatrix * viewMatrix * transformationMatrix;
    /// }
    /// ";
    ///     let variables = vec![
    ///         VertexShaderVariable::new(
    ///             "position", 
    ///             DataType::new(FLOAT, VEC3), 
    ///             VertexShaderVariableType::ModelInput
    ///         ),
    ///         VertexShaderVariable::new(
    ///             "outputPosition",
    ///             DataType::new(FLOAT, VEC4),
    ///             VertexShaderVariableType::FragmentOutput
    ///         ),
    ///         VertexShaderVariable::new(
    ///             "projectionMatrix",
    ///             DataType::new(FLOAT, MAT4),
    ///             VertexShaderVariableType::External(
    ///                 ExternalShaderVariableType::UniformInput
    ///             )
    ///         ),
    ///         VertexShaderVariable::new(
    ///             "viewMatrix",
    ///             DataType::new(FLOAT, MAT4),
    ///             VertexShaderVariableType::External(
    ///                 ExternalShaderVariableType::UniformInput
    ///             )
    ///         ),
    ///         VertexShaderVariable::new(
    ///             "transformationMatrix",
    ///             DataType::new(FLOAT, MAT4),
    ///             VertexShaderVariableType::External(
    ///                 ExternalShaderVariableType::UniformInput
    ///             )
    ///         )
    ///     ];
    /// 
    ///     let result = manager.create_vertex_shader(
    ///         "example", 
    ///         main_function_body, 
    ///         other_functions, 
    ///         variables, 
    ///         Vec::new()
    ///     );
    /// 
    ///     result
    /// }
    /// ```
    fn create_vertex_shader(
        &self,
        debug_name: &str,
        main_function_body: &str,
        other_functions: &str,
        variables: Vec<VertexShaderVariable>,
        libraries: Vec<Arc<dyn ShaderLibrary>>,
    ) -> Arc<dyn VertexShader>;

    // TODO Document when it is more stable
    fn create_fragment_shader(
        &self, 
        debug_name: &str,
        main_function: &str,
        other_functions: &str,
        variables: Vec<FragmentShaderVariable>,
        libraries: Vec<Arc<dyn ShaderLibrary>>
    ) -> Arc<dyn FragmentShader>;

    /// Creates a new *ShaderLibrary* with the given debug name and functions.
    /// 
    /// The *debug_name* is just a string that the Griphin implementations will
    /// use when reporting warnings or errors about this library.
    /// 
    /// The *functions* is the important part of this library. This should be
    /// a multi-line string containing 1 or more GLSL functions (both headers
    /// and body). The shaders relying on the library will be able to call
    /// these functions.
    /// 
    /// ## Example
    /// ```
    /// use griphin::*;
    /// use std::sync::Arc;
    /// 
    /// fn create_example_library(manager: &dyn ShaderManager) -> Arc<dyn ShaderLibrary> {
    ///     let functions = "
    /// vec3 interpolate(float alpha, vec3 a, vec3 b) {
    ///     return alpha * a + (1.0 - alpha) * b;
    /// } 
    /// ";
    ///     let result = manager.create_library("example", functions);
    /// 
    ///     result
    /// }
    /// ```
    fn create_library(&self, debug_name: &str, functions: &str) -> Arc<dyn ShaderLibrary>;
}
