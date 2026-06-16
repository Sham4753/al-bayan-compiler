use bayan_compiler::interpreter::Interpreter;
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::generator::Generator;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "شغّل" | "run" => {
            if args.len() < 3 { eprintln!("❌ استخدم: بيان شغّل <ملف.بيان>"); return; }
            let mut interp = Interpreter::new();
            match interp.execute_file(&args[2]) {
                Ok(()) => { println!("\n✅ تم التنفيذ\n"); interp.show_state(); }
                Err(e) => eprintln!("{}", e),
            }
        }
        "حلل" | "analyze" => {
            if args.len() < 3 { eprintln!("❌ استخدم: بيان حلل <كلمة>"); return; }
            let word = &args[2];
            match Musarrif::analyse(word) {
                Ok(a) => {
                    println!("📝 الكلمة: {}", a.original);
                    println!("   🔤 الجذر: {} | ⚖️ الوزن: {} | ⏳ الزمن: {:?}", a.jidhr, a.wazn, a.zaman);
                    let gen = Generator::new();
                    if let Ok(code) = gen.generate(&a) {
                        println!("   ⚙️  {} {}", code.intrinsic, if code.is_parallel {"⚙️متوازي"} else {""});
                    }
                }
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "تفاعلي" | "repl" => {
            println!("🕌 الوضع التفاعلي | اكتب 'خروج' للخروج\n");
            let mut interp = Interpreter::new();
            loop {
                print!("بيان> "); io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if input.is_empty() { continue; }
                if input == "خروج" || input == "exit" { break; }
                if input == "حالة" { interp.show_state(); continue; }
                match interp.execute(input) {
                    Ok(v) => { if !matches!(v, bayan_compiler::runtime::Value::Nothing) { println!("↳ {:?}", v); } }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        _ => help(),
    }
}

fn help() {
    println!("🕌 لغة البيان v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}\n", bayan_compiler::BAYAN_SLOGAN);
    println!("  بيان شغّل <ملف>    تنفيذ برنامج");
    println!("  بيان حلل <كلمة>    تحليل كلمة");
    println!("  بيان تفاعلي        وضع المحادثة");
}
