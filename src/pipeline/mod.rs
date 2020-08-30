use crate::*;

use std::any::Any;

mod topology;

pub use topology::*;

pub trait GraphicsPipeline {
    fn as_any(&self) -> &dyn Any;
}
