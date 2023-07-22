kernel/target/x86_64-hinataos/debug/kernel.elf:
	cd kernel && cargo build && cd .. \
	cp kernel/target/x86_64-hinataos/debug/kernel.elf mnt/kernel.elf

bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi:
	cd bootloader && cargo build && cd .. \
	cp bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI

run: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootx64.efi
	qemu-system-x86_64 \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF_CODE.fd,format=raw,readonly=on \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF_VARS.fd,format=raw \
		-drive format=raw,file=fat:rw:mnt \
		-serial stdio \
		-no-reboot \

run-gdb: kernel/target/x86_64-hinataos/debug/kernel.elf bootloader/target/x86_64-unknown-uefi/debug/bootx64.efi
	qemu-system-x86_64 \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF_CODE.fd,format=raw,readonly=on \
		-drive if=pflash,file=thirdparty/RELEASEX64_OVMF_VARS.fd,format=raw \
		-drive format=raw,file=fat:rw:mnt \
		-serial stdio \
		-no-reboot \
		-s -S

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