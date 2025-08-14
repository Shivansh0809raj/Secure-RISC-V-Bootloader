use core::fmt::{self, Write};

const UART_BASE: *mut u8 = 0x1000_0000 as *mut u8;

pub fn console_init() {}

pub fn putchar(c: char) {
    unsafe {
        core::ptr::write_volatile(UART_BASE, c as u8);
    }
}

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::console::putchar('\n');
    };
    ($($arg:tt)*) => {{
        let mut console = $crate::console::Console;
        let _ = core::fmt::write(&mut console, format_args!($($arg)*));
        $crate::console::putchar('\n');
    }};
}
