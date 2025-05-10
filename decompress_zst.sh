#!/bin/bash

# Decompresses all .zst files in a directory and all subdirectories
find . -type f -name '*.zst' -exec unzstd -f {} \;
