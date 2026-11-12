#!/usr/bin/env bash
set -euo pipefail
declare WORK
full=$1 desktop=$2
expect "s['outputs']['screen']['released']" True
check "$full" "0.5,0.5,$desktop" "0.02,0.98,$desktop"
if grep -q "desktop window created" "$WORK/wprsd.log"; then fail "a fresh start took the screen over"; fi
echo "first start checks passed"
