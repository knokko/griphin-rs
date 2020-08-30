use crate::*;

use std::any::Any;

mod attachment;
mod manager;
mod topology;

pub use attachment::*;
pub use manager::*;
pub use topology::*;

pub trait GraphicsPipeline {

    fn as_any(&self) -> &dyn Any;
}

