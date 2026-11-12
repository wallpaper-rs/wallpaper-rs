#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
shown='(text|title|placeholderText|subtitle|description|displayText|Accessible\.name|Accessible\.description|ToolTip\.text)'
allowed='^("(https?://[^"]*|wallpaper-rs|BCDF-GHJK|Aa|sculpted_1)")$'
while IFS= read -r hit; do
  literal=$(sed -E "s/^[^:]*:[0-9]+:\s*$shown\s*:\s*(\"[^\"]*\").*/\2/" <<<"$hit")
  literal=$(sed -E 's/^\S+ (\x22)/\1/' <<<"$literal")
  if ! grep -Eq "$allowed" <<<"$literal"; then
    echo "text shown without translation: $hit" >&2
    fail=1
  fi
done < <(grep -rnE "^\s*$shown\s*:\s*\"[^\"]*[A-Za-z]{2}" apps/wprs-app/qml --include='*.qml' || true)
derived=$(grep -rnE 'charAt\(0\)\.toUpperCase|\.replace\(/_/g' apps/wprs-app/qml --include='*.qml' | grep -v 'facade/SceneWords.qml' || true)
if [ -n "$derived" ]; then
  echo "label made from an identifier outside SceneWords.label:" >&2
  echo "$derived" >&2
  fail=1
fi
python3 -I - po/es/*.po <<'PY' || fail=1
import re, sys

def field(entry, key):
    found = re.search(r'^' + key + r' ((?:"(?:[^"\\]|\\.)*"\n?)+)', entry, re.M)
    return "".join(re.findall(r'"((?:[^"\\]|\\.)*)"', found.group(1))) if found else None

joined = re.compile(r'[a-záéíóúñ][A-ZÁÉÍÓÚÑ][a-záéíóúñ]')
bad = 0
for path in sys.argv[1:]:
    for entry in open(path, encoding="utf-8").read().split("\n\n"):
        text, spanish = field(entry, "msgid"), field(entry, "msgstr")
        if not text or not spanish:
            continue
        problems = []
        if sorted(re.findall(r'%\d', text)) != sorted(re.findall(r'%\d', spanish)):
            problems.append("different placeholders")
        if text.endswith("…") != spanish.endswith("…"):
            problems.append("ellipsis differs")
        if text.endswith(":") != spanish.endswith(":"):
            problems.append("colon differs")
        if joined.search(spanish) and not joined.search(text):
            problems.append("two sentences run together")
        if problems:
            bad = 1
            print(f"{path}: {', '.join(problems)}: {text[:60]!r}", file=sys.stderr)
sys.exit(bad)
PY
python3 -I scripts/translations-rust.py || fail=1
exit $fail
