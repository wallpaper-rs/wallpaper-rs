#!/usr/bin/env bash
set -euo pipefail
declare scene ROOT
craft=$scene/craft
ctl scene new "$craft" --id org.example.craft >/dev/null
ctl scene add-layer "$craft" solid --id card >/dev/null
ctl scene add-layer "$craft" solid --id hole >/dev/null
ctl scene effects "$craft" card add vignette --param intensity=0.6 --name edge || fail "no effect started the stack"
ctl scene effects "$craft" card add film_grain --at 0 || fail "no second effect was added"
ctl scene effects "$craft" card set 1 --param radius=1.5 --enabled true || fail "the effect was not set"
if ctl scene effects "$craft" card set 1 --param intensity=4 2>/dev/null; then fail "an intensity of 4 was kept"; fi
ctl scene effects "$craft" card mask 1 --from hole --channel luminance --invert true || fail "the effect was not masked"
ctl scene effects "$craft" card move 1 0 || fail "the effect was not moved"
ctl scene effects "$craft" card remove 1 || fail "the effect was not removed"
ctl scene add "$craft" /scene/layers/1/effects/- '{"effect": "vignette"}' || fail "the list of effects was not created"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with masked effects was not previewed"
ctl scene add-layer "$craft" particles --id dust >/dev/null
ctl scene particles "$craft" sparks create --parent dust --on static || fail "no child system was created"
ctl scene particles "$craft" dust settings --capacity 300 --seed 4 --prewarm 1 --space world --rate 30 || fail "the system was not set"
if ctl scene particles "$craft" dust settings --prewarm 900 2>/dev/null; then fail "a warm-up of 900 s was kept"; fi
ctl scene particles "$craft" dust start --life 1,3 --size 2,5 --color '#ffffff,#88ccff' --velocity -10,0:10,-40 ||
  fail "the starting ranges were not set"
ctl scene particles "$craft" dust start --clear velocity || fail "the velocity range was not cleared"
ctl scene particles "$craft" dust force gravity 0,40 || fail "no gravity was added"
ctl scene particles "$craft" dust force drag 0.2 --index 0 || fail "the force was not replaced"
ctl scene particles "$craft" dust force oscillate size --freq 1,2 --scale 0.5,1 || fail "no swing was added"
ctl scene particles "$craft" dust force attractor 40,30 --strength 30 --falloff linear || fail "no attractor was added"
ctl scene particles "$craft" dust force turbulence --strength 10 --scale 0.01 || fail "no turbulence was added"
ctl scene particles "$craft" dust force remove 1 || fail "the force was not removed"
ctl scene particles "$craft" dust force turbulence --strength -20 --scale 0.01 || fail "a turbulence that pulls back was refused"
words=$(ctl scene particles "$craft" dust settings --prewarm -5 2>&1) && fail "a negative warm-up was kept"
case $words in *unexpected*) fail "a negative warm-up was not checked by the editor: $words" ;; esac
if ctl scene particles "$craft" dust start --color '#fff,notacolor' 2>/dev/null; then fail "a wrong starting color was taken"; fi
if ctl scene canvas "$craft" bloom --hdr 1 2>/dev/null; then fail "a glare of 1 was taken"; fi
ctl scene particles "$craft" dust child add sparks --on death --probability 0.5 || fail "no child was added"
ctl scene particles "$craft" dust child set 1 sparks --on spawn --max 3 || fail "the child was not replaced"
ctl scene particles "$craft" dust child remove 0 || fail "the child was not removed"
ctl scene particles "$craft" dust size-over-life 0:1 0.5:2 1:0 || fail "the size over life was not set"
ctl scene particles "$craft" dust color-over-life '0:#ffffff' '1:#ff000000' || fail "the color over life was not set"
ctl scene particles "$craft" dust render --stretch 0.1,2,40 --frames random --fps 8 --mix true --brightness 2.5 ||
  fail "the look of the particles was not set"
