
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use lzos::println;
use bootloader::{BootInfo, entry_point};
use alloc::boxed::Box;
use lzos::task::{Task, simple_executor::SimpleExecutor};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use lzos::allocator;
    use lzos::memory::{self, BootInfoFrameAllocator};
    use x86_64::{VirtAddr, structures::paging::Translate};

    println!("Hello World{}\n ok", "!");

    lzos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    println!("Id did not crash!");
    lzos::hlt_loop()
}

async fn async_number () -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    lzos::hlt_loop();
}
