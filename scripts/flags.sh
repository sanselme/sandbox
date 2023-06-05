#!/bin/sh

# Copyright © 2022 Schubert Anselme <schubert@anselm.es>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

set -e

# NOTE: https://www.banjocode.com/post/bash/flags-bash

SKIP_VERIFICATION=false
TAG=""

print_help() {
  cat <<EOF
Usage: $0 [-t tag] [--skip-verification]

Options:
  -t, --tag:            tag
  --skip-verification:  skip verification
EOF
}

while [ "${1}" != "" ]; do
  case ${1} in
  --skip-verification)
    SKIP_VERIFICATION=true
    ;;
  -t | --tag)
    shift
    TAG=${1}
    ;;
  -h | --help)
    print_help
    ;;
  *)
    print_help
    exit 1
    ;;
  esac
  shift
done

if [ "${TAG}" = "" ]; then
  echo "You must provide a tag"
  echo ""
  print_help
  exit 1
fi

if [ ${SKIP_VERIFICATION} = true ]; then
  # TODO: do not verify
  echo "skipping verification..."
else
  # TODO: do verify
  echo "verifying..."
fi
