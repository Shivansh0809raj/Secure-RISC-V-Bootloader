use crate::println;

pub struct TftpClient;

impl TftpClient {
    pub fn fetch_update(server_ip: &str, filename: &str) -> Option<&'static [u8]> {
        println!("Connecting to TFTP server at {}...", server_ip);
        println!("Requesting file {}...", filename);

        // Simulate TFTP fetch (mimic kernel.bin contents)
        static mut DATA: [u8; 4096] = [0; 4096];
        unsafe {
            println!("Simulating TFTP fetch of kernel.bin");
            // Simulate 14-byte kernel.bin data
            for i in 0..14 {
                DATA[i] = (i + 1) as u8; // [1, 2, ..., 14]
            }
            let size = 14;
            println!("Received {} bytes: {:?}", size, &DATA[..size]);
            if size == 0 {
                println!("Failed to fetch OTA update!");
                None
            } else {
                Some(core::slice::from_raw_parts(DATA.as_ptr(), size))
            }
        }
    }
}
