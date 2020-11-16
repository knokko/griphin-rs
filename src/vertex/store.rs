use crate::*;
use cgmath::*;

use std::io::Write;

/// Represents a byte buffer (or more accurately *Vec\<u8\>*) that is filled with vertex data
/// (positions, normal vectors...) and ready to be sent through the *Gateway*.
///
/// Use the *new* function of this struct to create instances of this struct. See the documentation
/// of *Vertex* for concrete examples.
pub struct VertexStore {
    raw_buffer: Vec<u8>,
}

impl VertexStore {
    /// Constructs a new *VertexStore* and fills it with the data of the given *vertices*. The
    /// returned store will be filled completely once this function returns (so it is impossible to
    /// get any *VertexStore* that is not filled). See the documentation of *Vertex* for examples.
    ///
    /// If *debug_level* is at least *Basic*, this function will also check if all vertex attributes
    /// are actually written. If *debug_level* is at least *High*, this function will do extensive
    /// checks on the attribute values that the vertices wrote to the store. See the documentation
    /// of *AttributeKind* for more information about these checks. Note that these checks can be
    /// really expensive when a lot of vertices are stored! So when you have a lot of vertices, you
    /// should keep the debug level low, unless something is not working and you would like to
    /// check the vertex data.
    ///
    /// The *writer* parameter determines where all debug messages will be printed. If it is *None*,
    /// they will be written to the standard output (this is usually what you want). If it is not
    /// *None*, it will be written to the *writer* itself (this is mostly convenient for the unit
    /// tests of this module).
    pub fn new<D: VertexDescription>(
        description: &D,
        vertices: &[impl Vertex<D>],
        debug_level: DebugLevel,
        mut writer: Option<&mut dyn Write>,
    ) -> Self {

        let log_id = "VertexStore::new";

        // Check if there is at least 1 vertex and at least 1 attribute
        if debug_level >= DebugLevel::Low {
            if vertices.is_empty() {
                log(&mut writer, log_id, "You passed an empty slice of vertices.");
            }
            if description.get_raw_description().get_attributes().is_empty() {
                log(&mut writer, log_id, "The vertex description doesn't have any attributes.");
            }
        }
        let vertex_size = description.get_raw_description().get_size();
        let buffer_size = vertex_size * vertices.len();
        let mut store_builder = VertexStoreBuilder {
            raw_buffer: vec![0; buffer_size],
            current_offset: 0,
        };

        for vertex in vertices {
            vertex.store(&mut store_builder, description);
            store_builder.current_offset += vertex_size;
        }

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
            let mut store_builder2 = VertexStoreBuilder {
                raw_buffer: vec![1; buffer_size],
                current_offset: 0,
            };
            for vertex in vertices {
                vertex.store(&mut store_builder2, description);
                store_builder2.current_offset += vertex_size;
            }

            if store_builder.raw_buffer != store_builder2.raw_buffer {
                log(
                    &mut writer,
                    log_id,
                    "Not the entire vertex buffer seems to have been filled.\
                    Did you forget to store one of the vertex attributes?",
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
            for vertex_index in 0..vertices.len() {
                let vertex_offset = vertex_index * vertex_size;
                for attribute in description.get_raw_description().get_attributes() {
                    let offset = attribute.offset + vertex_offset;
                    let num_components = attribute.get_data_type().get_shape().get_size() as usize;
                    let mut float_values = Vec::with_capacity(num_components);
                    let mut int_values = Vec::with_capacity(num_components);
                    for counter in 0..num_components {
                        let mut byte_values = [0; 4];
                        for index in 0..4 {
                            byte_values[index] =
                                store_builder.raw_buffer[offset + 4 * counter + index];
                        }
                        float_values.push(f32::from_ne_bytes(byte_values));
                        int_values.push(i32::from_ne_bytes(byte_values));
                    }

                    let data_shape = attribute.get_data_type().get_shape();
                    let data_kind = attribute.get_data_type().get_kind();

                    match attribute.get_kind() {
                        AttributeKind::Position { max } => {
                            if data_kind != FLOAT {
                                log(
                                    &mut writer,
                                    log_id,
                                    "A position vertex attribute is not of type float",
                                );
                            }
                            for float_value in float_values {
                                if float_value.is_nan() {
                                    log(&mut writer, log_id, "A vertex position component is NaN");
                                } else if float_value > max {
                                    log(&mut writer, log_id, "A vertex position component is too large");
                                } else if float_value < -max {
                                    log(&mut writer, log_id, "A vertex position component is too small");
                                }
                            }
                        }
                        AttributeKind::Normal => {
                            if data_kind != FLOAT {
                                log(
                                    &mut writer,
                                    log_id,
                                    "A normal vertex attribute is not of type float",
                                );
                            }
                            if data_shape != VEC2 && data_shape != VEC3 {
                                log(
                                    &mut writer,
                                    log_id,
                                    "A normal vertex attribute is not a 2d or 3d vector",
                                );
                            } else {
                                let mut length_squared = 0.0;
                                for component in float_values {
                                    length_squared += component * component;
                                }
                                if length_squared.is_nan() {
                                    log(&mut writer, log_id, "A normal vertex component is NaN");
                                }
                                if length_squared > 1.05 {
                                    log(&mut writer, log_id,
                                        "A normal vertex has a length that is larger than 1.05");
                                }
                                if length_squared < 0.95 {
                                    log(
                                        &mut writer, log_id,
                                        "A normal vertex has a length that is smaller than 0.95"
                                    );
                                }
                            }
                        }
                        AttributeKind::FloatTexCoords => {
                            if data_kind != FLOAT {
                                log(
                                    &mut writer,
                                    log_id,
                                    "A FloatTexCoords attribute is not of type FLOAT",
                                );
                            }
                            for coordinate in float_values {
                                if coordinate.is_nan() {
                                    log(&mut writer, log_id, "A float texture coordinate is NaN");
                                } else if coordinate < -0.05 || coordinate > 1.05 {
                                    log(
                                        &mut writer,
                                        log_id,
                                        "A float texture coordinate is not between 0 and 1",
                                    );
                                }
                            }
                        }
                        AttributeKind::IntTexCoords { texture_size } => {
                            if data_kind != INT {
                                log(
                                    &mut writer,
                                    log_id,
                                    "An IntTexCoords attribute is not of type INT",
                                );
                            }
                            for coordinate in int_values {
                                if coordinate < 0 || coordinate >= texture_size as i32 {
                                    log(&mut writer, log_id, "An IntTexCoords component is negative or not smaller than the texture size");
                                }
                            }
                        }
                        AttributeKind::Index { bound } => {
                            if data_kind != INT {
                                log(&mut writer, log_id, "An Index attribute is not of type INT");
                            }
                            for index in int_values {
                                if index < 0 || index >= bound as i32 {
                                    log(&mut writer, log_id, "An index attribute is out of range");
                                }
                            }
                        }
                        AttributeKind::Other => { /* I can't check values for unknown data kinds */
                        }
                    };
                }
            }

            /*
             * This test checks if more than half of the vertices has the same
             * position. Even though this should not easily occur, it is very nasty
             * to debug because all vertices will be mapped to the same screen
             * position, making the entire scene completely invisible.
             */
            for attribute in description.get_raw_description().get_attributes() {
                match attribute.get_kind() {
                    AttributeKind::Position { max: _ } => {
                        let num_components =
                            attribute.get_data_type().get_shape().get_size() as usize;
                        let mut positions = Vec::new();
                        for vertex_index in 0..vertices.len() {
                            let vertex_offset = vertex_index * vertex_size;
                            let position_offset = vertex_offset + attribute.offset;
                            let mut position = Vec::with_capacity(num_components);
                            for component_index in 0..num_components {
                                let mut component_bytes = [0; 4];
                                for byte_index in 0..4 {
                                    component_bytes[byte_index] = store_builder.raw_buffer
                                        [position_offset + 4 * component_index + byte_index];
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
                            log(
                                &mut writer,
                                log_id,
                                "More than half of the vertices has the same position",
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        store_builder.finish()
    }

    /// Gets a reference to the raw byte buffer of this *VertexStore*. This method is intended to
    /// be used by Griphin implementations, but users are free to use it as well.
    pub fn get_raw_buffer(&self) -> &Vec<u8> {
        &self.raw_buffer
    }
}

/// A wrapper struct around a raw byte buffer (actually *Vec\<u8\>*) that is currently being filled
/// with vertex data (positions, texture coordinates...). An instance of this struct is created
/// during *VertexStore::new* and that instance will be dropped when that function returns.
///
/// Whenever a *Vertex* is being stored during the *VertexStore::new* function, its *store* method
/// will be called, and it will get a mutable reference to a *VertexStoreBuilder* as parameter. (If
/// the debug level is at least *Basic*, this method will even be called twice per vertex.) During
/// that method call, it should call the *put*... methods of the *VertexStoreBuilder* to store the
/// values of its attributes. See the documentation of *Vertex* for an example implementation.
///
/// This struct has 2 kinds of public methods:
///
/// 1) The single-value *put* methods: *put_bool*, *put_int*, and *put_float*
///
/// 2) The vector *put_vec* methods like *put_vec3f*
///
/// All these methods require a *VertexAttributeHandle* and a value. The attribute handle indicates
/// for which attribute the value is, and the value is just the value the vertex has for that
/// attribute.
pub struct VertexStoreBuilder {
    raw_buffer: Vec<u8>,

    current_offset: usize,
}

impl VertexStoreBuilder {
    fn finish(self) -> VertexStore {
        VertexStore {
            raw_buffer: self.raw_buffer,
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_int(&mut self, attribute: VertexAttributeHandle, value: i32) {
        let offset = self.current_offset + attribute.offset;
        self.put_int_at(offset, value);
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    fn put_int_at(&mut self, offset: usize, value: i32) {
        let as_bytes = value.to_ne_bytes();
        for index in 0..4 {
            self.raw_buffer[offset + index] = as_bytes[index];
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_float(&mut self, attribute: VertexAttributeHandle, value: f32) {
        let offset = self.current_offset + attribute.offset;
        self.put_float_at(offset, value);
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    fn put_float_at(&mut self, offset: usize, value: f32) {
        let as_bytes = value.to_ne_bytes();
        for index in 0..4 {
            self.raw_buffer[offset + index] = as_bytes[index];
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_bool(&mut self, attribute: VertexAttributeHandle, value: bool) {
        let offset = self.current_offset + attribute.offset;
        self.put_bool_at(offset, value);
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    fn put_bool_at(&mut self, offset: usize, value: bool) {
        // I think this is how booleans are handled on the GPU, but I'm not sure.
        // I never actually tried this x)
        if value {
            self.put_int_at(offset, 1);
        } else {
            self.put_int_at(offset, 0);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec2i(&mut self, attribute: VertexAttributeHandle, values: Vector2<i32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..2 {
            self.put_int_at(base_offset + 4 * index, values[index]);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec2f(&mut self, attribute: VertexAttributeHandle, values: Vector2<f32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..2 {
            self.put_float_at(base_offset + 4 * index, values[index]);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec3i(&mut self, attribute: VertexAttributeHandle, values: Vector3<i32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..3 {
            self.put_int_at(base_offset + 4 * index, values[index]);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec3f(&mut self, attribute: VertexAttributeHandle, values: Vector3<f32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..3 {
            self.put_float_at(base_offset + 4 * index, values[index]);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec4i(&mut self, attribute: VertexAttributeHandle, values: Vector4<i32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..4 {
            self.put_int_at(base_offset + 4 * index, values[index]);
        }
    }

    /// Sets the value for *attribute* for the current vertex to *value*
    pub fn put_vec4f(&mut self, attribute: VertexAttributeHandle, values: Vector4<f32>) {
        let base_offset = self.current_offset + attribute.offset;
        for index in 0..4 {
            self.put_float_at(base_offset + 4 * index, values[index]);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_basic_put_ats() {
        let mut store = VertexStoreBuilder {
            raw_buffer: vec![2; 17],
            current_offset: 30,
        };

        store.put_bool_at(1, true);
        store.put_int_at(6, -1234567890);
        store.put_float_at(12, 4.89176);

        test_basic_helper(store);
    }

    fn test_basic_helper(store: VertexStoreBuilder) {
        // Test that the untouched offsets remain 2
        assert_eq!(2, store.raw_buffer[0]);
        assert_eq!(2, store.raw_buffer[5]);
        assert_eq!(2, store.raw_buffer[10]);
        assert_eq!(2, store.raw_buffer[11]);
        assert_eq!(2, store.raw_buffer[16]);

        // Now verify that we can read back the right values
        let bool_bytes = store.raw_buffer[1..5].try_into().unwrap();
        let int_bytes = store.raw_buffer[6..10].try_into().unwrap();
        let float_bytes = store.raw_buffer[12..16].try_into().unwrap();
        assert_eq!(true, i32::from_ne_bytes(bool_bytes) == 1);
        assert_eq!(-1234567890, i32::from_ne_bytes(int_bytes));
        assert_eq!(4.89176, f32::from_ne_bytes(float_bytes));
    }

    #[test]
    fn test_basic_puts() {
        // We abuse our ability to directly create AttributeIDs a bit.
        // (Regular client code can't create attributes like this.)
        // But, this is just a unit test anyway.
        let bool_attribute = VertexAttributeHandle { offset: 1 };
        let int_attribute = VertexAttributeHandle { offset: 6 };
        let float_attribute = VertexAttributeHandle { offset: 12 };

        let mut store = VertexStoreBuilder {
            raw_buffer: vec![2; 17],
            current_offset: 0,
        };

        store.put_bool(bool_attribute, true);
        store.put_int(int_attribute, -1234567890);
        store.put_float(float_attribute, 4.89176);

        test_basic_helper(store);

        let mut store = VertexStoreBuilder {
            raw_buffer: vec![2; 20],
            current_offset: 3,
        };

        store.put_bool(bool_attribute, true);
        store.put_int(int_attribute, -1234567890);
        store.put_float(float_attribute, 4.89176);

        // Removing the first elements should yield the same result as
        // we got when using offset 0 instead of offset 3.
        for _counter in 0..store.current_offset {
            store.raw_buffer.remove(0);
        }

        test_basic_helper(store);
    }

    #[test]
    fn test_vec_puts() {
        // Again abusing direct access to VertexAttributeID
        let attribute_pos_int = VertexAttributeHandle { offset: 1 };
        let attribute_color_int = VertexAttributeHandle { offset: 10 };
        let attribute_pos_float = VertexAttributeHandle { offset: 23 };
        let attribute_color_float = VertexAttributeHandle { offset: 32 };
        let attribute_translucent_int = VertexAttributeHandle { offset: 45 };
        let attribute_translucent_float = VertexAttributeHandle { offset: 62 };

        let mut store = VertexStoreBuilder {
            raw_buffer: vec![5; 80],
            current_offset: 1,
        };

        store.put_vec2i(attribute_pos_int, Vector2 { x: 500, y: -123456 });
        store.put_vec3i(
            attribute_color_int,
            Vector3 {
                x: 10_000,
                y: -456543,
                z: 0,
            },
        );
        store.put_vec2f(
            attribute_pos_float,
            Vector2 {
                x: f32::INFINITY,
                y: 1234.567,
            },
        );
        store.put_vec3f(
            attribute_color_float,
            Vector3 {
                x: -98.76,
                y: 0.0,
                z: 4.0,
            },
        );
        store.put_vec4i(
            attribute_translucent_int,
            Vector4 {
                x: 97,
                y: -1234567,
                z: 834,
                w: 4,
            },
        );
        store.put_vec4f(
            attribute_translucent_float,
            Vector4 {
                x: 21.3,
                y: 5.99,
                z: -91.3,
                w: 4.0,
            },
        );

        // Test that the default value 5 was never touched
        assert_eq!(5, store.raw_buffer[0]);
        assert_eq!(5, store.raw_buffer[1]);
        assert_eq!(5, store.raw_buffer[10]);
        assert_eq!(5, store.raw_buffer[23]);
        assert_eq!(5, store.raw_buffer[32]);
        assert_eq!(5, store.raw_buffer[45]);
        assert_eq!(5, store.raw_buffer[62]);
        assert_eq!(5, store.raw_buffer[79]);

        // We will be reading a lot of values, so lets make it more convenient
        let get_int_at = |offset: usize| {
            let bytes = [
                store.raw_buffer[offset],
                store.raw_buffer[offset + 1],
                store.raw_buffer[offset + 2],
                store.raw_buffer[offset + 3],
            ];
            i32::from_ne_bytes(bytes)
        };
        let get_float_at = |offset: usize| {
            let bytes = [
                store.raw_buffer[offset],
                store.raw_buffer[offset + 1],
                store.raw_buffer[offset + 2],
                store.raw_buffer[offset + 3],
            ];
            f32::from_ne_bytes(bytes)
        };

        assert_eq!(500, get_int_at(2));
        assert_eq!(-123456, get_int_at(6));
        assert_eq!(10_000, get_int_at(11));
        assert_eq!(-456543, get_int_at(15));
        assert_eq!(0, get_int_at(19));
        assert_eq!(97, get_int_at(46));
        assert_eq!(-1234567, get_int_at(50));
        assert_eq!(834, get_int_at(54));
        assert_eq!(4, get_int_at(58));
        assert_eq!(f32::INFINITY, get_float_at(24));
        assert_eq!(1234.567, get_float_at(28));
        assert_eq!(-98.76, get_float_at(33));
        assert_eq!(0.0, get_float_at(37));
        assert_eq!(4.0, get_float_at(41));
        assert_eq!(21.3, get_float_at(63));
        assert_eq!(5.99, get_float_at(67));
        assert_eq!(-91.3, get_float_at(71));
        assert_eq!(4.0, get_float_at(75));
    }
}
