#!/usr/bin/env bash

set -euo pipefail

mkdir build
meson setup build
pushd build

meson compile
