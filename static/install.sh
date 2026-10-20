#!/bin/sh
# Install LispM on macOS, or bring it up to date:
#
#   curl -fsSL https://lispmachinelab.github.io/install.sh | sh
#
# The installer is lispm-desktop's install.sh, which says what it does;
# this runs the one of LispM's latest release, so that this address always
# installs the latest release.  Read it at
# https://github.com/LispMachineLab/lispm-desktop/blob/main/install.sh

set -eu

# Print the tag of lispm-desktop's latest release, as v0.2.0, or nothing.
latest_release() {
  curl -fsSL https://api.github.com/repos/LispMachineLab/lispm-desktop/releases/latest 2>/dev/null |
    sed -n 's/^ *"tag_name": *"\(v[0-9][^"]*\)".*/\1/p' | head -n 1
}

main() {
  release=$(latest_release)
  installer=$(curl -fsSL "https://raw.githubusercontent.com/LispMachineLab/lispm-desktop/${release:-main}/install.sh") || {
    echo "LispM: the installer could not be downloaded; check the connection and try again" >&2
    exit 1
  }
  sh -c "$installer" install.sh "$@"
}

main "$@"
