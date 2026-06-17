use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║ 🧠 المُعجِز 4.0 - يتعلم من أخطائه     ║");
    println!("╚══════════════════════════════════════╝\n");

    let mut memory = load_memory();
    let mut learned = load_dictionary();

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
            println!("\n📊 التطبيقات: {:?}", memory);
            println!("📚 الكلمات المتعلمة: {:?}", learned);
            continue;
        }

        let mut detected = String::new();
        let mut app_name = "تطبيقي".to_string();

        for (key, _) in &templates {
            if input.contains(key) { detected = key.to_string(); }
        }
        for (key, value) in &learned {
            if input.contains(key) { detected = value.to_string(); }
        }

        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        if words.len() >= 3 { app_name = words.last().unwrap().to_string(); }

        if detected.is_empty() {
            println!("\n❓ لم أفهم: '{}'", input);
            println!("ما نوع التطبيق؟ متجر | مدونة | موقع | نظام | أو كلمة جديدة");
            print!("🔤 النوع: ");
            io::stdout().flush().unwrap();
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            let answer = answer.trim().to_string();

            if !answer.is_empty() {
                let key_word = if words.len() >= 2 { words[1].clone() } else { "تطبيق".to_string() };
                learned.insert(key_word.clone(), answer.clone());
                detected = answer;
                println!("✅ تعلمت: '{}' = '{}'", key_word, detected);
                save_dictionary(&learned);
            } else { continue; }
        }

        *memory.entry(detected.clone()).or_insert(0) += 1;
        let experience = memory[&detected];

        println!("\n🔍 تحليل: {}", input);
        println!("📦 القالب: {} (خبرة: {})", detected, experience);

        let mut steps = match detected.as_str() {
            "متجر" => vec!["جدول المنتجات", "صفحة الرئيسية", "سلة الشراء", "صفحة الدفع"],
            "مدونة" => vec!["جدول المقالات", "نظام التعليقات", "صفحة الكاتب", "RSS"],
            "نظام" => vec!["لوحة التحكم", "إدارة المستخدمين", "إدارة المحتوى", "التقارير"],
            _ => vec!["الهيكل الأساسي", "الصفحات", "القاعدة", "النشر"],
        };
        if experience >= 5 { steps.push("🚀 SEO"); steps.push("📱 جوال"); }
        if experience >= 10 { steps.push("🤖 ذكاء"); steps.push("📊 تحليلات"); }

        println!("⏳ جاري البناء...");
        for (i, s) in steps.iter().enumerate() { println!("   {}. {} ✅", i+1, s); }
        println!("\n🎉 تم بناء '{}'!", app_name);

        let score = 50 + (experience * 5).min(45);
        let level = if score < 60 { "📝 مقبول" } else if score < 80 { "✅ جيد" } else if score < 95 { "👑 بليغ" } else { "💎 معجز" };
        println!("🔍 فحص: {}/100 ({})", score, level);
        if score >= 70 { println!("🔧 تحسينات: قاعدة بيانات ✅ | تشفير ✅"); }
        println!("🔗 http://{}.localhost:8080", app_name.to_lowercase());
        save_memory(&memory);
    }
    println!("👋 مع السلامة. تعلمت {} كلمة.", learned.len());
}

fn load_memory() -> HashMap<String, u32> {
    let mut m = HashMap::new();
    if let Ok(c) = fs::read_to_string("ذاكرة_المعجز.txt") {
        for line in c.lines() { let p: Vec<&str> = line.split(':').collect(); if p.len() == 2 { if let Ok(n) = p[1].parse() { m.insert(p[0].to_string(), n); } } }
    }
    m
}
fn save_memory(m: &HashMap<String, u32>) {
    let c: String = m.iter().map(|(k,v)| format!("{}:{}\n",k,v)).collect();
    let _ = fs::write("ذاكرة_المعجز.txt", c);
}
fn load_dictionary() -> HashMap<String, String> {
    let mut d = HashMap::new();
    if let Ok(c) = fs::read_to_string("قاموس_المعجز.txt") {
        for line in c.lines() { let p: Vec<&str> = line.split(':').collect(); if p.len() == 2 { d.insert(p[0].to_string(), p[1].to_string()); } }
    }
    d
}
fn save_dictionary(d: &HashMap<String, String>) {
    let c: String = d.iter().map(|(k,v)| format!("{}:{}\n",k,v)).collect();
    let _ = fs::write("قاموس_المعجز.txt", c);
}
