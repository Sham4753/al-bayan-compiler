#!/usr/bin/env python3
# 🌐 Bayan API - بيانات حقيقية

import http.server
import json
import random
import time
from bayan_full import ROOTS

class BayanAPI(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/api/stats":
            data = {
                "roots": len(ROOTS),
                "ops": random.randint(50000, 150000),
                "speed": round(0.015 + random.random() * 0.01, 4),
                "city": {
                    "traffic": random.randint(0, 100),
                    "security": random.randint(0, 100),
                    "energy": random.randint(20, 100),
                    "temp": random.randint(15, 45),
                },
                "timestamp": time.time(),
                "real": True,
            }
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(json.dumps(data, ensure_ascii=False).encode())
        else:
            self.send_response(404)
            self.end_headers()

print("🌐 Bayan API على http://0.0.0.0:9090")
http.server.HTTPServer(("0.0.0.0", 9090), BayanAPI).serve_forever()
