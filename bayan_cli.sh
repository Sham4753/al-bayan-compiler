#!/bin/bash
# ============================================
# أداة سطر أوامر البيان - Bayan CLI
# ============================================

GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

help() {
    echo -e "${CYAN}🕌 لغة البيان${NC}"
    echo "  bayan run <أمر>     - تنفيذ أمر عربي"
    echo "  bayan file <ملف>    - تنفيذ ملف بيان"
    echo "  bayan help          - هذه المساعدة"
}

case "$1" in
    "run")
        if [ -z "$2" ]; then
            echo -e "${RED}❌ استخدم: bayan run <أمر>${NC}"
            exit 1
        fi
        echo "$2" > /tmp/bayan_cmd.bayan
        RESULT=$(cargo run --bin bayan -- شغّل /tmp/bayan_cmd.bayan 2>&1 | grep "↳" | head -1 | cut -d'"' -f2)
        echo -e "${GREEN}✅ $RESULT${NC}"
        ;;
    "file")
        if [ -z "$2" ]; then
            echo -e "${RED}❌ استخدم: bayan file <ملف.بيان>${NC}"
            exit 1
        fi
        cargo run --bin bayan -- شغّل "$2"
        ;;
    *)
        help
        ;;
esac
