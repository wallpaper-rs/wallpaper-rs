#!/usr/bin/env bash
set -euo pipefail
image=${1:?usage: ARCH=<base image> $0 <image name>}
base=${ARCH:?ARCH is not set}
docker run --name prepare --device /dev/dri "$base" bash -c '
  set -e
  install() {
    pacman -Sy --noconfirm --needed archlinux-keyring >/dev/null
    pacman -Syu --noconfirm --needed \
      plasma-desktop plasma-workspace kwin kpipewire spectacle \
      pipewire wireplumber dbus gst-plugins-base gst-plugins-good \
      mesa vulkan-swrast vulkan-icd-loader \
      imagemagick tar gzip libcap gdb jq qt6-declarative kirigami qqc2-desktop-style >/dev/null
  }
  for attempt in 1 2 3 4; do
    install && break
    [ "$attempt" -lt 4 ] || exit 1
    sleep $((attempt * 20))
  done
  pacman -Q plasma-desktop plasma-workspace kwin kpipewire mesa kirigami
'
docker commit prepare "$image" >/dev/null
docker rm prepare >/dev/null
