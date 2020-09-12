use crate::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct NodeInput {

    source: NodeInputSource,
    shader_variable_name: String
}

#[derive(Clone)]
pub enum NodeInputSource {

    /// The input values should be cleared to some fixed value
    Cleared(Grid),

    /// The input values should come from an output of another Node in the same
    /// RenderFlow
    Internal(Arc<GridState>), 

    /// A source from outside the RenderFlow should ensure the input values are
    /// set to the right value before the flow starts
    External(Grid),

    /// The input values are attributes of the Model that is about to be drawn
    Model,

    /// The input value is a Texture that is chosen right before drawing, but
    /// submitted to the gpu in advance
    Texture,

    /// The input value is a global variable that is chosen right before
    /// drawing
    Uniform
}