#!/bin/bash
# SPDX-License-Identifier: GPL-3.0
# source scripts/alias.sh
# source scripts/environment.sh

apt-get update -yq
apt-get install -yq --no-install-recommends \
  binutils \
  bison \
  build-essential \
  clang \
  flex \
  gcc \
  git \
  gnupg2 \
  libc6-dev \
  libcurl4 \
  libedit2 \
  libelf-dev \
  libgcc-9-dev \
  libncurses6 \
  libsqlite3-0 \
  libssl-dev \
  libstdc++-9-dev \
  libxml2 \
  libz3-dev \
  lld \
  llvm \
  pkg-config \
  tzdata \
  uuid-dev \
  zlib1g-dev
