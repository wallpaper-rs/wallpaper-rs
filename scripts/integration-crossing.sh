#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT WORK
crossing=$WORK/crossing
mkdir -p "$crossing"
python3 - "$crossing/own.wpak" "$ROOT/tests/fixtures/green-64x32.png" <<'PACKAGE'
import json, sys, zipfile
manifest = {"schema": "wallpaper-rs.wpak/1", "id": "org.example.crossing", "version": "1.0.0", "title": "Crossing",
            "license": "CC0-1.0", "kind": "scene", "entry": "scene.json", "preview": "preview.png"}
way = {"descriptor": "t/mark.json", "wgsl": "t/mark.wgsl", "duration_ms": 10000}
scene = {"schema": "wallpaper-rs.scene/1", "canvas": {"size": [64, 32], "fit": "stretch"},
         "transition": {"in": way}, "layers": [{"id": "blue", "type": "solid", "color": "#0000ff"}]}
mark = {"id": "mark", "stage": "transition", "cost": {"taps": 1, "passes": 1, "scale": 1}}
wgsl = """fn transition(c: Crossing) -> Crossing {
    var o = c;
    o.color = select(after(c.uv), vec4<f32>(1.0, 0.0, 1.0, 1.0), c.progress < 0.999);
    return o;
}
"""
with zipfile.ZipFile(sys.argv[1], "w") as package:
    package.writestr("manifest.json", json.dumps(manifest))
    package.writestr("scene.json", json.dumps(scene))
    package.writestr("t/mark.json", json.dumps(mark))
    package.writestr("t/mark.wgsl", wgsl)
    package.write(sys.argv[2], "preview.png")
PACKAGE
read -r _ _ _ s31 < <(ctl stream TEST-31 160 90)
ctl set "$ROOT/tests/fixtures/green-64x32.png" -o TEST-31 --fit stretch
shown "$s31" 0 255 0
job=$(bus Library1 Import asb 1 "$crossing/own.wpak" false | out "d[0]")
wait_for 20 finished "$job" || fail "transition scene import: $(bus Daemon1 GetJob t "$job")"
own=$(reply "r['job']['output']['imported'][0]['uid']" Daemon1 GetJob t "$job")
ctl lib apply "$own" -o TEST-31
shown "$s31" 255 0 255
wait_for 20 looks "$s31" 0 0 255 || fail "the scene did not settle after its transition: $("$BIN/probe" "$s31" 3)"
ctl config transition.kind none
ctl set "$ROOT/tests/fixtures/green-64x32.png" -o TEST-31 --fit stretch
shown "$s31" 0 255 0
ctl lib apply "$own" -o TEST-31
wait_for 20 looks "$s31" 0 0 255 || fail "the transition of a scene played with transitions turned off: $("$BIN/probe" "$s31" 3)"
ctl config transition.kind fade
ctl release TEST-31
bus Library1 Remove asb 1 "$own" false >/dev/null
echo "a wallpaper with a transition comes in with it"
