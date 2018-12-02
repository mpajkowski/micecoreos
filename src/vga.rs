#![allow(dead_code)]

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    fn update_cursor(&mut self, x: u8, y: u8) {
        let pos: u16 = y as u16 * BUFFER_WIDTH as u16 + x as u16;

        use x86_64::instructions::port::Port;
        let mut p3d4: Port<u8> = Port::new(0x3D4);
        let mut p3d5: Port<u8> = Port::new(0x3D5);

        unsafe {
            p3d4.write(0x0F);
            p3d5.write((pos as u8) & 0xFF);
            p3d4.write(0x0E);
            p3d5.write(((pos >> 8) & 0xFF) as u8);
        }
    }
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Writer {
        let color_code = ColorCode::new(Color::LightGreen, Color::Black);

        Writer {
            column_position: 0,
            row_position: 0,
            color_code: color_code,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x8 => self.backspace(),
                0x20...0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        self.buffer
            .update_cursor(self.column_position as u8, self.row_position as u8);
    }

    fn new_line(&mut self) {
        let last_line = BUFFER_HEIGHT - 1;

        if self.row_position != last_line {
            self.row_position += 1;
        } else {
            for row in 2..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
        }
        let row = self.row_position;
        self.clear_row(row);
        self.column_position = 0;
    }

    fn backspace(&mut self) {
        if self.row_position < 1 {
            return;
        }

        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        let row = self.row_position;
        let col = self.column_position;

        if self.column_position > 1 {
            self.column_position -= 1
        } else {
            self.row_position -= 1;
            self.column_position = BUFFER_WIDTH;
        }

        self.buffer.chars[row][col - 1].write(blank);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo() {}
}
