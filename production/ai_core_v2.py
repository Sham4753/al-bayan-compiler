#!/usr/bin/env python3
"""
🧠 AI Core v2 - التعلم العميق
يتعلم من الردود ويصنفها تلقائياً
"""
import json, os, re, hashlib
from datetime import datetime
from collections import Counter

class AICoreV2:
    def __init__(self):
        self.memory_file = "logs/ai_memory_v2.json"
        self.memory = self.load()
        self.level = len(self.memory.get("successes", []))
    
    def load(self):
        if os.path.exists(self.memory_file):
            with open(self.memory_file) as f:
                return json.load(f)
        return {"patterns": [], "successes": [], "failures": [], "signatures": {}}
    
    def analyze_response(self, text, status_code):
        """يحلل الرد ويستخرج بصمات"""
        signature = {
            "length": len(text),
            "status": status_code,
            "has_error": any(w in text.lower() for w in ['error','exception','warning','sql','mysql']),
            "has_login": any(w in text.lower() for w in ['login','password','username','signin']),
            "has_admin": any(w in text.lower() for w in ['admin','dashboard','cpanel']),
            "has_api": '{' in text[:100] or '[' in text[:100],
            "is_html": '<html' in text.lower() or '<!doctype' in text.lower(),
            "hash": hashlib.md5(text[:500].encode()).hexdigest()
        }
        return signature
    
    def learn(self, target, attack, payload, response_text, status_code, success):
        """يتعلم من الهجوم"""
        signature = self.analyze_response(response_text, status_code)
        
        entry = {
            "target": target,
            "attack": attack,
            "payload": str(payload)[:100],
            "signature": signature,
            "success": success,
            "timestamp": datetime.now().isoformat()
        }
        
        self.memory["patterns"].append(entry)
        
        if success:
            self.memory["successes"].append(entry)
            # حفظ بصمة النجاح
            sig_key = f"{target}|{attack}"
            self.memory["signatures"][sig_key] = signature
        else:
            self.memory["failures"].append(entry)
        
        self.level = len(self.memory["successes"])
        self.save()
    
    def predict_best_attack(self, target, response_text, status_code):
        """يتنبأ بأفضل هجوم بناءً على الرد"""
        current_sig = self.analyze_response(response_text, status_code)
        
        # إذا الرد يحتوي على login → جرب Dict Attack
        if current_sig["has_login"]:
            return ["dict_attack", "brute_force"]
        
        # إذا الرد JSON → جرب API Fuzzer
        if current_sig["has_api"]:
            return ["api_fuzzer", "ssti"]
        
        # إذا الرد HTML كبير → جرب Dir Fuzzer
        if current_sig["is_html"] and current_sig["length"] > 5000:
            return ["dir_fuzzer", "ssti"]
        
        # افتراضي
        return ["port_scan", "ssti"]
    
    def save(self):
        os.makedirs("logs", exist_ok=True)
        with open(self.memory_file, 'w') as f:
            json.dump(self.memory, f, indent=2)
    
    def status(self):
        print(f"🧠 AI Core v2 - Level {self.level}")
        print(f"   📚 Patterns: {len(self.memory['patterns'])}")
        print(f"   ✅ Successes: {len(self.memory['successes'])}")
        print(f"   ❌ Failures: {len(self.memory['failures'])}")
        print(f"   🔑 Signatures: {len(self.memory['signatures'])}")
        
        # إحصائيات
        if self.memory["successes"]:
            attacks = Counter([s["attack"] for s in self.memory["successes"]])
            print(f"\n   🏆 أفضل الهجمات:")
            for attack, count in attacks.most_common(3):
                print(f"      {attack}: {count} نجاح")

if __name__ == "__main__":
    ai = AICoreV2()
    
    # تعلم من هجماتنا
    ai.learn("demo.testfire.net", "dict_attack", "admin:admin", 
             "<html>Welcome</html>", 200, True)
    ai.learn("qrlist.app", "ssti", "{{7*7}}", 
             '{"data":[],"success":true}', 200, True)
    ai.learn("qrlist.app", "sql_injection", "' OR '1'='1", 
             "<html>Error</html>", 200, False)
    ai.learn("yallago.net", "brute_force", "admin:password", 
             "<html>Login</html>", 302, False)
    
    ai.status()
    
    # تنبؤ
    print(f"\n🔮 Predictions for new target:")
    preds = ai.predict_best_attack("new-target.com", "<html>Login page</html>", 200)
    for p in preds:
        print(f"   🎯 {p}")
