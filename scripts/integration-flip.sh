#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT WORK
flip=$WORK/flip
mkdir -p "$flip"
python3 - "$flip" "$ROOT/tests/fixtures/green-64x32.png" "$ROOT/tests/fixtures/quadrants-64x32.webm" <<'PACKAGE'
import json, struct, sys, zipfile, zlib
def png(mirror_x, mirror_y):
    rows = b""
    for y in range(32):
        row = b"\0"
        for x in range(64):
            u, v = (63 - x if mirror_x else x), (31 - y if mirror_y else y)
            row += bytes([u * 4, v * 8, 40, 255])
        rows += row
    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 64, 32, 8, 6, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(rows)) + chunk(b"IEND", b"")
def package(name, kind, entry, data, flip):
    manifest = {"schema": "wallpaper-rs.wpak/2", "id": f"org.example.{name}", "version": "1.0.0", "title": name,
                "license": "CC0-1.0", "kind": kind, "entry": entry, "photosensitive": False,
                "content": {"rating": "general"}, "origin": {"kind": "original"},
                "previews": {"thumbnail": "preview.png"}}
    manifest["requires"] = {"features": ["display2"] if kind == "image" else []}
    if flip:
        manifest["requires"]["features"] += ["flip"] + (["video_properties"] if kind == "video" else [])
        manifest["display"] = {"flip": flip}
    with zipfile.ZipFile(f"{sys.argv[1]}/{name}.wpak", "w") as out:
        out.writestr("manifest.json", json.dumps(manifest))
        out.write(sys.argv[2], "preview.png")
        out.writestr(entry, data)
package("plain", "image", "image.png", png(False, False), None)
package("across", "image", "image.png", png(False, False), "horizontal")
package("upside", "image", "image.png", png(False, False), "vertical")
package("mirror", "image", "image.png", png(True, False), None)
package("turned", "image", "image.png", png(False, True), None)
clip = open(sys.argv[3], "rb").read()
package("reel", "video", "loop.webm", clip, None)
package("reel_across", "video", "loop.webm", clip, "horizontal")
package("reel_upside", "video", "loop.webm", clip, "vertical")
PACKAGE
read -r _ _ _ flipped < <(ctl stream TEST-15 64 32)
frame_hash() { "$BIN/probe" "$flipped" 3 2>/dev/null | sed -n 's/.* hash \([0-9a-f]*\)$/\1/p'; }
bring() {
  local job
  job=$(bus Library1 Import asb 1 "$flip/$1.wpak" false | out "d[0]")
  wait_for 20 finished "$job" || fail "import of $1: $(bus Daemon1 GetJob t "$job")"
  reply "r['job']['output']['imported'][0]['uid']" Daemon1 GetJob t "$job"
}
hash_of() {
  ctl lib apply "$(bring "$1")" -o TEST-15 >/dev/null || fail "$1 was not set"
  local ends=$((SECONDS + 20)) one two
  while [ "$SECONDS" -lt "$ends" ]; do
    one=$(frame_hash); sleep 0.4; two=$(frame_hash)
    [ -n "$one" ] && [ "$one" = "$two" ] && { echo "$one"; return 0; }
  done
  fail "$1 never settled on a frame"
}
plain=$(hash_of plain); across=$(hash_of across); upside=$(hash_of upside)
mirror=$(hash_of mirror); turned=$(hash_of turned)
[ "$plain" != "$across" ] || fail "a horizontal flip left the picture as it was"
[ "$plain" != "$upside" ] || fail "a vertical flip left the picture as it was"
[ "$across" != "$upside" ] || fail "both flips drew the same picture"
[ "$across" = "$mirror" ] || fail "a horizontal flip is not the mirrored picture ($across, $mirror)"
[ "$upside" = "$turned" ] || fail "a vertical flip is not the picture turned over ($upside, $turned)"
reel=$(hash_of reel); reel_across=$(hash_of reel_across); reel_upside=$(hash_of reel_upside)
[ "$reel" != "$reel_across" ] || fail "a horizontal flip left the video as it was"
[ "$reel" != "$reel_upside" ] || fail "a vertical flip left the video as it was"
[ "$reel_across" != "$reel_upside" ] || fail "both flips drew the same video"
ctl release TEST-15
echo "display flip pixels from the stream checks passed"
