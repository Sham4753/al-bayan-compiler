//! الجذر: ف-ص-ل (المنطق الشرطي والتفريع)

/// تفرع شرطي (فَصَلَ)
pub fn فصل<F1, F2, T>(شرط: bool, مسار_الصحيح: F1, مسار_الخاطئ: F2) -> T
where
    F1: FnOnce() -> T,
    F2: FnOnce() -> T,
{
    if شرط {
        مسار_الصحيح()
    } else {
        مسار_الخاطئ()
    }
}

/// تفريع متعدد (فَصَّلَ)
pub fn تفصيل<T: PartialEq>(قيمة: T, حالات: Vec<(T, String)>, افتراضي: Option<String>) -> String {
    for (حالة, نتيجة) in حالات {
        if قيمة == حالة {
            return نتيجة;
        }
    }
    افتراضي.unwrap_or_else(|| "لا توجد حالة مطابقة".to_string())
}

/// استخراج خاصية (اِنفَصَلَ)
pub fn استخراج<T, R>(كائن: &T, مستخرج: fn(&T) -> R) -> R {
    مستخرج(كائن)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_فصل_صحيح() {
        let result = فصل(true, || "نعم", || "لا");
        assert_eq!(result, "نعم");
    }

    #[test]
    fn test_فصل_خاطئ() {
        let result = فصل(false, || "نعم", || "لا");
        assert_eq!(result, "لا");
    }

    #[test]
    fn test_تفصيل() {
        let result = تفصيل(2, vec![
            (1, "واحد".to_string()),
            (2, "اثنان".to_string()),
        ], None);
        assert_eq!(result, "اثنان");
    }
}
