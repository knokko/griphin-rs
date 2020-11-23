use crate::*;

use std::any::Any;

// TODO Give this type some actual useful methods
pub trait RenderTask {
    fn as_any(&self) -> &dyn Any;
}

pub struct RenderTaskSketch {
    moment: u32,
    inputs: Vec<RenderTaskInputSketch>,
    outputs: Vec<RenderTaskOutputSketch>,
    depth_stencil_grid: AbstractGridID
}

impl RenderTaskSketch {
    pub(crate) fn new(
        moment: u32,
        inputs: Vec<RenderTaskInputSketch>,
        outputs: Vec<RenderTaskOutputSketch>,
        depth_stencil_grid: AbstractGridID
    ) -> Self {
        Self { moment, inputs, outputs, depth_stencil_grid }
    }

    pub fn get_moment(&self) -> u32 {
        self.moment
    }

    pub fn get_inputs(&self) -> &Vec<RenderTaskInputSketch> {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &Vec<RenderTaskOutputSketch> {
        &self.outputs
    }

    pub fn get_depth_stencil_grid(&self) -> AbstractGridID {
        self.depth_stencil_grid
    }
}

pub struct RenderTaskInputSketch {
    grid_id: AbstractGridID,
    shader_variable_name: StringRef
}

impl RenderTaskInputSketch {
    pub(crate) fn new(grid_id: AbstractGridID, shader_variable_name: StringRef) -> Self {
        Self { grid_id, shader_variable_name }
    }

    pub fn get_grid_id(&self) -> AbstractGridID {
        self.grid_id
    }

    pub fn get_shader_variable_name(&self) -> &StringRef {
        &self.shader_variable_name
    }
}

pub struct RenderTaskOutputSketch {
    grid_id: AbstractGridID,
    shader_variable_name: StringRef
}

impl RenderTaskOutputSketch {
    pub(crate) fn new(grid_id: AbstractGridID, shader_variable_name: StringRef) -> Self {
        Self { grid_id, shader_variable_name }
    }

    pub fn get_grid_id(&self) -> AbstractGridID {
        self.grid_id
    }

    pub fn get_shader_variable_name(&self) -> &StringRef {
        &self.shader_variable_name
    }
}

pub struct RenderTaskBuilder<'a> {
    pub inputs: Vec<RenderTaskInputBuilder<'a>>,
    pub outputs: Vec<RenderTaskOutputBuilder<'a>>,
    pub depth_stencil_grid: &'a mut FlowGridBuilder
}

// TODO Use proper handles to shader variables rather than just their names
pub struct RenderTaskInputBuilder<'a> {
    pub grid: &'a FlowGridBuilder,
    pub shader_variable_name: StringRef
}

pub struct RenderTaskOutputBuilder<'a> {
    pub grid: &'a mut FlowGridBuilder,
    pub shader_variable_name: StringRef
}