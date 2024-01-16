use crate::llvm_version;
use llvm_sys::prelude::{
    LLVMTypeRef,
    LLVMValueRef,
};
use llvm_sys::core::{
    LLVMPrintValueToString,
    LLVMReplaceAllUsesWith,
    LLVMDumpValue,
    LLVMTypeOf,
};

#[llvm_version(4..7)]
use llvm_sys::core::{
    LLVMGetValueName,
    LLVMSetValueName,
};


#[llvm_version(8..17)]
use llvm_sys::core::{
    LLVMGetValueName2 as LLVMGetValueName,
    LLVMSetValueName2 as LLVMSetValueName,
};

macro_rules! values {
    (int($ty:expr) $e:expr) => {
        crate::value::int_value::IntValue::new_const($e, $ty, false)
    };
    (int($ty:expr) $e:expr, sign_extend) => {
        crate::value::int_value::IntValue::new_const($e, $ty, true)
    };
    (int_ref $e:expr) => {
        crate::value::int_value::IntValue::new_llvm_ref($e)
    };

    (float($ty:expr) $e:expr) => {
        crate::value::float_value::FloatValue::new_const($e, $ty)
    };

    (float_ref $e:expr) => {
        crate::value::float_value::FloatValue::new_llvm_ref($e)
    };

    // (fn $name:expr ($($t:ty),*) -> $r:ty { $e:expr }) => {
    //     crate::value::function_value::FunctionValue::new_constant(
    //         crate::types::function_types::FunctionType::new(
    //             vec![$(crate::types!($t)),*],
    //             crate::types!($r),
    //             false
    //         ),
    //         $e,
    //         $name
    //     )
    // };
}


use crate::types::{Type, TypeEnum};

pub trait Value {
    fn get_type_ref(&self) -> LLVMTypeRef;
    fn get_type(&self) -> TypeEnum;

    fn as_value_ref(&self) -> LLVMValueRef;
    fn is_null_or_undef(&self) -> bool;
    fn is_const(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_undef(&self) -> bool;
    fn print_to_string(&self) -> String {
        let llvm_str = unsafe { LLVMPrintValueToString(self.as_value_ref()) };
        let str_slice = unsafe { std::ffi::CStr::from_ptr(llvm_str) }.to_str().unwrap();
        let string = str_slice.to_owned();
        string
    }
    fn print_to_stderr(&self) {
        eprintln!("{}", self.print_to_string());
    }
    fn replace_all_uses_with(&self, other: &dyn Value) {
        unsafe { LLVMReplaceAllUsesWith(self.as_value_ref(), other.as_value_ref()) };
    }
    fn set_name(&self, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { LLVMSetValueName(self.as_value_ref(), name.as_ptr(), name.as_bytes().len()) }
    }
    fn get_name(&self) -> &str {
        let name = unsafe { LLVMGetValueName(self.as_value_ref(), &mut 0) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        let name = name.to_str().unwrap();
        name
    }
    fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_value_ref()) }
    }
}

pub mod int_value;
pub mod float_value;
pub mod function_value;

pub enum ValueEnum {
    IntValue(int_value::IntValue),
    FloatValue(float_value::FloatValue),
    FunctionValue(function_value::FunctionValue),
}

impl ValueEnum {
    pub fn get_type(&self) -> TypeEnum {
        match self {
            ValueEnum::IntValue(int_value) => int_value.get_type(),
            ValueEnum::FloatValue(float_value) => float_value.get_type(),
            ValueEnum::FunctionValue(function_value) => function_value.get_type(),
        }
    }

    pub fn as_llvm_ref(&self) -> LLVMValueRef {
        match self {
            ValueEnum::IntValue(int_value) => int_value.as_value_ref(),
            ValueEnum::FloatValue(float_value) => float_value.as_value_ref(),
            ValueEnum::FunctionValue(function_value) => function_value.as_value_ref(),
        }
    }

    pub fn into_int_value(self) -> int_value::IntValue {
        match self {
            ValueEnum::IntValue(int_value) => int_value,
            _ => panic!("Not an int value"),
        }
    }
}

impl Into<ValueEnum> for LLVMValueRef {
    fn into(self) -> ValueEnum {
        let value_type = unsafe { LLVMTypeOf(self) };
        let value_type_enum = value_type.into();
        match value_type_enum {
            TypeEnum::IntType(_) => ValueEnum::IntValue(int_value::IntValue::new_llvm_ref(self)),
            TypeEnum::FloatType(_) => ValueEnum::FloatValue(float_value::FloatValue::new_llvm_ref(self)),
            TypeEnum::FunctionType(_) => ValueEnum::FunctionValue(function_value::FunctionValue::new_llvm_ref(self)),
            _ => panic!("Unknown type"),
        }
    }
}

impl Into<LLVMValueRef> for ValueEnum {
    fn into(self) -> LLVMValueRef {
        match self {
            ValueEnum::IntValue(int_value) => int_value.as_value_ref(),
            ValueEnum::FloatValue(float_value) => float_value.as_value_ref(),
            ValueEnum::FunctionValue(function_value) => function_value.as_value_ref(),
        }
    }
}