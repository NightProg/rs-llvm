#![feature(core_intrinsics)]
pub mod types;
pub mod module;
pub mod context;
pub mod value;
pub mod builder;
pub mod execution_engine;
pub mod basic_block;

pub(crate) use rs_llvm_macro::llvm_version;


#[llvm_version(4)]
extern crate llvm_sys_40 as llvm_sys;

#[llvm_version(5)]
extern crate llvm_sys_50 as llvm_sys;

#[llvm_version(6)]
extern crate llvm_sys_60 as llvm_sys;

#[llvm_version(7)]
extern crate llvm_sys_70 as llvm_sys;

#[llvm_version(8)]
extern crate llvm_sys_80 as llvm_sys;

#[llvm_version(9)]
extern crate llvm_sys_90 as llvm_sys;

#[llvm_version(10)]
extern crate llvm_sys_100 as llvm_sys;

#[llvm_version(11)]
extern crate llvm_sys_110 as llvm_sys;

#[llvm_version(12)]
extern crate llvm_sys_120 as llvm_sys;

#[llvm_version(13)]
extern crate llvm_sys_130 as llvm_sys;

#[llvm_version(14)]
extern crate  llvm_sys_140 as llvm_sys;

#[llvm_version(15)]
extern crate llvm_sys_150 as llvm_sys;

#[llvm_version(16)]
extern crate llvm_sys_160 as llvm_sys;

#[llvm_version(17)]
extern crate llvm_sys_170 as llvm_sys;