#!/usr/bin/env bash
set -euo pipefail
expect "s['battery_percent'] is None or 0 <= s['battery_percent'] <= 100" True
ctl config pause.on_battery below:20
expect "s['options']['pause.on_battery']" below:20
ctl config playback.lowest_below_percent 35
if ctl config pause.on_battery below:101 2>/dev/null; then fail "a battery level over 100 was accepted"; fi
ctl config pause.on_battery always && ctl config playback.lowest_below_percent 0
ctl stream TEST-30 160 90 >/dev/null
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/hearing.png"
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/hearing2.png"
ctl lib import "$WORK/hearing.png" "$WORK/hearing2.png" --wait >/dev/null
sound() { ctl lib search "hearing" --json | python3 -c "import json, sys; r = json.load(sys.stdin); print(sorted((i['title'], i.get('sound_muted'), i.get('sound_volume')) for i in r['items']))"; }
ctl lib apply "$WORK/hearing.png" -o TEST-30 --sound off --volume 30
[ "$(sound)" = "[('hearing', True, 30), ('hearing2', None, None)]" ] || fail "the sound of one wallpaper: $(sound)"
ctl sound on -o TEST-30
ctl sound --item "$WORK/hearing2.png" off
[ "$(sound)" = "[('hearing', False, 30), ('hearing2', True, None)]" ] || fail "the sound per wallpaper: $(sound)"
ctl sound follow -o TEST-30
ctl sound follow --item "$WORK/hearing2.png"
[ "$(sound)" = "[('hearing', None, None), ('hearing2', None, None)]" ] || fail "the sound back to follow: $(sound)"
if ctl sound --volume 101 2>/dev/null; then fail "a volume over 100 was accepted"; fi
ctl release TEST-30 && expect "list(s['outputs'])" "[]"
ctl lib rm "$WORK/hearing.png" "$WORK/hearing2.png"
echo "battery level and sound of a wallpaper checks passed"
