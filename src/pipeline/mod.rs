use crate::*;

use std::any::Any;

mod builder;
mod topology;

pub use builder::*;
pub use topology::*;

pub trait GraphicsPipeline {
    fn as_any(&self) -> &dyn Any;

    fn get_external_variables(&self) -> &Vec<ExternalShaderVariable>;
}
