cdacBOOT: Secure Bootloader for RISC-V

cdacBOOT is a secure bootloader designed for the RISC-V architecture (riscv64gc-unknown-none-elf), running on QEMU’s virt machine. It performs secure kernel loading, ECDSA signature verification, Over-The-Air (OTA) updates via a simulated TFTP client, and UART-based debugging output. The bootloader is written in Rust in a no_std environment, with a small assembly entry point, and ensures that only trusted kernel images are executed.
Features

Secure Boot: Loads a kernel from a simulated flash at 0x80000000 and verifies its integrity using ECDSA signatures (NIST P-256 curve) with SHA-256 hashing.
OTA Updates: Fetches updates via a simulated TFTP client from 192.168.1.1:69, stores them in flash at 0x90000000, and verifies their signatures.
UART Output: Logs boot progress, verification results, and errors to uart.log via QEMU’s UART emulation.
No_std Environment: Built for bare-metal RISC-V, ensuring minimal dependencies and lightweight execution.
QEMU Compatibility: Runs on QEMU’s virt machine with virtio-net for TFTP simulation.

Prerequisites
Ensure the following tools and configurations are installed on a Linux environment (Ubuntu 20.04 or later recommended):

Rust with Nightly Toolchain:

Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Set nightly: rustup toolchain install nightly && rustup default nightly
Add RISC-V target: rustup target add riscv64gc-unknown-none-elf
Verify: rustc --version (should show nightly)


QEMU for RISC-V:

Install: sudo apt update && sudo apt install qemu-system-riscv64
Version: 7.0 or later
Verify: qemu-system-riscv64 --version


RISC-V GNU Toolchain:

Install: sudo apt install gcc-riscv64-unknown-elf binutils-riscv64-unknown-elf
Verify: riscv64-unknown-elf-gcc --version


TFTP Server (tftpd-hpa):

Install: sudo apt install tftpd-hpa
Configure: Create /tftpboot/kernel.bin with [1, 2, ..., 14] (e.g., echo -ne '\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e' > /tftpboot/kernel.bin)
Start: sudo systemctl start tftpd-hpa
Verify: sudo systemctl status tftpd-hpa


Make:

Install: sudo apt install make
Verify: make --version


Rust Dependencies (automatically fetched by cargo):

Bootloader: p256 = { version = "0.13", features = ["ecdsa"], default-features = false }, sha2 = { version = "0.10", default-features = false }
Keygen: p256 = "0.13", sha2 = "0.10", rand_core = { version = "0.6", features = ["std"] }


Optional Tools:

rustfmt: rustup component add rustfmt --toolchain nightly
xxd: sudo apt install xxd (for verifying kernel.bin)
tcpdump: sudo apt install tcpdump (for tftp.pcap)
libssl-dev: sudo apt install libssl-dev (for keygen, if needed)


File System Permissions:

Ensure /tftpboot is writable: sudo chmod -R 777 /tftpboot
Verify write access to the project directory for uart.log, qemu.log, etc.



Setup Instructions

Clone the Repository:
git clone <repository-url>
cd cdacBOOT


Generate Keys and Signatures:

Navigate to the keygen directory:cd keygen
cargo build --release --target x86_64-unknown-linux-gnu
cargo run --release --target x86_64-unknown-linux-gnu


Copy the output (PUBLIC_KEY, KERNEL_SIGNATURE, UPDATE_SIGNATURE) to src/lib.rs.


Configure TFTP Server:

Create /tftpboot/kernel.bin:sudo mkdir -p /tftpboot
echo -ne '\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e' | sudo tee /tftpboot/kernel.bin
sudo chmod 777 /tftpboot/kernel.bin


Start the TFTP server:sudo systemctl start tftpd-hpa





Build Instructions

Clean Previous Builds:
cargo clean
make clean


Build the Bootloader:
make all


Compiles Rust code to target/riscv64gc-unknown-none-elf/release/libcdac_boot.a.
Assembles boot.s to build/boot.o.
Links into build/cdac_boot.elf using bootloader.ld.



Run Instructions
Run the bootloader in QEMU:
qemu-system-riscv64 -M virt -m 512M -nographic -serial file:uart.log \
    -kernel build/cdac_boot.elf -bios none -monitor none -d int,guest_errors 2> qemu.log \
    -device virtio-net-device,netdev=net0 \
    -netdev user,id=net0,hostfwd=udp:127.0.0.1:6970-192.168.1.1:69


Outputs:
uart.log: Contains boot logs, including ASCII banner, kernel/OTA hashes, and verification results.
qemu.log: Contains QEMU debug information.



Key Technologies

Rust: no_std environment for bare-metal RISC-V.
QEMU: Emulates the RISC-V virt machine with UART and virtio-net.
ECDSA (p256): Verifies kernel and OTA update authenticity.
SHA-256 (sha2): Computes hashes for verification.
TFTP: Simulated client for OTA updates, supported by QEMU’s network emulation.
RISC-V Assembly: Entry point in boot.s.
Linker Script: bootloader.ld defines memory layout.

Debugging

UART Logs: Check uart.log for boot progress and errors.
QEMU Logs: Check qemu.log for execution details (-d int,guest_errors).
Disassembly: Generate with riscv64-unknown-elf-objdump -d build/cdac_boot.elf > disassembly.txt.
TFTP Debugging: Use tcpdump to capture traffic (tftp.pcap) or verify /tftpboot/kernel.bin with xxd.

Notes

The TFTP client is simulated, returning [1, 2, ..., 14] for kernel.bin. A real TFTP client requires a virtio-net driver and UDP stack.
The OTA update is verified but not applied to replace the kernel in this version.
Ensure src/lib.rs uses the latest version (artifact_id: 2c870a0a-5de2-4f1e-abb1-20aa71ace43d) to avoid duplicate “Flash write successful!” messages.

Contributing
Contributions are welcome! Please submit issues or pull requests to the repository. Ensure code adheres to rustfmt (see rustfmt.toml).
