use super::sbi::{sbi_call_0, sbi_call_1, Sbiret};

const EXTENSION_ID: i32 = 0x10;

/// Return: Returns the current SBI specification version.
/// This function must always succeed. The minor number
/// of the SBI specification is encoded in the low 24 bits,
/// with the major number encoded in the next 7
/// bits. Bit 31 must be 0 and is reserved for future expansion
fn sbi_get_spec_version() -> Result<usize, Sbiret> {
    unsafe { sbi_call_0(EXTENSION_ID, 0) }
}

/// Returns the current SBI implementation ID, which is different for every
/// SBI implementation. It is
/// intended that this implementation ID allows software to probe for
/// SBI implementation quirks.
fn sbi_get_impl_id() -> Result<usize, Sbiret> {
    unsafe { sbi_call_0(EXTENSION_ID, 1) }
}

/// Returns the current SBI implementation version.
/// The encoding of this version number is specific to
/// the SBI implementation
fn sbi_get_impl_version() -> Result<usize, Sbiret> {
    unsafe { sbi_call_0(EXTENSION_ID, 2) }
}
/// Returns 0 if the given SBI extension ID (EID) is not available,
/// or 1 if it is available unless defined as
/// any other non-zero value by the implementation
fn sbi_probe_extension(extension_id: usize) -> Result<usize, Sbiret> {
    unsafe { sbi_call_1(EXTENSION_ID, 3, extension_id) }
}
/// Return a value that is legal for the **mvendorid** CSR and 0 is always
/// a legal value for this CSR
fn sbi_get_mvendorid() -> Result<usize, Sbiret> {
    unsafe { sbi_call_0(EXTENSION_ID, 4) }
}

/// Return a value that is legal for the **marchid** CSR and 0 is always a legal
/// value for this CSR.
fn sbi_get_mimpid() -> Result<usize, Sbiret> {
    unsafe { sbi_call_0(EXTENSION_ID, 5) }
}

#[derive(Debug)]
#[repr(usize)]
enum SbiImplementationIDs {
    BBL = 0,
    OpenSBI = 1,
    Xvisor = 2,
    KVM = 3,
    RustSBI = 4,
    Diosix = 5,
    Coffer = 6,
    XenProject = 7,
    PolarFire = 8,
    Coreboot = 9,
    Oreboot = 10,
    Bhyve = 11,
}
