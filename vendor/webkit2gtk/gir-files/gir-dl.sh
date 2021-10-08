#!/bin/bash
set -e -u -o pipefail

MIRRORS="$1"
echo $MIRRORS
WGETPARAMS="--tries=10 --retry-on-http-error=500 --waitretry=1 --show-progress"
EXTRA_MIRROR=security.ubuntu.com/ubuntu/pool
if [ $# -lt 2 ]; then
  MIRROR="mirrors.kernel.org"
else
  MIRROR="$2"
fi
wget $WGETPARAMS -O tmp.html "$MIRRORS"
URL=`cat tmp.html | grep -oP "http://($MIRROR|$EXTRA_MIRROR)/[^\"]+"`
rm tmp.html
echo $URL
wget $WGETPARAMS -O tmp.deb "$URL"
ar x tmp.deb data.tar.xz
rm tmp.deb
tar xf data.tar.xz --strip-components 4 ./usr/share/gir-1.0
rm data.tar.xz
