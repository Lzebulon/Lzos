use core::{
    cell::RefCell,
    fmt::{self, Write},
};

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    LightBrown,
    White = 16,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = c;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

pub struct VGAOutInner {
    vga: Option<Writer>,
}

pub struct VGAOut {
    inner: RefCell<VGAOutInner>,
}

impl core::ops::Deref for VGAOut {
    type Target = RefCell<VGAOutInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl VGAOut {
    const fn new() -> Self {
        Self {
            inner: RefCell::new(VGAOutInner { vga: None }),
        }
    }

    fn _init(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.vga = Some(Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Red, Color::White),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        });
    }

    pub fn init() {
        VGA_OUT._init();
    }
}

unsafe impl Sync for VGAOut {}

static VGA_OUT: VGAOut = VGAOut::new();

pub fn _printk(args: fmt::Arguments) {
    let mut vga_inner = VGA_OUT.borrow_mut();
    if let Some(vga) = &mut vga_inner.vga {
        vga.write_fmt(args).unwrap();
    }
}

/// Prints on vga buffer.
#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {
        ($crate::vga::_printk(format_args!($($arg)*)));
    };
}

/// Prints on vga buffer, with a newline.
#[macro_export]
macro_rules! printkln {
    () => ($crate::printk!("\n"));
    ($($arg:tt)*) => ($crate::printk!("{}\n", format_args!($($arg)*)));
}
