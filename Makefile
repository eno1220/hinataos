.PHONY: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi

kernel/target/x86_64-hinataos/debug/kernel.elf:
	cd kernel && cargo build && cd .. \

bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi:
	cd bootloader && cargo build && cd .. \

debug: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
	cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf &&\
	cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI

run: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
	cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf &&\
	cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI &&\
	qemu-system-x86_64 \
		-cpu Skylake-Client \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on \
		-drive format=raw,file=fat:rw:mnt \
		-monitor telnet:0.0.0.0:1234,server,nowait \
		-vga std \
		-d int \
		-m 4G \
		-serial stdio \
		-no-reboot \

run-gdb: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
	qemu-system-x86_64 \
		-cpu Skylake-Client \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on \
		-drive format=raw,file=fat:rw:mnt \
		-serial stdio \
		-vga std \
		-no-reboot \
		-s -S

run-monitor: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
	qemu-system-x86_64 \
		-cpu Skylake-Client \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on \
		-drive format=raw,file=fat:rw:mnt \
		-vga std \
		-monitor telnet:0.0.0.0:1234,server,nowait \
		-no-reboot \

build: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi

clean:
	cd kernel && cargo clean && cd .. \
	cd bootloader && cargo clean && cd .. \
	rm -rf mnt/kernel.elf mnt/EFI/BOOT/BOOTX64.EFI

clippy:
	cd kernel && cargo clippy && cd .. \
	cd bootloader && cargo clippy && cd ..

fmt:
	cd kernel && cargo fmt && cd .. \
	cd bootloader && cargo fmt && cd ..