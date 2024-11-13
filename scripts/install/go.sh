#!/bin/bash
# SPDX-License-Identifier: GPL-3.0
# source scripts/alias.sh
source scripts/environment.sh

# install golang
if [[ -z $(command -v go) ]]; then
  sudo apt-get update -yq
  sudo apt-get install -yq --no-install-recommends golang
fi

# install sbctl
if [[ -z $(command -v sbctl) ]]; then
  go install github.com/foxboron/sbctl/cmd/sbctl@0.16
fi

go version
sbctl status
