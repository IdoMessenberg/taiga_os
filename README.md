# <img src="/res/images/logos/logo_minimalist.png" alt="Taiga OS logo" title="logo" align="left" height="40" /> Taiga OS - a simple operating system
<p align="center">
<img src="/res/images/logos/logo_light.png#gh-dark-mode-only" alt="taiga os logo" title="logo" align="center" height="200" />


<img src="/res/images/logos/logo_dark.png#gh-light-mode-only" alt="Taiga OS logo" title="logo" align="center" height="200" />
</p>

simple operating system written from scratch, in rust 

## how to build
* rust compiler
* qemu

1. install rust targets   `rustup target add x86_64-unknown-uefi` and
    `rustup target add x86_64-unknown-none`
2. build the the operating system `./batch/build` and `d`
3. run `./batch/run`


## config file
default config file

```toml
# conf.toml
[loader_paths]
kernel-path = "kernelx64"
font-path = "font.psf"

[graphics.theme]
dark-mode = true
white = 0xEBE3BD
black = 0x282828
red = 0xCF271F
green = 0x989718
blue = 0x478788
yellow = 0xD79820
orange = 0xE08016
purple = 0xB06087
gray = 0xE8D8B0
dark-gray = 0xA09181
light-red = 0xF84837
light-green = 0xB8B827
light-blue = 0x80A798
light-yellow = 0xF8BF2F
light-orange = 0xF3942C
light-purple = 0xD08798

```
