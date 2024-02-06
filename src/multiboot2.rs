#![allow(dead_code)]

const MULTIBOOT_HEADER: usize = 1;

/*  How many bytes from the start of the file we search for the header. */
const MULTIBOOT_SEARCH: usize = 32768;
const MULTIBOOT_HEADER_ALIGN: usize = 8;

/*  The magic field should contain this. */
const MULTIBOOT2_HEADER_MAGIC: usize = 0xe85250d6;

/*  This should be in %eax. */
const MULTIBOOT2_BOOTLOADER_MAGIC: usize = 0x36d76289;

/*  Alignment of multiboot modules. */
const MULTIBOOT_MOD_ALIGN: usize = 0x00001000;

/*  Alignment of the multiboot info structure. */
const MULTIBOOT_INFO_ALIGN: usize = 0x00000008;

/*  Flags set in the ’flags’ member of the multiboot header. */

const MULTIBOOT_TAG_ALIGN: u8 = 8;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
enum TagType {
    End = 0,
    CmdLine = 1,
    BootLoaderName = 2,
    Module = 3,
    BasicMeminfo = 4,
    BootDev = 5,
    MMAP = 6,
    VBE = 7,
    Framebuffer = 8,
    ElfSections = 9,
    APM = 10,
    EFI32 = 11,
    EFI64 = 12,
    SMBIOS = 13,
    ACPIOld = 14,
    ACPINew = 15,
    Network = 16,
    EFI_MMAP = 17,
    EFI_BS = 18,
    EFI32_IH = 19,
    EFI64_IH = 20,
    LoadBaseAddr = 21,
}

#[derive(Debug)]
#[repr(u8)]
enum HeaderTag {
    End = 0,
    InformationRequest = 1,
    Address = 2,
    EntryAddress = 3,
    ConsoleFlags = 4,
    Framebuffer = 5,
    ModuleAlign = 6,
    EFIBS = 7,
    EntryAddressEFI32 = 8,
    EntryAddressEFI64 = 9,
    Relocatable = 10,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Architecture {
    I386 = 0,
    MIPS32 = 4,
}

const MULTIBOOT_HEADER_TAG_OPTIONAL: usize = 1;

#[derive(Debug)]
#[repr(u8)]
enum LoadPreference {
    NONE = 0,
    LOW = 1,
    HIGH = 2,
}

#[derive(Debug)]
#[repr(u8)]
enum ConsoleFlags {
    ConsoleRequired = 1,
    EgaTextSupported = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MultibootHeader {
    /*  Must be MULTIBOOT_MAGIC - see above. */
    pub magic: u32,

    /*  ISA */
    pub architecture: Architecture,

    /*  Total header length. */
    pub header_length: u32,

    /*  The above fields plus this one must equal 0 mod 2^32. */
    pub checksum: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTag {
    typ: HeaderTag,
    flags: u16,
    size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagInformationRequest {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    requests: [u32],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagAddress {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    header_addr: u32,
    load_addr: u32,
    load_end_addr: u32,
    bss_end_addr: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagEntryAddress {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    entry_addr: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagConsoleFlags {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    console_flags: ConsoleFlags,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagFramebuffer {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagModuleAlign {
    typ: HeaderTag,
    flags: u16,
    size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootHeaderTagRelocatable {
    typ: HeaderTag,
    flags: u16,
    size: u32,
    min_addr: u32,
    max_addr: u32,
    align: u32,
    preference: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MultibootColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
#[repr(u32)]
enum Memory {
    Available = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    Nvs = 4,
    Badram = 5,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootMmapEntry {
    addr: u64,
    len: u64,
    typ: u32,
    zero: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTag {
    typ: TagType,
    size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagString {
    typ: TagType,
    size: u32,
    string: &'static str,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagModule {
    typ: TagType,
    size: u32,
    mod_start: u32,
    mod_end: u32,
    cmdline: &'static str,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagBasicMeminfo {
    typ: TagType,
    size: u32,
    mem_lower: u32,
    mem_upper: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagBootdev {
    typ: TagType,
    size: u32,
    biosdev: u32,
    slice: u32,
    part: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagMmap {
    typ: TagType,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    entries: [MultibootMmapEntry],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootVbeInfoBlock {
    external_specification: [u8; 512],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootVbeModeInfoBlock {
    external_specification: [u8; 256],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagVbe {
    typ: TagType,
    size: u32,

    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,

    vbe_control_info: MultibootVbeInfoBlock,
    vbe_mode_info: MultibootVbeModeInfoBlock,
}

#[derive(Debug)]
#[repr(u8)]
enum FrameBufferType {
    Indexed = 0,
    RGB = 1,
    EGAText = 2,
}
#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagFramebufferCommon {
    typ: TagType,
    size: u32,

    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: FrameBufferType,
    reserved: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MultibootFramebufferColorPalette1 {
    framebuffer_palette_num_colors: u16,
    framebuffer_palette: MultibootColor, // [MultibootColor]
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MultibootFramebufferColorPalette2 {
    framebuffer_red_field_position: u8,
    framebuffer_red_mask_size: u8,
    framebuffer_green_field_position: u8,
    framebuffer_green_mask_size: u8,
    framebuffer_blue_field_position: u8,
    framebuffer_blue_mask_size: u8,
}

union MultibootTagFramebufferUnion {
    color_palette1: MultibootFramebufferColorPalette1,
    color_palette2: MultibootFramebufferColorPalette2,
}

#[repr(C)]
pub struct MultibootTagFramebuffer {
    common: MultibootTagFramebufferCommon,

    union: MultibootTagFramebufferUnion,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagElfSections {
    typ: TagType,
    size: u32,
    num: u32,
    entsize: u32,
    shndx: u32,
    sections: &'static str,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagApm {
    typ: TagType,
    size: u32,
    version: u16,
    cseg: u16,
    offset: u32,
    cseg_16: u16,
    dseg: u16,
    flags: u16,
    cseg_len: u16,
    cseg_16_len: u16,
    dseg_len: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagEFI32 {
    typ: TagType,
    size: u32,
    pointer: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagEFI64 {
    typ: TagType,
    size: u32,
    pointer: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagSmbios {
    typ: TagType,
    size: u32,
    major: u8,
    minor: u8,
    reserved: [u8; 6],
    tables: [u8; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagOldAcpi {
    typ: TagType,
    size: u32,
    rsdp: [u8; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagNewAcpi {
    typ: TagType,
    size: u32,
    rsdp: [u8; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagNetwork {
    typ: TagType,
    size: u32,
    dhcpack: [u8; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagEfiMmap {
    typ: TagType,
    size: u32,
    descr_size: u32,
    descr_vers: u32,
    efi_mmap: [u8; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagEfi32Ih {
    typ: TagType,
    size: u32,
    pointer: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagEfi64Ih {
    typ: TagType,
    size: u32,
    pointer: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagLoadBaseAddr {
    typ: TagType,
    size: u32,
    load_base_addr: u32,
}


impl MultibootHeader {
    pub unsafe fn load(addr: usize) -> Self {
        *(addr as *const Self)
    }
}
