use crate::*;

use std::any::Any;
use std::sync::Arc;

/// Represents a group of *AbstractColorGrid*s and *AbstractDepthStencilGrid*s, that
/// 'belong together' in the sense that their concrete *ColorGrid*s and
/// *DepthStencilGrid*s will be used during the same *RenderFlow*s. 
/// 
/// *AbstractGridGroup*s are 'abstract' in the sense that they do *not* occupy any
/// GPU memory and not much RAM either. Also, this trait doesn't even dictate the
/// size of its concrete *GridGroup*s, so it's not required that they all have the
/// same size.
/// 
/// *AbstractGridGroup*s are very important because they are the 'entrypoint' from
/// which you can create concrete *GridGroup*s, *GraphicsPipeline*s, and
/// *RenderFlow(Builder)*s. (Use the methods of this trait for this.)
/// 
/// To obtain an instance of *AbstractGridGroup*, use the 
/// *create_abstract_grid_group* method of the *Instance*. 
pub trait AbstractGridGroup {

    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;

    /// Creates a concrete *GridGroup* based on this *AbstractGridGroup*, with the
    /// given *width* and *height*. 
    /// 
    /// Only *GridGroup*s created from this *AbstractGridGroup* can be used for 
    /// the *GraphicsPipeline*s and *RenderFlow*s that are also created by this
    /// *AbstractGridGroup*. 
    fn create_concrete(&self, width: u32, height: u32) -> Arc<dyn GridGroup>;

    /// Creates a *GraphicsPipeline* using the given *ShaderPair* and *topology*. 
    /// 
    /// The created *GraphicsPipeline* can only be used during *RenderFlow*s that
    /// are also created by this *AbstractGridGroup*, and the pipeline will only
    /// be able to draw on *GridGroup*s that are created by this *AbstractGridGroup*.
    fn create_graphics_pipeline(&self, shaders: &ShaderPair, topology: PrimitiveTopology) -> Arc<dyn GraphicsPipeline>;

    /// Creates a new empty *RenderFlowBuilder* that you will need to 'fill in' and
    /// then pass to the *create_render_flow* method of this *AbstractGridGroup* to
    /// turn it into a usable *RenderFlow*.
    fn create_render_flow_builder(&self) -> RenderFlowBuilder;

    /// Creates a *RenderFlow* that will use the graph from the given
    /// *RenderFlowBuilder*.
    /// 
    /// This *RenderFlow* can only be used on *GridGroup*s that are created by this
    /// same *AbstractGridGroup* and only *GraphicsPipeline*s created by this same
    /// *AbstractGridGroup* can be used during the *RenderFlow*.
    fn create_render_flow(&self, builder: RenderFlowBuilder) -> Arc<dyn RenderFlow>;
}

/// Represents a concrete instance of an *AbstractGridGroup*. Unlike its abstact
/// counterpart, instances of this trait have a size and occupy GPU memory. 
/// 
/// The grids of *GridGroup*s are drawing targets for *GraphicsPipeline*s (in other
/// words, graphics pipelines can draw stuff on the grids of a *GridGroup*). 
/// 
/// Furthermore, the grids can do a couple of other operations, like copying its
/// content to other grids (even to other *GridGroup*s), and they can be displayed
/// to the screen.
pub trait GridGroup {
    fn as_any(&self) -> &dyn Any;

    // TODO Stabilize the use cases of individual grids
    fn get_color_grid(&self, id: AbstractGridID) -> Arc<dyn ColorGrid>;

    fn get_depth_stencil_grid(&self, id: AbstractGridID) -> Arc<dyn DepthStencilGrid>;
}

/// *AbstractGridGroupBuilder* structs contain the information needed to create
/// a new *AbstractGridGroup*.
/// 
/// It simply contains a *color_grids* list and a *depthy_stencil_grids* list. 
/// Just push *AbstractColorGridBuilder*s and *AbstractDepthStencilGridBuilder*s 
/// to these lists until you are satisfied.
/// 
/// Finally use the *create_abstract_grid_group* method of the *Instance* to turn
/// it into an *AbstractGridGroup*.
pub struct AbstractGridGroupBuilder {

    /// A *Vec* containing all abstract color grids the abstract grid group should
    /// get.
    pub color_grids: Vec<AbstractColorGridBuilder>,

    /// A *Vec* containing all abstract depth stencil grids the abstract grid group
    /// should get.
    pub depth_stencil_grids: Vec<AbstractDepthStencilGridBuilder>
}

/// An instance of this struct is returned alongside its corresponding
/// *AbstractGridGroup* by the *create_abstract_grid_group* method of the
/// *Instance*.
/// 
/// This struct is needed by the user to find out which *AbstractGridID*s were
/// assigned to which *AbstractColorGridBuilder*s and which
/// *AbstractDepthStencilGridBuilder*s. This is the only reliable way in
/// which the user can get the right *AbstractGridID*s, so these should *not*
/// be forgotten.
/// 
/// The *colors* component of this struct corresponds to the *color_grids*
/// component of the *AbstractGridGroupBuilder*: the id that got assigned
/// to *color_grids[i]* is stored in *colors[i]* (for all 0 <= i < 
/// *color_grids.len()*). Similarly, the id that got assigned to 
/// *depth_stencil_grids[i]* is stored in *depth_stencils[i]*.
pub struct GridGroupIDs {

    pub colors: Vec<AbstractGridID>,
    pub depth_stencils: Vec<AbstractGridID>
}