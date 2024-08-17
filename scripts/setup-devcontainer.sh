#!/bin/bash
# SPDX-License-Identifier: GPL-3.0

set -euxo pipefail

: "${ARCH:=$(dpkg --print-architecture)}"
: "${BUF_VERSION:=1.36.0}"
: "${SBCTL_VERSION:=0.15.4}"

# configure environment
[[ ! -d /home/vscode/.oh-my-zsh ]] &&
  sh -c "$(curl -fsSLo /home/vscode/.oh-my-zsh https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

sudo -E mkdir -p /home/vscode/.ssh
sudo ln -sf /workspace/config/zshrc /root/.zshrc
sudo ln -sf /workspace/config/zshrc /home/vscode/.zshrc
sudo ln -sf /workspace/config/sshconfig /home/vscode/.ssh/config
sudo ln -sf /workspace/config/gitconfig /home/vscode/.gitconfig

sudo ln -sf /workspace/scripts/aliases.sh /etc/profile.d/aliases.sh
sudo ln -sf /workspace/scripts/environment.sh /etc/profile.d/environment.sh

sudo -E sed -i 's/bash/zsh/g' /etc/passwd
sudo -E chown -R vscode:vscode /home/vscode/

source /etc/profile

# install packages
sudo -E apt-get update -y
sudo -E apt-get install -y \
  bison \
  build-essential \
  clang \
  dkms \
  flex \
  gcc \
  golang \
  libelf-dev \
  libssl-dev \
  lld \
  llvm \
  nodejs \
  protobuf-compiler \
  protobuf-compiler-grpc

sudo -E mkdir -p \
  "${CARGO_HOME}" \
  "${RUSTUP_HOME}" \
  "${GOPATH}"

# install sbctl
[[ -z $(command -v sbctl) ]] && {
  curl -fsSLo "/tmp/sbctl-${SBCTL_VERSION}-linux-${ARCH}.tar.gz" "https://github.com/Foxboron/sbctl/releases/download/${SBCTL_VERSION}/sbctl-${SBCTL_VERSION}-linux-${ARCH}.tar.gz"
  tar -xvf "/tmp/sbctl-${SBCTL_VERSION}-linux-${ARCH}.tar.gz" -C /tmp/
  sudo -E install /tmp/sbctl/sbctl /usr/local/bin/
  sbctl status || true
}

# install buf
[[ -z $(command -v buf) ]] && {
  curl -fsSLo "/tmp/buf-$(uname -s)-$(uname -m)" "https://github.com/bufbuild/buf/releases/download/v${BUF_VERSION}/buf-$(uname -s)-$(uname -m)"
  sudo -E install "/tmp/buf-$(uname -s)-$(uname -m)" /usr/local/bin/buf
  buf --version
}

# install rust
[[ -z $(command -v rustc) ]] && {
  curl -fsSLo /tmp/rustup-init.sh https://sh.rustup.rs
  sudo -E RUSTUP_HOME="${RUSTUP_HOME}" CARGO_HOME="${CARGO_HOME}" sh /tmp/rustup-init.sh -y
  rustup component add rust-src
  rustc --version
}

# install trunk.io
[[ -z $(command -v trunk) ]] && {
  curl https://get.trunk.io -fsSL | sudo bash
  sudo -E chmod 755 "$(command -v trunk)"
  trunk --version
}

# todo: install k0s

# post
sudo -E chmod -R 777 "${RUSTUP_HOME}" "${CARGO_HOME}"

# cleanup
rm -f /tmp/rustup-init.sh
rm -f /tmp/buf-*
rm -rf /tmp/sbctl*
