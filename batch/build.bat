@echo off

cd src

cd bootloader
cargo build --release
cd ../

cd kernel
cargo build --release
cd ../

cd ../
xcopy /y target\x86_64-unknown-uefi\release\bootx64.efi bin\efi\boot
xcopy /y target\x64_kernel_target\release\kernel bin\
if not EXIST bin\font.psf (
    xcopy /y resources\fonts\kernel_terminal_font.psf bin\
    ren bin\kernel_terminal_font.psf font.psf
)

pause