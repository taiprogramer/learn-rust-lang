#!/bin/sh
# Usage: ./run.sh code.rs

rustc $1 -o program.bin && ./program.bin; rm program.bin

