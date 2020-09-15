use crate::*;

use std::any::Any;

mod topology;

pub use topology::*;

/// A combination of a *ShaderPair* and *PrimitiveTopology*, that is made to work
/// on the concrete *GridGroup*s of a single previously chosen *AbstractGridGroup*.
/// 
/// The *ShaderPair* determines which vertex shader and which fragment shader will
/// be used. The *PrimitiveTopology* determines what kind of structure the input
/// vertices and indices are expected to have (triangles? lines?). 
/// 
/// The *AbstractGridGroup* determines on which concrete *GridGroup*s the pipeline 
/// can be used: it can only draw on the grids of these grid groups. Furthermore, it
/// can only be used during *RenderFlow*s that are bound to the same
/// *AbstractGridGroup*.
pub trait GraphicsPipeline {

    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;

    /// Gets a reference to a *Vec* containing all *ExternalShaderVariable*s of
    /// the *ShaderPair* of this *GraphicsPipeline*. 
    /// 
    /// See the *get_external_variables* method of *ShaderPair* for more
    /// information.
    fn get_external_variables(&self) -> &Vec<ExternalShaderVariable>;
}
