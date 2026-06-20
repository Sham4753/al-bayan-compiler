use inkwell::context::Context;
use inkwell::targets::{Target, TargetMachine, FileType, RelocMode, CodeModel};
use inkwell::OptimizationLevel;
use std::error::Error;
use std::collections::HashMap;

/// ترجمة الجذور العربية إلى قيم عددية
fn translate_root(root: &str) -> u64 {
    let mut map = HashMap::new();
    map.insert("فَتَحَ", 80);    // فتح منفذ HTTP
    map.insert("حَفِظَ", 256);   // تشفير AES-256
    map.insert("بَعَثَ", 1);     // إرسال
    map.insert("حَلَّلَ", 0);    // تحليل
    map.insert("اِحتَسَبَ", 64); // مسح
    map.insert("خَزَنَ", 512);   // تخزين
    map.insert("جَمَعَ", 1024);  // تجميع
    map.insert("رَسَمَ", 2048);  // رسم
    *map.get(root).unwrap_or(&0)
}

fn main() -> Result<(), Box<dyn Error>> {
    // إعداد LLVM
    Target::initialize_native(&Default::default())?;
    let triple = TargetMachine::get_default_triple();
    let target = Target::get_first().ok_or("no target")?;
    let machine = target.create_target_machine(
        &triple, "generic", "",
        OptimizationLevel::Aggressive, RelocMode::Default, CodeModel::Default,
    ).ok_or("no machine")?;

    let context = Context::create();
    let module = context.create_module("البيان");
    let builder = context.create_builder();
    let i64_type = context.i64_type();

    // إنشاء دالة لكل جذر
    let roots = vec!["فَتَحَ", "حَفِظَ", "بَعَثَ", "حَلَّلَ", "اِحتَسَبَ", "خَزَنَ", "جَمَعَ", "رَسَمَ"];
    for root in &roots {
        let fn_type = i64_type.fn_type(&[], false);
        let function = module.add_function(root, fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let value = translate_root(root);
        builder.build_return(Some(&i64_type.const_int(value, false)));
    }

    // كتابة الملف الهدف (Object File)
    machine.write_to_file(&module, FileType::Object, "/tmp/bayan_roots.o".as_ref())?;
    println!("✅ تم بناء ملف الكائن: /tmp/bayan_roots.o");
    println!("🧠 الجذور المُترجمة: {:?}", roots);
    println!("⚡ البيان الآن أسرع – تُنفَّذ مباشرة على المعالج!");
    Ok(())
}
