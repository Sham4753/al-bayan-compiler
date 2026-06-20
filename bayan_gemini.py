#!/usr/bin/env python3
# 🧠 البيان + Gemini = الذكاء العربي

from bayan_full import ROOTS

class BayanGemini:
    def __init__(self):
        self.roots = ROOTS
        self.memory = {}
        self.thoughts = []
    
    def think_in_bayan(self, idea):
        """يفكر بالبيان"""
        # تحليل الفكرة بالجذور العربية
        words = idea.split()
        roots_found = []
        
        for word in words:
            for i in range(len(word), 2, -1):
                chunk = word[:i]
                if chunk in self.roots:
                    roots_found.append({
                        "word": word,
                        "root": chunk,
                        "meaning": self.roots[chunk],
                    })
                    break
        
        return {
            "idea": idea,
            "roots": roots_found,
            "count": len(roots_found),
        }
    
    def generate_bayan_code(self, idea):
        """يولد كود البيان من فكرة"""
        thoughts = self.think_in_bayan(idea)
        
        code = ["// 🧠 كود مولّد بالذكاء العربي"]
        code.append("")
        
        # استخدام الجذور المستخرجة
        for r in thoughts['roots']:
            code.append(f"{r['word']}  // {r['meaning']}")
        
        if not thoughts['roots']:
            code.append("اِحتَسَبَ  // فحص النظام")
            code.append("حَلَّلَ   // تحليل")
        
        code.append("بَعَثَ    // إرسال النتيجة")
        
        return "\n".join(code)
    
    def merge_with_gemini(self, gemini_response):
        """يدمج رد Gemini مع جذور البيان"""
        analysis = self.think_in_bayan(gemini_response)
        
        # استخراج الجذور من رد Gemini
        bayan_code = self.generate_bayan_code(gemini_response)
        
        return {
            "gemini_said": gemini_response,
            "bayan_roots": analysis['roots'],
            "bayan_code": bayan_code,
            "total_roots_available": len(self.roots),
        }

# محاكاة حوار مع Gemini
bg = BayanGemini()

print("🧠 البيان + Gemini")
print("="*50)

# ١. Gemini يقترح فكرة
gemini_idea = "بناء نظام ذكي لتحليل المشاعر في النصوص العربية"
print(f"\n💬 Gemini: {gemini_idea}")

# ٢. البيان تحلل الفكرة
analysis = bg.think_in_bayan(gemini_idea)
print(f"\n🔍 البيان: وجدت {analysis['count']} جذر في الفكرة")
for r in analysis['roots']:
    print(f"   • {r['word']} → {r['root']} ({r['meaning']})")

# ٣. البيان تولد الكود
code = bg.generate_bayan_code(gemini_idea)
print(f"\n📝 البيان: ولدت الكود التالي:")
print(code)

# ٤. إحصائيات
print(f"\n📊 إحصائيات:")
print(f"   • جذور البيان: {len(ROOTS)}")
print(f"   • الجذور المستخدمة: {analysis['count']}")
print(f"   • الكفاءة: {analysis['count']/len(ROOTS)*100:.2f}%")

print(f"\n🕌 البيان + Gemini = الذكاء العربي")
