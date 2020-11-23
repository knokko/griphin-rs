use crate::*;

use std::collections::HashMap;

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
    drawing_nodes: Vec<RenderTaskSketch>,

    used_grids: HashMap<AbstractGridID, FlowGridSketch>,

    id: RenderFlowBuilderID
}

impl RenderFlowBuilder {
    /// Creates a new *RenderFlowBuilder* with the given id. Note that this method
    /// should only be used by Griphin implementations, *not* by client code. Client
    /// code should instead use the *create_render_flow_builder* method of an
    /// *AbstractGridGroup*.
    pub fn new(id: RenderFlowBuilderID) -> Self {
        Self {
            drawing_nodes: Vec::new(),
            used_grids: HashMap::new(),
            id
        }
    }

    /// Gets the id of this *RenderFlowBuilder*. Note that this method should only
    /// be used by Griphin implementations, *not* by client code. Client code
    /// shouldn't care about this id.
    pub fn get_id(&self) -> &RenderFlowBuilderID {
        &self.id
    }

    /// Adds a new drawing node to this *RenderFlowBuilder* (and thus to the future
    /// *RenderFlow*). See the documentation of this struct for more information
    /// about drawing nodes.
    pub fn add_render_task(&mut self, mut task: RenderTaskBuilder) {
        // Find a suitable moment to schedule this render task
        let mut moment = 1;

        // It must be *after* the last time each input grid was last modified
        for input in &task.inputs {
            if input.grid.last_write_moment >= moment {
                moment = input.grid.last_write_moment + 1;
            }
        }

        // It must also be *after* the last time each output grid was last read or modified
        for output in &task.outputs {
            if output.grid.last_write_moment >= moment {
                moment = output.grid.last_write_moment + 1;
            }
            if output.grid.last_read_moment.get() >= moment {
                moment = output.grid.last_read_moment.get() + 1;
            }
        }

        // It must also be *after* the last time the depth grid was last read or modified
        if task.depth_stencil_grid.last_read_moment.get() >= moment {
            moment = task.depth_stencil_grid.last_read_moment.get();
        }
        if task.depth_stencil_grid.last_write_moment >= moment {
            moment = task.depth_stencil_grid.last_write_moment;
        }

        // Update the last read and last modified of the grids
        for input in &task.inputs {
            input.grid.last_read_moment.set(moment);
        }
        for output in &mut task.outputs {
            output.grid.last_write_moment = moment;
            // Modifying a grid implicitly also reads it
            output.grid.last_read_moment.set(moment);
        }

        // Also update the last modified of the depth grid
        task.depth_stencil_grid.last_write_moment = moment;
        task.depth_stencil_grid.last_read_moment.set(moment);

        let input_sketches = task.inputs.into_iter().map(|builder| {
            RenderTaskInputSketch::new(builder.grid.get_grid_id(), builder.shader_variable_name)
        }).collect();
        let output_sketches = task.outputs.into_iter().map(|builder| {
           RenderTaskOutputSketch::new(builder.grid.get_grid_id(), builder.shader_variable_name)
        }).collect();

        self.drawing_nodes.push(RenderTaskSketch::new(
            moment, input_sketches, output_sketches,
            task.depth_stencil_grid.get_grid_id()
        ));
    }

    pub fn add_grid_node(&mut self, grid: AbstractGridID, preserve_content: bool) -> FlowGridBuilder {
        if grid.get_group_id() != self.id.get_group_id() {
            panic!("The given grid doesn't belong to the AbstractGridGroup that made this RenderFlowBuilder");
        }
        if self.used_grids.insert(grid, FlowGridSketch::new(preserve_content)).is_some() {
            panic!("This RenderFlowBuilder has already created a grid node for the given grid");
        }
        FlowGridBuilder::new(grid)
    }

    pub fn preserve_grid_node(&mut self, grid_node: FlowGridBuilder) {
        self.used_grids.get_mut(&grid_node.get_grid_id())
            .expect("The grid is was inserted when grid_node was constructed")
            .preserve_final_content = true;
    }
}

/// The type to be used as identifier for *RenderFlowBuilder*s. Griphin
/// implementations need this identifier to prevent users from using the nodes of
/// a *RenderFlowBuilder* in another *RenderFlowBuilder*, which could give hard
/// to debug problems.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct RenderFlowBuilderID {
    group_id: AbstractGridGroupID,
    local_id: u32
}

impl RenderFlowBuilderID {
    /// Constructs a new *RenderFlowBuilderID*. This function should only be used by Griphin
    /// implementations.
    pub fn new(group_id: AbstractGridGroupID, local_id: u32) -> Self {
        Self { group_id, local_id }
    }

    /// Gets the id of the *AbstractGridGroup* that created the *RenderFlowBuilder* this ID belongs
    /// to.
    pub fn get_group_id(&self) -> AbstractGridGroupID {
        self.group_id
    }
}
