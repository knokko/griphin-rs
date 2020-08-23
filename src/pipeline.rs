use crate::*;

use std::any::Any;

pub trait GraphicsPipeline {

    fn as_any(&self) -> &dyn Any;
}

pub enum PrimitiveTopology {

    Points,
    Lines,
    LineStrips{primitive_restart: bool},
    Triangles,
    TriangleStrips{primitive_restart: bool}
}