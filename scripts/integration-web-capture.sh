#!/usr/bin/env bash
set -euo pipefail
cp -r "$ROOT/tests/fixtures/tint" "$pages/tintdir"
ctl scene thumbnail "$pages/tintdir" --capture -o TEST-W >/dev/null
shot=$(ctl scene show "$pages/tintdir" /manifest/previews/thumbnail | tr -d '"')
[ -n "$shot" ] && [ -f "$pages/tintdir/$shot" ] || fail "no thumbnail was captured from the page: $shot"
center=$(python3 - "$pages/tintdir/$shot" <<'PY'
import struct, sys, zlib
data = open(sys.argv[1], "rb").read()
assert data[:8] == b"\x89PNG\r\n\x1a\n", "not a png"
pos, idat, width, height, kind = 8, b"", 0, 0, 2
while pos < len(data):
    size, name = struct.unpack(">I4s", data[pos:pos + 8])
    body = data[pos + 8:pos + 8 + size]
    if name == b"IHDR":
        width, height, depth, kind = struct.unpack(">IIBB", body[:10])
        assert depth == 8
    if name == b"IDAT":
        idat += body
    pos += 12 + size
step = {2: 3, 6: 4}[kind]
raw, stride = zlib.decompress(idat), width * step
rows, prev = [], bytearray(stride)
for y in range(height):
    start = y * (stride + 1)
    mode, line = raw[start], bytearray(raw[start + 1:start + 1 + stride])
    for i in range(stride):
        a = line[i - step] if i >= step else 0
        b, c = prev[i], (prev[i - step] if i >= step else 0)
        p = a + b - c
        near = min((abs(p - a), a), (abs(p - b), b), (abs(p - c), c))[1]
        line[i] = (line[i] + [0, a, b, (a + b) // 2, near][mode]) & 255
    rows.append(line)
    prev = line
mid = rows[height // 2][width // 2 * step:width // 2 * step + 3]
print(tuple(mid))
PY
)
[ "$center" = "(0, 255, 0)" ] || fail "the captured page frame is not the page: center $center"
echo "web page thumbnail captured through the daemon"
