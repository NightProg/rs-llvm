use rs_llvm::context::Context;
use rs_llvm::execution_engine::ExecutionEngine;
use rs_llvm::types::function_types::FunctionType;
use rs_llvm::value::ValueEnum;

fn main() {
    let context = Context::new();
    let module = context.new_module("test");
    let mut builder = context.new_builder();

    let fn_type = FunctionType::new(vec![
        context.i32_type().to_type_enum(),
        context.i32_type().to_type_enum(),
    ], context.i32_type().to_type_enum(), false);

    let function = module.add_function("add", fn_type);

    let basic_block = context.append_basic_block("entry", function);

    builder.position_at_end(basic_block);
    builder.build_ret(
        ValueEnum::IntValue(
            builder.build_int_add(
                &function.get_param_nth(0).unwrap().into_int_value(),
                &function.get_param_nth(1).unwrap().into_int_value(),
                "add",
            )
        )
    );

    module.dump();

    ExecutionEngine::init();
    let execution_engine = ExecutionEngine::new_with_module(&module.clone());


    let function = execution_engine.get_function_address("add");
    let function: extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(function) };

    println!("{}", function(1, 2));





}