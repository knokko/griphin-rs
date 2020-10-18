use crate::*;

#[derive(Debug)]
pub struct VertexAttribute {

    name: String,
    data_type: DataType,
    kind: AttributeKind,
    pub(super) offset: usize
}

impl VertexAttribute {

    pub(super) fn new(name: String, data_type: DataType, kind: AttributeKind, offset: usize) -> Self {
        Self { name, data_type, kind, offset }
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    pub fn get_kind(&self) -> AttributeKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttributeID {

    pub(super) offset: usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeKind {

    Position{max: f32},
    Normal,
    FloatTexCoords,
    IntTexCoords{texture_size: u32},
    Index{bound: u32},
    Other
}