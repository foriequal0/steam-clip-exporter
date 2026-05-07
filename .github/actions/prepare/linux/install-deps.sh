#!/usr/bin/env bash

set -euo pipefail

TOOLCHAINS=(
  build-essential
  meson
  cargo
  rustc
  rustfmt
  rust-clippy
)

DEPS=(
  libgtk-4-dev
  libadwaita-1-dev
  desktop-file-utils
  gtk-update-icon-cache
)

export DEBIAN_FRONTEND=noninteractive
apt-get install --update --yes \
  "${TOOLCHAINS[@]}" "${DEPS[@]}"
