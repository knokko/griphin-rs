use crate::*;

/// Represents an output of a drawing node. This consists of a 
/// *DrawingNodeOutputDestination* that specifies what to do with the output data, 
/// and the name of the shader variable from which the data will come.
#[derive(Clone)]
pub struct DrawingNodeOutput {

    destination: DrawingNodeOutputDestination,
    shader_variable_name: String
}

/// This enum specifies what to do with the output data of a drawing node. This can
/// either be transferred to a grid node or be stored in a grid and preserved for
/// longer. 
/// 
/// See the documentation of the individual possibilities for more information 
/// about them.
#[derive(Clone)]
pub enum DrawingNodeOutputDestination {

    /// The data should be stored in a grid node of the same render flow. This data
    /// can then be used as input for another drawing node (but is not necessary).
    Internal(GridNodeID),

    /// The data should be stored in the given grid and remain there at least until
    /// the end of the *RenderFlow*. The goal of this option is to use the data
    /// somewhere outside the current render flow, but it's not the concern of this
    /// node or of the render flow what its purpose is. This could for instance be
    /// used as external input for another render flow or be presented to the screen.
    External(Grid)
}