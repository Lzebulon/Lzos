use super::sbi::{sbi_call_2, sbi_call_4, sbi_call_5, Sbiret};

const RFENCE_EXTENSION: i32 = 0x52464E43; // "RFNC"

/// Instructs remote harts to execute `FENCE.I` instruction
fn sbi_remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> Result<usize, Sbiret> {
    unsafe { sbi_call_2(RFENCE_EXTENSION, 0, hart_mask, hart_mask_base) }
}

/// Instructs the remote harts to execute one or more **SFENCE.VMA** instructions,
/// covering the range of
/// virtual addresses between **start_addr** and **start_addr + size**
fn sbi_remote_sfence_vma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_4(
            RFENCE_EXTENSION,
            1,
            hart_mask,
            hart_mask_base,
            start_addr,
            size,
        )
    }
}

/// Instruct the remote harts to execute one or more **SFENCE.VMA** instructions,
/// covering the range of
/// virtual addresses between start_addr and **start_addr + size**.
/// This covers only the given **ASID**
fn sbi_remote_sfence_vma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_5(
            RFENCE_EXTENSION,
            2,
            hart_mask,
            hart_mask_base,
            start_addr,
            size,
            asid,
        )
    }
}

/// Instruct the remote harts to execute one or more **HFENCE.GVMA** instructions,
/// covering the range of
/// guest physical addresses between **start_addr** and **start_addr + size** only for
/// the given **VMID**.
/// This function call is only valid for harts implementing hypervisor extension.
fn sbi_remote_hfence_gvma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_5(
            RFENCE_EXTENSION,
            3,
            hart_mask,
            hart_mask_base,
            start_addr,
            size,
            asid,
        )
    }
}

/// Instruct the remote harts to execute one or more **HFENCE.GVMA** instructions,
/// covering the range of
/// guest physical addresses between **start_addr** and **start_addr + size**
/// for all the guests. This
/// function call is only valid for harts implementing hypervisor extension.
fn sbi_remote_hfence_gvma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> Result<usize, Sbiret> {
    unsafe {
        sbi_call_4(
            RFENCE_EXTENSION,
            4,
            hart_mask,
            hart_mask_base,
            start_addr,
            size,
        )
    }
}
