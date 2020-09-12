use std::any::Any;

pub trait ColorGrid {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorStartOperation {
    Preserve, // Use pre-init image layout
    Clear,    // Use undefined image layout
    DontCare, // Use undefined image layout
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorPurpose {
    Nothing,
    Display,
    ShaderRead,
    Transfer, // Transfer source
    Replace,  // Transfer destination
}

pub struct ColorGridBuilder {

    pub start_operation: ColorStartOperation,
    pub purpose: ColorPurpose
}