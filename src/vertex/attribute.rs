use crate::*;

/// Represents an attribute of a *VertexDescription*. A *VertexAttribute* has a
/// name, *DataType*, and *AttributeKind*. It also has an offset, but that is
/// only for internal use.
/// 
/// To get an instance of this struct, use the *add_attribute* method of
/// an instance of *VertexDescription*. You should store every *VertexAttribute*
/// you create, because the implementation of *Vertex* corresponding to the
/// *VertexDescription* will need it to implement its *store* method.
#[derive(Debug)]
pub struct VertexAttribute {

    name: StringRef,
    data_type: DataType,
    kind: AttributeKind,
    pub(super) offset: usize
}

impl VertexAttribute {

    pub(super) fn new(name: &StringRef, data_type: DataType, kind: AttributeKind, offset: usize) -> Self {
        Self { name: name.clone(), data_type, kind, offset }
    }

    pub fn get_name(&self) -> &StringRef {
        &self.name
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