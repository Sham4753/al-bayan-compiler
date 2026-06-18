#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}╔══════════════════════════════════╗${NC}"
echo -e "${GREEN}║   🕌 Bayan REPL - مُحاكي تفاعلي   ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════╝${NC}"
echo -e "${CYAN}اكتب أي جذر عربي. خروج للخروج.${NC}"
echo ""

COUNT=0
while true; do
    echo -en "${YELLOW}بيان>${NC} "
    read -r cmd
    
    if [ "$cmd" = "خروج" ]; then
        echo -e "${GREEN}👋 نفذت $COUNT عملية${NC}"
        break
    fi
    
    if [ "$cmd" = "مساعدة" ]; then
        echo -e "${CYAN}جرب: احتسب | حفظ | بعث | انبعث | خزن | جمع | رسم | قرأ | كتب${NC}"
        continue
    fi
    
    echo "$cmd" > /tmp/repl.bayan
    RESULT=$(cargo run --bin bayan -- شغّل /tmp/repl.bayan 2>&1 | grep "↳" | head -1 | cut -d'"' -f2)
    
    if [ -n "$RESULT" ]; then
        echo -e "${GREEN}✅ $RESULT${NC}"
        COUNT=$((COUNT+1))
    else
        echo -e "${RED}❌ لم يفهم${NC}"
    fi
done
