#!/bin/bash

position=$1
time_limit=$2
do_compile=$3

if [ "$#" -ne 3 ]; then
    do_compile=0
fi

cd "$(dirname "$0")"

if [ $do_compile -eq 1 ]; then
    cargo build --release
else
    target/release/othello $position $time_limit
fi