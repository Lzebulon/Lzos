use super::sbi::{sbi_call_1, sbi_call_3, Sbiret};

const DEBUG_CONSOLE_EXTENSION: i32 = 0x4442434E; // "DBCN"

/// Write bytes to the debug console from input memory.
/// The `num_bytes` parameter specifies the number of bytes in the input memory.
/// The physical base
/// address of the input memory is represented by two XLEN bits wide parameters.
/// The `base_addr_lo`
/// parameter specifies the lower XLEN bits and the `base_addr_hi` parameter
/// specifies the upper
/// XLEN bits of the input memory physical base address.
/// This is a non-blocking SBI call and it may do partial/no writes if the
/// debug console is not able to
/// accept more bytes.
pub unsafe fn sbi_debug_console_write(
    num_bytes: usize,
    base_addr_lo: usize,
    base_addr_hi: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_3(
            DEBUG_CONSOLE_EXTENSION,
            0,
            num_bytes,
            base_addr_lo,
            base_addr_hi,
        )
    }
}

pub fn safe_debug_console_write(buffer: &[u8]) -> Result<usize, Sbiret> {
    let n = buffer.len();

    let base_addr = buffer.as_ptr() as usize;
    let base_addr_lo = base_addr & 0xFFFF_FFFF;
    let base_addr_hi = (base_addr >> 32) & 0xFFFF_FFFF;

    unsafe { sbi_debug_console_write(n, base_addr_lo, base_addr_hi) }
}

/// Read bytes from the debug console into an output memory.
/// The `num_bytes` parameter specifies the maximum number of bytes which can be
/// written into the
/// output memory. The physical base address of the output memory is represented
/// by two XLEN bits
/// wide parameters. The `base_addr_lo` parameter specifies the lower XLEN bits
/// and the
/// `base_addr_hi` parameter specifies the upper XLEN bits of the output memory
/// physical base
/// address.
/// This is a non-blocking SBI call and it will not write anything into the
/// output memory if there are no
/// bytes to be read in the debug console
unsafe fn sbi_debug_console_read(
    num_bytes: usize,
    base_addr_lo: usize,
    base_addr_hi: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_3(
            DEBUG_CONSOLE_EXTENSION,
            1,
            num_bytes,
            base_addr_lo,
            base_addr_hi,
        )
    }
}

pub fn safe_debug_console_read(buffer: &mut [u8]) -> Result<usize, Sbiret> {
    let n = buffer.len();

    let base_addr = buffer.as_mut_ptr() as usize;
    let base_addr_lo = base_addr & 0xFFFF_FFFF;
    let base_addr_hi = (base_addr >> 32) & 0xFFFF_FFFF;

    unsafe { sbi_debug_console_read(n, base_addr_lo, base_addr_hi) }
}

/// Write a single byte to the debug console.
/// This is a blocking SBI call and it will only return after writing the
/// specified byte to the debug console.
/// It will also return, with `SBI_ERR_FAILED`, if there are I/O errors.
pub fn sbi_debug_console_write_byte(byte: u8) -> Result<usize, Sbiret> {
    unsafe { sbi_call_1(DEBUG_CONSOLE_EXTENSION, 2, byte as usize) }
}
