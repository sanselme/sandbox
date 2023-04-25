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

# NOTE: https://linuxconfig.org/bash-script-flags-usage-with-arguments-examples

while getopts 'lha:' flag; do
  case "${flag}" in
  l)
    echo "You have supplied the -l flag"
    ;;
  h)
    echo "You have supplied the -h flag"
    ;;
  a)
    AVAL="${OPTARG}"
    echo "The value provided is ${AVAL}"
    ;;
  ?)
    echo "Script usage: $(basename \$0) [-l] [-h] [-a value]" >&2
    exit 1
    ;;
  esac
done

# NOTE: $/${} is unnecessary on arithmetic variables
# trunk-ignore(shellcheck/SC2004)
shift "$(($OPTIND - 1))"
