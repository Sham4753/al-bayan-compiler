use std::io::{self, Write, Read};
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   🧠 المُعجِز 2.0 - يتعلم ويتطور       ║");
    println!("╚══════════════════════════════════════╝\n");

    // تحميل الذاكرة
    let mut memory: HashMap<String, u32> = load_memory();
    let templates = vec![
        ("متجر", "منتجات، سلة، دفع"),
        ("مدونة", "مقالات، تعليقات، كتّاب"),
        ("موقع", "من أنا، أعمال، تواصل"),
        ("نظام", "لوحة تحكم، مستخدمين"),
    ];

    println!("📋 القوالب (الأكثر استخداماً أولاً):");
    let mut sorted: Vec<_> = templates.iter().collect();
    sorted.sort_by(|a, b| {
        let ca = memory.get(a.0).unwrap_or(&0);
        let cb = memory.get(b.0).unwrap_or(&0);
        cb.cmp(ca)
    });
    for (key, desc) in &sorted {
        let count = memory.get(*key).unwrap_or(&0);
        println!("   • {} ({} استخدام) - {}", key, count, desc);
    }
    println!("   📊 المجموع: {} تطبيقاً تم بناؤه", memory.values().sum::<u32>());

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

        for (key, _) in &templates {
            if input.contains(key) { detected = key; }
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() >= 3 { app_name = words.last().unwrap(); }

        // تحديث الذاكرة
        *memory.entry(detected.to_string()).or_insert(0) += 1;

        println!("\n🔍 تحليل: {}", input);
        println!("📦 القالب: {} (خبرة: {} تطبيق سابق)", detected, memory[detected]);

        // كل ما زادت الخبرة، زادت التفاصيل
        let experience = memory[detected];
        let mut steps = match detected {
            "متجر" => vec!["جدول المنتجات", "صفحة الرئيسية", "سلة الشراء", "صفحة الدفع"],
            "مدونة" => vec!["جدول المقالات", "نظام التعليقات", "صفحة الكاتب", "RSS"],
            _ => vec!["الهيكل الأساسي", "الصفحات", "القاعدة", "النشر"],
        };

        // إضافة تحسينات بناءً على الخبرة
        if experience >= 5 {
            steps.push("🚀 تحسين SEO تلقائي");
            steps.push("📱 دعم الجوال");
        }
        if experience >= 10 {
            steps.push("🤖 توصيات ذكاء اصطناعي");
            steps.push("📊 لوحة تحليلات متقدمة");
        }

        println!("⏳ جاري البناء (المستوى: {})...\n", if experience < 3 {"مبتدئ"} else if experience < 7 {"محترف"} else {"خبير"});
        for (i, s) in steps.iter().enumerate() {
            println!("   {}. {} ✅", i+1, s);
        }

        println!("\n🎉 تم بناء '{}'!", app_name);
        println!("🔗 http://{}.localhost:8080", app_name.to_lowercase());
        println!("💾 تم حفظ الخبرة في الذاكرة\n");

        // حفظ الذاكرة
        save_memory(&memory);
    }
    println!("👋 مع السلامة. المُعجِز تعلم {} تطبيقاً اليوم.", memory.values().sum::<u32>());
}

fn load_memory() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    if let Ok(contents) = fs::read_to_string("ذاكرة_المعجز.txt") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                if let Ok(count) = parts[1].trim().parse() {
                    map.insert(parts[0].to_string(), count);
                }
            }
        }
    }
    map
}

fn save_memory(memory: &HashMap<String, u32>) {
    let contents: String = memory.iter()
        .map(|(k, v)| format!("{}:{}\n", k, v))
        .collect();
    let _ = fs::write("ذاكرة_المعجز.txt", contents);
}
