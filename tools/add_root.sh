#!/bin/bash
if [ $# -lt 2 ]; then
    echo "استخدام: ./tools/add_root.sh <intrinsic> <الوصف>"
    exit 1
fi

CMD=$1
DESC=$2
FILE="src/runtime.rs"

if grep -q "$CMD" "$FILE"; then
    echo "⏭️ موجود"
    exit 0
fi

LINE=$(grep -n "intrinsic غير معروف" "$FILE" | head -1 | cut -d: -f1)
sed -i "${LINE}i\\            \"$CMD\" => Ok(Value::Text(\"$DESC\".to_string()))," "$FILE"
echo "✅ تم: $CMD"
