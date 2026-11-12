#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
ctl=${1:-target/debug}/wprs-ctl
count=0
failed=0
for dir in docs/crear/ejemplos/*/; do
  dir=${dir%/}
  count=$((count + 1))
  if [ ! -f "$dir/scene.json" ] && [ -f "$dir/manifest.json" ]; then
    if ! "$ctl" pack check "$dir" >/dev/null; then
      echo "does not pass wprs-ctl pack check: $dir" >&2
      "$ctl" pack check "$dir" >&2 || true
      failed=$((failed + 1))
    fi
  elif [ ! -f "$dir/scene.json" ]; then
    echo "neither scene.json nor manifest.json: $dir" >&2
    failed=$((failed + 1))
  elif ! "$ctl" scene validate "$dir" >/dev/null; then
    echo "does not pass wprs-ctl scene validate: $dir" >&2
    "$ctl" scene validate "$dir" >&2 || true
    failed=$((failed + 1))
  fi
done
if [ "$count" -eq 0 ]; then
  echo "no example folders under docs/crear/ejemplos" >&2
  exit 1
fi
if [ "$failed" -gt 0 ]; then
  echo "$failed of $count examples failed" >&2
  exit 1
fi
echo "$count examples pass"
python3 -I - <<'PY'
import os
import re
import sys

root = "docs/crear"
fence = re.compile(r"^\s*(```|~~~)")
link = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")
head = re.compile(r"^#{1,6}\s+(.*?)\s*#*\s*$")


def slug(text):
    text = re.sub(r"[`*]", "", text).strip().lower()
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"[^\w\- ]", "", text, flags=re.UNICODE)
    return text.replace(" ", "-")


def read(path):
    inside, lines = False, []
    for line in open(path, encoding="utf-8"):
        if fence.match(line):
            inside = not inside
        elif not inside:
            lines.append(line.rstrip("\n"))
    return lines


def anchors(path):
    seen, found = {}, set()
    for line in read(path):
        match = head.match(line)
        if match:
            base = slug(match.group(1))
            found.add(base if base not in seen else f"{base}-{seen[base]}")
            seen[base] = seen.get(base, 0) + 1
    return found


cache, bad, total = {}, [], 0
for name in sorted(os.listdir(root)):
    if not name.endswith(".md"):
        continue
    page = os.path.join(root, name)
    for number, line in enumerate(read(page), 1):
        for target in link.findall(re.sub(r"`[^`]*`", "", line)):
            if re.match(r"[a-z]+:", target):
                continue
            total += 1
            path, _, frag = target.partition("#")
            dest = os.path.normpath(os.path.join(root, path)) if path else page
            if not os.path.exists(dest):
                bad.append(f"{page}:{number}: {target} does not exist")
            elif frag and dest.endswith(".md"):
                cache.setdefault(dest, anchors(dest))
                if frag not in cache[dest]:
                    bad.append(f"{page}:{number}: {target} has no such heading")
for line in bad:
    print(line, file=sys.stderr)
if bad:
    print(f"{len(bad)} of {total} links between the pages are broken", file=sys.stderr)
    sys.exit(1)
print(f"{total} links between the pages work")
PY
