//! الجذر: ك-ت-ب (الكتابة والإخراج)

use std::fs;
use std::io;

/// كتابة إلى ملف (كَتَبَ)
pub fn كتابة_ملف(مسار: &str, محتوى: &str) -> Result<(), io::Error> {
    fs::write(مسار, محتوى)
}

/// إخراج إلى الشاشة (كَتَبَ على المنفذ)
pub fn اطبع(نص: &str) {
    println!("{}", نص);
}

/// كتابة في السجل (اِكتَتَبَ)
pub fn كتابة_سجل(رسالة: &str) {
    eprintln!("📋 [سجل البيان]: {}", رسالة);
}

/// طلب حفظ في خدمة خارجية (اِستَكتَبَ)
pub fn طلب_حفظ(محتوى: &str) -> String {
    format!("تم حفظ '{}' في الخدمة السحابية", محتوى)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_كتابة_ملف() {
        let مسار = "/data/data/com.termux/files/home/اختبار_البيان.txt";
        let result = كتابة_ملف(مسار, "السلام عليكم");
        assert!(result.is_ok());
        let content = std::fs::read_to_string(مسار).unwrap();
        assert_eq!(content, "السلام عليكم");
        // تنظيف
        let _ = std::fs::remove_file(مسار);
    }

    #[test]
    fn test_طلب_حفظ() {
        let result = طلب_حفظ("بيانات");
        assert!(result.contains("تم حفظ"));
    }
}
