use std::any::Any;

// TODO Make this trait usable
pub trait DepthStencilGrid {
    fn as_any(&self) -> &dyn Any;
}

pub enum DepthStencilStartOperation {
    Preserve,
    Clear,
    DontCare,
}

pub enum DepthStencilPurpose {
    Nothing,
    ShaderRead,
    Transfer, // Transfer source
    Replace,  // Transfer destination
}

pub struct AbstractDepthStencilGridBuilder {

    pub start_operation: DepthStencilStartOperation,
    pub purpose: DepthStencilPurpose
}