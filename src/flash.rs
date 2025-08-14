use crate::println;

pub struct Flash;

static mut FLASH_STORAGE: [u8; 4096] = [0; 4096]; // Store OTA data

impl Flash {
    pub fn write_kernel(data: &[u8], address: u64) -> bool {
        println!("Writing {} bytes to flash at 0x{:x}...", data.len(), address);
        if address == 0x9000_0000 && data.len() <= 4096 {
            unsafe {
                for (i, &byte) in data.iter().enumerate() {
                    FLASH_STORAGE[i] = byte;
                }
            }
            println!("Flash write successful!");
            true
        } else {
            println!("Invalid flash address or data size!");
            false
        }
    }

    pub fn read_kernel(address: u64, size: usize) -> &'static [u8] {
        println!("Reading {} bytes from flash at 0x{:x}...", size, address);
        static mut DATA: [u8; 4096] = [0; 4096];
        unsafe {
            if address == 0x8000_0000 {
                for i in 0..size.min(4096) {
                    DATA[i] = (i % 256) as u8;
                }
                println!("Simulated kernel data: first 16 bytes {:?}", &DATA[..16.min(size)]);
                core::slice::from_raw_parts(DATA.as_ptr(), size.min(4096))
            } else if address == 0x9000_0000 {
                println!("Reading OTA data: first 16 bytes {:?}", &FLASH_STORAGE[..16.min(size)]);
                core::slice::from_raw_parts(FLASH_STORAGE.as_ptr(), size.min(4096))
            } else {
                println!("Non-kernel address, returning zeros");
                for i in 0..size.min(4096) {
                    DATA[i] = 0;
                }
                core::slice::from_raw_parts(DATA.as_ptr(), size.min(4096))
            }
        }
    }
}
