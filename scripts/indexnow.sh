#!/bin/sh
# Tell the search engines that take IndexNow (Bing, and through it the
# answer engines that search with Bing; Yandex, Naver, Seznam and others)
# that the site's pages are new, so they need not wait to crawl them:
#
#   scripts/indexnow.sh [--print]     --print shows the request, sends nothing
#
# The pages are those in the deployed sitemap.xml.  The key is the name of
# static/<key>.txt, which the site serves at its root, so the engines can
# check that the request is the site's.  Pages run it after each deploy.
#
# Copyright (C) 2026  Ryan.K
# SPDX-License-Identifier: AGPL-3.0-or-later

set -eu

HOST=lispm.ai
ENDPOINT=https://api.indexnow.org/indexnow

here=$(dirname "$0")
key=$(ls "$here/../static" | sed -n 's/^\([0-9a-f]\{32\}\)\.txt$/\1/p')
[ -n "$key" ] || {
  echo "indexnow.sh: no static/<key>.txt" >&2
  exit 1
}

urls=$(curl -fsSL "https://$HOST/sitemap.xml" | sed -n 's|.*<loc>\(.*\)</loc>.*|\1|p')
[ -n "$urls" ] || {
  echo "indexnow.sh: the sitemap names no pages" >&2
  exit 1
}

body=$(printf '%s\n' "$urls" | jq -Rn --arg host "$HOST" --arg key "$key" \
  '{host: $host, key: $key, keyLocation: "https://\($host)/\($key).txt", urlList: [inputs]}')

if [ "${1-}" = --print ]; then
  printf 'POST %s\n%s\n' "$ENDPOINT" "$body"
  exit 0
fi

status=$(printf '%s' "$body" | curl -sS -o /dev/null -w '%{http_code}' -X POST \
  -H 'Content-Type: application/json; charset=utf-8' --data-binary @- "$ENDPOINT")
case $status in
  200 | 202) echo "IndexNow took $(printf '%s\n' "$urls" | wc -l | tr -d ' ') pages ($status)" ;;
  *)
    echo "indexnow.sh: IndexNow answered $status" >&2
    exit 1
    ;;
esac
