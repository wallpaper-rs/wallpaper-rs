#!/bin/sh
set -eu
app=io.github.wallpaper_rs.WallpaperRs
if ! command -v dbus-run-session >/dev/null; then
  echo "no session bus here, so the daemon was not run inside the sandbox"
  exit 0
fi
dir=$(mktemp -d "$HOME/.wprs-serve-XXXXXX")
export app dir
trap 'rm -rf "$dir"' EXIT
cat >"$dir/inner.sh" <<'INNER'
ctl() { flatpak run --filesystem="$dir:ro" --command=wprs-ctl "$app" "$@"; }
flatpak run --command=wprsd "$app" >"$dir/daemon.log" 2>&1 &
daemon=$!
ready=
for _ in $(seq 120); do
  if ctl state >/dev/null 2>&1; then ready=1 && break; fi
  sleep 1
done
if [ -z "$ready" ]; then
  echo "the daemon did not answer inside the sandbox"
  tail -n 40 "$dir/daemon.log"
  kill "$daemon" 2>/dev/null || true
  exit 1
fi
said=$(ctl set "$dir/missing.jpg" 2>&1 || true)
case $said in *"flatpak override --user --filesystem="*) ;; *) echo "a missing file gave no way to share its folder: $said"; kill "$daemon"; exit 1 ;; esac
said=$(ctl input pointer 1 1 2>&1 || true)
case $said in *"inside the Flatpak"*) ;; *) echo "the pointer of wprs-ctl was taken inside the sandbox: $said"; kill "$daemon"; exit 1 ;; esac
kill "$daemon"
wait "$daemon" 2>/dev/null || true
echo "the daemon answered, explained a missing file and refused the pointer inside the sandbox"
INNER
dbus-run-session -- sh "$dir/inner.sh"
