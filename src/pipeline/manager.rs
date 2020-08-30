use crate::*;

use std::any::Any;

pub trait PipelineManager {

    fn as_any(&self) -> &dyn Any;

    fn create_color_attachment(&self);
}