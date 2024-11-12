#!/bin/bash
# SPDX-License-Identifier: GPL-3.0
# source scripts/alias.sh
source scripts/environment.sh

# install nodejs
if [[ -z $(command -v node) ]]; then
  apt-get update -yq
  apt-get install -yq --no-install-recommends nodejs npm
fi

echo """
node $(node --version)
npm $(npm --version)
"""
