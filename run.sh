cd bootloader
cargo build
cd ..
mkdir -p mnt/efi/boot
cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/efi/boot/bootx64.efi
cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf
qemu-system-x86_64 \
    -drive if=pflash,file=thirdparty/RELEASEX64_OVMF_CODE.fd,format=raw,readonly=on \
    -drive if=pflash,file=thirdparty/RELEASEX64_OVMF_VARS.fd,format=raw \
    -drive format=raw,file=fat:rw:mnt \
    -serial stdio \
    -no-reboot \

