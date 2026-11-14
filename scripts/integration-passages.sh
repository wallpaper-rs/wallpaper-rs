#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT crossing s31
green=$ROOT/tests/fixtures/green-64x32.png
passage "$crossing/leaves.wpak" out "1.0, 0.0, 0.0" 6000
passage "$crossing/comes.wpak" in "1.0, 0.0, 1.0" 6000
leaves=$(imported "$crossing/leaves.wpak")
comes=$(imported "$crossing/comes.wpak")
ctl set "$green" -o TEST-31 --fit stretch
shown "$s31" 0 255 0
ctl config transition.less_motion always
ctl lib apply "$comes" -o TEST-31
expect "s['outputs']['TEST-31']['transition']['why']" reduced
expect "s['outputs']['TEST-31']['transition']['plays']" fade
shown "$s31" 0 0 255
looks "$s31" 255 0 255 && fail "the transition played with less motion"
ctl config transition.kind.TEST-31 none
ctl config transition.kind.TEST-31 | grep -qx "transition.kind.TEST-31 = none" || fail "the transition of one screen is not read back"
ctl lib apply "$leaves" -o TEST-31
expect "s['outputs']['TEST-31']['transition']['plays']" none
ctl config transition.kind.TEST-31 general
ctl config transition.less_motion never
ctl lib apply "$comes" -o TEST-31
shown "$s31" 255 0 0
shown "$s31" 255 0 255
expect "s['outputs']['TEST-31']['transition']['plays']" own
wait_for 20 looks "$s31" 0 0 255 || fail "the scene that comes in did not settle after both passages: $("$BIN/probe" "$s31" 3)"
ctl config transition.less_motion system
ctl release TEST-31
bus Library1 Remove asb 1 "$leaves" false >/dev/null
bus Library1 Remove asb 1 "$comes" false >/dev/null
echo "less motion gives the plain fade, and a wallpaper that leaves with a transition gives way to one that comes in with its own"
