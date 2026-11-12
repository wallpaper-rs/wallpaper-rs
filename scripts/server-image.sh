#!/usr/bin/env bash
set -euo pipefail
IMAGE=${1:?usage: WPRS_VERSION=<version> [WPRS_IMAGE_CACHE=<dir>] [WPRS_IMAGE_LOAD=1] $0 <registry>/<owner>/wprs-server <tag>...}
shift
ROOT=$(cd "$(dirname "$0")/.." && pwd)
BUILDKIT=docker.io/moby/buildkit:v0.34.0@sha256:b059f8d7226d0b326bc871af489a5dddf65e36af4d20d14251b072974d9ee87c
SCANNER=docker.io/docker/buildkit-syft-scanner:1.12.0@sha256:ae4f3b554449e7e25548e7d8ccc029d17357348e30c6e3df01b92bc93654d6a9
WORK=$(mktemp -d)
builder=wprs-$$
trap 'docker buildx rm "$builder" >/dev/null 2>&1 || true; rm -rf "$WORK"' EXIT
docker buildx create --name "$builder" --driver docker-container --driver-opt "image=$BUILDKIT" >/dev/null
tags=()
for tag in "${@:?no tag given}"; do tags+=(--tag "$IMAGE:$tag"); done
cache=()
if [ -n "${WPRS_IMAGE_CACHE:-}" ]; then
  [ -d "$WPRS_IMAGE_CACHE" ] && cache+=(--cache-from "type=local,src=$WPRS_IMAGE_CACHE")
fi
if [ -n "${WPRS_IMAGE_LOAD:-}" ]; then
  docker buildx build "$ROOT" --builder "$builder" --file "$ROOT/apps/wprs-server/Dockerfile" --platform linux/amd64 \
    --build-arg WPRS_VERSION "${cache[@]}" --cache-to "type=local,dest=$WORK/cache-new,mode=max" "${tags[@]}" --load
  if [ -n "${WPRS_IMAGE_CACHE:-}" ]; then rm -rf "$WPRS_IMAGE_CACHE" && mv "$WORK/cache-new" "$WPRS_IMAGE_CACHE"; fi
  exit 0
fi
cosign=$("$ROOT/scripts/tools.sh" cosign "$WORK")
syft=$("$ROOT/scripts/tools.sh" syft "$WORK")
mkdir "$WORK/src" && git -C "$ROOT" archive HEAD | tar -x -C "$WORK/src"
"$syft" scan "dir:$WORK/src" -q --source-name wprs-server --source-version "${WPRS_VERSION:-local}" \
  -o "spdx-json=$WORK/sources.spdx.json"
docker buildx build "$ROOT" --builder "$builder" --file "$ROOT/apps/wprs-server/Dockerfile" --platform linux/amd64 \
  --build-arg WPRS_VERSION "${cache[@]}" \
  --attest "type=sbom,generator=$SCANNER" --provenance=true "${tags[@]}" --push --metadata-file "$WORK/build.json"
digest=$(python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["containerimage.digest"])' "$WORK/build.json")
"$cosign" sign --yes "$IMAGE@$digest"
"$cosign" attest --yes --type spdxjson --predicate "$WORK/sources.spdx.json" "$IMAGE@$digest"
echo "$IMAGE@$digest"
