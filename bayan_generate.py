import sys

# الجذور الأساسية
BASE_ROOTS = ["كتب", "قرأ", "حسب", "خزن", "بعث", "جمع", "فصل", "رسم", "علم", "حفظ"]

# أنماط الاشتقاق
PATTERNS = {
    "فاعل": lambda r: r[0] + "ا" + r[1] + r[2],      # كاتب، قارئ
    "مفعول": lambda r: "م" + r[0] + r[1] + "و" + r[2], # مكتوب، مقروء
    "فعل": lambda r: r[0] + "َ" + r[1] + "َ" + r[2],   # كَتَبَ
    "يفعل": lambda r: "ي" + r[0] + r[1] + r[2],        # يكتب
    "استفعل": lambda r: "است" + r[0] + r[1] + r[2],    # استكتب
    "افتعل": lambda r: "ا" + r[0] + "ت" + r[1] + r[2], # اكتتب
    "فعال": lambda r: r[0] + r[1] + "ا" + r[2],        # كتاب
    "مفعل": lambda r: "م" + r[0] + r[1] + r[2],        # مكتب
}

def generate(base_root):
    """توليد كل الاشتقاقات من جذر واحد"""
    derived = []
    for name, pattern in PATTERNS.items():
        try:
            word = pattern(base_root)
            derived.append(f"{word}:{name}")
        except:
            pass
    return derived

# جرب
if len(sys.argv) > 1:
    root = sys.argv[1]
    results = generate(root)
    for r in results:
        print(f"✅ {r}")
    print(f"📊 {len(results)} اشتقاق من جذر '{root}'")
else:
    # توليد من كل الجذور الأساسية
    total = 0
    for root in BASE_ROOTS:
        results = generate(root)
        total += len(results)
        for r in results:
            print(f"✅ {r}")
    print(f"\n🔥 {total} اشتقاق من {len(BASE_ROOTS)} جذر أساسي")
