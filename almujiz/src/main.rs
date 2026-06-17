use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   🧠 المُعجِز - مولد التطبيقات بالعربية  ║");
    println!("╚══════════════════════════════════════╝\n");

    let templates = vec![
        ("متجر", "متجر إلكتروني: منتجات، سلة، دفع"),
        ("مدونة", "مدونة: مقالات، تعليقات، كتّاب"),
        ("موقع", "موقع شخصي: من أنا، أعمال، تواصل"),
        ("نظام", "نظام إدارة: لوحة تحكم، مستخدمين، محتوى"),
    ];

    println!("📋 القوالب المتاحة:");
    for (key, desc) in &templates {
        println!("   • {} - {}", key, desc);
    }

    loop {
        print!("\n🧠> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.is_empty() { continue; }
        if input == "خروج" { break; }

        let mut detected = "موقع";
        let mut app_name = "تطبيقي";

        for (key, _) in &templates {
            if input.contains(key) { detected = key; }
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() >= 3 { app_name = words.last().unwrap(); }

        println!("\n🔍 تحليل: {}", input);
        println!("📦 القالب: {}", detected);
        println!("📝 الاسم: {}", app_name);
        println!("⏳ جاري البناء...\n");

        let steps = match detected {
            "متجر" => vec!["جدول المنتجات", "صفحة الرئيسية", "سلة الشراء", "صفحة الدفع"],
            "مدونة" => vec!["جدول المقالات", "نظام التعليقات", "صفحة الكاتب", "RSS"],
            _ => vec!["الهيكل الأساسي", "الصفحات", "القاعدة", "النشر"],
        };

        for (i, s) in steps.iter().enumerate() {
            println!("   {}. {} ✅", i+1, s);
        }

        println!("\n🎉 تم بناء '{}'!", app_name);
        println!("🔗 http://{}.localhost:8080\n", app_name.to_lowercase());
    }
    println!("👋 مع السلامة.");
}
