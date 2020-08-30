use std::any::Any;

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
