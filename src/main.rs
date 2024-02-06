#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]
#![warn(clippy::pedantic)]

use core::{arch::global_asm, panic::PanicInfo};

extern crate alloc;
use alloc::vec;
use vga::VGAOut;

use crate::{interrupts::init_idt, serial::init_serial, multiboot2::MultibootHeader};

mod allocator;
mod interrupts;
mod multiboot2;
mod serial;
mod vga;

// Bootloader
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("arch/x86_64/boot.s"), options(att_syntax));

/// First rust function call
/// This is the entry point of your kernel after that the assembly code
/// switch long mode
#[no_mangle]
extern "C" fn kernel_main(multiboot_info_address: usize) -> ! {
    init();

    printkln!("Hello from lzos");

    #[cfg(test)]
    test_main();

    let multiboot = unsafe {
       MultibootHeader::load(multiboot_info_address)
    };

    unsafe {
        let size = *(multiboot_info_address as *const usize);
        printkln!("{}", size);
    }

    printkln!("multiboot_info pointer : 0x{multiboot_info_address:x}\n {:?}", multiboot);
    printkln!("0x{:x}", multiboot.magic);
    let v = vec![5, 7, 100, 10];

    for i in v {
        printkln!("{i}");
    }

    loop {}
}


/// Initialize all component of the kernel
fn init() {
    // init vga, for print in VGA buffer with printkln
    VGAOut::init();

    init_serial();

    init_idt();
}


/// panic function
/// Print panic info and entry in infinity loop
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    printkln!("{panic_info}");
    loop {}
}


/// Run test
/// [not_implemented]
#[cfg(test)]
pub fn test_runner(_tests : &[&dyn Fn()]) {
    unimplemented!();
}
