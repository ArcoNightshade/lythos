#!/usr/bin/env bash
# build-disk.sh — build a bootable raw disk image with a btrfs root filesystem
#
# Produces lythos.img: MBR-partitioned, 256 MiB, single btrfs partition,
# GRUB written to the MBR gap (first 1 MiB), kernel at /boot/lythos.
#
# Usage:
#   ./scripts/build-disk.sh            # debug kernel
#   RELEASE=1 ./scripts/build-disk.sh  # release kernel
#
# Run the result:
#   qemu-system-x86_64 -drive file=lythos.img,format=raw \
#       -serial stdio -display none -m 128M
#
# Flash to real hardware (replace sdX — THIS ERASES THE DRIVE):
#   sudo dd if=lythos.img of=/dev/sdX bs=4M status=progress && sync
#
# Requirements (choose one):
#   Option A — native Linux (requires root for losetup/mount/grub-install):
#     sudo apt install grub-pc-bin grub-common btrfs-progs parted util-linux
#     sudo ./scripts/build-disk.sh
#   Option B — Docker (macOS or any host without native Linux tools):
#     Start Docker Desktop, then run this script (no sudo needed on host).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
IMG_OUT="$REPO_DIR/lythos.img"
IMG_SIZE_MB=256

# ── Build kernel (always runs on the host via cargo) ─────────────────────────

cd "$REPO_DIR"

if [[ "${RELEASE:-0}" == "1" ]]; then
    echo "[build-disk] building release kernel..."
    cargo build --release
    KERNEL_ELF="$REPO_DIR/target/x86_64-lythos/release/lythos"
else
    echo "[build-disk] building debug kernel..."
    cargo build
    KERNEL_ELF="$REPO_DIR/target/x86_64-lythos/debug/lythos"
fi

[[ -f "$KERNEL_ELF" ]] || { echo "error: kernel ELF not found at $KERNEL_ELF" >&2; exit 1; }

# ── assemble <img> <kernel> <grub_cfg> ───────────────────────────────────────
#
# Creates and populates the disk image.  Must run as root on Linux.
# Called either directly (native) or inside a Docker container.

assemble() {
    local img="$1" kernel="$2" grub_cfg="$3"

    echo "[build-disk] creating ${IMG_SIZE_MB} MiB raw image..."
    dd if=/dev/zero of="$img" bs=1M count="$IMG_SIZE_MB" status=none

    echo "[build-disk] partitioning (MBR, btrfs partition at 1 MiB)..."
    parted -s "$img" mklabel msdos
    parted -s "$img" mkpart primary btrfs 1MiB 100%
    parted -s "$img" set 1 boot on

    echo "[build-disk] attaching loop device..."
    local loop part
    loop=$(losetup --find --partscan --show "$img")
    part="${loop}p1"

    _cleanup() {
        umount /mnt 2>/dev/null || true
        losetup -d "$loop" 2>/dev/null || true
    }
    trap _cleanup EXIT

    echo "[build-disk] formatting $part as btrfs..."
    mkfs.btrfs -f -L lythos "$part" >/dev/null

    echo "[build-disk] populating filesystem..."
    mkdir -p /mnt
    mount "$part" /mnt
    mkdir -p /mnt/boot/grub
    cp "$kernel"   /mnt/boot/lythos
    cp "$grub_cfg" /mnt/boot/grub/grub.cfg

    echo "[build-disk] installing GRUB to MBR (i386-pc, modules: btrfs part_msdos)..."
    grub-install \
        --target=i386-pc \
        --boot-directory=/mnt/boot \
        --modules="btrfs part_msdos" \
        --no-floppy \
        "$loop"

    umount /mnt
    losetup -d "$loop"
    trap - EXIT
}

# ── Dispatch: native Linux or Docker ─────────────────────────────────────────

_have_native_tools() {
    for cmd in losetup mkfs.btrfs grub-install parted; do
        command -v "$cmd" &>/dev/null || return 1
    done
}

mkdir -p "$(dirname "$IMG_OUT")"

if _have_native_tools && [[ $EUID -eq 0 ]]; then
    echo "[build-disk] using native tools..."
    assemble "$IMG_OUT" "$KERNEL_ELF" "$SCRIPT_DIR/grub-disk.cfg"

elif _have_native_tools && [[ $EUID -ne 0 ]]; then
    echo "error: native tool path requires root — re-run with sudo" >&2
    exit 1

elif command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    echo "[build-disk] falling back to Docker (linux/amd64)..."
    # Export assemble() into the container via declare -f so the inner script
    # can call it with container-local paths.
    docker run --rm --platform linux/amd64 \
        --privileged \
        -v "$KERNEL_ELF:/kernel:ro" \
        -v "$SCRIPT_DIR/grub-disk.cfg:/grub.cfg:ro" \
        -v "$(dirname "$IMG_OUT"):/out" \
        -e IMG_SIZE_MB="$IMG_SIZE_MB" \
        debian:bookworm-slim \
        bash -c "
            set -euo pipefail
            apt-get update -qq
            apt-get install -y -qq grub-pc-bin grub-common btrfs-progs parted util-linux
            $(declare -f assemble)
            assemble /out/lythos.img /kernel /grub.cfg
        "
else
    {
        echo ""
        echo "error: Linux tools not found and Docker is not available."
        echo ""
        echo "  Option A (native Linux, requires root):"
        echo "    sudo apt install grub-pc-bin grub-common btrfs-progs parted util-linux"
        echo "    sudo ./scripts/build-disk.sh"
        echo ""
        echo "  Option B (Docker, macOS or any host):"
        echo "    Start Docker Desktop, then re-run this script."
        echo ""
    } >&2
    exit 1
fi

# ── Done ──────────────────────────────────────────────────────────────────────

echo ""
echo "[build-disk] done: $IMG_OUT ($(du -sh "$IMG_OUT" | cut -f1))"
echo ""
echo "Run with QEMU:"
echo "  qemu-system-x86_64 -drive file=lythos.img,format=raw \\"
echo "      -serial stdio -display none -m 128M"
echo ""
echo "Flash to real hardware (replace sdX — THIS ERASES THE TARGET DRIVE):"
echo "  sudo dd if=lythos.img of=/dev/sdX bs=4M status=progress && sync"
echo ""
