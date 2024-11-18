#!/bin/sh
echo -ne '\033c\033]0;2023\a'
base_path="$(dirname "$(realpath "$0")")"
"$base_path/d5p2.arm32" "$@"
