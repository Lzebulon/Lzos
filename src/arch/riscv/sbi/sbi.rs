use core::arch::asm;
#[allow(dead_code)]

/// This file defined function to call RISC-V sbi

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(non_camel_case_types)]
/// Standards SBI errors
pub enum SbiErrors {
    /// Completed successfully
    SBI_SUCCESS = 0,
    /// Failed
    SBI_ERR_FAILED = -1,
    /// Not supported
    SBI_ERR_NOT_SUPPORTED = -2,
    /// Invalid parameter(s)
    SBI_ERR_INVALID_PARAM = -3,
    /// Denied or not allowed
    SBI_ERR_DENIED = -4,
    /// Invalid address(s)
    SBI_ERR_INVALID_ADDRESS = -5,
    /// Already available
    SBI_ERR_ALREADY_AVAILABLE = -6,
    /// Already started
    SBI_ERR_ALREADY_STARTED = -7,
    /// Already stopped
    SBI_ERR_ALREADY_STOPPED = -8,
    /// Shared memory not available
    SBI_ERR_NO_SHMEM = -9,
    /// Invalid state
    SBI_ERR_INVALID_STATE = -10,
    /// Bad (or invalid) range
    SBI_ERR_BAD_RANGE = -11,
    /// Failed due to timeout
    SBI_ERR_TIMEOUT = -12,
    /// Input/Output error
    SBI_ERR_IO = -13,
}

impl SbiErrors {
    fn new(x: isize) -> Self {
        match x {
            0 => Self::SBI_SUCCESS,
            -1 => Self::SBI_ERR_FAILED,
            -2 => Self::SBI_ERR_NOT_SUPPORTED,
            -3 => Self::SBI_ERR_INVALID_PARAM,
            -4 => Self::SBI_ERR_DENIED,
            -5 => Self::SBI_ERR_INVALID_ADDRESS,
            -6 => Self::SBI_ERR_ALREADY_AVAILABLE,
            -7 => Self::SBI_ERR_ALREADY_STARTED,
            -8 => Self::SBI_ERR_ALREADY_STOPPED,
            -9 => Self::SBI_ERR_NO_SHMEM,
            -10 => Self::SBI_ERR_INVALID_STATE,
            -11 => Self::SBI_ERR_BAD_RANGE,
            -12 => Self::SBI_ERR_TIMEOUT,
            -13 => Self::SBI_ERR_IO,
            _ => panic!("error"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Sbiret {
    error: SbiErrors,
    value: usize,
}

impl Sbiret {
    fn new(error: SbiErrors, value: usize) -> Self {
        Self { error, value }
    }

    fn new_from_asm(error: isize, value: usize) -> Self {
        Sbiret::new(SbiErrors::new(error), value)
    }

    fn is_success(self) -> bool {
        self.error == SbiErrors::SBI_SUCCESS
    }
    fn is_error(self) -> bool {
        self.error != SbiErrors::SBI_SUCCESS
    }
}

/// Call sbi function that have no argument
pub unsafe fn sbi_call_0(extension_id: i32, function_id: i32) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        in("a6") function_id,
        in("a7") extension_id,
        lateout("a0") error,
        lateout("a1") value,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

/// Call sbi function that have 1 argument
pub unsafe fn sbi_call_1(
    extension_id: i32,
    function_id: i32,
    arg0: usize,
) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        inlateout("a0") arg0 => error ,
        lateout("a1") value,
        in("a6") function_id,
        in("a7") extension_id,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

/// Call sbi function that have 2 argument
pub unsafe fn sbi_call_2(
    extension_id: i32,
    function_id: i32,
    arg0: usize,
    arg1: usize,
) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        inlateout("a0") arg0 => error ,
        inlateout("a1") arg1 => value,
        in("a6") function_id,
        in("a7") extension_id,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

/// Call sbi function that have 3 argument
pub unsafe fn sbi_call_3(
    extension_id: i32,
    function_id: i32,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        inlateout("a0") arg0 => error ,
        inlateout("a1") arg1 => value,
        in("a6") function_id,
        in("a7") extension_id,
        in("a2") arg2,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

/// Call sbi function that have 4 argument
pub unsafe fn sbi_call_4(
    extension_id: i32,
    function_id: i32,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        inlateout("a0") arg0 => error ,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
        in("a3") arg3,
        in("a6") function_id,
        in("a7") extension_id,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

/// Call sbi function that have 5 argument
pub unsafe fn sbi_call_5(
    extension_id: i32,
    function_id: i32,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Sbiret> {
    let error: isize;
    let value: usize;

    asm!(
        "ecall",
        inlateout("a0") arg0 => error ,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a6") function_id,
        in("a7") extension_id,
    );

    match error {
        0 => Ok(value),
        _ => Err(Sbiret::new_from_asm(error, value)),
    }
}

// TODO: macro
// with argument number take the right sbi
