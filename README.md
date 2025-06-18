# <img src="/res/images/logos/logo_minimalist.svg" alt="Taiga OS logo" title="logo" align="left" height="40" /> Taiga OS - a simple operating system
<p align="center">
<img src="/res/images/logos/logo_light.svg#gh-dark-mode-only" alt="taiga os logo" title="logo" align="center" height="400" />


<img src="/res/images/logos/logo_dark.svg#gh-light-mode-only" alt="Taiga OS logo" title="logo" align="center" height="400" />
</p>

simple self contained operating system written from scratch in rust 

## how to build
### needed
* rust compiler
* qemu

1. install rust targets   `rustup target add x86_64-unknown-uefi` and
    `rustup target add x86_64-unknown-none`
2. build the the operating system `./tools/build.sh`
3. run `./tools/run.sh`

## features

* [x] custom bootloader
    * [x] UEFI boot
    * [x] config file 
    * [x] ...

* [ ] kernel
    * [x] page frame allocator
    * [x] idt, gdt
    * [x] terminal output
    * [x] terminal input
    * [x] paging
    * [ ] heap allocation
    * [ ] file system

