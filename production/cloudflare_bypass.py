#!/usr/bin/env python3
"""
🧱 Cloudflare Bypass - يفتح الجدار
يبحث عن Origin IP الحقيقي ويتجاوز Cloudflare
"""
import requests, sys, json, os, re, socket, subprocess
from datetime import datetime
from urllib.parse import urlparse

class CloudflareBypass:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.origins = []
        self.results = {}
    
    def log(self, msg): print(f"[🧱] {msg}")
    
    def dns_history(self):
        """البحث في تاريخ DNS"""
        self.log("📡 DNS History...")
        
        # SecurityTrails API
        try:
            r = requests.get(f"https://securitytrails.com/domain/{self.domain}/dns/a", timeout=10)
            ips = re.findall(r'[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+', r.text)
            for ip in ips:
                if not ip.startswith(('104.','172.','162.')):
                    self.origins.append(ip)
                    self.log(f"   🚨 Origin IP: {ip}")
        except:
            pass
    
    def certificate_search(self):
        """البحث في شهادات SSL"""
        self.log("🔒 SSL Certificates...")
        try:
            result = subprocess.run(
                ["curl", "-sk", f"https://crt.sh/?q=%25.{self.domain}&output=json"],
                capture_output=True, text=True, timeout=10
            )
            data = json.loads(result.stdout)
            for entry in data[:20]:
                name = entry.get('name_value','')
                ips = re.findall(r'[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+', name)
                for ip in ips:
                    if not ip.startswith(('104.','172.','162.')):
                        self.origins.append(ip)
        except:
            pass
    
    def spf_check(self):
        """فحص SPF Record"""
        self.log("📧 SPF...")
        try:
            result = subprocess.run(
                ["dig","+short","TXT",self.domain],
                capture_output=True, text=True, timeout=5
            )
            ips = re.findall(r'ip4:([0-9.]+)', result.stdout)
            for ip in ips:
                if not ip.startswith(('104.','172.','162.')):
                    self.origins.append(ip)
                    self.log(f"   🚨 SPF Origin IP: {ip}")
        except:
            pass
    
    def scan_origin(self, ip):
        """فحص Origin IP مباشرة"""
        self.log(f"🔍 فحص {ip}...")
        results = {}
        
        # HTTP
        try:
            r = requests.get(f"http://{ip}", headers={"Host":self.domain}, timeout=5)
            results["http"] = r.status_code
        except:
            pass
        
        # HTTPS
        try:
            r = requests.get(f"https://{ip}", headers={"Host":self.domain}, timeout=5, verify=False)
            results["https"] = r.status_code
        except:
            pass
        
        # cPanel
        for port in [2083, 2087, 10000, 20000]:
            try:
                s = socket.socket(); s.settimeout(2)
                if s.connect_ex((ip, port)) == 0:
                    results[f"port_{port}"] = "open"
                s.close()
            except:
                pass
        
        return results
    
    def run(self):
        self.log(f"🧱 Cloudflare Bypass: {self.target}")
        
        self.dns_history()
        self.certificate_search()
        self.spf_check()
        
        self.origins = list(set(self.origins))
        self.log(f"\n📊 {len(self.origins)} Origin IPs found")
        
        for ip in self.origins[:5]:
            results = self.scan_origin(ip)
            if results:
                self.results[ip] = results
                self.log(f"   {ip}: {results}")
        
        return self.results

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    CloudflareBypass(target).run()
