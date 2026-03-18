mod allocator;
mod page;

/// Align an address to the next alignment (if the address is already
/// align, the address is return)
fn align_up(addr: usize, align: usize) -> usize {
    let reminder = addr % align;
    if reminder == 0 {
        addr
    } else {
        addr - reminder + align
    }
}

/// Align an address to the previous alignment (if the address is already
/// align, the address is return)
fn align_down(addr: usize, align: usize) -> usize {
    addr - (addr % align)
}
