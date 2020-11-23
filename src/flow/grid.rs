use crate::*;

use std::any::Any;
use std::cell::Cell;

/// Represents a grid node of a *RenderFlow*. See the documentation of
/// *RenderFlowBuilder* for more information about the nodes.
///
/// TODO Add a way to use it
pub trait FlowGrid {
    fn as_any(&self) -> &dyn Any;
}

pub struct FlowGridSketch {
    preserve_initial_content: bool,
    pub(crate) preserve_final_content: bool
}

impl FlowGridSketch {
    pub(crate) fn new(preserve_initial_content: bool) -> Self {
        Self {
            preserve_initial_content,
            // This value will be determined later
            preserve_final_content: false
        }
    }

    pub fn preserve_initial_content(&self) -> bool {
        self.preserve_initial_content
    }

    pub fn preserve_final_content(&self) -> bool {
        self.preserve_final_content
    }
}

pub struct FlowGridBuilder {
    grid_id: AbstractGridID,
    pub(crate) last_read_moment: Cell<u32>,
    pub(crate) last_write_moment: u32,
}

impl FlowGridBuilder {
    pub(crate) fn new(grid_id: AbstractGridID) -> Self {
        FlowGridBuilder {
            grid_id,
            last_read_moment: Cell::new(0),
            last_write_moment: 0
        }
    }

    pub fn get_grid_id(&self) -> AbstractGridID {
        self.grid_id
    }
}
