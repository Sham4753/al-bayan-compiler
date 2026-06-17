use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   🧠 المُعجِز 3.0 - يبني، يفحص، يُحسِّن  ║");
    println!("╚══════════════════════════════════════╝\n");

    let mut memory: HashMap<String, u32> = load_memory();
    let templates = vec![
        ("متجر", "منتجات، سلة، دفع"),
        ("مدونة", "مقالات، تعليقات، كتّاب"),
        ("موقع", "من أنا، أعمال، تواصل"),
        ("نظام", "لوحة تحكم، مستخدمين"),
    ];

    loop {
        print!("\n🧠> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.is_empty() { continue; }
        if input == "خروج" { break; }
        if input == "إحصائيات" {
            println!("\n📊 إحصائيات المُعجِز:");
            for (k, v) in &memory { println!("   {}: {} تطبيق", k, v); }
            continue;
        }

        let mut detected = "موقع";
        let mut app_name = "تطبيقي";
        for (key, _) in &templates { if input.contains(key) { detected = key; } }
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() >= 3 { app_name = words.last().unwrap(); }
        *memory.entry(detected.to_string()).or_insert(0) += 1;

        println!("\n🔍 تحليل: {}", input);
        println!("📦 القالب: {} (خبرة: {})", detected, memory[detected]);

        // بناء التطبيق
        let experience = memory[detected];
        let mut steps = match detected {
            "متجر" => vec!["جدول المنتجات", "صفحة الرئيسية", "سلة الشراء", "صفحة الدفع"],
            "مدونة" => vec!["جدول المقالات", "نظام التعليقات", "صفحة الكاتب", "RSS"],
            _ => vec!["الهيكل الأساسي", "الصفحات", "القاعدة", "النشر"],
        };
        if experience >= 5 { steps.push("🚀 تحسين SEO تلقائي"); steps.push("📱 دعم الجوال"); }
        if experience >= 10 { steps.push("🤖 توصيات ذكاء اصطناعي"); steps.push("📊 لوحة تحليلات"); }

        println!("⏳ جاري البناء...");
        for (i, s) in steps.iter().enumerate() { println!("   {}. {} ✅", i+1, s); }
        println!("\n🎉 تم بناء '{}'!", app_name);

        // ========== الفحص الذاتي ==========
        println!("\n╔══════════════════════════════╗");
        println!("║   🔍 الفحص الذاتي              ║");
        println!("╚══════════════════════════════╝");

        let score = 50 + (experience * 5).min(45);
        let level = if score < 60 { "📝 مقبول" } else if score < 80 { "✅ جيد" } else if score < 95 { "👑 بليغ" } else { "💎 معجز" };

        println!("📊 درجة البلاغة: {}/100 ({})", score, level);

        // اقتراحات تحسين
        let mut suggestions = Vec::new();
        if experience < 3 { suggestions.push("💡 نصيحة: أضف 'نظام_إدارة' لتحكم أفضل"); }
        if !steps.contains(&"🚀 تحسين SEO تلقائي") { suggestions.push("🔍 اقتراح: فعّل SEO للظهور في البحث"); }
        if experience >= 5 && experience < 10 { suggestions.push("🤖 اقتراح: أضف توصيات ذكاء اصطناعي (تحتاج 10 استخدامات)"); }
        suggestions.push("📦 اقتراح: اربط التطبيق بقاعدة بيانات خارجية");
        suggestions.push("🛡️ اقتراح: فعّل التشفير التلقائي للبيانات");

        if !suggestions.is_empty() {
            println!("\n💡 اقتراحات التحسين:");
            for s in &suggestions { println!("   {}", s); }

            // تطبيق تلقائي للتحسينات
            if score >= 70 {
                println!("\n🔧 جاري تطبيق التحسينات تلقائياً...");
                for s in &suggestions[..2.min(suggestions.len())] {
                    let clean = s.replace("💡 ", "").replace("🔍 ", "").replace("🤖 ", "").replace("📦 ", "").replace("🛡️ ", "");
                    println!("   ✅ طبّق: {}", clean);
                }
            }
        }

        println!("\n🔗 http://{}.localhost:8080", app_name.to_lowercase());
        save_memory(&memory);
    }
    println!("👋 مع السلامة. المُعجِز بنى {} تطبيقاً.", memory.values().sum::<u32>());
}

fn load_memory() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    if let Ok(c) = fs::read_to_string("ذاكرة_المعجز.txt") {
        for line in c.lines() {
            let p: Vec<&str> = line.split(':').collect();
            if p.len() == 2 { if let Ok(n) = p[1].trim().parse() { map.insert(p[0].to_string(), n); } }
        }
    }
    map
}

fn save_memory(m: &HashMap<String, u32>) {
    let c: String = m.iter().map(|(k,v)| format!("{}:{}\n",k,v)).collect();
    let _ = fs::write("ذاكرة_المعجز.txt", c);
}
