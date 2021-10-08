#!/bin/bash
set -x -e

# `///` used as `//` not works in Windows in this case
for file in *.gir; do
	xmlstarlet ed -L \
		-d '//_:doc/@line' \
		-d '//_:doc/@filename' \
		-d '///_:source-position' \
		"$file"
done
