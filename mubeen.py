#!/usr/bin/env python3
# 🧠 مُبين - معالج لغات طبيعية بالبيان

from bayan_full import ROOTS
import re

class Mubeen:
    def __init__(self):
        self.roots = ROOTS
        self.memory = {}
    
    def analyze_text(self, text):
        """يحلل النص ويستخرج الجذور والمعاني"""
        words = re.findall(r'\w+', text)
        
        found_roots = []
        emotions = []
        actions = []
        
        for word in words:
            # البحث عن الجذر
            for i in range(len(word), 2, -1):
                chunk = word[:i]
                if chunk in self.roots:
                    found_roots.append({
                        "word": word,
                        "root": chunk,
                        "meaning": self.roots[chunk],
                    })
                    break
            
            # كشف العواطف
            if word in ["فرح", "حزن", "غضب", "خاف", "أحب", "كره"]:
                emotions.append(word)
            
            # كشف الأفعال
            if word in ["قال", "سأل", "أجاب", "نادى", "كتب", "قرأ"]:
                actions.append(word)
        
        return {
            "text": text,
            "roots": found_roots,
            "emotions": emotions,
            "actions": actions,
            "word_count": len(words),
            "root_count": len(found_roots),
        }
    
    def summarize(self, text):
        """يلخص النص"""
        analysis = self.analyze_text(text)
        
        summary = f"""
╔══════════════════════════════════════╗
║   🧠 مُبين - تحليل النص              ║
╠══════════════════════════════════════╣
║   📝 الكلمات: {analysis['word_count']}                     ║
║   🔤 الجذور المكتشفة: {analysis['root_count']}             ║
║   ❤️ العواطف: {', '.join(analysis['emotions']) or 'لا يوجد'}    ║
║   🎬 الأفعال: {', '.join(analysis['actions']) or 'لا يوجد'}    ║
╚══════════════════════════════════════╝

🔍 الجذور المستخرجة:"""
        for r in analysis['roots']:
            summary += f"\n   • {r['word']} → جذر: {r['root']} ({r['meaning']})"
        
        return summary
    
    def understand_intent(self, text):
        """يفهم قصد النص"""
        analysis = self.analyze_text(text)
        
        # تحديد القصد من العواطف والأفعال
        if "فرح" in analysis['emotions'] or "أحب" in analysis['emotions']:
            intent = "إيجابي - تعبير عن سعادة"
        elif "حزن" in analysis['emotions'] or "خاف" in analysis['emotions']:
            intent = "سلبي - تعبير عن حزن أو خوف"
        elif analysis['actions']:
            intent = f"فعلي - يحتوي على {len(analysis['actions'])} فعل"
        else:
            intent = "وصفي - نص وصفي"
        
        return {
            "intent": intent,
            "confidence": min(analysis['root_count'] / max(analysis['word_count'], 1) * 100, 100),
            "roots_used": [r['root'] for r in analysis['roots']],
        }

# تجربة
mubeen = Mubeen()

# ١. تحليل نص أدبي
poem = """
أحب الكتابة في الليل عندما ينام الجميع
وأكتب عن فرح القلب وحزن العيون
أسأل القمر عن سر الجمال
فيجيبني بضوء فضي هادئ
"""

print("📜 تحليل نص أدبي:")
print(mubeen.summarize(poem))

# ٢. فهم القصد
print("\n🎯 فهم القصد:")
intent = mubeen.understand_intent(poem)
print(f"   القصد: {intent['intent']}")
print(f"   الثقة: {intent['confidence']:.1f}%")
print(f"   الجذور: {', '.join(intent['roots_used'])}")

# ٣. إحصائيات
print(f"\n📊 الجذور المحملة: {len(ROOTS)}")
print("🧠 مُبين: لغة تفهم أدبها")
