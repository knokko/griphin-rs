use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait ShaderManager: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;

    fn create_shader(
        &self,
        debug_name: &str,
        main_function: &str,
        variables: ShaderVariables,
        libraries: Vec<Arc<dyn ShaderLibrary>>,
    ) -> Arc<dyn Shader>;

    fn create_library(&self, debug_name: &str, functions: &str) -> Arc<dyn ShaderLibrary>;
}
