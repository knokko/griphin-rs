mod attribute;
mod description;
mod store;
mod usage;
mod buffer;

pub use attribute::*;
pub use description::*;
pub use store::*;
pub use usage::*;
pub use buffer::*;

/// Structs implementing this trait represent vertices on the CPU side. They should have attributes
/// like position and texture coordinates that are meant to be sent to the graphics card, and
/// eventually to the vertex shader.
///
/// For every struct implementing this trait, there should be a corresponding *VertexDescription*
/// that describes the attribute of the struct.
///
/// This is a simple trait with only 1 method, but a very important one. This *store* method
/// should write the attribute values of the vertex to a given *VertexStoreBuilder*.
///
/// The following example shows how to use this trait:
/// ```
/// # #[macro_use] extern crate lazy_static; fn main() {
/// use griphin::*;
/// use cgmath::Vector3;
///
/// use std::sync::Arc;
///
/// // Simple vertex description for only 1 attribute
/// struct ExampleVertexDescription {
///
///     // This is not a vertex attribute, but required to capture some information for Griphin
///     raw: RawVertexDescription,
///
///     // This is a handle for the (only) position attribute
///     position: VertexAttributeHandle
/// }
///
/// // Required to make the captured 'raw' information available to Griphin
/// impl VertexDescription for ExampleVertexDescription {
///     fn get_raw_description(&self) -> &RawVertexDescription {
///         &self.raw
///     }
/// }
///
/// impl ExampleVertexDescription {
///     fn new() -> Self {
///         // Initialize a new empty raw vertex description
///         let mut raw = RawVertexDescription::new();
///         // Add the only attribute: the position
///         let position = raw.add_attribute(
///             &str_ref("position"), // We will just call it position
///             DataType::new(FLOAT, VEC3), // It will be a 3-component vector of floats
///             AttributeKind::Position {max: 5.0} // In this example, we won't go far from origin
///         );
///         Self { raw, position }
///     }
/// }
///
/// // We only need 1 instance of ExampleVertexDescription. You can use *lazy_static* to avoid
/// // repeated creation of it. (Creating more instances is allowed, but not necessary.)
/// lazy_static! {
///     static ref EXAMPLE_VERTEX_DESCRIPTION: Arc<ExampleVertexDescription> = Arc::new(ExampleVertexDescription::new());
/// }
/// // In this simple case, we could also just construct the instance when we need it, but doing
/// // that repeatedly could cause performance problems in more complicated scenarios.
///
/// // Now its time to define the actual struct we described above
/// struct ExampleVertex {
///     position: Vector3<f32>
/// }
///
/// impl Vertex<ExampleVertexDescription> for ExampleVertex {
///     // We need to implement this method
///     fn store(&self,store: &mut VertexStoreBuilder, description: &ExampleVertexDescription) {
///         // This is the part where we actually need the description struct:
///         // We are given access to its attribute handles, which we use to efficiently store the
///         // position of this vertex at the right place in the store.
///         store.put_vec3f(description.position, self.position);
///     }
/// }
///
/// // Let's construct a couple of vertices, the coordinates we give are just example values
/// // But note that we can NOT use any values above 5.0 or below -5.0 because we gave 5.0 as
/// // maximum value when defining the position attribute in ExampleVertexDescription.
/// let vertices = vec![
///     ExampleVertex { position: Vector3 { x: 1.5, y: 0.5, z: -2.0 } },
///     ExampleVertex { position: Vector3 { x: 0.5, y: -4.0, z: 2.0 } },
///     ExampleVertex { position: Vector3 { x: -3.0, y: 0.0, z: 5.0} }
/// ];
///
/// // This is where we store the data of the 3 vertices above into a store/buffer. The returned
/// // *_store* can be send through the *Gateway*. But, that part is not demonstrated in this
/// // example. See the documentation of *Gateway* for the steps you need to do hereafter.
/// let _store = VertexStore::new(
///     // The vertex store needs to know the description for internal reasons, and to pass it as
///     // parameter to the *store* method of each of the vertices.
///     EXAMPLE_VERTEX_DESCRIPTION.as_ref(),
///     // We also need to give a reference to the vertices we want to send. Since it only needs a
///     // reference, we can reuse these vertices in future calls (possibly after modifying them).
///     &vertices,
///     // The debug level can be used to easily control how much effort you want Griphin to put
///     // into verifying that your vertex data is proper. Using high debug levels can be quite
///     // expensive if you pass a lot of vertices, but it doesn't matter if you have only 3. When
///     // you do have a lot of vertices, you should only use high debug levels if those vertices
///     // don't seem to be rendered (properly). Then, Griphin will print to the (standard) output
///     // whenever it notices that you assigned weird values to the vertex attributes (or forgot
///     // to set any).
///     DebugLevel::High,
///     // If the debug level is high enough, this last parameter determines where Griphin will
///     // report potential suspicious vertex data. If you use *None*, it will be printed to the
///     // standard output. Since you shouldn't really need complex debugging outside development,
///     // you should normally just keep this *None*. This parameter is mostly convenient for the
///     // internal unit tests of Griphin.
///     None
/// );
/// # }
/// ```
/// This example needed a lot of code just to store the data of 3 small vertices. But note that a
/// big part of the code was dedicated too comments, so it should be a lot shorter in real code
/// (where you don't need so many comments). But more importantly, this system of storing vertex
/// data was not primarily designed so that users need few lines of code.
///
/// The most important goal was to make it easy to debug and achieve good performance when debugging
/// is disabled. Due to the *VertexDescription*, Griphin knows what most vertex attributes are used
/// for, and thus which values these attributes should (not) have. (For instance, if the debug level
/// is high enough, it will check that not too many vertices have the same position, that the
/// normal vectors have a length of approximately 1.0, that the texture coordinates are between 0.0
/// and 1.0 (or between 0 and texture size in case of integer texture coordinates)...
pub trait Vertex<D> {
    /// Stores all attribute values of this vertex into the given *VertexStoreBuilder*. See the
    /// documentation of this trait for an example implementation.
    fn store(&self, store: &mut VertexStoreBuilder, description: &D);
}

