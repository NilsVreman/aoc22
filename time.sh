#!/usr/bin/bash

echo "Building all"
cargo build --release

ALL_DAYS=$(ls target/release | grep day[0-9].\$)

echo "Running all"
start_time_tot="$(date -u +%s.%N)"
for d in $ALL_DAYS ; do
    if [[ $d != "day16" ]] && [[ $d != "day19" ]]
    then
        start_time="$(date -u +%s.%N)"
        ./target/release/$d >> /dev/null
        end_time="$(date -u +%s.%N)"
        elapsed="$(bc <<<"$end_time-$start_time")"
        echo "$(basename $d): $elapsed s"
    fi
done
end_time_tot="$(date -u +%s.%N)"
elapsed_tot="$(bc <<<"$end_time_tot-$start_time_tot")"
echo "-------------------"
echo "Total: $elapsed_tot s"
