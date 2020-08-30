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

pub trait Shader: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;

    fn get_debug_name(&self) -> &str;

    fn get_variables(&self) -> &ShaderVariables;
}
