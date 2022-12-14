#!/usr/bin/bash

echo "Building all"
cargo build --release

ALL_DAYS=$(ls target/release | grep day[0-9].\$)

echo "Running individual"
for d in $ALL_DAYS ; do
    start_time="$(date -u +%s.%N)"
    ./target/release/$d >> /dev/null
    end_time="$(date -u +%s.%N)"
    elapsed="$(bc <<<"$end_time-$start_time")"
    echo "$(basename $d): $elapsed s"
done

echo "Running all"
start_time="$(date -u +%s.%N)"
for d in $ALL_DAYS ; do
    ./target/release/$d >> /dev/null
done
end_time="$(date -u +%s.%N)"

elapsed="$(bc <<<"$end_time-$start_time")"
echo "Total: $elapsed s"