#!/usr/bin/env bash
set -euo pipefail

CACHE="${CLEARFOLD_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/clearfold}"
TARGET="${CARGO_TARGET_DIR:-$CACHE/cargo-target}"
IMAGES="${CLEARFOLD_IMAGES:-/mnt/DevStore/Images/Clearfold}"

STAGE0="$TARGET/x86_64-unknown-uefi/debug/clearfold-stage0.efi"
KERNEL="$TARGET/x86_64-unknown-none/debug/clearfold-kernel"

IMAGE="$IMAGES/clearfold-m0b.img"

mkdir -p "$IMAGES"

cargo build \
    -p clearfold-kernel \
    --target x86_64-unknown-none

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
    "$STAGE0" \
    ::/EFI/BOOT/BOOTX64.EFI

mcopy \
    -i "$IMAGE" \
    "$KERNEL" \
    ::/KERNEL.ELF

echo
echo "Clearfold M0b image created:"
echo "$IMAGE"
