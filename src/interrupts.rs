use core::arch::asm;

use crate::{arch::riscv::sbi::time_extension::sbi_set_timer, printkln};

/// setup s mode interrupt in direct mode
///
/// The `interrupt_handler` in parameter **MUST** use
/// `extern "riscv-interrupt-s"` to be sure that `sret` (or equivalent)
/// instruction is use in return instruction. Without this the
/// interrupt handler will not work.
pub fn setup_s_mode_interrupt_direct(interrupt_handler: extern "riscv-interrupt-s" fn()) {
    unsafe {
        asm!(
            "csrw stvec, {f}",
            "csrsi sstatus, 2",
            f = in(reg) interrupt_handler
        )
    }
}

pub fn enable_s_mode_interrupt(code: usize) {
    unsafe {
        asm!(
            "csrs sie, {c}",
            c = in(reg) code
        )
    }
}

pub fn clear_pending_bit(code: usize) {
    unsafe {
        asm!(
            "csrc sip, {c}",
            c = in(reg) code
        )
    }
}

/// return time store in rdtime
pub fn get_time() -> u64 {
    let r;
    unsafe {
        asm!(
            "rdtime {ret}",
            ret = out(reg) r
        )
    };
    r
}

pub fn set_timer_later() {
    let t = get_time();

    if let Err(err) = sbi_set_timer(t + 10_000_000) {
        printkln!("error with set_timer_later : {err:?}");
    };
}

// NOTE: this function didn't use `ret` instruction to return
// but use `sret`
pub extern "riscv-interrupt-s" fn s_mode_direct_interrupt_handler() {
    clear_pending_bit(1 << 5);
    set_timer_later();
    printkln!("It's TIME !!");
}
