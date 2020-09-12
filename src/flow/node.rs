use crate::*;

use std::any::Any;
use std::sync::Arc;

pub trait Node {

    fn as_any(&self) -> &dyn Any;
}

pub struct NodeBuilder {

    pub pipeline: Arc<dyn GraphicsPipeline>,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>
}