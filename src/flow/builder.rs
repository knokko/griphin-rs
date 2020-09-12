use crate::*;

pub struct RenderFlowBuilder {

    nodes: Vec<NodeBuilder>,

    next_grid_state_id: u32
}

impl RenderFlowBuilder {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_grid_state_id: 0
        }
    }

    pub fn add_node(&mut self, node: NodeBuilder) {
        self.nodes.push(node);
    }

    pub fn create_grid_state(&mut self, grid: &Grid) -> GridState {
        self.next_grid_state_id += 1;
        GridState::new(self.next_grid_state_id - 1, grid)
    }
}