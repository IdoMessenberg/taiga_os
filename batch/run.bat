@echo off
qemu-system-x86_64 -cpu qemu64 -m 256M -drive  if=pflash,format=raw,unit=0,file=res/ovmf/edk2-x86_64-code.fd,readonly=on -drive if=pflash,format=raw,unit=1,file=res/ovmf/edk2-i386-vars.fd -drive format=raw,file=fat:rw:bin -net none -serial file:logs.log -no-reboot
pause