use crate::*;

#[derive(Debug)]
pub struct VertexDescription {

    attributes: Vec<VertexAttribute>,
    current_offset: usize
}

impl VertexDescription {

    pub fn new() -> Self {
        Self { attributes: Vec::new(), current_offset: 0 }
    }

    pub fn add_attribute(&mut self, name: &str, data_type: DataType, kind: AttributeKind) -> VertexAttributeID {
        let old_offset = self.current_offset;
        let attribute = VertexAttribute::new(name.to_string(), data_type, kind, old_offset);
        self.current_offset += (data_type.get_shape().get_size() as usize) * 4;
        self.attributes.push(attribute);
        VertexAttributeID { offset: old_offset }
    }

    /// Gets the size of vertices described by this *VertexDescription*, in bytes.
    pub fn get_size(&self) -> usize {
        self.current_offset
    }

    pub fn get_attributes(&self) -> &Vec<VertexAttribute> {
        &self.attributes
    }
}