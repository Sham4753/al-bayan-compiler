#!/usr/bin/env python3
import sys, socket, requests, concurrent.futures, subprocess, os
from datetime import datetime

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
LOG_DIR = os.path.join(BASE_DIR, "logs")
os.makedirs(LOG_DIR, exist_ok=True)

sys.path.insert(0, BASE_DIR)
from bayan_full import ROOTS as ALL_ROOTS

TIMESTAMP = datetime.now().strftime("%Y%m%d_%H%M%S")
target_raw = sys.argv[1] if len(sys.argv) > 1 else "scanme.nmap.org"
LOG_FILE = os.path.join(LOG_DIR, f"scan_{target_raw.replace('.','_')}_{TIMESTAMP}.log")

def write_log(msg):
    with open(LOG_FILE, "a", encoding="utf-8") as f:
        f.write(msg + "\n")
    print(msg)

target = target_raw
code = sys.stdin.read().strip()
roots = [r.strip() for r in code.split('\n') if r.strip()]

def ip(t):
    try: return socket.gethostbyname(t)
    except: return t

def scan_1000():
    i = ip(target)
    ports = list(range(1, 1001))
    o = []
    def c(p):
        try:
            s=socket.socket();s.settimeout(0.1)
            if s.connect_ex((i,p))==0:o.append(p)
            s.close()
        except:pass
    with concurrent.futures.ThreadPoolExecutor(100)as e:e.map(c,ports)
    if not o: return "🔒 محمي - 0 منافذ مفتوحة"
    vulns=[]
    danger_ports = {21:"FTP",22:"SSH",23:"Telnet",25:"SMTP",53:"DNS",
                    80:"HTTP",110:"POP3",143:"IMAP",443:"HTTPS",
                    3306:"MySQL",3389:"RDP",5432:"PostgreSQL",
                    6379:"Redis",8080:"HTTP-Alt",8443:"HTTPS-Alt",
                    27017:"MongoDB",9200:"Elasticsearch"}
    for p in o:
        if p in danger_ports:
            vulns.append(f"{danger_ports[p]}({p})")
    return f"🔍 {len(o)} منفذ مفتوح\n🚨 {', '.join(vulns) if vulns else 'لا خدمات خطرة ظاهرة'}"

def fingerprint():
    try:
        r=requests.get(f"http://{target}",timeout=5,headers={'User-Agent':'Mozilla/5.0'})
        s=r.headers.get('Server','غير معروف')
        return f"🖥️ السيرفر: {s} | رمز الاستجابة: {r.status_code}"
    except Exception as e:
        return f"⚠️ تعذر الاتصال: {str(e)[:80]}"

def nmap_full():
    try:
        r=subprocess.run(['nmap','-sV','-p','22,80,443,3306,8080',ip(target)],
                        capture_output=True,text=True,timeout=30)
        return r.stdout.strip() if r.stdout else "✅ لم يُكتشف شيء"
    except Exception as e:
        return f"⚠️ Nmap فشل: {str(e)[:80]}"

def exploit_all():
    t=[]
    sql_errors = ['sql syntax','mysql_fetch','unclosed quotation mark',
                  'you have an error in your sql','microsoft ole db',
                  'odbc driver','postgresql','sqlite']
    try:
        r=requests.get(f"http://{target}/?id='",timeout=5)
        if any(err in r.text.lower() for err in sql_errors):
            t.append("SQLi(محتمل)")
    except:pass
    try:
        payload = "<script>alert('XSS')</script>"
        r=requests.get(f"http://{target}/?q={payload}",timeout=5)
        if payload in r.text:
            t.append("XSS(منعكس)")
    except:pass
    try:
        r=requests.get(f"http://{target}/?file=../../etc/passwd",timeout=5)
        if 'root:' in r.text:
            t.append("PathTraversal")
    except:pass
    return f"💀 {', '.join(t)}" if t else "✅ لا ثغرات ظاهرة"

A=scan_1000; B=fingerprint; C=nmap_full; D=exploit_all

ACTIONS = {}
for root, meaning in ALL_ROOTS.items():
    m = meaning
    if any(w in m for w in ['فحص','مسح','كشف','بحث','استطلاع','فتح','اتصال','مستمع']):
        ACTIONS[root]=A
    elif any(w in m for w in ['اختراق','هجوم','استغلال','ثغرة','خرق']):
        ACTIONS[root]=D
    elif any(w in m for w in ['تحليل','ذكاء','حلل','Nmap']):
        ACTIONS[root]=C
    else:
        ACTIONS[root]=B

write_log(f"══════ البيان - تقرير فحص ══════")
write_log(f"🎯 الهدف: {target}")
write_log(f"📅 التاريخ: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
write_log(f"📜 عدد الجذور المُستدعاة: {len(roots)}")
write_log(f"📁 ملف السجل: {LOG_FILE}")
write_log(f"══════════════════════════════════\n")

for root in roots:
    fn = ACTIONS.get(root, B)
    result = fn()
    write_log(f"⚡ الجذر: {root}")
    write_log(f"{result}")
    write_log("─" * 40)

write_log(f"\n🧠 {len(ACTIONS)} فعل مُتاح | 📁 النتائج محفوظة في: {LOG_FILE}")
write_log("✨ الكود قرآن ✨")
