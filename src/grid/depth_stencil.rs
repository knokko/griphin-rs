use std::any::Any;

// TODO Make this trait usable
pub trait DepthStencilGrid {
    fn as_any(&self) -> &dyn Any;
}

/// Every *DepthStencilGrid(Builder)* has an associated
/// *DepthStencilStartOperation*. This operation determines what should be done
/// with the depth/stencil grid when its *RenderFlow* starts. See the
/// documentation of each of the possibilities for more information.
pub enum DepthStencilStartOperation {
    /// The depth and stencil values that are stored before a *RenderFlow* starts,
    /// should be preserved during the render flow (until it is overwritten by
    /// a drawing node within the render flow). This start operation is needed
    /// when you need to read data that was written during another render flow.
    Preserve,

    /// The depth and stencil values should be cleared to a fixed value at the
    /// start of its *RenderFlow*. This value can be chosen right before the
    /// render flow starts (this value is pretty much always the same for the
    /// depth values, but more interesting for the stencil values).
    Clear,

    /// It doesn't matter what happens with the depth and stencil values that
    /// happen to be in the grid before the render flow starts. This is often
    /// the case when the values are going to be overwritten before they are
    /// read (or the values are never read at all).
    DontCare,
}

/// Every *DepthStencilGrid(Builder)* has an associated *DepthStencil*.
/// This purpose states what the depth stencil values in the grid are going
/// to be used for after its *RenderFlow* has ended. See the documentation of
/// the possibilities for more information.
pub enum DepthStencilPurpose {
    /// The depth and stencil values in the grid won't be used (anymore) after
    /// its *RenderFlow* has ended.
    Nothing,

    /// The depth and/or stencil values in the grid will be read by a shader in
    /// a different *RenderFlow*.
    ShaderRead,

    /// The depth and stencil values in the grid will be copied to a different
    /// depth stencil grid.
    Transfer, // Transfer source

    /// The depth and stencil values in the grid will be replaced by the depth
    /// and stencil values of a different depth stencil grid.
    Replace, // Transfer destination
}

/// A struct that contains the information necessary to create an
/// *AbstractDepthStencilGrid*. They have a *DepthStencilStartOperation* and a
/// *DepthStencilPurpose*, which define what should happen with the depth stencil
/// grids at the beginning and the end of *RenderFlow*s. See the documentation of
/// these two types for more information.
///
/// To use an instance of this struct, add it to the *depth_stencil_grids* of an
/// *AbstractGridGroupBuilder*.
pub struct AbstractDepthStencilGridBuilder {
    /// What should happen with the grid upon the start of its *RenderFlow*. See
    /// the documentation of *DepthStencilStartOperation* for more information.
    pub start_operation: DepthStencilStartOperation,

    /// What the depth and/or stencil values in the grid will be used for after
    /// its *RenderFlow* has ended. See the documentation of *DepthStencilPurpose*
    /// for more information.
    pub purpose: DepthStencilPurpose,
}
