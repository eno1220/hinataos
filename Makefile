QEMU_ARGS=\
	-cpu Skylake-Client \
	-drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on \
	-drive format=raw,file=fat:rw:mnt \
	-vga std \
	-monitor telnet:0.0.0.0:1234,server,nowait \
	-no-reboot \
	-m 2048M \

.PHONY: kernel/target/x86_64-hinataos/debug/kernel.elf
kernel/target/x86_64-hinataos/debug/kernel.elf:
	cd kernel && cargo build && cd .. \

.PHONY: bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi:
	cd bootloader && cargo build && cd .. \

.PHONY: release
release: build
	cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf &&\
	cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI

.PHONY: run
run: build
	cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf &&\
	cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI &&\
	qemu-system-x86_64 \
		$(QEMU_ARGS) \
		-serial stdio \

.PHONY: debug
debug: build
	qemu-system-x86_64 \
		$(QEMU_ARGS) \
		-s -S \

.PHONY: build
build: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi