use ffi::lto::*;
use std::ops::Drop;
use cbox::CBox;
use std::ffi::{CStr, CString};
use libc::c_char;
use object::Symbol;
use std::mem;

struct LTOCodeGenerator(*mut LLVMOpaqueLTOCodeGenerator);

struct LTOModule(*mut LLVMOpaqueLTOModule);


impl LTOCodeGenerator {
    pub fn new() -> Self {
        LTOCodeGenerator(unsafe { lto_codegen_create() })
    }

    fn get(&self) -> *mut LLVMOpaqueLTOCodeGenerator {
        self.0
    }

    pub fn add_module(&self, module: LTOModule) -> Result<(), String> {
        if unsafe { lto_codegen_add_module(self.get(), module.0) } == (false as u8) {
            Ok(())
        } else {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };

            Err(CString::from(error).into_string().unwrap())
        }
    }

    /// Adds a symbol to the list of global symbols that must exist in the final
    /// generated code. If a function is not listed there, it might be inlined into
    ///  every usage and optimized away. For every single module, the functions
    /// referenced from code outside of the ThinLTO modules need to be added here.
    pub fn preserve_symbol(&self, symbol: Symbol) {
        unsafe { lto_codegen_add_must_preserve_symbol(self.get(), symbol.get()) }
    }

    pub fn compile(&self) {
        unimplemented!()
    }

    /// Sets debug option
    pub fn debug_options(&self, opts: &str) {
        unsafe { lto_codegen_debug_options(self.get(), opts.as_ptr() as *const c_char) }
    }

    /// Runs optimization for the merged module
    pub fn optimize(&self) -> Result<(), String> {
        if unsafe { lto_codegen_optimize(self.get()) } == true as u8 {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    /// Sets extra arguments that libLTO should pass to the assembler
    pub fn assembler_args(&self,args:Vec<&str>) {
        unsafe {
            lto_codegen_set_assembler_args(
                self.get(),
                args.join("").as_ptr() as *mut *const c_char,
                args.len() as i32,
            )
        }
    }

    /// Sets the location of the assembler tool to run. If not set, libLTO
    /// will use gcc to invoke the assembler
    pub fn assembler_path(&self,path:&str)  {
        unsafe {
            lto_codegen_set_assembler_path(self.get(),path.as_ptr() as *const c_char)
        }
    }

    /// Sets the cpu to generate code for
    pub fn set_cpu(&self, cpu:&str) {
        unsafe {
            lto_codegen_set_cpu(self.get(),cpu.as_ptr() as *const c_char)
        }
    }
    /// Sets if debug info should be generated.
    pub fn set_debug_model(&self,model:LTODebugModel) -> Result<(),String> {
        if unsafe{lto_codegen_set_debug_model(self.get(),model.into())} == true as u8 {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        }else {
            Ok(())
        }
    }
}

impl Drop for LTOCodeGenerator {
    fn drop(&mut self) {
        unsafe { lto_codegen_dispose(self.0) }
    }
}

impl From<LTODebugModel>  for lto_debug_model { 
    fn from(model:LTODebugModel) -> lto_debug_model {
        match model {
            LTODebugModel::None =>lto_debug_model::LTO_DEBUG_MODEL_NONE ,
            LTODebugModel::DWARF => lto_debug_model::LTO_DEBUG_MODEL_DWARF,
        }
    }
}

enum LTODebugModel {
    DWARF,
    None
}
