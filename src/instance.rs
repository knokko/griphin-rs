use crate::*;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// The Griphin *Instance* is the root of a Griphin implementation. All important
/// Griphin objects can be created directly or indirectly from the *Instance*.
///
/// Every Griphin implementation should have a struct that implements *Instance*.
/// Users of Griphin should create such an instance (and depend directly on that
/// specific Griphin implementation) during start-up of the application. The code
/// that constructs that instance struct should be the only code that depends
/// directly on the Griphin implementation (the rest of the code should just see
/// this struct as a dynamic Griphin *Instance* and only depend on this crate).
///
/// That instance struct may need quite some parameters, but that will depend on the
/// implementation (a Vulkan-based instance is probably harder to construct than an
/// OpenGL-based instance). But once it is constructed, it should not need any more
/// information than defined by the methods of this crate.
pub trait Instance: Debug + Send + Sync + 'static {
    /// Gets the *ShaderManager* of this *Instance*. The shader manager handles all
    /// tasks strongly related to shaders. It is a separate trait so that the
    /// *Instance* trait itself can stay small.
    fn get_shader_manager(&self) -> Arc<dyn ShaderManager>;

    /// Gets the *Gateway* of this *Instance*. The gateway handles most tasks that require a lot of
    /// data to be transferred from the CPU to the GPU (like vertices and textures). It is a
    /// separate trait so that the *Instance* itself can stay small.
    fn get_gateway(&self) -> Arc<dyn Gateway>;

    /// Creates a new *AbstractGridGroup* based on the information provided in the
    /// given *AbstractGridGroupBuilder*. This method will return a tuple of an
    /// *AbstractGridGroup* (within an *Arc*) and *GridGroupIDs*. See the
    /// documentation of *AbstractGridGroup(Builder)* and *GridGroupIDs* for more
    /// information. The *builder* can be modified after this call returns, but
    /// changes made afterwards won't be reflected in the result of this method.
    fn create_abstract_grid_group(
        &self,
        builder: &AbstractGridGroupBuilder,
    ) -> (Arc<dyn AbstractGridGroup>, GridGroupIDs);

    /// This method should be used by the Griphin implementation to get access to
    /// the implementation-specific struct that implements this trait.
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct DummyInstance {
        number: u8,
    }

    impl DummyInstance {
        fn new(number: u8) -> Arc<Self> {
            Arc::new(Self { number })
        }

        fn downcast<R>(
            instance: Arc<dyn Instance>,
            use_function: &mut dyn FnMut(&DummyInstance) -> R,
        ) -> R {
            use_function(instance.as_any().downcast_ref::<DummyInstance>().unwrap())
        }
    }

    impl Instance for DummyInstance {
        fn get_shader_manager(&self) -> Arc<dyn ShaderManager> {
            unimplemented!()
        }

        fn get_gateway(&self) -> Arc<dyn Gateway> {
            unimplemented!()
        }

        fn create_abstract_grid_group(
            &self,
            _builder: &AbstractGridGroupBuilder,
        ) -> (Arc<dyn AbstractGridGroup>, GridGroupIDs) {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn Any {
            self
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
