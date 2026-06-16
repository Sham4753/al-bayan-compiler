use bayan_compiler::tasreef::TasreefRegister;
use bayan_compiler::musarrif::Musarrif;

fn main() {
    println!("🕌 لغة البيان - المترجم v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}", bayan_compiler::BAYAN_SLOGAN);
    println!();

    // تحميل سجل التصريف
    let register = TasreefRegister::new();
    println!("✅ سجل التصريف: {} تركيباً", register.register.len());

    // اختبار المُصَرِّف
    println!("\n📖 اختبار المُصَرِّف:");
    let words = vec![
        "قَرَأَ",
        "سَيُحَسِّبُ",
        "يُحَاسِبُ",
        "اِستَقرَأَ",
        "يَحْفَظُهُ",
        "اِحتَسَبَ",
        "اِنبَعَثَ",
    ];

    for word in words {
        match Musarrif::analyse(word) {
            Ok(m) => {
                println!("  ✅ {} -> جذر:{} | وزن:{} | زمن:{:?} | ضمائر:{:?}",
                    m.original, m.jidhr, m.wazn, m.zaman, m.damair
                );
            }
            Err(e) => println!("  ❌ {} -> {}", word, e),
        }
    }
}
