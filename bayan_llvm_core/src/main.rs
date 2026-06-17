use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use std::error::Error;

type MainFunc = unsafe extern "C" fn() -> i64;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("بسم_الله");
    let builder = context.create_builder();
    let engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive)?;

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // اِحتَسَبَ = 64
    // حَفِظَ = 256
    // بَعَثَ = 8080
    let result = i64_type.const_int(64, false); // اِحتَسَبَ
    builder.build_return(Some(&result));

    println!("🕌 البيان ← LLVM IR ← Machine Code");
    println!("╔══════════════════════════╗");
    println!("║   LLVM IR:               ║");
    module.print_to_stderr();
    println!("╚══════════════════════════╝");

    unsafe {
        let main: JitFunction<MainFunc> = engine.get_function("main")?;
        let value = main.call();
        println!("\n✅ النتيجة: {}", value);
        println!("🧠 اِحتَسَبَ = {} (الذاكرة: {}MB)", value, value);
    }

    Ok(())
}
