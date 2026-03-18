//! We define here the global allocator use by rust.
//! This global allocator is important to be able to use
//! data structure like Vec.

use core::{alloc::GlobalAlloc, cell::UnsafeCell, panic, ptr::null_mut, sync::atomic::AtomicUsize};

use crate::printkln;

/// Size of your kernel heap
const ARENA_SIZE: usize = 64 * 1024 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;

const ALLOC_ABORT: bool = false;

struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let align_mask = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            if ALLOC_ABORT {
                panic!("ALLOC : align > MAX_SUPPORTED_ALIGN")
            }
            return null_mut();
        }

        let mut allocated = 0;

        if self
            .remaining
            .fetch_update(
                core::sync::atomic::Ordering::SeqCst,
                core::sync::atomic::Ordering::SeqCst,
                |mut remaining| {
                    if size > remaining {
                        return None;
                    }
                    remaining -= size;
                    remaining &= align_mask;
                    allocated = remaining;
                    Some(remaining)
                },
            )
            .is_err()
        {
            if ALLOC_ABORT {
                panic!("ALLOC : size > remaining")
            }
            return null_mut();
        };

        self.arena.get().cast::<u8>().add(allocated)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        printkln!("Pourquoi dealloc ????");
    }
}

struct AllocCell {
    free: bool,
    start: *mut usize,
    size: *mut usize,
    arena: [u8],
}

struct ArrayAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
}

unsafe impl Sync for ArrayAllocator {}

unsafe impl GlobalAlloc for ArrayAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if self.remaining.load(core::sync::atomic::Ordering::SeqCst) < size {
            return null_mut();
        }
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
