//! Small shell to interract with the kernel

use crate::{arch::riscv::sbi::debug_console_extension::safe_debug_console_read, printkln};

struct Shell;

impl Shell {
    fn new() -> Self {
        Self
    }

    fn run(&mut self) {
        loop {
            let mut buffer = [0; 256];

            if let Err(err) = safe_debug_console_read(&mut buffer) {
                printkln!("Error while try to read for shell {err:?}");
            }
        }
    }
}
