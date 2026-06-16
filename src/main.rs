use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::generator::Generator;
use bayan_compiler::runtime::{BayanRuntime, Value};

fn main() {
    println!("🕌 لغة البيان - المحرك التنفيذي v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}\n", bayan_compiler::BAYAN_SLOGAN);

    let generator = Generator::new();
    let mut runtime = BayanRuntime::new();

    let words = vec![
        ("قَرَأَ", None),
        ("سَيُحَسِّبُ", Some(Value::List(vec![Value::Number(5.0), Value::Number(10.0)]))),
        ("يُحَاسِبُ", None),
        ("اِستَقرَأَ", None),
        ("يَحْفَظُهُ", Some(Value::Text("بياناتي".to_string()))),
        ("اِحتَسَبَ", None),
        ("اِنبَعَثَ", None),
    ];

    println!("╔══════════════════════════════════════════╗");
    println!("║     المُصَرِّف ← المُوَلِّد ← المُنفذ    ║");
    println!("╚══════════════════════════════════════════╝\n");

    for (word, input) in &words {
        println!("════════════════════════");
        println!("📝 الكلمة: '{}'", word);

        match Musarrif::analyse(word) {
            Ok(analysis) => {
                println!("   🔍 تحليل: جذر={} | وزن={} | زمن={:?}",
                    analysis.jidhr, analysis.wazn, analysis.zaman);

                match generator.generate(&analysis) {
                    Ok(code) => {
                        println!("   ⚙️  كود: {}", code.ir.trim());
                        println!("   🚀 تنفيذ...");

                        match runtime.execute_intrinsic(&code.intrinsic, input.clone()) {
                            Ok(result) => println!("   ✅ النتيجة: {:?}", result),
                            Err(e) => println!("   ❌ خطأ: {}", e),
                        }
                    }
                    Err(e) => println!("   ❌ فشل التوليد: {}", e),
                }
            }
            Err(e) => println!("   ❌ فشل التحليل: {}", e),
        }
        println!();
    }

    println!("════════════════════════");
    println!("📋 سجل العمليات:");
    for entry in &runtime.log {
        println!("   • {}", entry);
    }
}
