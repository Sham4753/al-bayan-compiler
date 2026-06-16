use bayan_compiler::tasreef::TasreefRegister;

fn main() {
    println!("🕌 لغة البيان - المترجم v{}", bayan_compiler::BAYAN_VERSION);
    println!("✨ {}", bayan_compiler::BAYAN_SLOGAN);
    println!();

    let register = TasreefRegister::new();
    println!("✅ تم تحميل سجل التصريف: {} تركيباً", register.register.len());
}
