#!/usr/bin/env sh

day=$(printf "day%02d" "$1")

time cargo run --release --bin "$day" < input/"$day".txt