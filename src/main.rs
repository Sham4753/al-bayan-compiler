use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::generator::Generator;

fn main() {
    println!("🕌 لغة البيان - المترجم v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}\n", bayan_compiler::BAYAN_SLOGAN);

    let generator = Generator::new();

    let words = vec![
        "قَرَأَ",
        "يُحَاسِبُ",
        "سَيُحَسِّبُ",
        "اِستَقرَأَ",
        "يَحْفَظُهُ",
        "اِحتَسَبَ",
        "اِنبَعَثَ",
    ];

    println!("╔══════════════════════════════════════════╗");
    println!("║   المُصَرِّف + المُوَلِّد = كود حي       ║");
    println!("╚══════════════════════════════════════════╝\n");

    for word in &words {
        match Musarrif::analyse(word) {
            Ok(analysis) => {
                match generator.generate(&analysis) {
                    Ok(code) => {
                        println!("📝 '{}'", word);
                        println!("   الجذر: {} | الوزن: {} | الزمن: {:?}",
                            analysis.jidhr, analysis.wazn, analysis.zaman);
                        println!("   {} {} {}",
                            if code.is_async { "⚡غير متزامن" } else { "●متزامن" },
                            if code.is_parallel { "⚙️متوازي" } else { "●فردي" },
                            if code.ir.contains("WITH_CONTEXT") { "👤معه ضمير" } else { "" }
                        );
                        println!("   كود وسيط: {}", code.ir.trim());
                        println!();
                    }
                    Err(e) => println!("❌ فشل التوليد: {}\n", e),
                }
            }
            Err(e) => println!("❌ فشل التحليل: {}\n", e),
        }
    }
}
