#!/usr/bin/env bash
set -euo pipefail
expect "s['battery_percent'] is None or 0 <= s['battery_percent'] <= 100" True
ctl config pause.below_percent 20
expect "s['options']['pause.below_percent']" 20
ctl config playback.lowest_below_percent 35
if ctl config pause.below_percent 101 2>/dev/null; then fail "a battery level over 100 was accepted"; fi
ctl config pause.below_percent 0 && ctl config playback.lowest_below_percent 0
ctl stream TEST-30 160 90 >/dev/null
ctl sound off -o TEST-30
expect "s['outputs']['TEST-30']['sound_muted']" True
ctl sound on --volume 30 -o TEST-30
expect "s['outputs']['TEST-30']['sound_muted'], s['outputs']['TEST-30']['sound_volume']" "(False, 0.3)"
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/hearing.png"
ctl lib import "$WORK/hearing.png" --wait >/dev/null
ctl lib apply "$WORK/hearing.png" -o TEST-30 --sound off
expect "s['outputs']['TEST-30']['sound_muted']" True
ctl sound follow -o TEST-30
expect "s['outputs']['TEST-30']['sound_muted'], s['outputs']['TEST-30']['sound_volume']" "(None, None)"
if ctl config playback.volume.TEST-30 1.5 2>/dev/null; then fail "a volume over 1 was accepted for a screen"; fi
ctl release TEST-30 && expect "list(s['outputs'])" "[]"
echo "battery level and sound of a screen checks passed"
