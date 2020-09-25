use crate::*;

use std::any::Any;

// TODO Make this trait actually usable
pub trait ColorGrid {
    fn as_any(&self) -> &dyn Any;

    fn debug_dump(&self, region: GridRegion) -> ColorGridDump;
}

/// Every *ColorGrid(Builder)* has an associated *ColorStartOperation*. This
/// operation determines what should be done with the color grid when its
/// *RenderFlow* starts. See the documentation of each of the possibilities
/// for more information.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorStartOperation {
    /// The colors the grid had before the start of the *RenderFlow* should
    /// be preserved during the render flow. This is needed when you want to
    /// read colors that were stored in a previous render flow (or were
    /// stored in a color grid by other means).
    Preserve, // Use pre-init image layout

    /// The colors of the grid should be cleared to some fixed value when
    /// its *RenderFlow* starts. This value can be chosen right before the
    /// render flow starts.
    Clear, // Use undefined image layout

    /// It doesn't matter which colors the grid has before the start of the
    /// render flow. This is for instance the case when you are going to
    /// write to the grid before reading from it.
    DontCare, // Use undefined image layout
}

/// Every *ColorGrid(Builder)* has an associated *ColorPurpose*. This purpose
/// states what the color grid is going to be used for after its
/// *RenderFlow* has ended. See the documentation of the possibilities for
/// more information.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorPurpose {
    /// The colors stored in the grid are no longer needed once the *RenderFlow*
    /// is over.
    Nothing,

    /// The colors in the grid are going to be displayed to the screen.
    Display,

    /// The colors in the grid are going to be read by a shader in a different
    /// *RenderFlow*.
    ShaderRead,

    /// The colors are going to be copied to some other color grid.
    Transfer, // Transfer source

    /// The colors are going to be replaced by the colors of some other color grid.
    Replace, // Transfer destination
}

/// A struct that contains the information necessary to create an
/// *AbstractColorGrid*. They have a *ColorStartOperation* and a *ColorPurpose*,
/// which define what should happen with the color grid at the beginning and
/// the end of *RenderFlow*s. See the documentation of these two types for more
/// information.
///
/// To use an instance of this struct, add it to the *color_grids* of an
/// *AbstractGridGroupBuilder*.
pub struct AbstractColorGridBuilder {
    /// What should happen with the grid upon the start of its *RenderFlow*. See
    /// the documentation of *ColorStartOperation* for more information.
    pub start_operation: ColorStartOperation,

    /// What the colors in the grid will be used for after its *RenderFlow* has
    /// ended. See the documentation of *ColorPurpose* for more information.
    pub purpose: ColorPurpose,
}
