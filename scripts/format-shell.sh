#!/bin/bash
for f in d:/Git/peek/scripts/*.sh; do
	echo "--- Processing: $f"
	tr -d '\r' <"$f" >"$f.tmp" && mv "$f.tmp" "$f" && echo "OK: $f" || echo "FAIL: $f"
done
