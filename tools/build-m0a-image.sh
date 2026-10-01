#!/usr/bin/env bash
set -euo pipefail

CACHE="${CLEARFOLD_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/clearfold}"
TARGET="${CARGO_TARGET_DIR:-$CACHE/cargo-target}"
IMAGES="${CLEARFOLD_IMAGES:-/mnt/DevStore/Images/Clearfold}"

EFI_BIN="$TARGET/x86_64-unknown-uefi/debug/clearfold-stage0.efi"
IMAGE="$IMAGES/clearfold-m0a.img"

mkdir -p "$IMAGES"

cargo build \
    -p clearfold-stage0 \
    --target x86_64-unknown-uefi

rm -f "$IMAGE"

truncate -s 64M "$IMAGE"

mkfs.vfat \
    -F 32 \
    -n CLEARFOLD \
    "$IMAGE"

mmd -i "$IMAGE" ::/EFI
mmd -i "$IMAGE" ::/EFI/BOOT

mcopy \
    -i "$IMAGE" \
    "$EFI_BIN" \
    ::/EFI/BOOT/BOOTX64.EFI

echo
echo "Clearfold M0a image created:"
echo "$IMAGE"
