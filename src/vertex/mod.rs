mod attribute;
mod description;
mod store;

pub use attribute::*;
pub use description::*;
pub use store::*;

pub trait Vertex<D> {

    fn store(&self, store: &mut VertexStore, description: &D);
}

#[cfg(test)]
mod tests {

    use crate::*;
    use cgmath::*;

    use std::sync::Arc;

    struct SimpleVertexDescription {

        raw: RawVertexDescription,

        position: VertexAttributeHandle,
        color: VertexAttributeHandle
    }

    impl SimpleVertexDescription {

        fn new() -> Self {
            let mut raw = RawVertexDescription::new();
            let position = raw.add_attribute(
                &str_ref("position"),
                DataType::new(FLOAT, VEC3),
                AttributeKind::Position { max: 4.0 }
            );
            let color = raw.add_attribute(
                &str_ref("color"),
                DataType::new(FLOAT, VEC4),
                AttributeKind::Other
            );
            Self {
                raw, position, color
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
        color: Vector4<f32>
    }

    impl Vertex<SimpleVertexDescription> for SimpleVertex {

        fn store(&self, store: &mut VertexStore, description: &SimpleVertexDescription) {
            store.put_vec3f(description.position, self.position);
            store.put_vec4f(description.color, self.color);
        }
    }

    lazy_static! {
        static ref SIMPLE_VERTEX_DESCRIPTION: Arc<SimpleVertexDescription> = Arc::new(SimpleVertexDescription::new());
    }

    #[test]
    fn test_static() {

        let vertices = vec![
            SimpleVertex {
                position: Vector3::new(0.0, 1.0, 0.0),
                color: Vector4::new(1.0, 1.0, 0.0, 0.5)
            }, SimpleVertex {
                position: Vector3::new(1.0, 0.0, 0.0),
                color: Vector4::new(0.0, 0.0, 1.0, 0.8)
            }, SimpleVertex {
                position: Vector3::new(0.0, -1.0, 1.0),
                color: Vector4::new(0.5, 1.0, 0.8, 1.0)
            }
        ];

        let mut _store = VertexStore::new(
            SIMPLE_VERTEX_DESCRIPTION.as_ref(),
            &vertices, DebugLevel::Basic, None
        );

        // Send the store to the Gateway
    }

    #[test]
    fn test_local() {

        let vertices = vec![
            SimpleVertex {
                position: Vector3::new(0.0, 1.0, 0.0),
                color: Vector4::new(1.0, 1.0, 0.0, 0.5)
            }, SimpleVertex {
                position: Vector3::new(1.0, 0.0, 0.0),
                color: Vector4::new(0.0, 0.0, 1.0, 0.8)
            }, SimpleVertex {
                position: Vector3::new(0.0, -1.0, 1.0),
                color: Vector4::new(0.5, 1.0, 0.8, 1.0)
            }
        ];

        let mut _store = VertexStore::new(
            &SimpleVertexDescription::new(),
            &vertices, DebugLevel::Basic, None
        );

        // Send the store to the Gateway
    }
}