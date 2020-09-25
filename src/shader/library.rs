use std::any::Any;
use std::fmt::Debug;

/// *ShaderLibrary*s make it possible to reuse code in shaders. Every
/// *ShaderLibrary* consists of 1 or more shader language functions and every
/// shader that uses a library, can use its functions.
/// 
/// So you can put code that is needed by multiple shaders in a *ShaderLibrary*
/// and let the shaders that need it depend on that library. (So you don't need
/// to copy these functions to all shaders that need it.)
/// 
/// To create a *ShaderLibrary*, use the *create_library* method of the
/// *ShaderManager*.
/// 
/// To let a shader depend on a library, put the library in the *libraries*
/// parameter in the *create_vertex_shader* or *create_fragment_shader* method
/// of the *ShaderManager*.
/// 
/// Note that shader libraries will help you avoid code duplication, but may or
/// may not help with avoiding binary duplication (Griphin implementations will
/// be allowed to for instance just concatenate the library code to the code of
/// each shader that depends on it).
pub trait ShaderLibrary: Debug + Send + Sync + 'static {

    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;

    /// Makes this library *global*. This will cause all *new* shaders to
    /// depend on this library, regardless of whether or not this library
    /// was passed to the *libraries* parameter when creating that shader.
    /// 
    /// This will *not* affect shaders that have been created already!
    fn make_global(&self);
}
