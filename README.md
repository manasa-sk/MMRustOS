# Rust-Based Open Source OS

## Overview

The OS currently supports x86_64 architecture with UEFI firmware.

---

Steps to Run:

## 0. Prerequisites

Use a Linux-Distro (or WSL from windows) to build and run the following OS.

NOTE: Follow WSL-specific steps for installation, below is for Linux (Ubuntu/Debian) systems.

## 1. Clone and Setup Repo

Clone the repository and create `esp` dir at the root of the folder.
Install the following packages and their dependencies:

```
rustup (get classic version is required)
qemu
```

## 2. Build

Run the following commands in the mentioned directories:

in `/boot/`:
```
cargo build --target x86_64-unknown-uefi
```

in `/kernel`:
```
cargo build
```

at project-root:
```
cp boot/target/x86_64-unknown-uefi/debug/boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp target/x86_64-unknown-none/debug/kernel /esp/kernel.elf
```

Then with root access, run:
```
qemu-system-x86_64   -machine q35   -m 512M   -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd   -drive format=raw,file=fat:rw:esp
```