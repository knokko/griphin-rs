use crate::*;

use std::any::Any;
use std::sync::Arc;

pub trait AbstractGridGroup {
    fn as_any(&self) -> &dyn Any;

    fn create_concrete(&self, width: u32, height: u32) -> Arc<dyn GridGroup>;

    fn create_graphics_pipeline(&self, shaders: &ShaderPair, topology: PrimitiveTopology) -> Arc<dyn GraphicsPipeline>;

    fn create_render_flow(&self) -> Arc<dyn RenderFlow>;
}

pub trait GridGroup {
    fn as_any(&self) -> &dyn Any;

    fn get_color_grid(&self, id: GridID) -> Arc<dyn ColorGrid>;

    fn get_depth_stencil_grid(&self, id: GridID) -> Arc<dyn DepthStencilGrid>;
}

pub struct AbstractGridGroupBuilder {

    pub color_grids: Vec<ColorGridBuilder>,
    pub depth_stencil_grids: Vec<DepthStencilGridBuilder>
}