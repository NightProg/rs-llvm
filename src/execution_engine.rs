use llvm_sys::execution_engine::{
    LLVMExecutionEngineRef,
    LLVMGetFunctionAddress,
    LLVMCreateExecutionEngineForModule,
    LLVMLinkInMCJIT
};

use llvm_sys::target::{
    LLVM_InitializeNativeTarget,
    LLVM_InitializeNativeAsmPrinter
};
use crate::module::Module;

#[derive(Debug)]
pub struct ExecutionEngine {
    pub(crate) execution_engine: LLVMExecutionEngineRef,
}

impl ExecutionEngine {

    pub fn init() {
        unsafe {
            LLVMLinkInMCJIT();
            assert_eq!(LLVM_InitializeNativeTarget(), 0);
            assert_eq!(LLVM_InitializeNativeAsmPrinter(), 0);

        }
    }
    pub fn new(execution_engine: LLVMExecutionEngineRef) -> Self {
        Self { execution_engine }
    }

    pub fn new_with_module(module: &Module) -> Self {
        let execution_engine: LLVMExecutionEngineRef;
        unsafe {
            let mut ee = std::mem::MaybeUninit::uninit();
            let mut err = std::mem::zeroed();
            let result = LLVMCreateExecutionEngineForModule(ee.as_mut_ptr(), module.module, &mut err);
            if result != 0 {
                assert!(!err.is_null());
                let err = std::ffi::CStr::from_ptr(err).to_str().unwrap();
                panic!("Failed to create execution engine: {}", err);
            }
            execution_engine = ee.assume_init();
        }

        Self { execution_engine }
    }

    pub fn get_execution_engine(&self) -> LLVMExecutionEngineRef {
        self.execution_engine
    }

    pub fn get_function_address(&self, name: &str) -> u64 {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { LLVMGetFunctionAddress(self.execution_engine, name.as_ptr())  }
    }
}