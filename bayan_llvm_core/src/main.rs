use inkwell::context::Context;
use inkwell::targets::{Target, TargetMachine, FileType, RelocMode, CodeModel};
use inkwell::OptimizationLevel;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Target::initialize_native(&Default::default())?;

    let triple = TargetMachine::get_default_triple();
    let target = Target::get_first().ok_or("no target")?;
    let machine = target.create_target_machine(
        &triple, "generic", "",
        OptimizationLevel::Aggressive, RelocMode::Default, CodeModel::Default,
    ).ok_or("no machine")?;

    let context = Context::create();
    let module = context.create_module("بسم_الله");
    let builder = context.create_builder();
    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    builder.build_return(Some(&i64_type.const_int(64, false)));

    machine.write_to_file(&module, FileType::Object, "/tmp/bayan.o".as_ref())?;
    println!("✅ Object file: /tmp/bayan.o");
    println!("🧠 اِحتَسَبَ = 64MB");
    Ok(())
}
