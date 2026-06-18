use bayan_compiler::interpreter::Interpreter;
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::parser::SentenceParser;
use bayan_compiler::optimizer::CodeOptimizer;
use bayan_compiler::balagha::BalaghaAnalyzer;
use bayan_compiler::orchestrator::Orchestrator;
use bayan_compiler::composer::Composer;
use bayan_compiler::executor::Executor;
use bayan_compiler::control::ControlFlow;
use bayan_compiler::variables::Variables;
use bayan_compiler::functions::Functions;
use bayan_compiler::exceptions::Exceptions;
use bayan_compiler::loops::Loops;
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
        "نفذ" | "exec" => {
            if args.len() < 3 { eprintln!("❌ بيان نفذ <كلمة>"); return; }
            let mut orch = Orchestrator::new();
            match orch.execute(&args[2]) {
                Ok(v) => println!("↳ {:?}", v),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "ركب" | "compose" => {
            if args.len() < 3 { eprintln!("❌ بيان ركب <جملة>"); return; }
            let text = &args[2..].join(" ");
            let s = Composer::compose(text);
            println!("{}", s.execute());
        }
        "نفذ-جملة" | "exec-sen" => {
            if args.len() < 3 { eprintln!("❌ بيان نفذ-جملة <جملة>"); return; }
            let text = &args[2..].join(" ");
            let mut exec = Executor::new();
            match exec.execute_sentence(text) {
                Ok(v) => println!("↳ {:?}", v),
                Err(e) => eprintln!("❌ {}", e),
            }
        }
        "إذا" | "if" => {
            if args.len() < 3 { eprintln!("❌ بيان إذا <شرط> فـ <نتيجة>"); return; }
            let text = &args[2..].join(" ");
            let mut ctrl = ControlFlow::new();
            println!("{:?}", ctrl.execute_if(text));
        }
        "let" => {
            if args.len() < 3 { eprintln!("❌ بيان let <اسم> = <قيمة>"); return; }
            let text = &args[2..].join(" ");
            let mut vars = Variables::new();
            match vars.set(text) {
                Ok(v) => println!("{:?}", v),
                Err(e) => eprintln!("{}", e),
            }
        }
        "دالة" | "fn" => {
            if args.len() < 3 { eprintln!("❌ بيان دالة <اسم>(<معامل>) = <جسم>"); return; }
            let text = &args[2..].join(" ");
            let mut funcs = Functions::new();
            match funcs.define(text) {
                Ok(v) => println!("{}", v),
                Err(e) => eprintln!("{}", e),
            }
        }
        "حاول" | "try" => {
            if args.len() < 3 { eprintln!("❌ بيان حاول <مهمة> وإلا فـ <بديل>"); return; }
            let text = &args[2..].join(" ");
            let mut exc = Exceptions::new();
            match exc.try_catch(text) {
                Ok(v) => println!("{}", v),
                Err(e) => eprintln!("{}", e),
            }
        }
        "كرر" | "repeat" => {
            if args.len() < 3 { eprintln!("❌ بيان كرر <فعل> <عدد> مرات"); return; }
            let text = &args[2..].join(" ");
            let mut looper = Loops::new();
            match looper.repeat(text) {
                Ok(v) => println!("{}", v),
                Err(e) => eprintln!("{}", e),
            }
        }
        "جذر" | "root" => {
            if args.len() < 3 { eprintln!("❌ بيان جذر <اسم>"); return; }
            let gen = bayan_compiler::generator_new::Generator::new();
            match gen.lookup(&args[2]) {
                Some(r) => println!("✅ {} -> {} ({})", r.arabic, r.intrinsic, r.description),
                None => eprintln!("❌ غير معروف"),
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
        match interp.execute(input) {
            Ok(v) => println!("↳ {:?}", v),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn help() {
    println!("🕌 لغة البيان v0.5.0");
    println!("✨ الكود قرآن\n");
    println!("  بيان شغّل <ملف>      تنفيذ برنامج");
    println!("  بيان نفذ <كلمة>      تنفيذ كلمة");
    println!("  بيان ركب <جملة>      تحليل جملة");
    println!("  بيان نفذ-جملة <جملة> تنفيذ جملة");
    println!("  بيان إذا <شرط> فـ    منطق شرطي");
    println!("  بيان let <اسم> =     تعريف متغير");
    println!("  بيان دالة <اسم>      تعريف دالة");
    println!("  بيان كرر <فعل>       تكرار");
    println!("  بيان حاول <مهمة>     استثناء");
    println!("  بيان تفاعلي           وضع المحادثة");
}

// أمر جديد: "جذر" للبحث في القاعدة
