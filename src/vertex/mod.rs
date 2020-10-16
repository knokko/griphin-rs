mod attribute;
mod description;
mod store;

pub use attribute::*;
pub use description::*;
pub use store::*;

pub trait Vertex {

    fn store(&self, store: &mut VertexStore);
}