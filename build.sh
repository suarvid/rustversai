#!/bin/bash
cargo build --release
cp target/release/othello /home/arvid/Documents/umu/ai_methods_applications/5dv181ht21/Othello/test_code
cd /home/arvid/Documents/umu/ai_methods_applications/5dv181ht21/Othello/test_code/
./othellostart ./othello_naive ./othello 2
