@echo off

cd bootloader
cargo build --release
cd ../

cd kernel
cargo build --release
cd ../

copy target\x86_64-unknown-uefi\release\bootx64.efi bin\efi\boot
copy target\x86_64-unknown-none\release\kernel bin

pause