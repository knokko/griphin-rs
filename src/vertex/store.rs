use crate::*;

pub struct VertexStore {

    raw_buffer: Vec<u8>,

    current_offset: usize
}

impl VertexStore {

    pub fn new(description: &VertexDescription, vertices: &Vec<impl Vertex>, debug_level: DebugLevel, mut writer: Option<&mut dyn std::io::Write>) -> Self {
        let vertex_size = description.get_size();
        let buffer_size = vertex_size * vertices.len();
        let mut store = Self { raw_buffer: vec![0; buffer_size], current_offset: 0 };

        for vertex in vertices {
            vertex.store(&mut store);
            store.current_offset += vertex_size;
        }

        let log_id = "VertexStore::new";

        if debug_level >= DebugLevel::Basic {
            /*
             * The following test creates a second vertex store, but with a
             * different initial byte value at each position. If the vertices store
             * all their attributes, all values in the stores should be replaced by
             * a (part of) a value of a vertex.
             * But if a vertex forgets to store one of its attributes, the initial
             * value at the position of that attribute will remain the same. Since
             * *store* uses 0 as initial value and *store2* uses 1 as initial value,
             * their buffers will be identical if and only if the vertices overwrite
             * all initial values (which they should).
             */
            let mut store2 = Self { raw_buffer: vec![1; buffer_size], current_offset: 0 };
            for vertex in vertices {
                vertex.store(&mut store2);
                store2.current_offset += vertex_size;
            }

            if store.raw_buffer != store2.raw_buffer {
                log(&mut writer, log_id, 
                    "Not the entire vertex buffer seems to have been filled.\
                    Did you forget to store one of the vertex attributes?"
                );
            }
        }

        if debug_level >= DebugLevel::High {
            /*
             * Every vertex attribute has an *AttributeKind*. This test checks if
             * the value of each attribute of each vertex seems 'reasonable'. (For
             * instance, positions shouldn't be NaN or very big and the length of
             * each normal vector should be approximately 1).
             */
            for vertex_index in 0 .. vertices.len() {
                let vertex_offset = vertex_index * vertex_size;
                for attribute in description.get_attributes() {
                    let offset = attribute.offset + vertex_offset;
                    let num_components = attribute.get_data_type().get_shape().get_size() as usize;
                    let mut float_values = Vec::with_capacity(num_components);
                    let mut int_values = Vec::with_capacity(num_components);
                    for counter in 0 .. num_components {
                        let mut byte_values = [0; 4];
                        for index in 0 .. 4 {
                            byte_values[index] = store.raw_buffer[offset + 4 * counter + index];
                        }
                        float_values.push(f32::from_ne_bytes(byte_values));
                        int_values.push(i32::from_ne_bytes(byte_values));
                    }

                    let data_shape = attribute.get_data_type().get_shape();
                    let data_kind = attribute.get_data_type().get_kind();

                    match attribute.get_kind() {
                        AttributeKind::Position{max} => {
                            if data_kind != FLOAT {
                                log(&mut writer, log_id, "A position vertex attribute is not of type float");
                            }
                            for float_value in float_values {
                                if float_value.is_nan() {
                                    log(&mut writer, log_id, "A vertex position is NaN");
                                } else if float_value > max {
                                    log(&mut writer, log_id, "A vertex position is too large");
                                }
                            }
                        }, AttributeKind::Normal => {
                            if data_kind != FLOAT {
                                log(&mut writer, log_id, "A normal vertex attribute is not of type float");
                            }
                            if data_shape != VEC2 && data_shape != VEC3 {
                                log(&mut writer, log_id, "A normal vertex attribute is not a 2d or 3d vector");
                            } else {
                                let mut length_squared = 0.0;
                                for component in float_values {
                                    length_squared += component * component;
                                }
                                if length_squared.is_nan() {
                                    log(&mut writer, log_id, "A normal vertex is NaN");
                                }
                                if length_squared < 0.95 || length_squared > 1.05 {
                                    log(&mut writer, log_id, "A normal vertex has a length that is not close to 1.0");
                                }
                            }
                        }, AttributeKind::FloatTexCoords => {
                            if data_kind != FLOAT {
                                log(&mut writer, log_id, "A FloatTexCoords attribute is not of type FLOAT");
                            }
                            for coordinate in float_values {
                                if coordinate.is_nan() {
                                    log(&mut writer, log_id, "A float texture coordinate is NaN");
                                } else if coordinate < -0.05 || coordinate > 1.05 {
                                    log(&mut writer, log_id, "A float texture coordinate is not between 0 and 1");
                                }
                            }
                        }, AttributeKind::IntTexCoords{texture_size} => {
                            if data_kind != INT {
                                log(&mut writer, log_id, "An IntTexCoords attribute is not of type INT");
                            }
                            for coordinate in int_values {
                                if coordinate < 0 || coordinate >= texture_size as i32 {
                                    log(&mut writer, log_id, "An IntTexCoords component is negative or not smaller than the texture size");
                                }
                            }
                        }, AttributeKind::Index{bound} => {
                            if data_kind != INT {
                                log(&mut writer, log_id, "An Index attribute is not of type INT");
                            }
                            for index in int_values {
                                if index < 0 || index >= bound as i32 {
                                    log(&mut writer, log_id, "An index attribute is out of range");
                                }
                            }
                        }, AttributeKind::Other => { /* I can't check values for unknown data kinds */ }
                    };
                }
            }

            /*
             * This test checks if more than half of the vertices has the same
             * position. Even though this should not easily occur, it is very nasty
             * to debug because all vertices will be mapped to the same screen
             * position, making the entire scene completely invisible.
             */
            for attribute in description.get_attributes() {
                match attribute.get_kind() {
                    AttributeKind::Position{max: _} => {
                        let num_components = attribute.get_data_type().get_shape().get_size() as usize;
                        let mut positions = Vec::new();
                        for vertex_index in 0 .. vertices.len() {
                            let vertex_offset = vertex_index * vertex_size;
                            let position_offset = vertex_offset + attribute.offset;
                            let mut position = Vec::with_capacity(num_components);
                            for component_index in 0 .. num_components {
                                let mut component_bytes = [0; 4];
                                for byte_index in 0 .. 4 {
                                    component_bytes[byte_index] = store.raw_buffer[position_offset + 4 * component_index + byte_index];
                                }
                                position.push(f32::from_ne_bytes(component_bytes));
                            }
                            positions.push(position);
                        }

                        let mut many_equals = false;
                        for position in &positions {
                            let mut equal_counter = 0;
                            for other_position in &positions {
                                if position == other_position {
                                    equal_counter += 1;
                                }
                            }
                            if equal_counter > vertices.len() / 2 {
                                many_equals = true;
                            }
                        }

                        if many_equals {
                            log(&mut writer, log_id, "More than half of the vertices has the same position");
                        }
                    }, _ => {}
                }
            }
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