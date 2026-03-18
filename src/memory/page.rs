const SATP_SV39: usize = 7 << 32;

#[repr(usize)]
enum SATPMode {
    SV39 = 7 << 32,
    SV48 = 8 << 32,
    SV57 = 9 << 32,
}

#[repr(u16)]
enum PageTableFlag {
    Valid = 1,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    Userland = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,
}

const DEFAULT_SATP_MODE: usize = SATPMode::SV39 as usize;

struct Page(usize);

struct VirtualAddressSv39(u64);
struct PhysicalAddressSv39(u64);
struct PageTableEntrySv39(u64);

impl VirtualAddressSv39 {
    fn get_offset(self: Self) -> u16 {
        (self.0 & 0b1111_1111_1111) as u16
    }

    fn get_vpn(self: &Self, offset: usize) -> u16 {
        assert!(offset <= 2);
        ((self.0 >> (12 + offset * 9)) & 0b1_1111_1111) as u16
    }

    fn get_vpns(self: &Self) -> [u16; 3] {
        [self.get_vpn(0), self.get_vpn(1), self.get_vpn(2)]
    }
}

impl PageTableEntrySv39 {
    fn get_ppn(self: &Self, offset: usize) -> u32 {
        assert!(offset <= 2);
        if offset == 2 {
            ((self.0 >> (10 + offset * 9)) & 0b11_1111_1111_1111_1111_1111_1111) as u32
        } else {
            ((self.0 >> (10 + offset * 9)) & 0b1_1111_1111) as u32
        }
    }

    fn get_ppns(self: &Self) -> [u32; 3] {
        [self.get_ppn(0), self.get_ppn(1), self.get_ppn(2)]
    }

    fn is_pointer(self: &Self) -> bool {
        self.0 & 0b1110 == 0
    }

    fn is_leaf(self: &Self) -> bool {
        self.0 & 0b1110 != 0
    }
}

fn activate_virtual_address() {}
