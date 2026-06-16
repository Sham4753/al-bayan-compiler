use bayan_compiler::interpreter::Interpreter;
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::parser::SentenceParser;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help(); return; }

    match args[1].as_str() {
        "شغّل" | "run" => {
            if args.len() < 3 { eprintln!("❌ بيان شغّل <ملف.بيان>"); return; }
            let mut interp = Interpreter::new();
            if let Err(e) = interp.execute_file(&args[2]) {
                eprintln!("{}", e);
            }
        }
        "حلل" | "analyze" => {
            if args.len() < 3 { eprintln!("❌ بيان حلل <كلمة>"); return; }
            match Musarrif::analyse(&args[2]) {
                Ok(a) => println!("📝 {} | 🔤 جذر:{} | ⚖️ وزن:{} | ⏳ {:?}", a.original, a.jidhr, a.wazn, a.zaman),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "جملة" | "sentence" => {
            if args.len() < 3 { eprintln!("❌ بيان جملة <نص>"); return; }
            let text = &args[2..].join(" ");
            match SentenceParser::parse(text) {
                Ok(s) => println!("{}", s.execute()),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "تفاعلي" | "repl" => repl_mode(),
        _ => help(),
    }
}

fn repl_mode() {
    println!("🕌 الوضع التفاعلي | 'خروج' للخروج\n");
    let mut interp = Interpreter::new();
    loop {
        print!("بيان> "); io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "خروج" || input == "exit" { break; }
        if input == "حالة" { interp.show_state(); continue; }

        if input.contains(' ') {
            match SentenceParser::parse(input) {
                Ok(s) => {
                    println!("{}", s.execute());
                    if let Some(ref verb) = s.verb {
                        if let Err(e) = interp.execute(&verb.original) {
                            eprintln!("{}", e);
                        }
                    }
                    continue;
                }
                Err(_) => {}
            }
        }

        match interp.execute(input) {
            Ok(v) => println!("↳ {:?}", v),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn help() {
    println!("🕌 لغة البيان v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}\n", bayan_compiler::BAYAN_SLOGAN);
    println!("  بيان شغّل <ملف>      تنفيذ برنامج");
    println!("  بيان حلل <كلمة>      تحليل كلمة");
    println!("  بيان جملة <نص>       تحليل جملة فعلية");
    println!("  بيان تفاعلي           وضع المحادثة");
}
