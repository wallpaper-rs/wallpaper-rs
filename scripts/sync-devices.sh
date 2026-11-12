#!/usr/bin/env bash
set -euo pipefail
e() { DBUS_SESSION_BUS_ADDRESS=unix:path=$WORK/e/bus "$BIN/wprs-ctl" "$@"; }
mkdir -p "$WORK/e/run" && chmod 700 "$WORK/e/run"
cat >"$WORK/e/bus.conf" <<CONF
<busconfig><type>session</type><listen>unix:path=$WORK/e/bus</listen><auth>EXTERNAL</auth>
<policy context="default"><allow send_destination="*" eavesdrop="true"/><allow eavesdrop="true"/><allow own="*"/></policy></busconfig>
CONF
: >"$WORK/e/env"
pids+=("$(dbus-daemon --config-file="$WORK/e/bus.conf" --fork --print-pid)")
daemon e
e server add "$URL" --wait >/dev/null
devices() { sql "SELECT count(*) FROM devices"; }
approve() { api -d "{\"user_code\": \"$(sed -n 's/^code = //p' <<<"$1")\", \"approve\": true}" "$URL/api/v1/auth/device/approve" >/dev/null; }
listed() { a server status --json | jq_ "[d['name'] for d in r['account']['devices']].count('$1')"; }
signed() { [ "$(value e "['state']")" = signed_out ]; }
before=$(devices)
if e server link --name Plain >"$WORK/nokeyring.out" 2>"$WORK/nokeyring.err"; then fail "a computer with no keyring linked"; fi
grep -q "No keyring" "$WORK/nokeyring.err" || fail "no word about the missing keyring: $(cat "$WORK/nokeyring.err")"
[ "$(devices)" = "$before" ] || fail "a link without a keyring left a device on the account"
[ "$(value e "['file_secrets']")" = False ] || fail "the file was chosen without being asked"
e server access file 2>/dev/null
[ "$(value e "['file_secrets']")" = True ] || fail "the file choice was not kept: $(e server status)"
file=$WORK/e/data/wallpaper-rs/server-secrets.json
mkdir "$file"
link=$(e server link --name Plain)
approve "$link"
wait_for 60 failed e "$(tail -n 1 <<<"$link")" || fail "the link that cannot keep its token did not fail: $(e job "$(tail -n 1 <<<"$link")")"
[ "$(devices)" = "$before" ] || fail "a failed link left a device on the account"
[ "$(value e "['link_problem']")" = other ] || fail "the failure was not kept: $(e server status)"
rmdir "$file"
link=$(e server link --name Plain)
approve "$link"
await e "$(tail -n 1 <<<"$link")"
[ "$(devices)" = $((before + 1)) ] || fail "the link left more than one device"
[ "$(value e "['link_problem']")" = None ] || fail "an old failure stayed: $(e server status)"
[ "$(stat -c %a "$file")" = 600 ] || fail "the file of the access is not private"
name=$(value e "['device']")
a server sync --wait >/dev/null
[ "$(listed "$name")" = 1 ] || fail "the new device is not listed: $(a server status)"
if printf 'correct-horse-battery\n' | a server revoke Device-0 --yes 2>/dev/null; then fail "an unknown device was revoked"; fi
printf 'correct-horse-battery\n' | a server revoke "$name" --yes --wait >/dev/null
[ "$(listed "$name")" = 0 ] || fail "the device stayed: $(a server status)"
[ "$(devices)" = "$before" ] || fail "the device stayed on the server"
if e server join "Device 1" 2>/dev/null; then fail "a revoked device joined"; fi
wait_for 30 signed || fail "the revoked device shows nothing: $(e server status)"
e server status | grep -q "signed out" || fail "ctl does not say it: $(e server status)"
e server remove
[ "$(value e "")" = None ] || fail "the revoked device kept the server"
[ ! -e "$file" ] || fail "the file of the access stayed after removing the server"
echo "device list, revoke and no-keyring link checks passed"
