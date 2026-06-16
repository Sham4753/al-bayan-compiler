//! الجذر: خ-ز-ن (التخزين والذاكرة)

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref المخزن: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// تخزين قيمة (خَزَنَ)
pub fn تخزين(مفتاح: &str, قيمة: &str) {
    let mut store = المخزن.lock().unwrap();
    store.insert(مفتاح.to_string(), قيمة.to_string());
}

/// استرجاع قيمة
pub fn استرجاع(مفتاح: &str) -> Option<String> {
    let store = المخزن.lock().unwrap();
    store.get(مفتاح).cloned()
}

/// حذف قيمة
pub fn حذف(مفتاح: &str) {
    let mut store = المخزن.lock().unwrap();
    store.remove(مفتاح);
}

/// عدد العناصر المخزنة
pub fn عدد_العناصر() -> usize {
    let store = المخزن.lock().unwrap();
    store.len()
}

/// تفريغ المخزن
pub fn تفريغ() {
    let mut store = المخزن.lock().unwrap();
    store.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_تخزين_واسترجاع() {
        تخزين("مفتاح", "قيمة");
        assert_eq!(استرجاع("مفتاح"), Some("قيمة".to_string()));
        حذف("مفتاح");
        assert_eq!(استرجاع("مفتاح"), None);
    }

    #[test]
    fn test_عدد_العناصر() {
        تفريغ();
        تخزين("أ", "1");
        تخزين("ب", "2");
        assert_eq!(عدد_العناصر(), 2);
        تفريغ();
    }
}
