pub trait ColorGrid {}

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
