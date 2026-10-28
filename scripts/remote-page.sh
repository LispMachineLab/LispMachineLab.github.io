#!/bin/sh
# Put LispM Remote's page into the built site, at lispm.ai/remote/:
#
#   scripts/remote-page.sh [DIR]      DIR is dist/remote by default
#
# The page is built in LispMachineLab/lispm-remote (`make page`) and
# published there as a release.  This downloads the release named below,
# checks it against its SHA-256, and unpacks it into DIR.  To publish a new
# page, change VERSION and SHA256 to the new release's.
#
# The page is an app, which pairs with LispM and shows nothing before: it
# is kept out of search engines, by a robots meta tag put into its head.
#
# Copyright (C) 2026  Ryan.K
# SPDX-License-Identifier: AGPL-3.0-or-later

set -eu

VERSION=0.5.0
SHA256=078e43f2793f66e012d34821b424dd140e441b1cd2e8824c01d2f912d07f6f17
URL=https://github.com/LispMachineLab/lispm-remote/releases/download/page-v$VERSION/lispm-remote-page-$VERSION.tar.gz

dir=${1:-dist/remote}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

curl -fsSL -o "$tmp/page.tar.gz" "$URL"
if command -v sha256sum >/dev/null 2>&1; then
  echo "$SHA256  $tmp/page.tar.gz" | sha256sum -c -
else
  echo "$SHA256  $tmp/page.tar.gz" | shasum -a 256 -c -
fi

rm -rf "$dir"
mkdir -p "$dir"
tar -xzf "$tmp/page.tar.gz" -C "$dir"

page="$dir/index.html"
awk '{ print } /<head>/ && !done { print "    <meta name=\"robots\" content=\"noindex\">"; done = 1 }' \
  "$page" >"$tmp/index.html"
grep -q '<meta name="robots" content="noindex">' "$tmp/index.html" || {
  echo "remote-page.sh: $page has no <head> to keep it out of search engines" >&2
  exit 1
}
mv "$tmp/index.html" "$page"
echo "LispM Remote's page $VERSION is in $dir"
