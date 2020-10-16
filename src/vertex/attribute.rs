use crate::*;

#[derive(Debug)]
pub struct VertexAttribute {

    name: String,
    data_type: DataType,
    offset: usize
}

impl VertexAttribute {

    pub(super) fn new(name: String, data_type: DataType, offset: usize) -> Self {
        Self { name, data_type, offset }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttributeID {

    pub(super) offset: usize
}