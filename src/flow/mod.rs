use std::any::Any;

mod builder;
mod input;
mod node;
mod output;

pub use builder::*;
pub use input::*;
pub use node::*;
pub use output::*;

/// A directed acyclic graph that specifies which graphics pipelines should be
/// used to draw at which grids at which moments. See the documentation of
/// *RenderFlowBuilder* for more information.
///
/// TODO Add a way to actually use *RenderFlow*s
pub trait RenderFlow {
    fn as_any(&self) -> &dyn Any;
}
