mod color;
mod depth_stencil;
mod dump;
mod group;
mod id;
mod region;

use std::sync::Arc;

pub use color::*;
pub use depth_stencil::*;
pub use dump::*;
pub use group::*;
pub use id::*;
pub use region::*;

#[derive(Clone)]
pub enum Grid {

    Color(Arc<dyn ColorGrid>),
    DepthStencil(Arc<dyn DepthStencilGrid>)
}