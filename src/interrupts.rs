use core::arch::asm;

#[repr(C)]
struct GateDescriptor {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    /// reserved + 2 bit ist
    attributes: u8,
    /// p (1) + dpl (2) + 0 + gate type (4)
    offset_middle: u16,
    offset_hight: u32,
    reserved: u32,
}

#[repr(u8)]
enum TypeDescriptor {
    InterruptGate = 0b1000_1110,
    TrapGate = 0b1000_1111,
}

#[derive(Default)]
#[repr(C)]
struct InterruptsDescriptorTable {
    division_error: usize,
    debug: usize,
    non_maskable_interrupt: usize,
    breakpoint: usize,
    overflow: usize,
    bound_range_exceeded: usize,
    invalid_opcode: usize,
    device_not_available: usize,
    double_fault: usize,
    _coprocessor_segment_overrun: usize,
    invalid_tss: usize,
    segment_not_present: usize,
    stack_segment_fault: usize,
    general_protection_fault: usize,
    page_fault: usize,
    reserved: usize,
    x87_floating_point_exception: usize,
    alignment_check: usize,
    machine_check: usize,
    simd_floating_point_exception: usize,
    virtualization_exception: usize,
    control_protection_exception: usize,
    vmm_communication_exception: usize,
    security_exception: usize,
    triple_fault: usize,
    _fpu_error_interrupt: usize,
}

impl InterruptsDescriptorTable {
    fn new() -> Self {
        Self::default()
    }

    fn load(&self) {
        unsafe { asm!("lidt ({})", in(reg) self, options(readonly, nostack, preserves_flags)) }
    }
}

pub fn init_idt() {
    let mut idt = InterruptsDescriptorTable::new();
}
