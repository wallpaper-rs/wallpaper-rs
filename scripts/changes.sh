#!/usr/bin/env bash
set -euo pipefail
BASE=${1:-}
ALWAYS=${2:-false}
rust=true
android=true
if [ "$ALWAYS" != true ] && [[ $BASE =~ ^[0-9a-f]{40}$ ]] && [[ $BASE != 0000000000000000000000000000000000000000 ]] &&
  git cat-file -e "$BASE^{commit}" 2>/dev/null; then
  files=$(git diff --name-only "$BASE" HEAD)
  if [ -n "$files" ]; then
    outside() { grep -qvE "$1" <<< "$files"; }
    outside '^android/' || rust=false
    outside '^(apps/(wprsd|wprs-ctl|wprs-app|wprs-web|wprs-server)/|crates/wprs-(pipewire|wayland|rgb|client|editor|video)/|integrations/|packaging/|scripts/(integration|sway|x11|sync|plasma|web|video|server|rpm|package|release|ci-drm|private-audio|ambient|phone|file-lengths)[^/]*$)' || android=false
  fi
fi
echo "rust=$rust"
echo "android=$android"
