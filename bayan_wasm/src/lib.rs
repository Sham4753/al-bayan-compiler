use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn version() -> String {
    "🕌 لغة البيان v0.4.0 - ✨ الكود قرآن".to_string()
}

#[wasm_bindgen]
pub fn basmala() -> String {
    "بِسْمِ اللَّهِ - بدأت الجلسة".to_string()
}

#[wasm_bindgen]
pub fn analyse(word: String) -> String {
    format!("تحليل: '{}' - جاهز للتشغيل في المتصفح", word)
}
