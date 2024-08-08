# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# SPDX-License-Identifier: GPL-3.0

# Export environment variables
export EDITOR="vi"
export GPG_TTY="$(tty)"

# Re-export PATH
export SCRIPTS="${HOME}/scripts"
export TOOLS="${HOME}/tools"

export RUSTUP_HOME="/usr/local/rust/rustup"
export CARGO_HOME="/usr/local/rust/cargo"

export PATH="${CARGO_HOME}/bin:${TOOLS}${PATH:+:${PATH}}"

# SSH Agent
ssh-add -l >> /dev/null
if [[ ! $? -eq 0 ]]; then
  eval $(ssh-agent -s)
  ssh-add -K
fi
