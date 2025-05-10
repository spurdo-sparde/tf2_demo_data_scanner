#!/bin/bash

# Runs the tf2 demo parser from https://github.com/demostf/parser for all files in a dir and all subdirs.
# You have to specify where it is located on your system.
DEMPARSER="$HOME/tf2/parser/target/release/parse_demo"
INPUT_DIR="$1"

[ -d "$INPUT_DIR" ] || { echo "Usage: $0 /path/to/dem/files"; exit 1; }

find "$INPUT_DIR" -type f -name '*.dem' | while read -r f; do
    out="${f%.dem}.json"
    "$DEMPARSER" "$f" > "$out"
done
