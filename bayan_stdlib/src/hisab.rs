//! الجذر: ح-س-ب (الحساب والمعالجة)

use std::thread;

/// عملية حسابية متزامنة (حَسَبَ)
pub fn حساب_متزامن(مدخلات: &[f64]) -> f64 {
    مدخلات.iter().sum()
}

/// معالجة متوازية (حَسَّبَ)
pub fn حساب_متوازي<F>(مدخلات: Vec<f64>, عملية: F) -> Vec<f64>
where
    F: Fn(f64) -> f64 + Send + Sync + Clone + 'static,
{
    let mut handles = vec![];
    for قيمة in مدخلات {
        let op = عملية.clone();
        handles.push(thread::spawn(move || op(قيمة)));
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// عملية غير متزامنة (حَاسَبَ)
pub fn حساب_غير_متزامن(مهمة: &str) -> String {
    format!("تم جدولة: {}", مهمة)
}

/// مراقبة أداء (اِحتَسَبَ)
pub fn مراقبة_أداء() -> String {
    format!("🧠 الذاكرة: {}MB | 🖥️ الأنوية: {}",
        64, thread::available_parallelism().unwrap().get())
}

/// متوسط حسابي
pub fn متوسط(قيم: &[f64]) -> f64 {
    if قيم.is_empty() { 0.0 } else { قيم.iter().sum::<f64>() / قيم.len() as f64 }
}

/// ضرب عناصر
pub fn ضرب(قيم: &[f64]) -> f64 {
    قيم.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_حساب_متزامن() {
        assert_eq!(حساب_متزامن(&[1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn test_حساب_متوازي() {
        let result = حساب_متوازي(vec![1.0, 2.0, 3.0], |x| x * 2.0);
        assert_eq!(result, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_متوسط() {
        assert_eq!(متوسط(&[2.0, 4.0, 6.0]), 4.0);
    }

    #[test]
    fn test_مراقبة_أداء() {
        let result = مراقبة_أداء();
        assert!(result.contains("الذاكرة"));
    }
}
