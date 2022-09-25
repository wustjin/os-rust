//1.定义颜色枚举
//不使用会警告，取消警告
#[allow(dead_code)]
//默认数字类型位i32，空间浪费
#[repr(u8)]
//实现默认trait
#[derive(Debug,Clone, Copy,PartialEq,Eq)]
 enum Color {
    Black=0,
    Blue=1,
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

 //颜色代码
 #[derive(Debug,Clone, Copy,PartialEq, Eq)]
 #[repr(transparent)]
 struct ColorCode(u8);//使用元组定义结构体

 impl ColorCode {
     fn new(foreground: Color,background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
     }
 }
//字符 = ascii + ColorCode
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//VGA字符缓冲区有25行，80列
//地址：0xb8000
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    //chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    //写入字符 
    pub fn write_byte(&mut self,byte: u8){
        match byte {
            b'\n' => self.new_line(),//换行符，换到下一行
            byte => {
                //超过列数，换下一行
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
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
            self.buffer.chars[row][col].write(blank);
        }
    }

    //写入字符串 
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不包含在上述范围之内的字节
                _ => self.write_byte(0xfe),
            }

        }
    }
}

use core::fmt::{self, write};
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


use lazy_static::lazy_static;
//延时初始化，静态变量在编译时就初始化了
//而一般变量在运行时初始化
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}