#!/usr/bin/env bash
set -euo pipefail

CACHE="${CLEARFOLD_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/clearfold}"
IMAGES="${CLEARFOLD_IMAGES:-/mnt/DevStore/Images/Clearfold}"

OVMF_CODE="/usr/share/OVMF/OVMF_CODE_4M.fd"
OVMF_VARS_TEMPLATE="/usr/share/OVMF/OVMF_VARS_4M.fd"
OVMF_VARS="$CACHE/qemu/clearfold-m0a-vars.fd"

IMAGE="$IMAGES/clearfold-m0a.img"

mkdir -p "$CACHE/qemu"

cp "$OVMF_VARS_TEMPLATE" "$OVMF_VARS"

exec qemu-system-x86_64 \
    -enable-kvm \
    -machine q35 \
    -cpu host \
    -m 512M \
    -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
    -drive if=pflash,format=raw,file="$OVMF_VARS" \
    -drive if=virtio,format=raw,file="$IMAGE" \
    -serial stdio