if ctl scene particles "$craft" dust render --brightness 99 2>/dev/null; then fail "a brightness of 99 was kept"; fi
if ctl scene particles "$craft" sparks delete 2>/dev/null; then fail "a system in use was deleted"; fi
ctl scene particles "$craft" lone create || fail "no lone system was created"
ctl scene particles "$craft" lone delete || fail "a lone system was not deleted"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with every particle field was not previewed"
ctl scene layer "$craft" card transform --position 100,-20 --rotation -15 --scale 2,2 --size 300,200 || fail "the layer was not placed"
ctl scene layer "$craft" card blend screen || fail "the blend mode was not set"
if ctl scene layer "$craft" card transform --rotation 900 2>/dev/null; then fail "a turn of 900 degrees was kept"; fi
ctl scene layer "$craft" card transform --tilt 20,-10 --z 120 || fail "the layer was not tilted"
if ctl scene layer "$craft" card transform --tilt 95,0 2>/dev/null; then fail "a tilt of 95 degrees was kept"; fi
ctl scene keyframe "$craft" /scene/layers/0/transform/scale 0 '[1,1]' >/dev/null || fail "the scale took no keyframe"
if ctl scene keyframe "$craft" /scene/layers/0/transform/scale 1 1,1 2>"$WORK/unfit.err"; then fail "a scale keyframe of text was kept"; fi
grep -q 'give a list of 2 numbers such as \[1, 1\]' "$WORK/unfit.err" || fail "the scale keyframe was refused without its shape: $(cat "$WORK/unfit.err")"
if ctl scene keyframe "$craft" /scene/layers/0/transform/skew 1 1,1 2>"$WORK/unset.err"; then fail "a skew keyframe of text was kept"; fi
grep -q 'give a list of 2 numbers such as \[1, 1\]' "$WORK/unset.err" || fail "the unset skew was refused without its shape: $(cat "$WORK/unset.err")"
if ctl scene layer "$craft" card transform --scale 3,3 2>/dev/null; then fail "a plain scale dropped the keyframes"; fi
ctl scene layer "$craft" card transform --scale 2,2 --at 1 || fail "the keyed scale took no keyframe at a second"
python3 -c "import json, sys; k = json.load(open(sys.argv[1]))['layers'][0]['transform']['scale']['keys']; sys.exit(0 if [1, [2, 2]] in k else 1)" "$craft/scene.json" ||
  fail "the keyed scale has no keyframe at 1 s"
