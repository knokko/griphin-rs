use crate::*;

use std::any::Any;

pub trait Framebuffer {

    fn as_any(&self) -> &dyn Any;

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;
}