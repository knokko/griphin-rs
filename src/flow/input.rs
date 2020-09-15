use crate::*;

/// Represents an input of a drawing node. This consists of a 
/// *DrawingNodeInputSource* that specifies where the input data comes from,
/// and the name of the shader variable to which this data will be bound.
#[derive(Clone)]
pub struct DrawingNodeInput {

    pub source: DrawingNodeInputSource,
    pub shader_variable_name: String
}

/// This enum specifies where the input data of a drawing node should come from. 
/// See the documentation of each of the possibilities for more information.
#[derive(Clone)]
pub enum DrawingNodeInputSource {

    /// The input values should be cleared to some fixed value, which can be
    /// chosen at runtime.
    Cleared(AbstractGridID),

    /// The input values should come from a grid node (of the same render flow).
    Internal(GridNodeID), 

    /// A source from outside the render flow should ensure the input values are
    /// stored in the given grid before the flow starts.
    External(AbstractGridID),

    /// The input values are attributes of the Model that is about to be drawn.
    Model,

    /// The input value is a Texture that is chosen right before drawing, but
    /// submitted to the gpu in advance.
    Texture,

    /// The input value is a global variable that is chosen right before
    /// drawing.
    Uniform
}