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
/// There should be an input link for every input variable of
pub struct DrawingNodeBuilder {

    pub pipeline: Arc<dyn GraphicsPipeline>,
    pub inputs: Vec<DrawingNodeInput>,
    pub outputs: Vec<NodeOutput>
}

pub trait GridNode {

    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub(super) struct GridNodeBuilder {

    pub grid: Grid,
    pub id: GridNodeID
}

impl GridNodeBuilder {

    pub(super) fn new(id: GridNodeID, grid: &Grid) -> Self {
        GridNodeBuilder { id, grid: grid.clone()}
    }
}

impl PartialEq for GridNodeBuilder {

    fn eq(&self, other: &GridNodeBuilder) -> bool {
        self.id == other.id
    }
}

impl Eq for GridNodeBuilder {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridNodeID {
    pub(super) flow_id: RenderFlowBuilderID,
    pub(super) own_id: u32
}