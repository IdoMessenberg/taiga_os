(cd bootloader; cargo build --release)
(cd kernel; cargo build --release)

mkdir -p bin/efi/boot
mkdir -p bin/fonts
mkdir -p bin/themes

cp target/x86_64-unknown-uefi/release/bootx64.efi bin/efi/boot
cp target/x86_64-unknown-none/release/kernelx64 bin/

cp res/assets/fonts/kernel_terminal_font.psf bin/fonts/default.psf
cp res/assets/config.toml bin/
cp res/assets/themes/default.toml bin/themes