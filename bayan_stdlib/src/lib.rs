// ============================================================
// مكتبة القرآن الأساسية للغة البيان
// ============================================================

pub mod qiraa;    // ق-ر-أ: القراءة
pub mod kitaba;   // ك-ت-ب: الكتابة
pub mod hisab;    // ح-س-ب: الحساب
pub mod khazn;    // خ-ز-ن: التخزين
pub mod bath;     // ب-ع-ث: البعث
pub mod jam3;     // ج-م-ع: الجمع
pub mod fasl;     // ف-ص-ل: الفصل
pub mod rasm;     // ر-س-م: الرسم
pub mod ilm;      // ع-ل-م: العلم
pub mod hifz;     // ح-ف-ظ: الحفظ

/// النوع الأساسي للبيانات في البيان
#[derive(Debug, Clone, PartialEq)]
pub enum قيمة {
    نص(String),
    رقم(f64),
    صحيح(i64),
    منطقي(bool),
    مصفوفة(Vec<قيمة>),
    كائن(Vec<(String, قيمة)>),
    لا_شيء,
}

pub use qiraa::*;
pub use kitaba::*;
pub use hisab::*;
pub use khazn::*;
pub use bath::*;
pub use jam3::*;
pub use fasl::*;
pub use rasm::*;
pub use ilm::*;
pub use hifz::*;
pub mod more_roots;
