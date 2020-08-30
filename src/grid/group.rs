use crate::*;

use std::any::Any;
use std::sync::Arc;

pub trait GridGroup {

    fn as_any(&self) -> &dyn Any;
}

pub trait GridGroupBuilder {

    fn as_any(&self) -> &dyn Any;

    fn add_color_grid(&self, on_start: ColorStartOperation, purpose: ColorPurpose) -> GridID;

    fn add_depth_stencil_grid(&self, on_start: DepthStencilStartOperation, purpose: DepthStencilPurpose) -> GridID;

    fn build(&self) -> Arc<dyn GridGroup>;
}