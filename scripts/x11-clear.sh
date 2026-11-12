#!/usr/bin/env bash
set -euo pipefail
full=$1 desktop=$2 green=$3
windows() { xdotool search --class '^wprsd$' 2>/dev/null | wc -l; }
ctl clear
expect "s['outputs']['screen']['released']" True
wait_for 10 test "$(windows)" = 0 || fail "the wallpaper window stayed after clearing"
check "$full" "0.5,0.5,$desktop" "0.02,0.98,$desktop"
[ -z "$(pixmap _XROOTPMAP_ID)" ] || fail "the root pixmap of the daemon stayed after clearing"
ctl restore
expect "s['outputs']['screen']['released']" False
check "$full" "0.5,0.5,$green"
wait_for 10 rooted || fail "no root pixmap after restoring"
echo "clear and restore checks passed"
