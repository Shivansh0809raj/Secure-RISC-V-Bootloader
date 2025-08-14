# Define paths and tools
TARGET = riscv64gc-unknown-none-elf
OBJDUMP = riscv64-unknown-elf-objdump
OBJCOPY = riscv64-unknown-elf-objcopy
AS = riscv64-unknown-elf-as
LD = riscv64-unknown-elf-ld
QEMU = qemu-system-riscv64

OUT_DIR = target/$(TARGET)/release
STATICLIB = $(OUT_DIR)/libcdac_boot.a

BUILD = build
BOOT_O = $(BUILD)/boot.o
ELF = $(BUILD)/cdac_boot.elf
BIN = $(BUILD)/cdac_boot.bin

.PHONY: all clean run-nobios check-tftp

all: $(BIN)

$(BUILD):
	mkdir -p $(BUILD)

# 1. Compile Rust code (but skip linking)
$(STATICLIB):
	cargo build --release --target $(TARGET) --verbose

# 2. Assemble boot.s
$(BOOT_O): src/boot.s | $(BUILD)
	$(AS) -march=rv64gc -o $@ $< -v

# 3. Link everything manually
$(ELF): $(BOOT_O) $(STATICLIB)
	$(LD) -T bootloader.ld -o $@ $(BOOT_O) -L $(OUT_DIR) -lcdac_boot -v

# 4. Generate flat binary
$(BIN): $(ELF)
	$(OBJCOPY) -O binary $< $@

check-tftp:
	@echo "Checking tftpd-hpa service..."
	@if ! systemctl is-active --quiet tftpd-hpa; then \
		echo "tftpd-hpa service is not running. Starting it..."; \
		sudo systemctl start tftpd-hpa; \
		if ! systemctl is-active --quiet tftpd-hpa; then \
			echo "Failed to start tftpd-hpa. Check 'systemctl status tftpd-hpa.service' and 'journalctl -xeu tftpd-hpa.service'"; \
			exit 1; \
		fi; \
	fi
	@echo "tftpd-hpa is running."

run-nobios: $(BIN) check-tftp
	$(QEMU) -M virt -m 512M -nographic -serial file:uart.log \
                -kernel build/cdac_boot.elf -bios none -monitor none -d int 2> qemu.log \
                -device virtio-net-device,netdev=net0 \
                -netdev user,id=net0,hostfwd=udp:127.0.0.1:6970-192.168.1.1:69

clean:
	cargo clean
	rm -rf build uart.log qemu.log