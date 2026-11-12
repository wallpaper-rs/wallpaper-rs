#!/usr/bin/env bash
set -euo pipefail
declare ROOT green desktop
both() { for output in HEADLESS-1 HEADLESS-2; do "$@" "$output"; done; }
shown() { expect "s['outputs']['$1']['released']" False && check "$1" "0.5,0.5,$green"; }
swaymsg -q reload
ctl set "$ROOT/tests/fixtures/green-64x32.png" --fit fit
both shown
swaymsg -q reload
sleep 1
both shown
swaymsg -q output HEADLESS-1 disable && swaymsg -q output HEADLESS-1 enable
swaymsg -q reload
sleep 1
both shown
ctl clear
for output in HEADLESS-1 HEADLESS-2; do expect "s['outputs']['$output']['released']" True; done
check HEADLESS-1 "0.5,0.5,$desktop" "0.9,0.9,$desktop"
expect "s['outputs']['HEADLESS-1']['previous']" True
wait_for 10 settled || fail "the daemon did not settle after clearing"
before=$(ticks); sleep 2
[ $(($(ticks) - before)) -le 10 ] || fail "the daemon kept working with no wallpaper"
ctl restore
both shown
ctl clear -o HEADLESS-2
expect "s['outputs']['HEADLESS-2']['released']" True
expect "s['outputs']['HEADLESS-1']['released']" False
check HEADLESS-1 "0.5,0.5,$green"
ctl set "$ROOT/tests/fixtures/green-64x32.png" --fit fit
shown HEADLESS-2
echo "clear and restore checks passed"
