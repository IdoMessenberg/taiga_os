# <img src="/resources/images/logos/logo_minimalist.png" alt="Taiga OS logo" title="logo" align="left" height="40" /> Taiga OS - a simple operating system
<p align="center">
<img src="/resources/images/logos/logo_light.png#gh-dark-mode-only" alt="taiga os logo" title="logo" align="center" height="200" />


<img src="/resources/images/logos/logo_dark.png#gh-light-mode-only" alt="Taiga OS logo" title="logo" align="center" height="200" />
</p>

this is a simple operating system written completely in rust from scratch with no external libraries 
## features 🐈
* [ ] custom x86_64 UEFI bootloader
* [ ] a simple shell
## how to build
* rust compiler
* qemu

1. install rust targets   `rustup target add x86_64-unknown-uefi` and
    `rustup target add x86_64-unknown-none`
2. build the the operating system `./batch/build` and `d`
3. run `./batch/run`

## current progress

![](resources/images/screenshots/3_loading_kernel.png)


## bugs
* efi status is u32  and not usize even though it needs to be usize because it brakes the load file function and it won't work
* can not load psf font and display characters 