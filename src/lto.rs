use ffi::link_time_optimizer::*;
use libc::{c_void,c_char};
use std::ops::Drop;

/// Dummy type for pointers to the LTO object
type LTOObject = *mut c_void;

/// This struct represents a llvm LTO object
pub struct LTOManager {
    object: LTOObject
}

pub enum LTOStatus {

}


impl LTOManager{
    pub fn new() -> LTOManager {
        LTOManager {
            object: unsafe{
                 llvm_create_optimizer()
            }
        }
    }
    
    pub fn optimize_module(&self,output_file:&str) -> LTOStatus { 
        unsafe {
            llvm_optimize_modules(self.object,output_file.as_ptr() as *const c_char)
        }.into()
    }

    pub fn read_object_file(&self,input_file:&str) -> LTOStatus {
        unsafe{llvm_read_object_file(self.object,input_file.as_ptr() as *const c_char)}.into()
    }
}


impl Drop for LTOManager {
    fn drop(&mut self) {
        unsafe {
            llvm_destroy_optimizer(self.object)
        }
    }
}


impl From<llvm_lto_status_t> for LTOStatus {
    fn from(error:llvm_lto_status_t) -> LTOStatus {
        unimplemented!()
    } 
}