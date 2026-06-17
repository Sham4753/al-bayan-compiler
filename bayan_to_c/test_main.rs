use bayan_to_c::BayanToC;

fn main() {
    let compiler = BayanToC::new();
    let c_code = compiler.compile("اِحتَسَبَ\nحَفِظَ\nبَعَثَ\nكرر شكر 3 مرات");
    println!("{}", c_code);
}
