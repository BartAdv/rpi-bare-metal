default: target/kernel.img

build: src/lib.rs
	xargo build --release --target arm-none-eabihf

target/boot.o: asm/boot.s
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c asm/boot.s -o target/boot.o

target/kernel.elf: build target/boot.o linker.ld
	arm-none-eabi-gcc -T linker.ld -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -ffreestanding -nostdlib target/boot.o target/arm-none-eabihf/release/librpi_bare_metal.rlib -o target/kernel.elf

target/kernel.img: target/kernel.elf
	arm-none-eabi-objcopy target/kernel.elf -O binary target/kernel.img

run-qemu: target/kernel.elf
	qemu-system-arm -M raspi2 -kernel target/kernel.elf -serial stdio

deploy: target/kernel.img
	mkdir -p /tmp/sd
	sudo mount -t vfat /dev/disk/by-uuid/8140-ADB7 /tmp/sd
	sudo cp target/kernel.img /tmp/sd/kernel.img
	sudo umount /tmp/sd