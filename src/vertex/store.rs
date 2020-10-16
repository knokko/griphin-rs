use crate::*;

pub struct VertexStore {

    raw_buffer: Vec<u8>,

    current_offset: usize
}

impl VertexStore {

    pub fn new(description: &VertexDescription, vertices: &Vec<impl Vertex>) -> Self {
        let vertex_size = description.get_size();
        let buffer_size = vertex_size * vertices.len();
        let mut store = Self { raw_buffer: vec![0; buffer_size], current_offset: 0 };

        for vertex in vertices {
            vertex.store(&mut store);
            store.current_offset += vertex_size;
        }

        store
    }

    pub fn put_int(&mut self, attribute: VertexAttributeID, value: i32) {
        let as_bytes = value.to_ne_bytes();
        let offset = self.current_offset + attribute.offset;
        for index in 0 .. 4 {
            self.raw_buffer[offset + index] = as_bytes[index];
        }
    }

    pub fn put_float(&mut self, attribute: VertexAttributeID, value: f32) {
        let as_bytes = value.to_ne_bytes();
        let offset = self.current_offset + attribute.offset;
        for index in 0 .. 4 {
            self.raw_buffer[offset + index] = as_bytes[index];
        }
    }

    pub fn put_bool(&mut self, attribute: VertexAttributeID, value: bool) {
        // I think this is how booleans are handled on the GPU, but I'm not sure.
        // I never actually tried this x)
        if value {
            self.put_int(attribute, 1);
        } else {
            self.put_int(attribute, 0);
        }
    }
}