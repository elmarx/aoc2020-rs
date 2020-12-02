#!/usr/bin/env sh

set -o errexit
set -o nounset

for i in src/bin/*; do
  DAY="$(basename -s .rs "${i}")"
  echo Solution "$DAY":
  cargo run -q --bin "$DAY"
  echo ""
done