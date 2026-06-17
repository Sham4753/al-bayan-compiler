#!/bin/bash
if [ $# -lt 2 ]; then
    echo "استخدام: ./tools/add_root.sh <الأمر> <النتيجة>"
    echo "مثال: ./tools/add_root.sh bayan.math.add 'جَبَرَ: جبر'"
    exit 1
fi

CMD=$1
RESULT=$2
FILE="src/runtime_entries.rs"

if grep -q "$CMD" "$FILE"; then
    echo "⏭️ $CMD موجود مسبقاً"
    exit 0
fi

sed -i "/\/\/ -- END_MARKER --/i \"$CMD\" => Ok(Value::Text(\"$RESULT\".to_string()))," "$FILE"
echo "✅ تمت إضافة $CMD"
echo "🔨 جرب: cargo build --lib"
