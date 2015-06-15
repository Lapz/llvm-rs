use ffi::prelude::LLVMContextRef;
use ffi::{core, LLVMContext};
use cbox::CBox;

/// Contains all the LLVM entities such as modules.
///
/// Every single entity attached to it has its lifetime to enforce the
/// rule that things from different contexts cannot interact and to keep
/// their pointer safety intact.
pub struct Context;
native_ref!(&Context = LLVMContextRef);
impl Context {
    /// Get a reference to the global context.
    ///
    /// This is marked as unsafe because this can result in undefined behaviour
    /// in a multithreaded context if they all use the same context.
    pub unsafe fn get_global() -> &'static Context {
        core::LLVMGetGlobalContext().into()
    }
    /// Create a new context, owned by the block that calls it.
    pub fn new<'a>() -> CBox<'a, Self> {
        CBox::new(unsafe { core::LLVMContextCreate() })
    }
}
dispose!(Context, LLVMContext, core::LLVMContextDispose);

/// Implemented by everything that is owned by a context.
pub trait GetContext {
    /// Returns a reference to the context that owns this value.
    ///
    /// ```rust
    /// use llvm::*;
    /// let context = Context::new();
    /// let module = Module::new("rowrowfightthapowa", &context);
    /// assert!(context == *module.get_context());
    /// ```
    fn get_context(&self) -> &Context;
}
