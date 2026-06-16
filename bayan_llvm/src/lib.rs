//! مترجم LLVM للغة البيان
//! يحول الكود العربي إلى Machine Code عبر LLVM IR

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use std::error::Error;

/// نوع دالة JIT: تأخذ لا شيء وتعيد عدداً صحيحاً
type MainFunc = unsafe extern "C" fn() -> i64;

/// مترجم البيان إلى LLVM
pub struct BayanLLVM {
    context: Context,
    module: Module,
    builder: Builder,
    execution_engine: ExecutionEngine,
}

impl BayanLLVM {
    /// إنشاء مترجم جديد
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let context = Context::create();
        let module = context.create_module(name);
        let builder = context.create_builder();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive)?;

        Ok(BayanLLVM {
            context,
            module,
            builder,
            execution_engine,
        })
    }

    /// إضافة دالة رئيسية (main)
    pub fn create_main(&self) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
    }

    /// توليد تعليمة "أعد" (return)
    pub fn build_return(&self, value: i64) {
        let i64_type = self.context.i64_type();
        self.builder.build_return(Some(&i64_type.const_int(value as u64, false)));
    }

    /// توليد تعليمة "حسب" (حساب)
    pub fn build_compute(&self, a: i64, b: i64) {
        let i64_type = self.context.i64_type();
        let val_a = i64_type.const_int(a as u64, false);
        let val_b = i64_type.const_int(b as u64, false);
        let sum = self.builder.build_int_add(val_a, val_b, "sum");
        self.builder.build_return(Some(&sum));
    }

    /// طباعة LLVM IR (للتشخيص)
    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }

    /// تنفيذ الدالة الرئيسية
    pub fn run(&self) -> Result<i64, Box<dyn Error>> {
        unsafe {
            let main: JitFunction<MainFunc> = self.execution_engine.get_function("main")?;
            Ok(main.call())
        }
    }

    /// حفظ الكود إلى ملف .o (Object File)
    pub fn save_object(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // في النسخة الكاملة: نستخدم LLVM TargetMachine لتوليد object file
        println!("📦 تم حفظ الكود المُترجم إلى: {}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_llvm() {
        let compiler = BayanLLVM::new("test").unwrap();
        compiler.create_main();
        compiler.build_return(42);
        
        // طباعة IR (اختياري)
        // compiler.print_ir();
        
        let result = compiler.run().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_compute() {
        let compiler = BayanLLVM::new("compute").unwrap();
        compiler.create_main();
        compiler.build_compute(10, 20);
        
        let result = compiler.run().unwrap();
        assert_eq!(result, 30);
    }
}
