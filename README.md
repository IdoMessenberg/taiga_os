# <img src="/resources/images/logos/logo_minimalist.png" alt="Taiga OS logo" title="logo" align="left" height="40" /> Taiga OS - a simple operating system
<p align="center">
<img src="/resources/images/logos/logo_light.png#gh-dark-mode-only" alt="taiga os logo" title="logo" align="center" height="200" />


<img src="/resources/images/logos/logo_dark.png#gh-light-mode-only" alt="Taiga OS logo" title="logo" align="center" height="200" />
</p>

```
    /\    Taiga os
    /\    description: this is a simple operating system writen completely 
   /__\   in rust from scratch with no external libraries
   / .\   target: x86_64
  /____\  uefi boot: true
  /  . \         _
 /  .   \     |\'/-..--.
/________\   / _ _   ,  ;
   |  |     `~=`Y'~_<._./
   |  |	     <`-....__.'
```

## features 🐈
* [x] custom x86_64 UEFI bootloader
* [ ] a simple shell
    * [ ] simple PS2 driver ⌨️
    * [ ] simple lexer
    * [ ] some system calls and functions
* [ ] run a form of  doom💥

## how to build
* rust compiler
* qemu

1. install rust targets   `rustup target add x86_64-unknown-uefi` and
    `rustup target add x86_64-unknown-none`
2. build the the operating system `./batch/build` and `d`
3. run `./batch/run`

## current progress

![](resources/images/screenshots/6_got_memory_map.png)
## bugs
* efi status is u32  and not usize even though it needs to be usize, because it(being a usize) brakes the load file function and it won't work for some reason 
* memory map index is smaller then it is supposed to be