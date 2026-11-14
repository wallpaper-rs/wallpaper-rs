#!/usr/bin/env bash
set -euo pipefail
declare pkg
keys=$pkg/author-keys
mkdir -p "$keys"
(cd "$keys" && a repo keygen first --no-password >/dev/null && a repo keygen second --no-password >/dev/null)
[ "$(a server author-key list --json | jq_ "len(r['items'])")" = 0 ] || fail "an account has author keys before it registered one"
first=$(a server author-key add "$keys/first.pub")
[ "${#first}" = 64 ] || fail "adding an author key printed no fingerprint: $first"
a server author-key list | grep -q "^$first " || fail "the registered key is not listed: $(a server author-key list)"
if a server author-key add "$keys/first.pub" 2>/dev/null; then fail "a key was registered twice"; fi
if a server author-key add "$keys/first.key" 2>/dev/null; then fail "a secret key was sent as a public key"; fi
if a server author-key rotate "$keys/first.key" "$keys/second.pub" --author u1 2>/dev/null; then
  fail "a repository that binds its authors in the root accepted a hand-over"
fi
if a server author-key remove nothing 2>/dev/null; then fail "a text that is no fingerprint was removed"; fi
a server author-key remove "$first" || fail "the key was not removed"
[ "$(a server author-key list --json | jq_ "len(r['items'])")" = 0 ] || fail "a removed key is still listed"
[ "$(a server uploads --json | jq_ "len(r['uploads'])")" = 0 ] || fail "finished uploads are listed as sessions: $(a server uploads --json)"
if a server uploads --cancel nothing 2>/dev/null; then fail "an upload that is not there was cancelled"; fi
if a server uploads --cancel "../me" 2>/dev/null; then fail "an upload id with a path was accepted"; fi
frost=$pkg/frost
mkdir -p "$frost"
cp "$ROOT/tests/fixtures/green-64x32.png" "$frost/image.png"
cp "$ROOT/tests/fixtures/green-64x32.png" "$frost/preview.png"
cat >"$frost/manifest.json" <<'MANIFEST'
{"schema": "wallpaper-rs.wpak/1", "id": "org.example.frost", "version": "1.0.0", "title": "Frost",
 "license": "CC0-1.0", "kind": "image", "entry": "image.png", "preview": "preview.png"}
MANIFEST
a pack sign --key "$keys/first.key" "$frost" >/dev/null
a pack build "$frost" -o "$pkg/frost.wpak" >/dev/null
if refused=$(a server upload "$pkg/frost.wpak" --wait 2>&1); then fail "a package signed by an unregistered key was accepted"; fi
grep -q "no author key" <<<"$refused" || fail "the refusal does not name the unknown signature: $refused"
first=$(a server author-key add "$keys/first.pub")
if refused=$(a server upload "$pkg/frost.wpak" --wait 2>&1); then
  fail "a repository whose root lists its authors took a key registered only in an account"
fi
grep -q "no author key" <<<"$refused" || fail "the second refusal does not name the signature: $refused"
a server author-key remove "$first" >/dev/null
echo "author key and upload session checks passed"
