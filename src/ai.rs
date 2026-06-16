use std::collections::HashMap;

pub struct BayanAI {
    knowledge: HashMap<String, String>,
    numbers: Vec<f64>,
}

impl BayanAI {
    pub fn new() -> Self {
        BayanAI { knowledge: HashMap::new(), numbers: vec![] }
    }

    pub fn learn(&mut self, question: &str, answer: &str) {
        self.knowledge.insert(question.to_string(), answer.to_string());
    }

    pub fn answer(&self, question: &str) -> String {
        self.knowledge.get(question).cloned().unwrap_or_else(|| "لا أعلم".to_string())
    }

    pub fn train_numbers(&mut self, data: Vec<f64>) {
        self.numbers = data;
    }

    pub fn predict(&self) -> f64 {
        if self.numbers.len() < 2 { return 0.0; }
        let last = self.numbers[self.numbers.len() - 1];
        let prev = self.numbers[self.numbers.len() - 2];
        last + (last - prev)
    }

    pub fn average(&self) -> f64 {
        if self.numbers.is_empty() { 0.0 }
        else { self.numbers.iter().sum::<f64>() / self.numbers.len() as f64 }
    }

    pub fn sentiment(&self, text: &str) -> &str {
        let positive = ["حب", "جميل", "رائع", "ممتاز", "فرح", "سعيد"];
        let negative = ["حزن", "حزين", "سيء", "قبيح", "غضب", "كره"];
        for word in &positive {
            if text.contains(word) { return "إيجابي 😊"; }
        }
        for word in &negative {
            if text.contains(word) { return "سلبي 😢"; }
        }
        "محايد 😐"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learn_answer() {
        let mut ai = BayanAI::new();
        ai.learn("ما البيان؟", "لغة برمجة عربية");
        assert_eq!(ai.answer("ما البيان؟"), "لغة برمجة عربية");
    }

    #[test]
    fn test_predict() {
        let mut ai = BayanAI::new();
        ai.train_numbers(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(ai.predict(), 5.0);
    }

    #[test]
    fn test_sentiment() {
        let ai = BayanAI::new();
        assert_eq!(ai.sentiment("أنا سعيد اليوم"), "إيجابي 😊");
        assert_eq!(ai.sentiment("حزين جداً"), "سلبي 😢");
    }
}
