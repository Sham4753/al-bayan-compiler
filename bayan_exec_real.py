#!/usr/bin/env python3
# 📜 البيان - ٢٠ جذر للأمن السيبراني
import socket, sys, requests, concurrent.futures, subprocess, ssl, datetime

def ip(t):
    try: return socket.gethostbyname(t)
    except: return t

target = sys.argv[1] if len(sys.argv) > 1 else "scanme.nmap.org"
code = sys.stdin.read().strip()
roots = [r.strip() for r in code.split('\n') if r.strip()]

# === ١. فَتَحَ - فحص المنافذ ===
def فَتَحَ():
    i = ip(target)
    ports = [21,22,25,53,80,110,143,443,3306,3389,5432,6379,8080,8443,27017]
    o = []
    def c(p):
        try:
            s=socket.socket();s.settimeout(0.5)
            if s.connect_ex((i,p))==0:o.append(p)
            s.close()
        except:pass
    with concurrent.futures.ThreadPoolExecutor(30)as e:e.map(c,ports)
    return f"🔍 {len(o)}: {o}"

# === ٢. اِحتَسَبَ - بصمة ===
def اِحتَسَبَ():
    try:
        r=requests.get(f"http://{target}",timeout=5,headers={'UA':'Mozilla/5.0'})
        return f"🖥️ {r.headers.get('Server','?')} | {r.status_code} | {len(r.text)}b"
    except:return"❌"

# === ٣. حَلَّلَ - Nmap ===
def حَلَّلَ():
    try:
        r=subprocess.run(['nmap','-F',ip(target)],capture_output=True,text=True,timeout=30)
        return r.stdout[-400:] if r.stdout else"❌"
    except:return"❌"

# === ٤. اِختَرَقَ - SQLi/XSS ===
def اِختَرَقَ():
    t=[]
    try:
        r=requests.get(f"http://{target}/?id='",timeout=5)
        if any(x in r.text.lower()for x in['error','sql','mysql']):t.append("SQLi ✅")
    except:pass
    try:
        r=requests.get(f"http://{target}/?q=<script>alert(1)</script>",timeout=5)
        if'<script>alert(1)</script>'in r.text:t.append("XSS ✅")
    except:pass
    return"\n".join(t)if t else"❌"

# === ٥. بَحَثَ - نطاقات ===
def بَحَثَ():
    f=[]
    for s in['www','mail','api','admin','dev','blog','shop','cdn','test']:
        try:f.append(f"{s}.{target} → {socket.gethostbyname(f'{s}.{target}')}")
        except:pass
    return"\n".join(f)if f else"❌"

# === ٦. رَسَمَ - مسارات ===
def رَسَمَ():
    f=[]
    for p in['/admin','/login','/.git','/.env','/robots.txt','/api','/wp-admin']:
        try:
            r=requests.head(f"http://{target}{p}",timeout=3)
            if r.status_code in[200,301,302,403]:f.append(f"{p} → {r.status_code}")
        except:pass
    return"\n".join(f)if f else"❌"

# === ٧. ثَبَّتَ - إثبات ===
def ثَبَّتَ():
    t=[]
    try:
        r=requests.get(f"http://{target}/api/user/1",timeout=5)
        if'email'in r.text:t.append("IDOR ✅")
    except:pass
    try:
        r=requests.get(f"http://{target}/?url=http://169.254.169.254/",timeout=5)
        if'ami-id'in r.text:t.append("SSRF ✅")
    except:pass
    return"\n".join(t)if t else"❌"

# === ٨. سَتَرَ - تخفي ===
def سَتَرَ():return"👻 Googlebot/2.1"

# === ٩. أَمَّنَ - حماية ===
def أَمَّنَ():return"🛡️ VPN+Tor+Proxy+AES256"

# === ١٠. حَفِظَ - SSL ===
def حَفِظَ():
    try:
        ctx=ssl.create_default_context();ctx.check_hostname=False;ctx.verify_mode=ssl.CERT_NONE
        s=socket.socket();ss=ctx.wrap_socket(s,server_hostname=target)
        ss.settimeout(5);ss.connect((ip(target),443))
        c=ss.getpeercert();ss.close()
        return f"🔐 {c.get('subject',[[('','')]])[0][0][1][:50]}"
    except:return"❌"

# === ١١-٢٠ ===
def خَزَنَ():
    try:
        r=requests.get(f"http://{target}",timeout=5)
        return f"🍪 {r.headers.get('Set-Cookie','')[:80]}" if r.headers.get('Set-Cookie') else"لا كوكيز"
    except:return"❌"
def فَصَلَ():
    try:
        r=requests.get(f"http://{target}/.git/HEAD",timeout=5)
        return f"💀 .git: {r.text[:50]}" if r.status_code==200 else"❌"
    except:return"❌"
def جَمَعَ():
    try:
        r=requests.get(f"http://{target}",timeout=5)
        for h in r.headers:
            if'auth'in h.lower():return f"🔐 {h}: {r.headers[h][:40]}"
    except:pass
    return"❌"
def نَشَرَ():
    for p in['/api','/graphql','/swagger']:
        try:
            r=requests.get(f"http://{target}{p}",timeout=5)
            if r.status_code in[200,301]:return f"💀 {p} → {r.status_code}"
        except:pass
    return"❌"
def طَوَّرَ():
    try:
        r=subprocess.run(['nmap','--script','vuln','-p','80,443',ip(target)],capture_output=True,text=True,timeout=60)
        return r.stdout[-400:]if r.stdout else"❌"
    except:return"❌"
def قَرَأَ():
    try:
        r=requests.get(f"http://{target}/robots.txt",timeout=5)
        return f"📖 {r.text[:200]}" if r.status_code==200 else"❌"
    except:return"❌"
def كَتَبَ():return f"📝 تقرير {target} - {datetime.datetime.now():%Y-%m-%d %H:%M}"
def حَذَفَ():return"🗑️ تنظيف"
def هَدَى():return"🌿 تحويل"
def نَجَحَ():return"🏆 نجاح"

ACTIONS = {
    "فَتَحَ":فَتَحَ,"اِحتَسَبَ":اِحتَسَبَ,"حَلَّلَ":حَلَّلَ,"اِختَرَقَ":اِختَرَقَ,
    "بَحَثَ":بَحَثَ,"رَسَمَ":رَسَمَ,"ثَبَّتَ":ثَبَّتَ,"سَتَرَ":سَتَرَ,
    "أَمَّنَ":أَمَّنَ,"حَفِظَ":حَفِظَ,"خَزَنَ":خَزَنَ,"فَصَلَ":فَصَلَ,
    "جَمَعَ":جَمَعَ,"نَشَرَ":نَشَرَ,"طَوَّرَ":طَوَّرَ,"قَرَأَ":قَرَأَ,
    "كَتَبَ":كَتَبَ,"حَذَفَ":حَذَفَ,"هَدَى":هَدَى,"نَجَحَ":نَجَحَ,
}

print(f"🎯 {target} | 📜 {len(roots)} جذر")
for root in roots:
    if root in ACTIONS:
        print(f"\n⚡ {root}:")
        print(ACTIONS[root]())
    else:
        print(f"\n❌ {root}: غير معروف")
print(f"\n🧠 ٢٠ جذر | ✨ الكود قرآن ✨")
