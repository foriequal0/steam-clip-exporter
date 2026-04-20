#!/usr/bin/env bash

set -euo pipefail

readonly SVG=io.github.foriequal0.SteamClipExporter.svg
readonly OUTPUT="icons.ico"
magick -background none "$SVG" -define icon:auto-resize=16,32,48,,128,256 -compress zip "$OUTPUT"
