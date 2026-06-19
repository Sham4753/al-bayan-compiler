#!/usr/bin/env python3
# 🤖 بوت البيان - أول منتج تجاري

from bayan_full import ROOTS

class BayanBot:
    def __init__(self, business_name="البيان"):
        self.name = business_name
        self.roots = ROOTS
        self.services = {
            "تحليل": "أحلل بياناتك بـ 3,260 جذر عربي",
            "تنبؤ": "أتنبأ بالمبيعات بدقة عالية",
            "أمان": "أحمي بياناتك بتشفير عربي",
            "سرعة": "أنجز 100,000 عملية في 0.0187 ثانية",
        }
    
    def respond(self, query):
        """يرد على استفسار العميل"""
        for key, value in self.services.items():
            if key in query:
                return f"✅ {value}"
        
        if "سعر" in query or "كم" in query:
            return "💰 الباقة الأساسية: 50$ شهرياً | الباقة المتقدمة: 150$ شهرياً"
        
        if "جرب" in query or "مثال" in query:
            return "📊 جرب: اكتب 'تنبؤ مبيعات' أو 'تحليل بيانات'"
        
        return f"🕌 {self.name} في خدمتك. اسأل عن: " + " | ".join(self.services.keys())
    
    def demo(self):
        """عرض توضيحي للعميل"""
        print(f"🤖 بوت {self.name}")
        print("="*40)
        
        queries = ["تحليل", "تنبؤ", "سعر", "جرب", "أمان"]
        for q in queries:
            print(f"👤 العميل: {q}")
            print(f"🤖 البوت: {self.respond(q)}")
            print()
        
        print(f"📚 {len(self.roots)} جذر | ⚡ 0.0187s | 💰 من 50$/شهر")

# عرض البوت
bot = BayanBot("حران تك")
bot.demo()
