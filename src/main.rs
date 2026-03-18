// this feature is necessary to be able to use the two specials
// extern ABI attributes `riscv-interrupt-{m,s}` necessary to handle
// interrupts (like we can't suppose with the name)
#![feature(abi_riscv_interrupt)]
#![feature(core_intrinsics)]
#![feature(panic_internals)]
// we can't use std lib, because we didn't have yet an os
#![no_std]
// we jump from assembly so we didn't have any main function
#![no_main]
#![warn(clippy::pedantic)]

use core::{arch::global_asm, panic::PanicInfo, str::from_utf8};

extern crate alloc;
use alloc::vec;
use arch::riscv::sbi::debug_console_extension::safe_debug_console_read;
use interrupts::{
    enable_s_mode_interrupt, s_mode_direct_interrupt_handler, set_timer_later,
    setup_s_mode_interrupt_direct,
};
#[cfg(target_arch = "x86_64")]
use vga::VGAOut;

#[cfg(target_arch = "x86_64")]
use crate::{interrupts::init_idt, multiboot2::MultibootHeader, serial::init_serial};

mod arch;
mod interrupts;
mod memory;
mod multiboot2;
mod proc;
mod schelduler;
mod small_shell;
mod utils;
#[cfg(target_arch = "x86_64")]
mod vga;

// Bootloader
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("arch/x86_64/boot.s"), options(att_syntax));

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("arch/riscv/boot.S"));

/// First rust function call
/// This is the entry point of your kernel after that the assembly code
/// switch long mode
#[no_mangle]
extern "C" fn kernel_main() -> ! {
    init();

    printkln!("Hello from lzos");

    printkln!("5 7 100 10");
    let v = vec![5, 7, 100, 10];

    printkln!("finish alloc v");

    for i in v {
        printkln!("{i}");
    }

    loop {
        // test zone
        let mut buffer = [0; 10];
        match safe_debug_console_read(&mut buffer) {
            Ok(a) => {
                if a > 0 {
                    let s = from_utf8(&buffer).unwrap();
                    printkln!("read : {a} : {:?} : {s}", buffer);
                }
            }
            Err(e) => {
                printkln!("erreur when try to read : {e:?}");
            }
        };
    }
}

#[cfg(target_arch = "x86_64")]
/// Initialize all component of the kernel
fn init() {
    // init vga, for print in VGA buffer with printkln
    VGAOut::init();

    init_serial();

    init_idt();
}

#[cfg(target_arch = "riscv64")]
fn init() {
    setup_s_mode_interrupt_direct(s_mode_direct_interrupt_handler);

    set_timer_later();

    enable_s_mode_interrupt(1 << 5);
    printkln!("initialized");
}

/// panic function
/// Print panic info and entry in infinity loop
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    printkln!("{panic_info}");
    loop {}
}
