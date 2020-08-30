use std::any::Any;

pub trait ColorAttachment {

    fn as_any(&self) -> &dyn Any;
}