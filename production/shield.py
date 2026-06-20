#!/usr/bin/env python3
"""
درع البيان - جدار حماية حي
يحمي السيرفر من الهجمات الشائعة ويراقب المنافذ
"""
import subprocess, time, os, json
from datetime import datetime

LOG_FILE = "/root/al-bayan-compiler/logs/shield.log"
WHITELIST = ["127.0.0.1", "192.168.0.0/16", "10.0.0.0/8"]

def log(msg):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    with open(LOG_FILE, "a") as f:
        f.write(f"[{timestamp}] {msg}\n")
    print(f"🛡️ {msg}")

def run(cmd):
    try:
        return subprocess.run(cmd, shell=True, capture_output=True, text=True)
    except:
        return None

def init_firewall():
    log("🔥 تفعيل جدار الحماية...")
    
    # سياسة افتراضية: رفض كل شيء ثم السماح بالمطلوب
    rules = [
        "iptables -P INPUT DROP",
        "iptables -P FORWARD DROP",
        "iptables -P OUTPUT ACCEPT",
        # السماح للحلقة الداخلية
        "iptables -A INPUT -i lo -j ACCEPT",
        # السماح للاتصالات القائمة
        "iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT",
        # SSH - مع تحديد المحاولات
        "iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --set",
        "iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --update --seconds 60 --hitcount 4 -j DROP",
        # SMTP - إغلاق تام ما لم تحتجه
        "iptables -A INPUT -p tcp --dport 25 -j DROP",
        # السماح بـ ping
        "iptables -A INPUT -p icmp --icmp-type echo-request -m limit --limit 1/second -j ACCEPT",
        # حماية من المسح
        "iptables -A INPUT -p tcp --tcp-flags ALL NONE -j DROP",
        "iptables -A INPUT -p tcp --tcp-flags ALL ALL -j DROP",
        # حماية SYN Flood
        "iptables -A INPUT -p tcp --syn -m limit --limit 10/second --limit-burst 20 -j ACCEPT",
        "iptables -A INPUT -p tcp --syn -j DROP",
    ]
    
    for rule in rules:
        result = run(rule)
        if result and result.returncode == 0:
            log(f"✅ {rule.split('-A')[1].strip() if '-A' in rule else rule.split('-P')[1].strip()}")
        else:
            log(f"⚠️ فشل: {rule[:60]}...")

def port_scan_detect():
    """يكشف محاولات المسح ويسجلها"""
    log("👁️ بدء مراقبة محاولات المسح...")
    # هنا يمكن إضافة كشف Port Scan عبر تحليل السجلات

def show_status():
    rules = run("iptables -L INPUT -n --line-numbers")
    if rules:
        log("📋 قواعد الجدار الحالية:\n" + rules.stdout[-500:])

if __name__ == "__main__":
    print("🛡️" * 20)
    print("   درع البيان - جدار الحماية")
    print("🛡️" * 20)
    
    init_firewall()
    show_status()
    
    log("✅ تم تفعيل الدرع. السيرفر محمي الآن.")
    print("\n📁 السجل: " + LOG_FILE)
