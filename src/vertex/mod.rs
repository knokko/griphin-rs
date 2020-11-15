mod attribute;
mod description;
mod store;

pub use attribute::*;
pub use description::*;
pub use store::*;

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
}
