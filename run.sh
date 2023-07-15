cd bootloader
cargo build
cd ..
mkdir -p mnt/efi/boot
cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/efi/boot/bootx64.efi
qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=thirdparty/RELEASEX64_OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    -serial stdio \
    -no-reboot \

