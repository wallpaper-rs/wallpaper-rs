#!/usr/bin/env bash
set -euo pipefail
declare scene
folder=$scene/applied
ctl scene new "$folder" --id org.example.applied --title Applied >/dev/null
ctl scene set "$folder" /scene/canvas/background '#0000ff'
ctl scene set "$folder" /manifest/version 1.0.0
read -r _ _ _ s20 < <(ctl stream TEST-20 160 90)
applied=$(ctl scene apply "$folder" -o TEST-20 2>"$WORK/apply.err") || fail "the folder was not applied: $(cat "$WORK/apply.err")"
case $applied in
  "applied: Applied 1.0.0 on TEST-20"*) ;;
  *) fail "scene apply said: $applied" ;;
esac
grep -q "Building the package" "$WORK/apply.err" || fail "scene apply did not say what it was doing: $(cat "$WORK/apply.err")"
shown "$s20" 0 0 255
ctl scene set "$folder" /scene/canvas/background '#ff0000'
ctl scene apply "$folder" -o TEST-20 >/dev/null 2>&1 || fail "the changed folder was not applied again"
shown "$s20" 255 0 0
printf '%s\n' 'export function init() { console.log("hello from the folder"); }' >"$WORK/folder-hello.js"
ctl scene import-script "$folder" "$WORK/folder-hello.js" >/dev/null || fail "the script was not imported"
frames() {
  (while :; do "$BIN/probe" "$s20" 1 >/dev/null 2>&1 || sleep 0.1; done) &
  local watcher=$!
  timeout 8 "$BIN/wprs-ctl" scene preview "$folder" -o TEST-20 --console >"$WORK/folder.out" 2>&1 || true
  kill "$watcher" 2>/dev/null || true
  wait "$watcher" 2>/dev/null || true
  grep -q "$1" "$WORK/folder.out"
}
frames "log scripts/folder-hello.js: hello from the folder" || fail "the console of a preview on a screen printed nothing: $(cat "$WORK/folder.out")"
grep -q "Previewing Applied on TEST-20" "$WORK/folder.out" || fail "the preview did not say where it was shown: $(cat "$WORK/folder.out")"
expect "s['outputs']['TEST-20']['drafted']" "True"
ctl scene preview "$folder" -o TEST-20 --stop 2>"$WORK/stop.err" || fail "the preview did not stop"
grep -q "Ended the preview on TEST-20" "$WORK/stop.err" || fail "stopping the preview said nothing: $(cat "$WORK/stop.err")"
expect "s['outputs']['TEST-20']['drafted']" "False"
shown "$s20" 255 0 0
printf '%s\n' 'import { scene } from "wprs";' '' 'export function update() {' '  scene.layer("nothing").opacity = 1;' '}' >"$WORK/folder-bad.js"
ctl scene import-script "$folder" "$WORK/folder-bad.js" >/dev/null || fail "the failing script was not imported"
frames "scripts/folder-bad.js line 4" || fail "a script error did not name its line: $(cat "$WORK/folder.out")"
ctl scene render "$folder" -o "$WORK/folder-render.png" --size 64 --time-of-day 15:42 >/dev/null 2>"$WORK/render.err" || fail "the folder was not rendered: $(cat "$WORK/render.err")"
grep -q "script" "$WORK/render.err" || fail "a render did not run or report the scripts: $(cat "$WORK/render.err")"
echo "folder checks passed"
