use llvm_sys::prelude::LLVMModuleRef;
use llvm_sys::core::{
    LLVMModuleCreateWithNameInContext,
    LLVMDumpModule,
    LLVMAppendModuleInlineAsm,
    LLVMAddFunction
};
use crate::context::Context;
use crate::types::function_types::FunctionType;
use crate::value::function_value::FunctionValue;

#[derive(Clone)]
pub struct Module {
    pub(crate) module: LLVMModuleRef,
    pub(crate) context: Context,
}

impl Module {
    pub fn new(name: &str, context: Context) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let module = unsafe {
            LLVMModuleCreateWithNameInContext(name.as_ptr(), context.context)
        };
        Self { module, context }
    }

    pub fn get_context(&self) -> Context {
        self.context.clone()
    }



    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }

    pub fn push_asm(&self, asm_code: String)  {
        let asm_code = std::ffi::CString::new(asm_code).unwrap();
        let len = asm_code.clone().into_bytes().len();
        unsafe {
            LLVMAppendModuleInlineAsm(self.module, asm_code.as_ptr(), len);
        }
    }

    pub fn add_function(&self, name: &str, function_type: FunctionType) -> FunctionValue {
        let name = std::ffi::CString::new(name).unwrap();
        let function = unsafe {
            LLVMAddFunction(self.module, name.as_ptr(), function_type.get_type_ref())
        };
        FunctionValue::new_llvm_ref(function)
    }
    
}