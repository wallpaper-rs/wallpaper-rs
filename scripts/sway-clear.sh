#!/usr/bin/env bash
set -euo pipefail
declare WORK ROOT green desktop
destroyed() { grep -c "Layer surface destroyed" "$WORK/sway.log" || true; }
more() { [ "$(destroyed)" -ge "$1" ]; }
ctl set "$ROOT/tests/fixtures/green-64x32.png" --fit fit
for output in HEADLESS-1 HEADLESS-2; do
  expect "s['outputs']['$output']['released']" False
  check "$output" "0.5,0.5,$green"
done
created=$(grep -c "layer surface created" "$WORK/wprsd.log")
gone=$(destroyed)
ctl clear
for output in HEADLESS-1 HEADLESS-2; do expect "s['outputs']['$output']['released']" True; done
wait_for 10 more $((gone + 2)) || fail "the layer surfaces were not destroyed"
for output in HEADLESS-1 HEADLESS-2; do check "$output" "0.5,0.5,$desktop" "0.9,0.9,$desktop"; done
expect "s['outputs']['HEADLESS-1']['previous']" True
wait_for 10 settled || fail "the daemon did not settle after clearing"
before=$(ticks); sleep 2
[ $(($(ticks) - before)) -le 10 ] || fail "the daemon kept working with no wallpaper"
ctl restore
for output in HEADLESS-1 HEADLESS-2; do
  expect "s['outputs']['$output']['released']" False
  check "$output" "0.5,0.5,$green"
done
[ "$(grep -c "layer surface created" "$WORK/wprsd.log")" -ge $((created + 2)) ] || fail "the surfaces were not created again"
gone=$(destroyed)
ctl clear -o HEADLESS-2
expect "s['outputs']['HEADLESS-2']['released']" True
expect "s['outputs']['HEADLESS-1']['released']" False
wait_for 10 more $((gone + 1)) || fail "the surface of one screen was not destroyed ($gone, $(destroyed)): $(grep -a "Layer surface" "$WORK/sway.log" | tail -n 6)"
check HEADLESS-1 "0.5,0.5,$green"
ctl set "$ROOT/tests/fixtures/green-64x32.png" --fit fit
expect "s['outputs']['HEADLESS-2']['released']" False
check HEADLESS-2 "0.5,0.5,$green"
echo "clear and restore checks passed"
