#[repr(u64)]
#[allow(non_camel_case_types)]
enum HSMState {
    /// The hart is physically powered-up and executing normally
    STARTED = 0,
    /// The hart is not executing in supervisor-mode or any lower privilege
    /// mode. It is probably powered-down by the SBI implementation if the
    /// underlying platform has a mechanism to physically power-down harts
    STOPPED = 1,
    /// Some other hart has requested to start (or power-up) the hart from the
    /// `STOPPED` state and the SBI implementation is still working to get the
    /// hart in the `STARTED` state
    START_PENDING = 2,
    /// The hart has requested to stop (or power-down) itself from the `STARTED`
    /// state and the SBI implementation is still working to get the hart in the
    /// `STOPPED` state.
    STOP_PENDING = 3,
    /// This hart is in a platform specific suspend (or low power) state
    SUSPENDED = 4,
    /// The hart has requested to put itself in a platform specific low power state
    /// from the `STARTED` state and the SBI implementation is still working to
    /// get the hart in the platform specific `SUSPENDED` state
    SUSPENDED_PENDING = 5,
    /// An interrupt or platform specific hardware event has caused the hart to
    /// resume normal execution from the `SUSPENDED` state and the SBI
    /// implementation is still working to get the hart in the `STARTED` state
    RESUME_PENDING = 6,
}
