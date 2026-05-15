#!/usr/bin/env bash

set -euo pipefail

TOOLCHAINS=(
  toolchain
  meson
  rust
)

DEPS=(
  gtk4
  libadwaita
)

PKGS=("${TOOLCHAINS[@]}" "${DEPS[@]}")

pacman -S --noconfirm --needed \
  "${PKGS[@]/#/${MINGW_PACKAGE_PREFIX}-}"

rustup override set stable-gnu
