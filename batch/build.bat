@echo off


cd bootloader
cargo build --release
cd ../

cd kernel
cargo build --release
cd ../

xcopy /y target\x86_64-unknown-uefi\release\bootx64.efi bin\efi\boot
xcopy /y target\x86_64-unknown-none\release\kernelx64 bin\
if not EXIST bin\font.psf (
    xcopy /y resources\fonts\kernel_terminal_font.psf bin\
    ren bin\kernel_terminal_font.psf font.psf
)

pause