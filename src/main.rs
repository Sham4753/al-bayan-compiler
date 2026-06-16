use bayan_compiler::tasreef::TasreefRegister;
use bayan_compiler::musarrif::Musarrif;

fn main() {
    println!("🕌 لغة البيان - تشخيص المُصَرِّف\n");

    let test_words = vec![
        "قَرَأَ",
        "يُحَاسِبُ",
        "سَيُحَسِّبُ",
        "اِستَقرَأَ",
        "يَحْفَظُهُ",
        "اِحتَسَبَ",
        "اِنبَعَثَ",
    ];

    for word in &test_words {
        println!("══════════════════════");
        println!("📝 الكلمة: '{}'", word);
        println!("📏 طول النص: {} بايت", word.len());
        println!("🔤 الحروف (chars): {:?}", word.chars().collect::<Vec<char>>());
        println!("🔢 البايتات (bytes): {:?}", word.as_bytes());

        // عرض كل حرف مع موقعه
        print!("🔍 تفصيل: ");
        for (i, c) in word.char_indices() {
            print!("[{}:'{}'] ", i, c);
        }
        println!();

        match Musarrif::analyse(word) {
            Ok(m) => println!("✅ جذر:{} | وزن:{} | زمن:{:?} | ضمائر:{:?}", m.jidhr, m.wazn, m.zaman, m.damair),
            Err(e) => println!("❌ خطأ: {}", e),
        }
        println!();
    }
}
