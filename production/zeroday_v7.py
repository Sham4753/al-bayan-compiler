#!/usr/bin/env python3
"""
🚀 Zero-Day Hunter v7 - Ultimate
الأذكى، الأدهى، الأسرع
"""
import requests, sys, json, os, re, socket, time, random
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor, as_completed

class ZeroDayV7:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.results = {"target":target,"timestamp":datetime.now().isoformat(),
                       "dns":{},"ports":[],"apis":[],"keys":[],"emails":[],"vulns":[],"score":0}
        self.session = requests.Session()
        self.start_time = time.time()
    
    def log(self, msg): print(f"[🚀] {msg}")
    
    def parallel_scan(self):
        """فحص متوازي لكل شيء"""
        self.log("⚡ Parallel Scan...")
        
        def check_port(p):
            s = socket.socket(); s.settimeout(0.5)
            if s.connect_ex((self.domain, p)) == 0:
                self.results["ports"].append(p)
            s.close()
        
        def check_path(path):
            try:
                r = self.session.get(f"{self.target}{path}", timeout=3, allow_redirects=False)
                if r.status_code in [200,302,403]:
                    return {"path":path,"status":r.status_code}
            except: pass
            return None
        
        def check_api(endpoint):
            try:
                r = self.session.get(f"{self.target}{endpoint}", timeout=3)
                if r.status_code == 200 and len(r.text) > 100:
                    if r.text.strip().startswith('{') or r.text.strip().startswith('['):
                        return {"endpoint":endpoint,"size":len(r.text)}
            except: pass
            return None
        
        ports = [80,443,22,21,3306,5432,6379,27017,8080,8443,9090,10000,20000]
        paths = ["/admin","/login","/api","/graphql","/.env","/robots.txt","/admin/login","/api/stores"]
        apis = ["/api/stores","/api/users","/api/products","/api/orders","/nuxt-api/domains"]
        
        with ThreadPoolExecutor(max_workers=30) as ex:
            # كل شي دفعة واحدة
            futures = []
            futures += [ex.submit(check_port, p) for p in ports]
            futures += [ex.submit(check_path, p) for p in paths]
            futures += [ex.submit(check_api, a) for a in apis]
            
            for f in as_completed(futures):
                try: f.result()
                except: pass
    
    def fast_http(self):
        """HTTP سريع"""
        try:
            r = self.session.get(self.target, timeout=5)
            text = r.text[:100000]  # أول 100KB فقط
            
            # استخراج سريع
            self.results["apis"] = list(set(re.findall(r'["\'](/[^"\']*(?:api|graphql|rest|nuxt-api)[^"\']*)["\']', text)))[:10]
            self.results["keys"] = list(set(re.findall(r'(?:key|token|secret)\s*[:=]\s*["\']([^"\']{8,})["\']', text, re.IGNORECASE)))[:10]
            self.results["emails"] = list(set(re.findall(r'[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}', text)))[:10]
            
            # SSTI
            if "49" in text:
                self.results["vulns"].append("SSTI")
            
            # API Stores
            try:
                r2 = self.session.get(f"{self.target}/api/stores", timeout=3)
                if r2.status_code == 200:
                    data = r2.json()
                    stores = data.get('data', data) if isinstance(data, dict) else data
                    if isinstance(stores, list):
                        self.results["api_stores"] = len(stores)
            except: pass
            
        except: pass
    
    def generate_report(self):
        """تقرير سريع"""
        r = self.results
        elapsed = time.time() - self.start_time
        
        report = f"""
╔══════════════════════════════════╗
║   🚀 Zero-Day Hunter v7         ║
║   Bayan Security Platform       ║
╚══════════════════════════════════╝

🎯 {r['target']}
📡 IP: {r['dns'].get('ip','?')}
⏱️ الوقت: {elapsed:.1f}s

═══════════════════════════════════
🔌 Ports: {len(r['ports'])}
🔗 APIs: {len(r['apis'])}
🔑 Keys: {len(r['keys'])}
📧 Emails: {len(r['emails'])}
🚨 Vulns: {len(r['vulns'])}
📊 Score: {r['score']}/100
═══════════════════════════════════
"""
        print(report)
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v7_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump(r, f, indent=2)
        self.log(f"📁 {fname}")
    
    def run(self):
        self.log(f"🚀 v7: {self.target}")
        
        # DNS
        try:
            self.results["dns"]["ip"] = socket.gethostbyname(self.domain)
        except: pass
        
        # Parallel
        self.parallel_scan()
        
        # HTTP
        self.fast_http()
        
        # Score
        score = 0
        score += len(self.results["ports"]) * 2
        score += len(self.results["apis"]) * 5
        score += len(self.results["keys"]) * 2
        score += len(self.results["emails"]) * 5
        score += len(self.results["vulns"]) * 20
        score += self.results.get("api_stores",0) * 3
        self.results["score"] = min(score, 100)
        
        self.generate_report()
        return self.results

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV7(target).run()
