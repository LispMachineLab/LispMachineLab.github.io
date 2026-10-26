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
# Copyright (C) 2026  Ryan.K
# SPDX-License-Identifier: AGPL-3.0-or-later

set -eu

VERSION=0.4.0
SHA256=c2b21d5ad693680d69a45400346f087da03a6bbf51796fae96c48e9f0fb7d2f2
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
echo "LispM Remote's page $VERSION is in $dir"
