#!/usr/bin/env bash
# Generates sample configs in both formats and checks them against the
# Hyprland 0.56.2 option / dispatcher / rule lists.
# Requires: cargo, lua5.4 (or lua), python3.
set -euo pipefail
cd "$(dirname "$0")/.."

LUA=$(command -v lua5.4 || command -v lua)
DATA=tests/hyprland-0.56.2
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

EASY_HYPRLAND_SAMPLES="$OUT" cargo test --quiet write_samples_for_validation >/dev/null

status=0
for f in "$OUT"/*.lua; do
    "$LUA" tests/hl_mock.lua "$DATA" "$f" || status=1
done
for f in "$OUT"/*.conf; do
    python3 tests/validate_conf.py "$DATA" "$f" || status=1
done
exit $status
