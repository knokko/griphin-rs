use std::any::Any;
use std::fmt::Debug;

pub trait ShaderLibrary: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;

    fn make_global(&self);
}
