use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait ShaderManager: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;

    fn create_vertex_shader(
        &self,
        debug_name: &str,
        main_function: &str,
        other_functions: &str,
        variables: Vec<VertexShaderVariable>,
        libraries: Vec<Arc<dyn ShaderLibrary>>,
    ) -> Arc<dyn VertexShader>;

    fn create_library(&self, debug_name: &str, functions: &str) -> Arc<dyn ShaderLibrary>;
}
