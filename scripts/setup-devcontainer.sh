#!/bin/bash
# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

set -euxo pipefail

# configure environment
[[ ! -d /home/vscode/.oh-my-zsh ]] && \
  sh -c "$(curl -fsSLo /home/vscode/.oh-my-zsh https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

sudo mkdir -p \
  "${CARGO_HOME}" \
  "${RUSTUP_HOME}" \
  /home/vscode/.ssh

sudo ln -sf /workspace/config/zshrc /root/.zshrc
sudo ln -sf /workspace/config/zshrc /home/vscode/.zshrc
sudo ln -sf /workspace/config/sshconfig /home/vscode/.ssh/config
sudo ln -sf /workspace/config/gitconfig /home/vscode/.gitconfig

sudo ln -sf /workspace/scripts/aliases.sh /etc/profile.d/aliases.sh
sudo ln -sf /workspace/scripts/environment.sh /etc/profile.d/environment.sh

sudo sed -i 's/bash/zsh/g' /etc/passwd
sudo chown -R vscode:vscode /home/vscode/

# install packages
sudo apt-get update -y
sudo apt-get install -y \
  pkg-config \
  libssl-dev

# install rust if not present
[[ -n $(command -v rustc &> /dev/null) ]] && {
  curl -fsSLo /tmp/rustup-init.sh https://sh.rustup.rs
  sudo RUSTUP_HOME="${RUSTUP_HOME}" CARGO_HOME="${CARGO_HOME}" sh /tmp/rustup-init.sh -y
}
