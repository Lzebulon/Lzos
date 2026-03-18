use crate::arch::riscv::sbi::debug_console_extension::sbi_debug_console_write_byte;
use core::fmt::{self};

#[cfg(target_arch = "x86_64")]
pub fn _printk(args: fmt::Arguments) {
    let mut vga_inner = VGA_OUT.borrow_mut();
    if let Some(vga) = &mut vga_inner.vga {
        vga.write_fmt(args).unwrap();
    }
}

// TODO: change RISC-V printer
pub struct RiscvDebugOut;

impl RiscvDebugOut {
    fn new() -> Self {
        Self
    }

    fn write_byte(self: &Self, byte: u8) {
        let _ = sbi_debug_console_write_byte(byte);
    }
}

impl core::fmt::Write for RiscvDebugOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | b'\r' => self.write_byte(byte),
                _ => self.write_byte(byte),
            }
        }
        Ok(())
    }
}

#[cfg(target_arch = "riscv64")]
pub fn _printk(args: fmt::Arguments) {
    use core::fmt::Write;

    let mut out = RiscvDebugOut::new();

    out.write_fmt(args).unwrap();
}

/// Prints on vga buffer.
#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {
        ($crate::utils::printer::_printk(format_args!($($arg)*)));
    };
}

/// Prints on vga buffer, with a newline.
#[macro_export]
macro_rules! printkln {
    () => ($crate::printk!("\n\r"));
    ($($arg:tt)*) => ($crate::printk!("{}\n\r", format_args!($($arg)*)));
}