#[cfg(test)]
mod tests {

    use crate::*;
    use cgmath::*;

    use std::sync::Arc;

    struct SimpleVertexDescription {
        raw: RawVertexDescription,

        position: VertexAttributeHandle,
        color: VertexAttributeHandle,
    }

    impl SimpleVertexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let position = raw.add_attribute(
                &str_ref("position"),
                DataType::new(FLOAT, VEC3),
                AttributeKind::Position { max: 4.0 },
            );
            let color = raw.add_attribute(
                &str_ref("color"),
                DataType::new(FLOAT, VEC4),
                AttributeKind::Other,
            );
            Self {
                raw,
                position,
                color,
            }
        }
    }

    impl VertexDescription for SimpleVertexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct SimpleVertex {
        position: Vector3<f32>,
        color: Vector4<f32>,
    }

    impl Vertex<SimpleVertexDescription> for SimpleVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &SimpleVertexDescription) {
            store.put_vec3f(description.position, self.position);
            store.put_vec4f(description.color, self.color);
        }
    }

    lazy_static! {
        static ref SIMPLE_VERTEX_DESCRIPTION: Arc<SimpleVertexDescription> =
            Arc::new(SimpleVertexDescription::new());
    }

    struct PositionVertexDescription {
        raw: RawVertexDescription,
        position: VertexAttributeHandle
    }

    impl PositionVertexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let position = raw.add_attribute(
                &str_ref("position"),
                DataType::new(FLOAT, VEC2),
                AttributeKind::Position { max: 10.0 }
            );
            Self { raw, position }
        }
    }

    impl VertexDescription for PositionVertexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct PositionVertex {
        position: Vector2<f32>
    }

    impl Vertex<PositionVertexDescription> for PositionVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &PositionVertexDescription) {
            store.put_vec2f(description.position, self.position);
        }
    }

    #[test]
    fn test_static() {
        let vertices = vec![
            SimpleVertex {
                position: Vector3::new(0.0, 1.0, 0.0),
                color: Vector4::new(1.0, 1.0, 0.0, 0.5),
            },
            SimpleVertex {
                position: Vector3::new(1.0, 0.0, 0.0),
                color: Vector4::new(0.0, 0.0, 1.0, 0.8),
            },
            SimpleVertex {
                position: Vector3::new(0.0, -1.0, 1.0),
                color: Vector4::new(0.5, 1.0, 0.8, 1.0),
            },
        ];

        let mut _store = VertexStore::new(
            SIMPLE_VERTEX_DESCRIPTION.as_ref(),
            &vertices,
            DebugLevel::Basic,
            None,
        );

        // Send the store to the Gateway
    }

    #[test]
    fn test_local() {
        let vertices = vec![
            SimpleVertex {
                position: Vector3::new(0.0, 1.0, 0.0),
                color: Vector4::new(1.0, 1.0, 0.0, 0.5),
            },
            SimpleVertex {
                position: Vector3::new(1.0, 0.0, 0.0),
                color: Vector4::new(0.0, 0.0, 1.0, 0.8),
            },
            SimpleVertex {
                position: Vector3::new(0.0, -1.0, 1.0),
                color: Vector4::new(0.5, 1.0, 0.8, 1.0),
            },
        ];

        let mut _store = VertexStore::new(
            &SimpleVertexDescription::new(),
            &vertices,
            DebugLevel::Basic,
            None,
        );

        // Send the store to the Gateway
    }

    #[test]
    fn test_no_vertices() {
        let vertices: Vec<SimpleVertex> = vec![];
        let mut output = Vec::new();
        VertexStore::new(
            SIMPLE_VERTEX_DESCRIPTION.as_ref(),
            &vertices, DebugLevel::Low, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("You passed an empty slice of vertices."));
    }

    #[test]
    fn test_no_attributes() {
        struct EmptyVertexDescription {
            raw: RawVertexDescription
        }
        impl VertexDescription for EmptyVertexDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct EmptyVertex {}
        impl Vertex<EmptyVertexDescription> for EmptyVertex {
            fn store(&self, _store: &mut VertexStoreBuilder, _description: &EmptyVertexDescription) {
            }
        }

        let vertices = [EmptyVertex {}];
        let mut output = Vec::new();
        VertexStore::new(
            &EmptyVertexDescription{raw: RawVertexDescription::new()}, &vertices,
            DebugLevel::Low, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("The vertex description doesn't have any attributes"));
    }

    #[test]
    fn test_unwritten_attribute() {
        struct BadSimpleVertex {

            position: Vector3<f32>
        }

        impl Vertex<SimpleVertexDescription> for BadSimpleVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &SimpleVertexDescription) {
                store.put_vec3f(description.position, self.position);
            }
        }

        let vertices = [
            BadSimpleVertex {position: Vector3 { x: 0.0, y: 1.0, z: 2.0 } }
        ];

        let mut writer = Vec::new();

        VertexStore::new(
            SIMPLE_VERTEX_DESCRIPTION.as_ref(),
            &vertices, DebugLevel::Basic, Some(&mut writer)
        );

        let output = String::from_utf8(writer).expect("Error message should be valid UTF-8");
        assert!(output.contains("Not the entire vertex buffer seems to have been filled"));
    }

    #[test]
    fn test_int_position() {
        struct IntPosVertexDescription {
            raw: RawVertexDescription,
            position: VertexAttributeHandle
        }

        impl IntPosVertexDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let position = raw.add_attribute(
                    &str_ref("position"),
                    DataType::new(INT, VEC2),
                    AttributeKind::Position { max: 1.0 }
                );
                Self { raw, position }
            }
        }

        impl VertexDescription for IntPosVertexDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct IntPosVertex {
            position: Vector2<i32>
        }

        impl Vertex<IntPosVertexDescription> for IntPosVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &IntPosVertexDescription) {
                store.put_vec2i(description.position, self.position);
            }
        }

        let vertices = [IntPosVertex { position: Vector2 { x: 10, y: 5 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &IntPosVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A position vertex attribute is not of type float"));
    }

    #[test]
    fn test_nan_position() {
        let vertices = [PositionVertex { position: Vector2 { x: f32::NAN, y: 0.5 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &PositionVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A vertex position component is NaN"));
    }

    #[test]
    fn test_big_position() {
        let vertices = [PositionVertex { position: Vector2 { x: 10.1, y: 0.5 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &PositionVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A vertex position component is too large"))
    }

    #[test]
    fn test_small_position() {
        let vertices = [PositionVertex { position: Vector2 { x: 10.0, y: -10.5 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &PositionVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A vertex position component is too small"))
    }

    #[test]
    fn test_normal_int() {
        struct IntNormalDescription {
            raw: RawVertexDescription,
            normal: VertexAttributeHandle
        }

        impl IntNormalDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let normal = raw.add_attribute(
                    &str_ref("normal"),
                    DataType::new(INT, VEC2),
                    AttributeKind::Normal
                );
                Self { raw, normal }
            }
        }

        impl VertexDescription for IntNormalDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct IntNormalVertex {
            normal: Vector2<i32>
        }

        impl Vertex<IntNormalDescription> for IntNormalVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &IntNormalDescription) {
                store.put_vec2i(description.normal, self.normal);
            }
        }

        let vertices = [IntNormalVertex { normal: Vector2 { x: 1, y: 0 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &IntNormalDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A normal vertex attribute is not of type float"));
    }

    #[test]
    fn test_normal_single() {
        struct SingleNormalDescription {
            raw: RawVertexDescription,
            normal: VertexAttributeHandle
        }

        impl SingleNormalDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let normal = raw.add_attribute(
                    &str_ref("normal"),
                    DataType::new(FLOAT, SINGLE),
                    AttributeKind::Normal
                );
                Self { raw, normal }
            }
        }

        impl VertexDescription for SingleNormalDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct SingleNormalVertex {
            normal: f32
        }

        impl Vertex<SingleNormalDescription> for SingleNormalVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &SingleNormalDescription) {
                store.put_float(description.normal, self.normal);
            }
        }

        let vertices = [SingleNormalVertex { normal: 1.0 }];
        let mut output = Vec::new();
        VertexStore::new(
            &SingleNormalDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );

        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A normal vertex attribute is not a 2d or 3d vector"));
    }

    struct NormalVertexDescription {
        raw: RawVertexDescription,
        normal: VertexAttributeHandle
    }

    impl NormalVertexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let normal = raw.add_attribute(
                &str_ref("normal"),
                DataType::new(FLOAT, VEC2),
                AttributeKind::Normal
            );
            Self { raw, normal }
        }
    }

    impl VertexDescription for NormalVertexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct NormalVertex {
        normal: Vector2<f32>
    }

    impl Vertex<NormalVertexDescription> for NormalVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &NormalVertexDescription) {
            store.put_vec2f(description.normal, self.normal);
        }
    }

    #[test]
    fn test_nan_normal() {
        let vertices = [NormalVertex { normal: Vector2 { x: 1.0, y: f32::NAN } }];
        let mut output = Vec::new();
        VertexStore::new(
            &NormalVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A normal vertex component is NaN"))
    }

    #[test]
    fn test_large_normal() {
        let vertices = [NormalVertex { normal: Vector2 { x: -0.8, y: -0.7 }}];
        let mut output = Vec::new();
        VertexStore::new(
            &NormalVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A normal vertex has a length that is larger than 1.05"));
    }

    #[test]
    fn test_small_normal() {
        let vertices = [NormalVertex { normal: Vector2 { x: 0.5, y: 0.6 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &NormalVertexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A normal vertex has a length that is smaller than 0.95"));
    }

    #[test]
    fn test_float_int_tex_coords() {
        struct FloatIntTexDescription {
            raw: RawVertexDescription,
            tex_coords: VertexAttributeHandle
        }

        impl FloatIntTexDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let tex_coords = raw.add_attribute(
                    &str_ref("texCoords"),
                    DataType::new(FLOAT, VEC2),
                    AttributeKind::IntTexCoords { texture_size: 128 }
                );
                Self { raw, tex_coords }
            }
        }

        impl VertexDescription for FloatIntTexDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct FloatIntTexVertex {
            tex_coords: Vector2<f32>
        }

        impl Vertex<FloatIntTexDescription> for FloatIntTexVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &FloatIntTexDescription) {
                store.put_vec2f(description.tex_coords, self.tex_coords);
            }
        }

        let vertices = [FloatIntTexVertex { tex_coords: Vector2 { x: 0.5, y: 0.8 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &FloatIntTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );

        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An IntTexCoords attribute is not of type INT"));
    }

    #[test]
    fn test_int_float_tex_coords() {
        struct IntFloatTexDescription {
            raw: RawVertexDescription,
            tex_coords: VertexAttributeHandle
        }

        impl IntFloatTexDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let tex_coords = raw.add_attribute(
                    &str_ref("texCoords"),
                    DataType::new(INT, VEC2),
                    AttributeKind::FloatTexCoords
                );
                Self { raw, tex_coords }
            }
        }

        impl VertexDescription for IntFloatTexDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct IntFloatTexVertex {
            tex_coords: Vector2<i32>
        }

        impl Vertex<IntFloatTexDescription> for IntFloatTexVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &IntFloatTexDescription) {
                store.put_vec2i(description.tex_coords, self.tex_coords);
            }
        }

        let vertices = [IntFloatTexVertex { tex_coords: Vector2 { x: 5, y: 8 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &IntFloatTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );

        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A FloatTexCoords attribute is not of type FLOAT"));
    }

    struct FloatTexDescription {
        raw: RawVertexDescription,
        tex_coords: VertexAttributeHandle
    }

    impl FloatTexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let tex_coords = raw.add_attribute(
                &str_ref("texCoords"),
                DataType::new(FLOAT, VEC2),
                AttributeKind::FloatTexCoords
            );
            Self { raw, tex_coords }
        }
    }

    impl VertexDescription for FloatTexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct FloatTexVertex {
        tex_coords: Vector2<f32>
    }

    impl Vertex<FloatTexDescription> for FloatTexVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &FloatTexDescription) {
            store.put_vec2f(description.tex_coords, self.tex_coords);
        }
    }

    #[test]
    fn test_nan_float_tex_coords() {
        let vertices = [FloatTexVertex { tex_coords: Vector2 { x: 0.3, y: f32::NAN } }];
        let mut output = Vec::new();
        VertexStore::new(
            &FloatTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A float texture coordinate is NaN"));
    }

    #[test]
    fn test_small_float_tex_coords() {
        let vertices = [FloatTexVertex {tex_coords: Vector2 { x: -0.1, y: 0.7 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &FloatTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A float texture coordinate is smaller than -0.05"));
    }

    #[test]
    fn test_big_float_tex_coords() {
        let vertices = [FloatTexVertex {tex_coords: Vector2 { x: 0.1, y: 1.7 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &FloatTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("A float texture coordinate is larger than 1.05"));
    }

    struct IntTexDescription {
        raw: RawVertexDescription,
        tex_coords: VertexAttributeHandle
    }

    impl IntTexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let tex_coords = raw.add_attribute(
                &str_ref("texCoords"),
                DataType::new(INT, VEC2),
                AttributeKind::IntTexCoords { texture_size: 32 }
            );
            Self { raw, tex_coords }
        }
    }

    impl VertexDescription for IntTexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct IntTexVertex {
        tex_coords: Vector2<i32>
    }

    impl Vertex<IntTexDescription> for IntTexVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &IntTexDescription) {
            store.put_vec2i(description.tex_coords, self.tex_coords);
        }
    }

    #[test]
    fn test_small_int_tex_coords() {
        let vertices = [IntTexVertex { tex_coords: Vector2 { x: -1, y: 15 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &IntTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An IntTexCoords component is negative"));
    }

    #[test]
    fn test_big_int_tex_coords() {
        let vertices = [IntTexVertex { tex_coords: Vector2 { x: 21, y: 32 } }];
        let mut output = Vec::new();
        VertexStore::new(
            &IntTexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An IntTexCoords component is not smaller than the texture size"));
    }

    #[test]
    fn test_float_index() {
        struct FloatIndexDescription {
            raw: RawVertexDescription,
            index: VertexAttributeHandle
        }

        impl FloatIndexDescription {
            fn new() -> Self {
                let mut raw = RawVertexDescription::new();
                let index = raw.add_attribute(
                    &str_ref("index"),
                    DataType::new(FLOAT, SINGLE),
                    AttributeKind::Index { bound: 10 }
                );
                Self { raw, index }
            }
        }

        impl VertexDescription for FloatIndexDescription {
            fn get_raw_description(&self) -> &RawVertexDescription {
                &self.raw
            }
        }

        struct FloatIndexVertex {
            index: i32
        }

        impl Vertex<FloatIndexDescription> for FloatIndexVertex {
            fn store(&self, store: &mut VertexStoreBuilder, description: &FloatIndexDescription) {
                store.put_int(description.index, self.index);
            }
        }

        let vertices = [FloatIndexVertex { index: 5 }];
        let mut output = Vec::new();
        VertexStore::new(
            &FloatIndexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An index attribute is not of type INT"))
    }

    struct IndexDescription {
        raw: RawVertexDescription,
        index: VertexAttributeHandle
    }

    impl IndexDescription {
        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let index = raw.add_attribute(
                &str_ref("index"),
                DataType::new(INT, SINGLE),
                AttributeKind::Index { bound: 10 }
            );
            Self { raw, index }
        }
    }

    impl VertexDescription for IndexDescription {
        fn get_raw_description(&self) -> &RawVertexDescription {
            &self.raw
        }
    }

    struct IndexVertex {
        index: i32
    }

    impl Vertex<IndexDescription> for IndexVertex {
        fn store(&self, store: &mut VertexStoreBuilder, description: &IndexDescription) {
            store.put_int(description.index, self.index);
        }
    }

    #[test]
    fn test_negative_index() {
        let vertices = [IndexVertex { index: -1 }, IndexVertex { index: 0 }];
        let mut output = Vec::new();
        VertexStore::new(
            &IndexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An index attribute is negative"));
    }

    #[test]
    fn test_big_index() {
        let vertices = [IndexVertex { index: 5}, IndexVertex { index: 10 }];
        let mut output = Vec::new();
        VertexStore::new(
            &IndexDescription::new(), &vertices,
            DebugLevel::High, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("An index attribute is not smaller than the bound"));
    }

    #[test]
    fn test_many_equal_positions() {
        let vertices = [
            PositionVertex { position: Vector2 { x: 1.0, y: 0.0 }},
            PositionVertex { position: Vector2 { x: 2.0, y: 3.0 }},
            PositionVertex { position: Vector2 { x: 1.0, y: 0.0 }},
            PositionVertex { position: Vector2 { x: 1.0, y: 0.0 }}
        ];
        let mut output = Vec::new();
        VertexStore::new(
            &PositionVertexDescription::new(), &vertices,
            DebugLevel::All, Some(&mut output)
        );
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("More than half of the vertices has the same position"));
    }

    // TODO Unit test for correct vertex data (check that no warnings are printed)
    #[test]
    fn test_good_vertices() {
        
    }
}
