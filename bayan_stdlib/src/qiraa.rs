//! الجذر: ق-ر-أ (القراءة والإدخال)

use std::fs;
use std::io::{self, Read};

/// قراءة ملف نصي كامل (قَرَأَ)
pub fn قراءة_ملف(مسار: &str) -> Result<String, io::Error> {
    fs::read_to_string(مسار)
}

/// قراءة ملف بشكل متدفق (قَارَأَ)
pub fn قراءة_متدفقة(مسار: &str) -> Result<impl Read, io::Error> {
    fs::File::open(مسار)
}

/// طلب قراءة من API خارجي (اِستَقرَأَ)
#[cfg(feature = "full")]
pub fn طلب_قراءة(رابط: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(رابط).send()?;
    response.text()
}

/// قراءة تكوين النظام (اِقتَرَأَ)
pub fn قراءة_تكوين(مفتاح: &str) -> Option<String> {
    std::env::var(مفتاح).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_قراءة_ملف() {
        let result = قراءة_ملف("Cargo.toml");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("bayan"));
    }

    #[test]
    fn test_قراءة_تكوين() {
        std::env::set_var("اختبار_البيان", "قيمة");
        assert_eq!(قراءة_تكوين("اختبار_البيان"), Some("قيمة".to_string()));
    }
}
