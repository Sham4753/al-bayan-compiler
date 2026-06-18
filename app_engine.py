#!/usr/bin/env python3
# 🚀 Bayan App Engine - محرك التطبيقات

import sys
from bayan_full import ROOTS

class AppEngine:
    def __init__(self):
        self.roots = ROOTS
        self.apps = {}
    
    def create_app(self, idea):
        """يحول فكرة إلى تطبيق"""
        words = idea.split()
        app_type = self.detect_type(words)
        app_name = self.extract_name(words)
        
        # بناء هيكل التطبيق
        app = {
            "name": app_name,
            "type": app_type,
            "pages": self.generate_pages(app_type),
            "features": self.generate_features(app_type),
            "code": self.generate_code(app_type, app_name),
        }
        
        self.apps[app_name] = app
        return app
    
    def detect_type(self, words):
        """يكتشف نوع التطبيق من الكلمات"""
        type_map = {
            "متجر": "ecommerce",
            "مدونة": "blog",
            "موقع": "website",
            "نظام": "dashboard",
            "تطبيق": "mobile",
            "خادم": "server",
            "قاعدة": "database",
            "شبكة": "network",
            "ذكاء": "ai",
            "تعلم": "ml",
        }
        for word in words:
            if word in type_map:
                return type_map[word]
        return "website"
    
    def extract_name(self, words):
        """يستخرج اسم التطبيق"""
        name_words = []
        for i, word in enumerate(words):
            if word in ["اسم", "يسمى", "يدعى"]:
                if i+1 < len(words):
                    return words[i+1]
        return words[-1] if words else "تطبيقي"
    
    def generate_pages(self, app_type):
        """يولد صفحات التطبيق"""
        pages = {
            "ecommerce": ["الرئيسية", "المنتجات", "سلة الشراء", "الدفع", "حسابي"],
            "blog": ["الرئيسية", "المقالات", "من نحن", "اتصل بنا"],
            "website": ["الرئيسية", "من أنا", "أعمالي", "تواصل"],
            "dashboard": ["لوحة التحكم", "المستخدمين", "الإحصائيات", "الإعدادات"],
            "mobile": ["تسجيل", "الرئيسية", "الملف", "الإعدادات"],
            "server": ["حالة", "سجلات", "إعدادات", "مراقبة"],
            "database": ["جداول", "استعلام", "تصدير", "نسخ"],
            "network": ["أجهزة", "منافذ", "حركة", "أمان"],
            "ai": ["تدريب", "اختبار", "تنبؤ", "تقييم"],
            "ml": ["بيانات", "نموذج", "تدريب", "نتائج"],
        }
        return pages.get(app_type, ["الرئيسية", "عن", "تواصل"])
    
    def generate_features(self, app_type):
        """يولد ميزات التطبيق"""
        features = {
            "ecommerce": ["عربة تسوق", "دفع إلكتروني", "تتبع الطلب", "تقييمات"],
            "blog": ["تعليقات", "مشاركة", "بحث", "أرشيف"],
            "website": ["معرض أعمال", "نموذج تواصل", "سيرة ذاتية"],
            "dashboard": ["رسوم بيانية", "تقارير", "تنبيهات", "صلاحيات"],
            "mobile": ["إشعارات", "كاميرا", "موقع", "مشاركة"],
            "server": ["مراقبة", "تنبيهات", "نسخ", "سجلات"],
        }
        return features.get(app_type, ["أساسي", "متقدم"])
    
    def generate_code(self, app_type, app_name):
        """يولد كود البيان للتطبيق"""
        code_lines = [
            f"// 🚀 {app_name} - تطبيق {app_type}",
            "",
            "// بداية التطبيق",
            "اِحتَسَبَ",
            "حَفِظَ",
        ]
        
        if app_type in ["ecommerce", "website", "blog"]:
            code_lines.extend(["رَسَمَ", "فَتَحَ", "اِنبَعَثَ"])
        if app_type in ["server", "network"]:
            code_lines.extend(["اِنبَعَثَ", "فَتَحَ", "بَعَثَ"])
        if app_type in ["database"]:
            code_lines.extend(["خَزَنَ", "جَمَعَ", "فَصَلَ"])
        if app_type in ["ai", "ml"]:
            code_lines.extend(["حَلَّلَ", "صَمَّمَ", "طَوَّرَ"])
        
        code_lines.extend([
            "",
            "// نهاية التطبيق",
            "بَعَثَ",
            "كَتَبَ",
        ])
        
        return "\n".join(code_lines)
    
    def show(self, app):
        """يعرض التطبيق"""
        print(f"""
╔══════════════════════════════════╗
║   🚀 {app['name']}                       ║
║   نوع: {app['type']}                       
║   صفحات: {len(app['pages'])}               
║   ميزات: {len(app['features'])}             
╚══════════════════════════════════╝

📄 الصفحات:""")
        for i, page in enumerate(app['pages'], 1):
            print(f"   {i}. {page}")
        
        print("\n✨ الميزات:")
        for i, feat in enumerate(app['features'], 1):
            print(f"   {i}. {feat}")
        
        print(f"\n📝 كود البيان:\n{app['code']}")
        print(f"\n🔢 الجذور المستخدمة: {len(ROOTS)} جذر متاح")

# محاكاة
engine = AppEngine()

# أمثلة
ideas = [
    "متجر اسم سوق لبيع العطور",
    "مدونة اسم تقنيات",
    "نظام اسم مراقب",
    "ذكاء اسم مفكر",
]

for idea in ideas:
    app = engine.create_app(idea)
    engine.show(app)
    print("\n" + "="*40 + "\n")

print(f"✅ محرك التطبيقات جاهز. {len(ROOTS)} جذر متاح.")
