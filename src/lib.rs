#![no_std]

mod console;
mod ecdsa;
mod tftp;
mod flash;

use core::panic::PanicInfo;
use sha2::{Digest, Sha256};

const KERNEL_ADDRESS: u64 = 0x8000_0000;
const FLASH_ADDRESS: u64 = 0x9000_0000;

// Signatures from keygen output
pub const PUBLIC_KEY: [u8; 65] = [
    4, 78, 247, 2, 184, 155, 160, 196, 240, 99, 122, 222, 65, 200, 72, 210,
    159, 91, 54, 78, 8, 127, 78, 21, 56, 253, 84, 73, 158, 145, 241, 4, 33,
    160, 254, 241, 244, 32, 0, 6, 56, 143, 159, 146, 38, 27, 127, 137, 75,
    212, 209, 59, 146, 117, 231, 44, 173, 158, 195, 210, 96, 96, 19, 9, 108
];
pub const KERNEL_SIGNATURE: [u8; 71] = [
    48, 69, 2, 32, 56, 143, 56, 252, 168, 139, 227, 183, 125, 57, 186, 49,
    96, 48, 175, 102, 43, 125, 35, 38, 44, 123, 58, 168, 48, 224, 101, 231,
    101, 53, 16, 110, 2, 33, 0, 223, 79, 147, 20, 195, 2, 24, 9, 76, 215,
    122, 0, 92, 150, 232, 159, 5, 228, 177, 25, 31, 249, 160, 162, 112, 210,
    255, 21, 215, 104, 169, 191
];
pub const UPDATE_SIGNATURE: [u8; 71] = [
    48, 69, 2, 33, 0, 175, 187, 251, 139, 131, 15, 30, 115, 20, 112, 153,
    210, 151, 187, 220, 36, 241, 37, 127, 51, 156, 177, 94, 4, 131, 138, 174,
    143, 22, 206, 69, 78, 2, 32, 1, 91, 99, 28, 94, 84, 128, 233, 32, 77,
    182, 132, 242, 130, 90, 63, 56, 124, 255, 12, 210, 28, 210, 90, 184, 159,
    177, 226, 56, 143, 18, 62
];

#[no_mangle]
#[inline(never)]
pub extern "C" fn loader_init() -> ! {
    console::console_init();
    console::putchar('B');
    console::putchar('C');
    console::putchar('D');

    println!();
    println!("   ____  ____    _    ____ ");
    println!("  / ___||  _ \\  / \\  /  _ \\");
    println!(" | |    | | | |/ _ \\ | |     ");
    println!(" | |___ | |_| / ___ \\| |_ //  ");
    println!("  \\____||____/_/   \\_\\____/ ");
    println!();
    println!("cdacBOOT v0.1.0: Secure Booting on QEMU virt machine...");
    
    // Read kernel from flash
    let kernel_image = flash::Flash::read_kernel(KERNEL_ADDRESS, 4096);
    println!("Read kernel: first 16 bytes {:?}", &kernel_image[..16.min(kernel_image.len())]);
    if kernel_image.iter().all(|&b| b == 0) {
        println!("Failed to read kernel from flash!");
        loop {}
    }

    // Compute and display kernel hash
    let mut hasher = Sha256::new();
    hasher.update(&kernel_image);
    let kernel_hash = hasher.finalize();
    println!("Kernel SHA-256 hash: {:?}", kernel_hash.as_slice());

    // Verify kernel signature
    println!("Verifying kernel signature...");
    if !ecdsa::verify_signature(&kernel_hash, &KERNEL_SIGNATURE, &PUBLIC_KEY) {
        println!("Kernel signature verification failed!");
        loop {}
    }
    println!("Kernel signature verified successfully.");
    println!("Handing execution to the kernel... [OK]");

    // Fetch OTA update
    if let Some(update) = tftp::TftpClient::fetch_update("192.168.1.1", "kernel.bin") {
        println!("OTA update received, applying...");
        if flash::Flash::write_kernel(update, FLASH_ADDRESS) {
            println!("Flash write successful!");
            let new_kernel = flash::Flash::read_kernel(FLASH_ADDRESS, update.len());
            println!("Read new kernel: first 16 bytes {:?}", &new_kernel[..16.min(new_kernel.len())]);

            // Compute and display update hash
            let mut hasher = Sha256::new();
            hasher.update(&new_kernel);
            let update_hash = hasher.finalize();
            println!("New kernel SHA-256 hash: {:?}", update_hash.as_slice());

            // Verify update signature
            println!("Verifying new kernel signature...");
            if ecdsa::verify_signature(&update_hash, &UPDATE_SIGNATURE, &PUBLIC_KEY) {
                println!("New kernel signature verified successfully.");
            } else {
                println!("New kernel verification failed, keeping old kernel!");
            }
        } else {
            println!("Failed to apply OTA update!");
        }
    } else {
        println!("No OTA update available.");
    }

    println!("Welcome to cdacBOOT!");
    console::putchar('E');
    println!();
    println!("Booting kernel at 0x{:x}...", KERNEL_ADDRESS);
    println!("Kernel booted successfully!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    println!(
        "Panic: {}",
        info.message().as_str().unwrap_or("Unknown error")
    );
    loop {}
}
