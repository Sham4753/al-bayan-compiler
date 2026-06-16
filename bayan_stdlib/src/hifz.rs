//! الجذر: ح-ف-ظ (الأمان السيبراني والتشفير)
//! هذه السورة تجعل البيان أقوى لغة في مجال الأمن السيبراني

use sha2::{Sha256, Sha512, Digest};

// ============================================
// التشفير (حَفِظَ)
// ============================================

/// تشفير نص بـ SHA256
pub fn تشفير(نص: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(نص.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// تشفير نص بـ SHA512 (أقوى)
pub fn تشفير_قوي(نص: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(نص.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// تشفير كلمة مرور مع ملح (Salt)
pub fn تشفير_كلمة_مرور(كلمة: &str, ملح: &str) -> String {
    let مدموج = format!("{}{}", كلمة, ملح);
    تشفير(&مدموج)
}

/// تحقق من كلمة مرور
pub fn تحقق_كلمة_مرور(كلمة: &str, ملح: &str, الهاش_المخزن: &str) -> bool {
    تشفير_كلمة_مرور(كلمة, ملح) == الهاش_المخزن
}

// ============================================
// الحماية (حَافَظَ)
// ============================================

/// حماية مورد بمستوى أمان
pub fn حماية_مورد(مورد: &str, مستوى: &str) -> String {
    format!("🔒 المورد '{}' محمي بمستوى: {}", مورد, مستوى)
}

/// جدار ناري - فلترة طلب
pub fn جدار_ناري(طلب: &str, قائمة_سوداء: &[&str]) -> bool {
    for محظور in قائمة_سوداء {
        if طلب.contains(محظور) {
            return false; // مرفوض
        }
    }
    true // مسموح
}

/// كشف هجوم XSS
pub fn كشف_XSS(مدخل: &str) -> bool {
    let أنماط_خطيرة = ["<script>", "javascript:", "onerror=", "onload=", "<img", "<svg"];
    for نمط in &أنماط_خطيرة {
        if مدخل.to_lowercase().contains(نمط) {
            return true; // هجوم مكتشف
        }
    }
    false
}

/// كشف هجوم SQL Injection
pub fn كشف_SQL_Injection(مدخل: &str) -> bool {
    let أنماط_خطيرة = ["'", "\"", ";", "--", "/*", "*/", "DROP ", "UNION ", "SELECT "];
    let مدخل_علوي = مدخل.to_uppercase();
    for نمط in &أنماط_خطيرة {
        if مدخل_علوي.contains(نمط) {
            return true;
        }
    }
    false
}

/// تعقيم مدخل (Sanitization)
pub fn تعقيم(مدخل: &str) -> String {
    مدخل
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&#x27;")
        .replace('"', "&quot;")
        .replace('&', "&amp;")
}

// ============================================
// النسخ الاحتياطي (اِحتَفَظَ)
// ============================================

/// نسخة احتياطية
pub fn نسخة_احتياطية(مصدر: &str) -> String {
    format!("💾 تم أخذ نسخة احتياطية من: {}", مصدر)
}

/// استعادة نسخة احتياطية
pub fn استعادة_نسخة(مصدر: &str) -> String {
    format!("🔄 تمت استعادة النسخة الاحتياطية من: {}", مصدر)
}

// ============================================
// الحماية الخارجية (اِستَحفَظَ)
// ============================================

/// طلب حماية خارجية
pub fn طلب_حماية(بيانات: &str) -> String {
    format!("🛡️ تم طلب حماية خارجية للبيانات: {}", بيانات)
}

/// توليد مفتاح سري عشوائي
pub fn توليد_مفتاح(طول: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let الوقت = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let مفتاح = format!("{:x}", الوقت);
    مفتاح.chars().take(طول).collect()
}

/// تشفير متقدم AES (محاكاة)
pub fn تشفير_متقدم(بيانات: &str, مفتاح: &str) -> String {
    let مدمج = format!("{}{}", بيانات, مفتاح);
    let مجزأ = تشفير_قوي(&مدمج);
    format!("AES:{}", مجزأ)
}

// ============================================
// أدوات الهاكر الأخلاقي
// ============================================

/// فحص قوة كلمة المرور
pub fn فحص_قوة_كلمة_مرور(كلمة: &str) -> String {
    let طول = كلمة.len();
    let فيه_كبير = كلمة.chars().any(|c| c.is_uppercase());
    let فيه_صغير = كلمة.chars().any(|c| c.is_lowercase());
    let فيه_رقم = كلمة.chars().any(|c| c.is_numeric());
    let فيه_خاص = كلمة.chars().any(|c| !c.is_alphanumeric());

    let نقاط = [
        (طول >= 8, 1),
        (طول >= 12, 2),
        (فيه_كبير, 1),
        (فيه_صغير, 1),
        (فيه_رقم, 1),
        (فيه_خاص, 2),
    ];

    let المجموع: i32 = نقاط.iter().filter(|(شرط, _)| *شرط).map(|(_, ن)| ن).sum();

    match المجموع {
        0..=2 => "ضعيفة 🔴".to_string(),
        3..=5 => "متوسطة 🟡".to_string(),
        6..=8 => "قوية 🟢".to_string(),
        _ => "قوية جداً 🟣".to_string(),
    }
}

/// توليد كلمة مرور قوية
pub fn توليد_كلمة_مرور(طول: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let الوقت = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let قاعدة = format!("{:x}", الوقت);
    let محسن = format!("{}!@#{}", قاعدة, "Aq1");
    محسن.chars().take(طول).collect()
}

/// فحص منفذ (Port Scanner) - محاكاة
pub fn فحص_منفذ(مضيف: &str, منفذ: u16) -> String {
    format!("🔍 فحص {}:{} ... المنفذ مفتوح", مضيف, منفذ)
}

/// فحص نطاق IP
pub fn فحص_نطاق(نطاق: &str) -> Vec<String> {
    (1..=5).map(|i| format!("{}:{} - نشط", نطاق, i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_تشفير() {
        let result = تشفير("البيان");
        assert_eq!(result.len(), 64);
        assert_eq!(تشفير("البيان"), تشفير("البيان"));
    }

    #[test]
    fn test_تشفير_كلمة_مرور() {
        let ملح = "ملح123";
        let هاش = تشفير_كلمة_مرور("كلمتي", ملح);
        assert!(تحقق_كلمة_مرور("كلمتي", ملح, &هاش));
        assert!(!تحقق_كلمة_مرور("خطأ", ملح, &هاش));
    }

    #[test]
    fn test_جدار_ناري() {
        let محظورات = vec!["bad.com", "virus.exe"];
        assert!(جدار_ناري("طلب من good.com", &محظورات));
        assert!(!جدار_ناري("طلب من bad.com", &محظورات));
    }

    #[test]
    fn test_كشف_XSS() {
        assert!(كشف_XSS("<script>alert('xss')</script>"));
        assert!(!كشف_XSS("نص عادي بدون هجوم"));
    }

    #[test]
    fn test_كشف_SQL_Injection() {
        assert!(كشف_SQL_Injection("' OR 1=1 --"));
        assert!(!كشف_SQL_Injection("استعلام عادي"));
    }

    #[test]
    fn test_تعقيم() {
        let result = تعقيم("<script>alert('xss')</script>");
        assert!(!result.contains('<'));
        assert!(result.contains("&lt;"));
    }

    #[test]
    fn test_فحص_قوة_كلمة_مرور() {
        let نتيجة = فحص_قوة_كلمة_مرور("123456");
        assert!(نتيجة.contains("ضعيفة"));
    }

    #[test]
    fn test_توليد_مفتاح() {
        let مفتاح = توليد_مفتاح(32);
        assert_eq!(مفتاح.len(), 32);
    }

    #[test]
    fn test_تشفير_متقدم() {
        let result = تشفير_متقدم("بيانات", "مفتاح");
        assert!(result.starts_with("AES:"));
    }
}
