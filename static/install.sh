#!/bin/sh
# Install LispM on macOS, or bring it up to date:
#
#   curl -fsSL https://lispmachinelab.github.io/install.sh | sh
#
# The installer is lispm-desktop's install.sh, which says what it does;
# this runs the current one, so that this address always installs the
# latest.  Read it at
# https://github.com/LispMachineLab/lispm-desktop/blob/main/install.sh

set -eu

main() {
  installer=$(curl -fsSL https://raw.githubusercontent.com/LispMachineLab/lispm-desktop/main/install.sh) || {
    echo "LispM: the installer could not be downloaded; check the connection and try again" >&2
    exit 1
  }
  sh -c "$installer" install.sh "$@"
}

main "$@"
