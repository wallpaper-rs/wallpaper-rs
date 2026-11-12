#!/usr/bin/env bash
set -euo pipefail
declare WORK ROOT
declare -F png >/dev/null
removal=$WORK/removal
mkdir -p "$removal"
cp "$ROOT/tests/fixtures/green-64x32.png" "$removal/a.png"
cp "$ROOT/tests/fixtures/bands-640x180.png" "$removal/b.png"
png "$removal/c.png" ff00ff
ctl lib import "$removal/a.png" "$removal/b.png" "$removal/c.png" --wait >/dev/null
ctl stream TEST-12 160 90 >/dev/null
ctl playlist new Vanish >/dev/null
ctl playlist add Vanish "$removal/a.png" "$removal/b.png"
ctl playlist start Vanish -o TEST-12
current=$(field "s['outputs']['TEST-12']['image']")
ctl lib rm "$current"
expect "s['outputs']['TEST-12']['image'] not in ('$current', None) and s['outputs']['TEST-12']['blank']" False
current=$(field "s['outputs']['TEST-12']['image']")
ctl lib rm "$current"
expect "s['outputs']['TEST-12']['blank']" True
ctl playlist stop -o TEST-12 >/dev/null 2>&1 || true
ctl playlist rm Vanish >/dev/null 2>&1 || true
ctl set "$removal/c.png" -o TEST-12
ctl clear -o TEST-12
rm -f "$removal/c.png"
if ctl restore -o TEST-12 2>"$removal/restore.err"; then fail "restore brought back a deleted file"; fi
grep -q "no longer there" "$removal/restore.err" || fail "restore of a deleted file: $(cat "$removal/restore.err")"
expect "s['outputs']['TEST-12']['blank']" True
ctl release TEST-12
echo "removal checks passed"
