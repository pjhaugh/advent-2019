#!/usr/bin/zsh

set -e

OUTDIR="$(dirname "${(%):-%N}")/inputs"

YEAR=2019
DAY=$1

curl -b session="$(cat ~/.aocsession)" "https://adventofcode.com/$YEAR/day/$DAY/input" > "$OUTDIR/input-$(printf %02d "$DAY")-$YEAR.txt"