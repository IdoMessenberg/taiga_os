# <img src="/resources/images/logos/logo_minimalist.png" alt="Taiga OS logo" title="logo" align="left" height="40" /> bootloader
a custom x86_64 uefi bootloader inspirited by [shipsimfan](https://github.com/shipsimfan) [los-rs-bootloader](https://github.com/shipsimfan/los-rs-bootloader) 
but with not static variables (other then the global allocator) and with minimal unsafe code
* [x] a custom uefi library
      the library was written from scratch following the [uefi specification 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
* [x] loading kernel entry point
* [ ] pass graphical output protocol info and memory map