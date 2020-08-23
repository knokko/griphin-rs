use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Instance : Debug + Send + Sync + 'static {

    fn create_framebuffer(&self, width: u32, height: u32) -> Arc<dyn Framebuffer>;

    fn create_graphics_pipeline(&self) -> Arc<dyn GraphicsPipeline>;

    fn get_shader_manager(&self) -> Arc<dyn ShaderManager>;

    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct DummyInstance {

        number: u8
    }

    impl DummyInstance {
        
        fn new(number: u8) -> Arc<Self> {
            Arc::new(Self {
                number
            })
        }

        fn downcast<R>(instance: Arc<dyn Instance>, use_function: &mut dyn FnMut(&DummyInstance) -> R) -> R {
            use_function(instance.as_any().downcast_ref::<DummyInstance>().unwrap())
        }
    }

    impl Instance for DummyInstance {

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn create_framebuffer(&self, _width: u32, _height: u32) -> Arc<dyn Framebuffer> {
            panic!("This dummy instance doesn't support framebuffers")
        }

        fn get_shader_manager(&self) -> Arc<dyn ShaderManager> {
            panic!("This dummy instance doesn't support shaders")
        }
    }

    #[test]
    fn test_any() {
        let original = create_dummy_instance(7);
        test_any_get(original, 7);
    }

    fn create_dummy_instance(number: u8) -> Arc<dyn Instance> {
        DummyInstance::new(number)
    }

    fn test_any_get(instance: Arc<dyn Instance>, expected: u8) {
        let number = DummyInstance::downcast(instance, &mut |dummy| dummy.number);
        assert_eq!(expected, number);
    }

    #[test]
    fn test_multithreading() {
        let original = create_dummy_instance(8);
        let cloned = Arc::clone(&original);
        std::thread::spawn(|| {
            test_any_get(cloned, 8);
        });
        test_any_get(original, 8);
    }
}