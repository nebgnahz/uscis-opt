#!/usr/bin/env bash
set -ex

for n in {0..19}
do
    t="$(($1 + ${n} * 100))"
    cargo run --release ${t}
done
