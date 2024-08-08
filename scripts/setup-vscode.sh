#!/bin/bash
# SPDX-License-Identifier: GPL-3.0

set -e

DEFAULT_EXTENSION_LIST=(
  "aaron-bond.better-comments"
  "bierner.github-markdown-preview"
  "christian-kohler.path-intellisense"
  "codezombiech.gitignore"
  "eamodio.gitlens"
  "esbenp.prettier-vscode"
  "formulahendry.code-runner"
  "GitHub.copilot"
  "hbenl.vscode-test-explorer"
  "jerrygoyal.shortcut-menu-bar"
  "minherz.copyright-inserter"
  "ms-vscode.hexeditor"
  "ms-vscode.test-adapter-converter"
  "ms-vscode.vscode-serial-monitor"
  "patbenatar.advanced-new-file"
  "redhat.vscode-yaml"
  "trunk.io"
  "vscode-icons-team.vscode-icons"
  "wayou.vscode-todo-highlight"
)

INSTALL_RECOMMENDATIONS=false
[[ ${1} == "--recommendations" ]] && INSTALL_RECOMMENDATIONS=true

# Install extensions except tunk.io for Windows
for extension in "${DEFAULT_EXTENSION_LIST[@]}"; do
  [[ ${OSTYPE} == "msys" && ${extension} == "trunk.io" ]] && {
    continue
  }

  code --install-extension "${extension}"
done

# Install recommendations if any
[[ -n ${INSTALL_RECOMMENDATIONS} ]] && {
  recommendations_list=$(jq -r '.recommendations' .vscode/extensions.json)
  [[ ${recommendations_list} == "[]" ]] && {
    echo "No recommendations to install"
    exit 0
  }

  for extension in $(jq -r '.[]' "${recommendations_list}"); do
    code --install-extension "${extension}"
  done
}
