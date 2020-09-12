use crate::*;

use std::any::Any;

mod builder;
mod grid;
mod input;
mod node;
mod output;

pub use builder::*;
pub use grid::*;
pub use input::*;
pub use node::*;
pub use output::*;

pub trait RenderFlow {
    fn as_any(&self) -> &dyn Any;
}