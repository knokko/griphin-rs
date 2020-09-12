use crate::*;

use std::sync::Arc;

pub struct GraphicsPipelineBuilder {

    pub shaders: ShaderPair,
    pub topology: PrimitiveTopology,
    pub grids: Arc<dyn GridGroup>
}