ctl scene layer "$craft" card transform --scale 3,3 --replace || fail "the keyframes were not replaced"
ctl scene set "$craft" /scene/particles/dust/emitters/0/rate '{"keys": [[0, 10], [1, 30]]}' >/dev/null
if ctl scene particles "$craft" dust settings --rate 20 2>/dev/null; then fail "a rate over keyframes was kept"; fi
ctl scene particles "$craft" dust settings --rate 20 --replace || fail "the keyframed rate was not replaced"
ctl scene layer "$craft" dust instance --alpha 0.5 --count 0.25 --color '#ffcc88' || fail "the instance was not set"
ctl scene layer "$craft" dust instance --unset count || fail "the instance value was not unset"
ctl scene manifest "$craft" license CC-BY-4.0 || fail "the license was not set"
ctl scene manifest "$craft" authors Ana Luis || fail "the authors were not set"
ctl scene manifest "$craft" tags sea night || fail "the tags were not set"
ctl scene manifest "$craft" tags --remove night || fail "the tag was not removed"
ctl scene manifest "$craft" origin derived --derived-from Ana || fail "the origin was not set"
if ctl scene manifest "$craft" license 'free as in beer' 2>/dev/null; then fail "an unknown license was kept"; fi
if ctl scene manifest "$craft" license "$(printf 'a%.0s' {1..300})" 2>/dev/null; then fail "a license of 300 characters was kept"; fi
ctl scene manifest "$craft" display --span all-outputs --accent '#ff8800' || fail "the display was not set"
if ctl scene manifest "$craft" display --fit fit 2>/dev/null; then fail "a scene took the fit of a picture"; fi
ctl scene add-property "$craft" slider --id speed >/dev/null
ctl scene property "$craft" speed --min -1 --max 10 --step 0.5 --default 4 --order 0 || fail "the property range was not set"
if ctl scene property "$craft" speed --default 40 2>/dev/null; then fail "a default out of the range was kept"; fi
ctl scene canvas "$craft" bloom --strength 2 --radius 24 || fail "the canvas does not glow"
ctl scene canvas "$craft" parallax --amount 1.5 --gyro true || fail "the canvas has no parallax"
if ctl scene canvas "$craft" parallax --delay 9 2>/dev/null; then fail "a parallax delay of 9 s was kept"; fi
ctl scene import "$craft" "$ROOT/tests/fixtures/green-64x32.png" --id sheet --atlas 2,4 >/dev/null || fail "no sprite sheet was imported"
ctl scene import "$craft" "$ROOT/tests/fixtures/green-64x32.png" --id bumps --linear >/dev/null || fail "no linear picture was imported"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with authored layers, canvas and manifest was not previewed"
ctl scene act "$craft" add_layer '{"kind": "solid", "id": "generic"}' >/dev/null || fail "scene act added no layer"
ctl scene act "$craft" set '{"pointer": "/scene/canvas/background", "value": "#204060"}' || fail "scene act set no value"
[ "$(ctl scene show "$craft" /scene/canvas/background)" = '"#204060"' ] || fail "scene act did not change the background"
echo '[{"hide": {"id": "generic", "hidden": true}}, {"lock": {"id": "generic", "locked": true}}]' | ctl scene act "$craft" batch - || fail "scene act did not apply a list from the standard input"
ctl scene show "$craft" /scene/ext/editor | grep -q generic || fail "the batch did not lock the layer"
ctl scene act "$craft" undo || fail "a batch was not undone"
if ctl scene show "$craft" /scene/ext/editor 2>/dev/null | grep -q generic; then fail "a batch was not undone as one step"; fi
if ctl scene act "$craft" remove_layer '{"id": "absent"}' 2>/dev/null; then fail "scene act removed a layer that is not there"; fi
ctl scene actions add_layer | grep -q "kind: string (required)" || fail "scene actions did not describe add_layer"
ctl scene ask "$craft" wind | grep -q strength || fail "scene ask did not answer"
ctl scene preview "$craft" -o preview:1 --time 0.5 --quality q1 --battery --calm || fail "a preview on battery with reduced motion was refused"
ctl scene preview "$craft" -o preview:1 --stop
shots=$scene/shots
mkdir -p "$shots"
ctl scene render "$craft" --out "$shots/full.png" --size 96 --time 0.5 >/dev/null || fail "the scene was not rendered to a picture"
ctl scene render "$craft" --out "$shots/low.png" --size 96 --time 0.5 --quality q0 --battery >/dev/null || fail "the scene was not rendered at the lowest level on battery"
for shot in full low; do [ -s "$shots/$shot.png" ] || fail "the picture $shot is empty"; done
ctl scene render "$craft" --out "$shots/clip.gif" --size 64 --frames 4 --fps 10 --pointer 0.2,0.5:0.8,0.5 --click >/dev/null || fail "the clip was not rendered"
[ "$(head -c 3 "$shots/clip.gif")" = GIF ] || fail "the clip is no animated picture"
ctl scene render "$craft" --out "$shots/seq.png" --size 64 --frames 3 >/dev/null || fail "the image sequence was not rendered"
for n in 0001 0003; do [ -s "$shots/seq-$n.png" ] || fail "the image sequence is missing frame $n"; done
ctl scene render "$craft" --out "$shots/versus.png" --size 96 --time 0.5 --against q0 --without-bloom --split 0.25 >/dev/null || fail "the comparison was not rendered"
[ -s "$shots/versus.png" ] || fail "the comparison picture is empty"
if ctl scene render "$craft" --out "$shots/bad.png" --pointer 2,0.5 2>/dev/null; then fail "a pointer outside the canvas was taken"; fi
if ctl scene render "$craft" --out "$shots/bad.txt" 2>/dev/null; then fail "a picture was written with an unknown extension"; fi
lic=$scene/licensed
ctl scene new "$lic" --id org.example.licensed >/dev/null
ctl scene set "$lic" /manifest/license '"CC-BY-SA-4.0"' >/dev/null
mkdir -p "$scene/author-keys"
(cd "$scene/author-keys" && ctl repo keygen ana --no-password >/dev/null && ctl repo keygen bo --no-password >/dev/null)
if (cd "$scene/author-keys" && ctl repo keygen ana --no-password 2>/dev/null); then fail "a key pair was replaced unasked"; fi
ctl scene build "$lic" -o "$scene/licensed.wpak" --key "$scene/author-keys/ana.key" >/dev/null || fail "the signed scene was not built"
ctl pack validate "$lic" --key "$scene/author-keys/ana.pub" | grep -q "^ok scene org.example.licensed" || fail "the signature of the folder was not accepted"
ctl pack validate "$scene/licensed.wpak" --key "$scene/author-keys/ana.pub" | grep -q "signer=" || fail "the signature of the package was not accepted"
if ctl pack validate "$scene/licensed.wpak" --key "$scene/author-keys/bo.pub" 2>/dev/null; then fail "another key vouched for the package"; fi
ctl pack check "$lic" --json | grep -q '"warnings"' || fail "pack check of the folder gave no report"
echo "package review and key checks passed"
