use bayan_compiler::bayan_engine::BayanEngine;
use std::env;

fn main() {
    println!("🕌 لغة البيان - المحرك المستقل v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}\n", bayan_compiler::BAYAN_SLOGAN);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("الاستخدام:");
        println!("  بيان شغّل \"ملف.بيان\"   # تنفيذ ملف بيان");
        println!("  بيان                    # عرض هذه المساعدة");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "شغّل" | "run" => {
            if args.len() < 3 {
                eprintln!("❌ يرجى تحديد ملف .بيان");
                return;
            }
            let file_path = &args[2];
            let mut engine = BayanEngine::new();

            match engine.execute_file(file_path) {
                Ok(()) => engine.show_memory(),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        _ => {
            println!("أمر غير معروف: '{}'", command);
            println!("استخدم: بيان شغّل \"ملف.بيان\"");
        }
    }
}
