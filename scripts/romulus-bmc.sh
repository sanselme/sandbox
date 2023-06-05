#!/bin/bash

set -e

qemu-system-arm \
  -uuid "$(uuidgen)" \
  -accel tcg \
  -machine romulus-bmc \
  -m 256 \
  -device usb-kbd \
  -device usb-mouse \
  -drive id=hd0,if=mtd,format=raw,file="${1}" \
  -netdev user,id=net0,hostname=romulus,hostfwd=tcp::2222-:22,hostfwd=tcp::2443-:443,hostfwd=udp::2623-:623 \
  -serial mon:stdio \
  -nographic
