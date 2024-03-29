use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::core::{
    LLVMContextCreate,
    LLVMAppendBasicBlockInContext,
};
use crate::basic_block::BasicBlock;
use crate::builder::Builder;

use crate::types::int_types;
use crate::module::Module;
use crate::value::function_value::FunctionValue;

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub(crate) context: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        let context = unsafe { LLVMContextCreate() };
        Self { context }
    }

    pub fn new_module(&self, name: &str) -> Module {
        Module::new(name, self.clone())
    }

    pub fn new_builder(&self) -> Builder {
        Builder::new(self)
    }

    pub fn i1_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(1, self.clone())
    }

    fn i8_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(8, self.clone())
    }

    pub fn i16_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(16, self.clone())
    }

    pub fn i32_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(32, self.clone())
    }

    pub fn i64_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(64, self.clone())
    }

    pub fn append_basic_block(&self, name: &str, fn_value: FunctionValue) -> BasicBlock {
        let name = std::ffi::CString::new(name).unwrap();
        let block = unsafe { LLVMAppendBasicBlockInContext(self.context, fn_value.function_value, name.as_ptr()) };
        BasicBlock::new(block, self.clone())
    }
}