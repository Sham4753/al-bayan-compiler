use bayan_to_c::BayanToC;

fn main() {
    let compiler = BayanToC::new();
    let code = "اِقرَأ ملف.txt\nاِحسِب البيانات\nشفر النتيجة\nاِرسِل التقرير الخادم";
    let c = compiler.compile(code);
    println!("{}", c);
}
