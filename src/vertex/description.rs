use crate::*;

/// Instances of this struct are used to describe the attributes of a list of vertices, and to
/// manage the offsets of these attributes. You need an instance of this struct each time you want
/// to create a new *VertexStore* (which is the only way to send vertex data through the Gateway).
///
/// To construct an instance of this struct, you can call the *new* function. Then, you should call
/// the *add_attribute* method for each attribute the vertices will have.
///
/// Usually, you should create a more specific and friendlier struct that implements
/// *VertexDescription* and has the *RawVertexDescription* as one of its fields. See the
/// documentation of *Vertex* for examples on how to do this.
#[derive(Debug)]
pub struct RawVertexDescription {

    attributes: Vec<VertexAttribute>,
    current_offset: usize
}

impl RawVertexDescription {

    /// Creates a new and empty *RawVertexDescription*. You should probably add some attributes
    /// using the *add_attribute* method.
    pub fn new() -> Self {
        Self { attributes: Vec::new(), current_offset: 0 }
    }

    /// Adds a new attribute to this vertex description, and returns a handle to it. You need to
    /// pass its *name*, *DataType*, and *AttributeKind* as parameters. See the documentation of
    /// *DataType* and *AttributeKind* for more information about these types.
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

    /// Gets a reference to the attribute vector of this description. The attribute at index 0 will
    /// be the first attribute added to this description. The attribute at index 1 will be the
    /// second attribute...
    pub fn get_attributes(&self) -> &Vec<VertexAttribute> {
        &self.attributes
    }
}

/// This trait should be implemented by structs that wrap a *RawVertexDescription*. Such structs
/// are needed to construct instances of *VertexStore*. This is a very simple trait with just a
/// single method that should give a reference to the wrapped *RawVertexDescription*.
///
/// The documentation of *Vertex* has examples that show how to use this trait.
pub trait VertexDescription {

    /// Gets a reference to the *RawVertexDescription* wrapped by this *VertexDescription*
    fn get_raw_description(&self) -> &RawVertexDescription;
}