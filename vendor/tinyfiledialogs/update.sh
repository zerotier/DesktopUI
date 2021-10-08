#!/bin/sh
set -eux

for ext in h c
do
	wget "https://sourceforge.net/projects/tinyfiledialogs/files/tinyfiledialogs.${ext}/download" -O - | tr -d '\r' > libtinyfiledialogs/tinyfiledialogs.${ext}
done

for p in $(ls patches -v)
do
	git apply patches/${p}
done
