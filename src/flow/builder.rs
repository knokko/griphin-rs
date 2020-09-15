use crate::*;

use super::GridNodeBuilder;

/// This struct contains the information an *AbstractGridGroup* needs to create
/// a new *RenderFlow*. 
/// 
/// *RenderFlow*s are bound to an *AbstractGridGroup* and can be used on
/// concrete *GridGroup*s of that *AbstractGridGroup*. A *RenderFlow* is an
/// acyclic directed graph, where each node is either a drawing node or
/// a grid node. All links from a grid node must go to a drawing node, and
/// vice versa.
/// 
/// A grid node represents one of the grids of the *(Abstract)GridGroup* at
/// a single moment during the *RenderFlow*. Most importantly, it specifies
/// what kind of content the grid will have at that moment. For instance, it
/// could be cleared to a fixed value, or store the output of a previous
/// drawing operation. Depending on the kind of content, it either has 0 or 1
/// incoming links, but can have any number of outgoing links.
/// 
/// Every drawing node uses a *GraphicsPipeline* (see its documentation for more 
/// info), which will be used for that drawing operation. Every *GraphicsPipeline*
/// has inputs and outputs, and some of these are grids. 
/// For each of these input grids, the node needs an incoming link from a grid
/// node. For each of these output grids, the node needs an outgoing link to a
/// grid node.
/// 
/// To obtain an instance of this struct, use the *create_render_flow_builder*
/// method of the *AbstractGridGroup* for which you want to create a new
/// *RenderFlow*. 
/// 
/// Use the *create_render_flow* method of the *AbstractGridGroup* once you
/// have added all nodes you want the *RenderFlow* to have to turn this
/// into a usable *RenderFlow*.
pub struct RenderFlowBuilder {

    drawing_nodes: Vec<DrawingNodeBuilder>,
    grid_nodes: Vec<GridNodeBuilder>,

    id: RenderFlowBuilderID,
    next_grid_state_id: u32
}

impl RenderFlowBuilder {

    /// Creates a new *RenderFlowBuilder* with the given id. Note that this method
    /// should only be used by Griphin implementations, *not* by client code. Client
    /// code should instead use the *create_render_flow* method of an
    /// *AbstractGridGroup*.
    pub fn new(id: RenderFlowBuilderID) -> Self {
        Self {
            drawing_nodes: Vec::new(),
            grid_nodes: Vec::new(),
            id,
            next_grid_state_id: 0
        }
    }

    /// Gets the id of this *RenderFlowBuilder*. Note that this method should only
    /// be used by Griphin implementations, *not* by client code. Client code
    /// shouldn't care about this id.
    pub fn get_id(&self) -> RenderFlowBuilderID {
        self.id
    }

    /// Adds a new drawing node to this *RenderFlowBuilder* (and thus to the future
    /// *RenderFlow*). See the documentation of this struct for more information
    /// about drawing nodes.
    pub fn add_drawing_node(&mut self, node: DrawingNodeBuilder) {
        self.drawing_nodes.push(node);
    }

    /// Adds a new grid node using the grid with the given *GridID* to this 
    /// *RenderFlowBuilder* (and thus to the future *RenderFlow*). The id of 
    /// the newly added grid node will be returned, because you will need this 
    /// id when constructing the inputs and outputs of the drawing nodes. See 
    /// the documentation of this struct for more information about grid nodes.
    pub fn add_grid_node(&mut self, grid: AbstractGridID) -> GridNodeID {
        let node = GridNodeBuilder::new(GridNodeID { flow_id: self.id, own_id: self.next_grid_state_id}, grid);
        self.next_grid_state_id += 1;
        let id = node.get_id();
        self.grid_nodes.push(node);
        id
    }
}

/// The type to be used as identifier for *RenderFlowBuilder*s. Griphin
/// implementations need this identifier to prevent users from using the nodes of
/// a *RenderFlowBuilder* in another *RenderFlowBuilder*, which could give hard
/// to debug problems.
pub type RenderFlowBuilderID = u32;