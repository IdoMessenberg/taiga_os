@echo off

cd bootloader
cargo +nightly fmt

cd lib/uefi
cargo +nightly fmt

cd ../../../

pause