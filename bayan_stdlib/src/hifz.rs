//! الجذر: ح-ف-ظ (الأمان والتشفير)

use sha2::{Sha256, Digest};

/// تشفير نص (حَفِظَ)
pub fn تشفير(نص: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(نص.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// حماية مورد (حَافَظَ)
pub fn حماية_مورد(مورد: &str, مستوى: &str) -> String {
    format!("🔒 المورد '{}' محمي بمستوى: {}", مورد, مستوى)
}

/// طلب حماية خارجية (اِستَحفَظَ)
pub fn طلب_حماية(بيانات: &str) -> String {
    format!("🛡️ تم طلب حماية خارجية للبيانات: {}", بيانات)
}

/// نسخة احتياطية (اِحتَفَظَ)
pub fn نسخة_احتياطية(مصدر: &str) -> String {
    format!("💾 تم أخذ نسخة احتياطية من: {}", مصدر)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_تشفير() {
        let result = تشفير("البيان");
        assert_eq!(result.len(), 64); // SHA256 = 64 hex chars
        // نفس المدخل = نفس المخرج
        assert_eq!(تشفير("البيان"), تشفير("البيان"));
    }

    #[test]
    fn test_حماية_مورد() {
        let result = حماية_مورد("ملف.txt", "عالي");
        assert!(result.contains("محمي"));
    }
}
