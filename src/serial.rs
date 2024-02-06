use core::{arch::asm, fmt::Write, ops::Add};

const BASE_PORT_ADDR: u16 = 0x3F8;

#[derive(Clone, Copy)]
struct Port {
    addr: u16,
}

impl Port {
    fn new(base_addr: u16) -> Self {
        Self { addr: base_addr }
    }

    fn write_byte(&self, value: u8) {
        unsafe {
            asm!(
                "outb %al, %dx",
                in("dx") self.addr,
                in("al") value,
                options(att_syntax)
            );
        }
    }

    fn readbyte(&self) -> u8 {
        let mut output;
        unsafe {
            asm!(
                "in %dx, %al",
                in("dx") self.addr,
                out("al") output,
                options(att_syntax)
            );
        }
        output
    }
}

impl Write for Port {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.write_byte(c);
        }
        Ok(())
    }
}

impl Add<u16> for Port {
    type Output = Port;

    fn add(self, rhs: u16) -> Self::Output {
        Port {
            addr: self.addr + rhs,
        }
    }
}

pub fn init_serial() {
    let port = Port::new(BASE_PORT_ADDR);
    (port + 1).write_byte(0x00); // Disable all interrupts
    (port + 3).write_byte(0x80); // Enable DLAB (set baud rate divisor)
    (port + 0).write_byte(0x03); // Set divisor to 3 (lo byte) 38400 baud
    (port + 1).write_byte(0x00); //                  (hi byte)
    (port + 3).write_byte(0x03); // 8 bits, no parity, one stop bit
    (port + 2).write_byte(0xC7); // Enable FIFO, clear them, with 14-byte threshold
    (port + 4).write_byte(0x0B); // IRQs enabled, RTS/DSR set
    (port + 4).write_byte(0x1E); // Set in loopback mode, test the serial chip
    (port + 0).write_byte(0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

    if (port + 0).readbyte() != 0xAE {
        panic!("byte faulty");
    }

    (port + 4).write_byte(0x0F);
}
