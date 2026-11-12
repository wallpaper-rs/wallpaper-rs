#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT WORK daemon
unshare --user --map-root-user --mount true 2>/dev/null || fail "no user namespace to stand in for the Flatpak sandbox"
stop_daemon "$daemon"
wait_for 10 name_free || fail "the daemon kept its name"
portal_asks=$(grep -c 'background portal' "$WORK/restarted.log" || true)
MAP_UID=$(id -u)
MAP_GID=$(id -g)
export BIN WORK MAP_UID MAP_GID
cat >"$WORK/sandbox-inner.sh" <<'INNER'
mount -t tmpfs tmpfs /run
mkdir -p /run/host/monitor
printf "Etc/UTC\n" >/run/host/monitor/timezone
( for _ in $(seq 60); do
    sleep 0.5
    printf "Etc/GMT-5\n" >/run/host/monitor/timezone.new
    mv /run/host/monitor/timezone.new /run/host/monitor/timezone
  done ) &
exec unshare --user --map-user="$MAP_UID" --map-group="$MAP_GID" env RUST_LOG="$log" FLATPAK_ID=io.github.wallpaper_rs.WallpaperRs "$BIN/wprsd"
INNER
unshare --user --map-root-user --mount bash "$WORK/sandbox-inner.sh" >"$WORK/sandboxed.log" 2>&1 &
sandboxed=$!; pids+=("$sandboxed")
await_daemon "$sandboxed" "$WORK/sandboxed.log" ctl state
"$BIN/wprs-ctl" stream TEST-16 64 32 --wait >"$WORK/sandbox-claim.out" 2>&1 & claim=$!; pids+=("$claim")
wait_for 10 grep -q serial "$WORK/sandbox-claim.out" || fail "no stream in the sandbox: $(cat "$WORK/sandbox-claim.out")"
read -r _ _ _ inside <"$WORK/sandbox-claim.out"
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/sandbox-green.png"
ctl set "$WORK/sandbox-green.png" -o TEST-16 --fit fit
seen() { "$BIN/probe" "$inside" 3 | grep "frame 64x32 .* center bgra \[0, 255, 0, 255\]" >/dev/null; }
wait_for 20 seen || fail "the sandboxed daemon serves no wallpaper: $("$BIN/probe" "$inside" 3 2>&1)"
refusal=$(ctl set "$WORK/sandbox-missing.png" 2>&1 || true)
grep -q 'flatpak override --user --filesystem=' <<<"$refusal" || fail "a missing file inside the sandbox gives no way to share its folder: $refusal"
refusal=$(ctl input pointer 1 1 2>&1 || true)
grep -q 'inside the Flatpak' <<<"$refusal" || fail "the pointer of wprs-ctl was taken inside the sandbox: $refusal"
ctl input status 2>&1 | grep -q 'clicks cannot reach the wallpaper here on: TEST-16' || fail "the sandbox does not say that clicks cannot reach TEST-16"
followed() { grep -q 'time zone changed.*Etc/GMT-5' "$WORK/sandboxed.log"; }
wait_for 20 followed || fail "the sandboxed daemon did not follow the time zone of the host: $(grep -i 'time zone' "$WORK/sandboxed.log" | tail -n 3)"
asked() { grep -q 'background portal' "$WORK/sandboxed.log"; }
wait_for 20 asked || fail "the sandboxed daemon never asked the background portal"
kill "$claim" 2>/dev/null || true
stop_daemon "$sandboxed" "$WORK/sandboxed.log"
start_daemon "$WORK/restarted.log"
! ctl input pointer 1 1 2>&1 | grep -q 'inside the Flatpak' || fail "the pointer of wprs-ctl was refused outside the sandbox"
[ "$(grep -c 'background portal' "$WORK/restarted.log" || true)" = "$portal_asks" ] || fail "a daemon outside the sandbox asked the background portal"
echo "daemon inside the sandbox checks passed"
