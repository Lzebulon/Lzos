use core::{
    arch::asm,
    intrinsics::{self, size_of},
};

const STACK_SIZE: usize = 4096;
pub(crate) const MAX_PROCS: usize = 32;

static mut PROCS: [Proc; MAX_PROCS] = [Proc {
    id: 0,
    func: || 0,
    sp: 0,
    registers: SavedRegs {
        ra: 0,
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
        s4: 0,
        s5: 0,
        s6: 0,
        s7: 0,
        s8: 0,
        s9: 0,
        s10: 0,
        s11: 0,
    },
}; 32];

#[derive(Debug, Default, Clone, Copy)]
struct SavedRegs {
    ra: usize,
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
}

#[derive(Clone, Copy)]
pub struct Proc {
    id: usize,
    // user
    // right
    func: fn() -> usize,
    sp: usize,
    registers: SavedRegs,
}

impl Proc {
    fn spawn(f: fn() -> usize) -> Self {
        Self {
            id: 0,
            func: f,
            sp: 0,
            registers: SavedRegs::default(),
        }
    }
}

pub fn switch_context_proc(p1: Proc, p2: Proc) {}

pub fn switch_context(prev_sp: usize, next_sp: usize) {
    unsafe {
        asm!(
            "addi sp, sp, -13 * 8",
            "sw ra,  0  * 8(sp)",
            "sw s0,  1  * 8(sp)",
            "sw s1,  2  * 8(sp)",
            "sw s2,  3  * 8(sp)",
            "sw s3,  4  * 8(sp)",
            "sw s4,  5  * 8(sp)",
            "sw s5,  6  * 8(sp)",
            "sw s6,  7  * 8(sp)",
            "sw s7,  8  * 8(sp)",
            "sw s8,  9  * 8(sp)",
            "sw s9,  10 * 8(sp)",
            "sw s10, 11 * 8(sp)",
            "sw s11, 12 * 8(sp)",
            "sw sp, (a0)",
            "lw sp, (a1)",
            "lw ra,  0  * 8(sp)",
            "lw s0,  1  * 8(sp)",
            "lw s1,  2  * 8(sp)",
            "lw s2,  3  * 8(sp)",
            "lw s3,  4  * 8(sp)",
            "lw s4,  5  * 8(sp)",
            "lw s5,  6  * 8(sp)",
            "lw s6,  7  * 8(sp)",
            "lw s7,  8  * 8(sp)",
            "lw s8,  9  * 8(sp)",
            "lw s9,  10 * 8(sp)",
            "lw s10, 11 * 8(sp)",
            "lw s11, 12 * 8(sp)",
            "addi sp, sp, 13 * 8",
            "ret",
        );
    }
}
