use bayan_llvm::BayanLLVM;

fn main() {
    let compiler = BayanLLVM::new("bayan_module").unwrap();
    let result = compiler.compile("اِحتَسَبَ").unwrap();
    println!("✅ نتيجة LLVM: {}", result);
    compiler.print_ir();
}
