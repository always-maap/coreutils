#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Never gonna give you up" > $OUTDIR/test1.txt
echo "hello"  "world" > $OUTDIR/test2.txt
echo -n "Never gonna give you up" > $OUTDIR/test1.n.txt
echo -n "hello"  "world" > $OUTDIR/test2.n.txt
