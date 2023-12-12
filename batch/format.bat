@echo off

cd bootloader
cargo +nightly fmt

cd lib/uefi
cargo +nightly fmt

cd ../efi_elf_loader
cargo +nightly fmt

cd ../../../

cd kernel
cargo +nightly fmt

pause