use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   🕌 مُعَلِّم البيان - تعلم البرمجة بالعربية  ║");
    println!("║   اكتب 'مساعدة' لتبدأ                     ║");
    println!("╚══════════════════════════════════════════╝");

    let mut history: Vec<String> = Vec::new();

    loop {
        print!("\n🕌> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.is_empty() { continue; }
        if input == "خروج" || input == "exit" {
            println!("👋 مع السلامة! نفذت {} أمراً.", history.len());
            break;
        }

        match input.as_str() {
            "مساعدة" | "help" => {
                println!("\n📖 الكلمات المتاحة:");
                println!("  قَرَأَ    - قراءة");
                println!("  حَفِظَ    - تشفير");
                println!("  بَعَثَ    - إرسال");
                println!("  رَسَمَ    - واجهة");
                println!("  اِحتَسَبَ  - فحص النظام");
                println!("\n✍️  جرب كتابة أي كلمة!");
            }
            "المستوى" => {
                let len = history.len();
                let level = if len >= 10 { "👑 خبير" } else if len >= 5 { "⚡ متقدم" } else { "🌱 مبتدئ" };
                println!("\n📊 عدد الأوامر: {} | المستوى: {}", len, level);
            }
            _ => {
                // نتجنب مشكلة الفهرسة على الحروف العربية
                let safe_name = input.chars().take(10).collect::<String>();
                history.push(input.clone());
                println!("✅ تم تنفيذ: {}", safe_name);
                
                match history.len() {
                    1 => println!("💡 أحسنت! أول أمر. جرب: حَفِظَ"),
                    2 => println!("💡 رائع! جرب: بَعَثَ"),
                    3 => println!("💡 ممتاز! جرب: رَسَمَ"),
                    5 => println!("🎉 وصلت 5 أوامر! أنت متقدم الآن."),
                    10 => println!("👑 10 أوامر! أنت خبير البيان."),
                    _ => println!("💪 استمر! لديك {} أمراً.", history.len()),
                }
            }
        }
    }
}
