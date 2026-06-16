//! الجذر: ر-س-م (الواجهات والعرض)

/// رسم عنصر (رَسَمَ)
pub fn رسم(عنصر: &str) -> String {
    format!("<{}>{}</{}>", عنصر, " ", عنصر)
}

/// رسم متكرر (رَسَّمَ)
pub fn رسِّم(عناصر: &[&str]) -> Vec<String> {
    عناصر.iter().map(|x| رسم(x)).collect()
}

/// إعادة رسم (اِرتَسَمَ)
pub fn إعادة_رسم() -> String {
    "🔄 تم تحديث الواجهة".to_string()
}

/// إنشاء جدول
pub fn جدول(رؤوس: &[&str], صفوف: &[Vec<&str>]) -> String {
    let mut result = String::new();
    result.push_str("<table>\n<tr>");
    for رأس in رؤوس {
        result.push_str(&format!("<th>{}</th>", رأس));
    }
    result.push_str("</tr>\n");
    for صف in صفوف {
        result.push_str("<tr>");
        for خلية in صف {
            result.push_str(&format!("<td>{}</td>", خلية));
        }
        result.push_str("</tr>\n");
    }
    result.push_str("</table>");
    result
}

/// إنشاء زر
pub fn زر(نص: &str, حدث: &str) -> String {
    format!("<button onclick=\"{}\">{}</button>", حدث, نص)
}

/// إنشاء حقل إدخال
pub fn حقل_إدخال(اسم: &str, نوع: &str) -> String {
    format!("<input name=\"{}\" type=\"{}\" />", اسم, نوع)
}

/// إنشاء نموذج
pub fn نموذج(عناصر: &[String], إجراء: &str) -> String {
    let mut result = format!("<form action=\"{}\">\n", إجراء);
    for عنصر in عناصر {
        result.push_str(&format!("  {}\n", عنصر));
    }
    result.push_str("</form>");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_رسم() {
        let result = رسم("div");
        assert!(result.contains("<div>"));
    }

    #[test]
    fn test_رسِّم() {
        let result = رسِّم(&["p", "span"]);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("<p>"));
    }

    #[test]
    fn test_جدول() {
        let result = جدول(
            &["اسم", "عمر"],
            &[vec!["أحمد", "30"], vec!["محمد", "25"]]
        );
        assert!(result.contains("<table>"));
        assert!(result.contains("أحمد"));
        assert!(result.contains("25"));
    }

    #[test]
    fn test_زر() {
        let result = زر("اضغط", "تنبيه()");
        assert!(result.contains("<button"));
        assert!(result.contains("اضغط"));
        assert!(result.contains("تنبيه()"));
    }

    #[test]
    fn test_حقل_إدخال() {
        let result = حقل_إدخال("البريد", "email");
        assert!(result.contains("email"));
    }

    #[test]
    fn test_نموذج() {
        let عناصر = vec![
            حقل_إدخال("اسم", "text"),
            زر("إرسال", "submit()"),
        ];
        let result = نموذج(&عناصر, "/submit");
        assert!(result.contains("<form"));
        assert!(result.contains("إرسال"));
    }
}
