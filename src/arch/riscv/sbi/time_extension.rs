use super::sbi::{sbi_call_1, Sbiret};

const TIMER_EXTENSION: i32 = 0x54494D45; // "TIME"

/// Programs the clock for next event after `stime_value` time. `stime_value`
/// is in absolute time.
/// If the supervisor wishes to clear the timer interrupt without
/// scheduling the next timer event, it may
/// request a timer interrupt infinitely far into the future
/// (i.e., (uint64_t)-1). Alternatively, to not receive
/// timer interrupts, it may mask timer interrupts by clearing the `sie.STIE`
/// CSR bit.
/// This function must clear the pending timer interrupt bit when stime_value
/// is set to some time in the
/// future, regardless of whether timer interrupts are masked or not.
/// This function always returns SBI_SUCCESS in sbiret.error
pub fn sbi_set_timer(stime_value: u64) -> Result<usize, Sbiret> {
    unsafe { sbi_call_1(TIMER_EXTENSION, 0, stime_value.try_into().unwrap()) }
}
