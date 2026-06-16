//! الجذر: ج-م-ع (التجميع والمصفوفات)

use std::thread;

/// تجميع عناصر في مصفوفة (جَمَعَ)
pub fn جمع_عناصر<T>(عناصر: Vec<T>) -> Vec<T> {
    عناصر
}

/// تجميع متوازي (جَمَّعَ)
pub fn جمع_متوازي<T, F, R>(عناصر: Vec<T>, دالة: F) -> Vec<R>
where
    F: Fn(T) -> R + Send + Sync + Clone + 'static,
    T: Send + 'static,
    R: Send + 'static,
{
    let mut handles = vec![];
    for عنصر in عناصر {
        let f = دالة.clone();
        handles.push(thread::spawn(move || f(عنصر)));
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// طول المصفوفة
pub fn طول<T>(مصفوفة: &[T]) -> usize {
    مصفوفة.len()
}

/// هل المصفوفة فارغة؟
pub fn فارغة<T>(مصفوفة: &[T]) -> bool {
    مصفوفة.is_empty()
}

/// تصفية عناصر
pub fn تصفية<T: Clone, F>(مصفوفة: &[T], شرط: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    مصفوفة.iter().filter(|x| شرط(x)).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_جمع_عناصر() {
        let result = جمع_عناصر(vec![1, 2, 3]);
        assert_eq!(طول(&result), 3);
    }

    #[test]
    fn test_تصفية() {
        let result = تصفية(&[1, 2, 3, 4, 5, 6], |x| *x % 2 == 0);
        assert_eq!(result, vec![2, 4, 6]);
    }
}
