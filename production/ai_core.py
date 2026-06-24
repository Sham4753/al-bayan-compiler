#!/usr/bin/env python3
"""
🧠 AI Core - نواة الذكاء الاصطناعي
يتعلم من كل هجوم ويصبح أقوى
"""
import json, os, re
from datetime import datetime

class AICore:
    def __init__(self):
        self.memory_file = "logs/ai_memory.json"
        self.memory = self.load_memory()
        self.intelligence = len(self.memory.get("patterns", []))
    
    def load_memory(self):
        if os.path.exists(self.memory_file):
            with open(self.memory_file) as f:
                return json.load(f)
        return {"patterns": [], "successes": [], "failures": [], "level": 0}
    
    def learn(self, target, attack_type, payload, response, success=False):
        """يتعلم من كل محاولة"""
        pattern = {
            "target": target,
            "type": attack_type,
            "payload": payload,
            "response_length": len(response),
            "response_keywords": re.findall(r'error|sql|uid=|root:|admin', response.lower()),
            "success": success,
            "timestamp": datetime.now().isoformat()
        }
        
        self.memory["patterns"].append(pattern)
        if success:
            self.memory["successes"].append(pattern)
            self.memory["level"] += 1
        else:
            self.memory["failures"].append(pattern)
        
        self.intelligence = len(self.memory["patterns"])
        self.save()
    
    def predict(self, target):
        """يتنبأ بأفضل هجوم"""
        # إذا الهدف فيه "laravel" → جرب SSTI
        # إذا الهدف فيه "wordpress" → جرب wp-admin
        # إذا الهدف فيه "login" → جرب Dict Attack
        
        predictions = []
        for pattern in self.memory["successes"]:
            if pattern["target"] in target:
                predictions.append(pattern["type"])
        
        return list(set(predictions))[:3]
    
    def save(self):
        os.makedirs("logs", exist_ok=True)
        with open(self.memory_file, 'w') as f:
            json.dump(self.memory, f, indent=2)
    
    def status(self):
        print(f"🧠 AI Core - Level {self.memory['level']}")
        print(f"   📚 Patterns: {len(self.memory['patterns'])}")
        print(f"   ✅ Successes: {len(self.memory['successes'])}")
        print(f"   ❌ Failures: {len(self.memory['failures'])}")
        print(f"   🧠 Intelligence: {self.intelligence}")

if __name__ == "__main__":
    ai = AICore()
    
    # تعلم من هجماتنا السابقة
    ai.learn("demo.testfire.net", "dict_attack", "admin:admin", "200 OK", True)
    ai.learn("qrlist.app", "ssti", "{{7*7}}", "49", True)
    ai.learn("qrlist.app", "sql_injection", "' OR '1'='1", "HTML page", False)
    ai.learn("yallago.net", "brute_force", "admin:password", "302", False)
    
    ai.status()
    
    print(f"\n🔮 Predictions for qrlist.app:")
    for p in ai.predict("qrlist.app"):
        print(f"   🎯 {p}")
