#!/usr/bin/env bash
jar=$WORK/cookies
step() { curl -fsS -o /dev/null -b "$jar" -c "$jar" "$@"; }
step --data-urlencode "token=$("$BIN/wprs-server" setup-token)" "$URL/setup"
step -d network=own "$URL/setup/network"
step -d handle=admin -d password=correct-horse-battery "$URL/setup/account"
step -d name=Example "$URL/setup/repository"
step -d saved=on "$URL/setup/saved"
first=$(curl -fsS -b "$jar" "$URL/setup" | grep -o 'register?invite=[0-9a-f]*' | head -n 1 | cut -d= -f2)
[ -n "$first" ] || fail "the access step did not show the first invitation"
step -d access=invite "$URL/setup/access"
curl -fsS -o /dev/null -H 'content-type: application/json' "$URL/api/v1/auth/register" \
  -d "{\"handle\": \"friend\", \"password\": \"second-horse-staple\", \"invite\": \"$first\"}" ||
  fail "the first invitation did not let a second person in"
curl -fsS -b "$jar" "$URL/setup" | grep 'Go to Administration' >/dev/null || fail "the setup did not finish"
