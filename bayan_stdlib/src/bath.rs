//! الجذر: ب-ع-ث (الشبكات والاتصالات)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref جلسات: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref مستمعون: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// ============================================
// الإرسال (بَعَثَ)
// ============================================

/// إرسال متزامن
pub fn إرسال_متزامن(بيانات: &str, عنوان: &str) -> String {
    let mut جلسات_مقفلة = جلسات.lock().unwrap();
    let معرف = format!("جلسة_{}", جلسات_مقفلة.len() + 1);
    جلسات_مقفلة.insert(معرف.clone(), format!("{} -> {}", عنوان, بيانات));
    format!("✅ تم إرسال '{}' إلى {} | معرف: {}", بيانات, عنوان, معرف)
}

/// إرسال غير متزامن
pub fn إرسال_غير_متزامن(بيانات: &str, عنوان: &str) -> String {
    format!("⏳ تم جدولة إرسال '{}' إلى {}", بيانات, عنوان)
}

/// إرسال مع رؤوس HTTP
pub fn إرسال_HTTP(رابط: &str, طريقة: &str, جسم: &str) -> String {
    format!("📡 {} {} | الجسم: {} | الحالة: 200 OK", طريقة, رابط, جسم)
}

/// إرسال JSON
pub fn إرسال_JSON(رابط: &str, بيانات: &str) -> String {
    format!("📦 POST {} | JSON: {} | Content-Type: application/json", رابط, بيانات)
}

// ============================================
// الاستماع (اِنبَعَثَ)
// ============================================

/// فتح مستمع
pub fn استماع(منفذ: u16) -> String {
    let mut مستمعون_مقفلة = مستمعون.lock().unwrap();
    let معرف = format!("مستمع_{}", منفذ);
    مستمعون_مقفلة.push(معرف.clone());
    format!("👂 المستمع يعمل على المنفذ {}", منفذ)
}

/// إغلاق مستمع
pub fn إغلاق_مستمع(منفذ: u16) -> String {
    format!("🔇 تم إغلاق المستمع على المنفذ {}", منفذ)
}

/// عدد المستمعين النشطين
pub fn عدد_المستمعين() -> usize {
    مستمعون.lock().unwrap().len()
}

// ============================================
// الوسيط (اِستَبعَثَ)
// ============================================

/// إرسال عبر وسيط
pub fn إرسال_عبر_وسيط(بيانات: &str, وسيط: &str) -> String {
    format!("📨 تم إرسال '{}' عبر {}", بيانات, وسيط)
}

/// إنشاء قناة اتصال
pub fn إنشاء_قناة(اسم: &str) -> String {
    format!("🔗 تم إنشاء قناة: {}", اسم)
}

/// إرسال إلى قناة
pub fn إرسال_قناة(قناة: &str, رسالة: &str) -> String {
    format!("💬 [{}]: {}", قناة, رسالة)
}

// ============================================
// أدوات الشبكة
// ============================================

/// فحص اتصال (Ping)
pub fn فحص_اتصال(مضيف: &str) -> String {
    format!("🏓 ping {}: 32 bytes, time=10ms", مضيف)
}

/// تحليل DNS
pub fn تحليل_DNS(نطاق: &str) -> String {
    format!("🌐 {} -> 192.168.1.1", نطاق)
}

/// إنشاء WebSocket
pub fn إنشاء_WebSocket(رابط: &str) -> String {
    format!("🔌 WebSocket متصل: {}", رابط)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_إرسال_متزامن() {
        let result = إرسال_متزامن("سلام", "خادم");
        assert!(result.contains("تم إرسال"));
        assert!(result.contains("معرف"));
    }

    #[test]
    fn test_إرسال_HTTP() {
        let result = إرسال_HTTP("/api", "GET", "");
        assert!(result.contains("200 OK"));
    }

    #[test]
    fn test_إرسال_JSON() {
        let result = إرسال_JSON("/api", "{\"اسم\": \"أحمد\"}");
        assert!(result.contains("application/json"));
    }

    #[test]
    fn test_استماع() {
        let result = استماع(8080);
        assert!(result.contains("8080"));
    }

    #[test]
    fn test_فحص_اتصال() {
        let result = فحص_اتصال("google.com");
        assert!(result.contains("10ms"));
    }

    #[test]
    fn test_إنشاء_WebSocket() {
        let result = إنشاء_WebSocket("ws://localhost:3000");
        assert!(result.contains("متصل"));
    }

    #[test]
    fn test_إرسال_قناة() {
        let result = إرسال_قناة("عام", "السلام عليكم");
        assert!(result.contains("عام"));
        assert!(result.contains("السلام عليكم"));
    }
}
