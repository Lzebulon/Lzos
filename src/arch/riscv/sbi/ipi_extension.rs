use super::sbi::{sbi_call_2, Sbiret};

const IPI_EXTENSION: i32 = 0x735049; // "sPI: s-mode IPI"

/// Send an inter-processor interrupt to all the harts defined in hart_mask.
/// Interprocessor interrupts
/// manifest at the receiving harts as the supervisor software interrupts
fn sbi_send_ipi(hart_mask: usize, hart_mask_base: usize) -> Result<usize, Sbiret> {
    unsafe { sbi_call_2(IPI_EXTENSION, 0, hart_mask, hart_mask_base) }
}
