use crate::*;

#[derive(Debug)]
pub struct RawVertexDescription {

    attributes: Vec<VertexAttribute>,
    current_offset: usize
}

impl RawVertexDescription {

    pub fn new() -> Self {
        Self { attributes: Vec::new(), current_offset: 0 }
    }

    pub fn add_attribute(&mut self, name: &StringRef, data_type: DataType, kind: AttributeKind) -> VertexAttributeHandle {
        let old_offset = self.current_offset;
        let attribute = VertexAttribute::new(name, data_type, kind, old_offset);
        self.current_offset += (data_type.get_shape().get_size() as usize) * 4;
        self.attributes.push(attribute);
        VertexAttributeHandle { offset: old_offset }
    }

    /// Gets the size of vertices described by this *VertexDescription*, in bytes.
    pub fn get_size(&self) -> usize {
        self.current_offset
    }

    pub fn get_attributes(&self) -> &Vec<VertexAttribute> {
        &self.attributes
    }
}

pub trait VertexDescription {

    fn get_raw_description(&self) -> &RawVertexDescription;
}