use bayan_compiler::interpreter::Interpreter;
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::parser::SentenceParser;
use bayan_compiler::optimizer::CodeOptimizer;
use bayan_compiler::balagha::BalaghaAnalyzer;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help(); return; }

    match args[1].as_str() {
        "شغّل" | "run" => {
            if args.len() < 3 { eprintln!("❌ بيان شغّل <ملف.بيان>"); return; }
            let mut interp = Interpreter::new();
            if let Err(e) = interp.execute_file(&args[2]) { eprintln!("{}", e); }
        }
        "حلل" | "analyze" => {
            if args.len() < 3 { eprintln!("❌ بيان حلل <كلمة>"); return; }
            match Musarrif::analyse(&args[2]) {
                Ok(a) => println!("📝 {} | جذر:{} | وزن:{} | {:?}", a.original, a.jidhr, a.wazn, a.zaman),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "جملة" | "sentence" => {
            if args.len() < 3 { eprintln!("❌ بيان جملة <نص>"); return; }
            let text = &args[2..].join(" ");
            match SentenceParser::parse(text) {
                Ok(s) => println!("{}", s.full_irab()),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "حسّن" | "optimize" => {
            if args.len() < 3 { eprintln!("❌ بيان حسّن <ملف.بيان>"); return; }
            let content = std::fs::read_to_string(&args[2]).unwrap_or_default();
            let sentences: Vec<_> = content.lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
                .filter_map(|l| SentenceParser::parse(l.trim()).ok())
                .collect();

            let mut optimizer = CodeOptimizer::new();
            optimizer.analyze(&sentences);
            println!("{}", optimizer.report());
        }
        "بلاغة" | "balagha" => {
            if args.len() < 3 { eprintln!("❌ بيان بلاغة <ملف.بيان>"); return; }
            let content = std::fs::read_to_string(&args[2]).unwrap_or_default();
            let sentences: Vec<_> = content.lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
                .filter_map(|l| SentenceParser::parse(l.trim()).ok())
                .collect();
            let report = BalaghaAnalyzer::analyze(&sentences);
            println!("{}", BalaghaAnalyzer::report(&report));
        }
        "تفاعلي" | "repl" => repl_mode(),
        _ => help(),
    }
}

fn repl_mode() {
    println!("🕌 الوضع التفاعلي | 'خروج' للخروج | 'فحص' لتحليل الكود\n");
    let mut interp = Interpreter::new();
    let mut history: Vec<bayan_compiler::parser::ArabicSentence> = Vec::new();

    loop {
        print!("بيان> "); io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "خروج" || input == "exit" { break; }
        if input == "حالة" { interp.show_state(); continue; }

        if input == "فحص" || input == "check" {
            let mut opt = CodeOptimizer::new();
            opt.analyze(&history);
            println!("{}", opt.report());
            continue;
        }

        if input.contains(' ') {
            match SentenceParser::parse(input) {
                Ok(s) => {
                    println!("{}", s.full_irab());
                    history.push(s.clone());
                    if let Some(ref verb) = s.verb {
                        if s.errors.is_empty() {
                            match interp.execute(&verb.original) {
                                Ok(v) => println!("↳ {:?}", v),
                                Err(e) => eprintln!("{}", e),
                            }
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
    println!("  بيان جملة <نص>       تحليل جملة");
    println!("  بيان حسّن <ملف>      تحليل وتحسين الكود");
    println!("  بيان تفاعلي           وضع المحادثة (اكتب 'فحص' للتحسين)");
}
