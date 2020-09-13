use crate::*;

use std::any::Any;
use std::sync::Arc;

/// Represents a drawing node of a *RenderFlow*. See the documentation of
/// *RenderFlowBuilder* for more information about the nodes.
/// 
/// TODO Add a way to use it
pub trait DrawingNode {

    fn as_any(&self) -> &dyn Any;
}

/// This struct contains the information needed to add a new drawing node to
/// a *RenderFlowBuilder*. It consists of a *GraphicsPipeline*, input links,
/// and output links. 
/// 
/// *GraphicsPipeline*s should have external variables. Each of these external
/// variables either serves as input or as output. For each of these input
/// variables, this node must have a corresponding *DrawingNodeInput* link. 
/// Similarly, this node must have a corresponding *DrawingNodeOutput* link 
/// for each external output variable of the graphics pipeline.
/// 
/// To add the node, use the *add_drawing_node* method of a
/// *RenderFlowBuilder*.
pub struct DrawingNodeBuilder {

    pub pipeline: Arc<dyn GraphicsPipeline>,
    pub inputs: Vec<DrawingNodeInput>,
    pub outputs: Vec<DrawingNodeOutput>
}

/// Represents a grid node of a *RenderFlow*. See the documentation of
/// *RenderFlowBuilder* for more information about the nodes.
/// 
/// TODO Add a way to use it
pub trait GridNode {

    fn as_any(&self) -> &dyn Any;
}

/// This struct contains information about a grid node of a *RenderFlowBuilder*. 
/// Only Griphin implementations should use this struct directly, and only the
/// griphin crate itself can construct instances of it.
#[derive(Clone)]
pub struct GridNodeBuilder {

    grid: Grid,
    id: GridNodeID
}

impl GridNodeBuilder {

    pub(super) fn new(id: GridNodeID, grid: &Grid) -> Self {
        GridNodeBuilder { id, grid: grid.clone()}
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn get_id(&self) -> GridNodeID {
        self.id
    }
}

impl PartialEq for GridNodeBuilder {

    fn eq(&self, other: &GridNodeBuilder) -> bool {
        self.id == other.id
    }
}

impl Eq for GridNodeBuilder {}

/// An identifier and 'handle' for a grid node of a *RenderFlow(Builder)*. This
/// handle can be used to refer to grid nodes from input and output links of
/// drawing nodes.
/// 
/// Instances of this struct will be returned by the *add_grid_node* method of
/// *RenderFlowBuilder* (and will refer to the grid node that was just added). 
/// There is no other way to obtain an instance of this struct.
/// 
/// You need an instance of this struct if you want to create an Internal input
/// or output for a drawing node.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridNodeID {
    pub(super) flow_id: RenderFlowBuilderID,
    pub(super) own_id: u32
}