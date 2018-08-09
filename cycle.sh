#!/usr/bin/env bash
set -ex

for n in {0..9}
do
    t="$(($1 + ${n} * 100))"
    USCIS_URL="http://localhost:8888" cargo run --release ${t}
done
