import glob
import re
import sys


def skip_string(text, i):
    i += 1
    while text[i] != '"':
        i += 2 if text[i] == "\\" else 1
    return i + 1


def close(text, i):
    depth = 0
    while True:
        c = text[i]
        if c == '"':
            i = skip_string(text, i)
            continue
        if c == "'":
            m = re.match(r"'(\\.|[^'\\])'", text[i : i + 4])
            if m:
                i += len(m.group(0))
                continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                return i
        i += 1


def split(text):
    out, depth, i, start = [], 0, 0, 0
    while i < len(text):
        c = text[i]
        if c == '"':
            i = skip_string(text, i)
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            out.append(text[start:i].strip())
            start = i + 1
        i += 1
    if text[start:].strip():
        out.append(text[start:].strip())
    return out


CALL = re.compile(
    r"(?<![A-Za-z_])(usage|bail!|anyhow!|ensure!|println!|eprintln!|eprint!|format!)\(|\.(context|with_context)\("
)
TOKENS = re.compile(r"^(file://|/|https?://|[a-z_.]+:|(use )?wprs-ctl |--)")
DIAGNOSTICS = ("apps/wprs-ctl/src/bench.rs", "apps/wprs-ctl/src/benching.rs")
FORWARDING = ("crates/wprs-core/src/edit/depth.rs",)
LITERAL = re.compile(r'"((?:[^"\\]|\\.)*)"')


def words(text):
    plain = re.sub(r"\{[^}]*\}|%\d", "", text)
    return len(re.findall(r"[A-Za-z]{3,}", plain)) >= 1 and " " in plain.strip()


def literal_of(argument):
    m = re.fullmatch(r'"((?:[^"\\]|\\.)*)"', argument, re.S)
    if m:
        return m.group(1)
    m = re.fullmatch(r'(?:\|\|\s*)?format!\(\s*"((?:[^"\\]|\\.)*)"\s*(?:,.*)?\)', argument, re.S)
    return m.group(1) if m else None


def scan_ctl(path):
    text = open(path, encoding="utf-8").read()
    found = []
    for m in CALL.finditer(text):
        op = m.end() - 1
        end = close(text, op)
        kind = m.group(1) or m.group(2)
        args = split(text[op + 1 : end])
        index = 1 if kind == "ensure!" else 0
        if len(args) <= index:
            continue
        lit = literal_of(args[index])
        if lit is None or not words(lit) or TOKENS.match(lit) or re.search(r"\{[^}]*:[<>^]", lit):
            continue
        if kind in ("println!", "eprintln!", "eprint!", "bail!", "anyhow!", "ensure!", "format!") and not (
            args[index].startswith('"')
        ):
            continue
        found.append((text.count("\n", 0, m.start()) + 1, kind, lit))
    return found


def scan_said(path):
    text = open(path, encoding="utf-8").read()
    found = []
    for m in re.finditer(r"(?<![A-Za-z_])said\(", text):
        op = m.end() - 1
        end = close(text, op)
        args = split(text[op + 1 : end])
        before = text[max(0, m.start() - 4) : m.start()]
        if before.endswith("fn ") or args[:3] == ["id", "text", "args"] or path in FORWARDING:
            continue
        if len(args) >= 3 and not args[1].startswith('"'):
            found.append((text.count("\n", 0, m.start()) + 1, "said", args[1]))
    return found


bad = 0
for path in sorted(glob.glob("apps/wprs-ctl/src/**/*.rs", recursive=True)):
    if "/tests" in path or path.endswith("tests.rs") or path in DIAGNOSTICS:
        continue
    for line, kind, lit in scan_ctl(path):
        print(f"{path}:{line}: {kind} shows English text without translation: {lit[:70]!r}", file=sys.stderr)
        bad = 1
for path in sorted(glob.glob("crates/*/src/**/*.rs", recursive=True)):
    if "/tests" in path:
        continue
    for line, kind, lit in scan_said(path):
        print(f"{path}:{line}: said() text is not a literal, so it is never translated: {lit[:50]}", file=sys.stderr)
        bad = 1
sys.exit(bad)
