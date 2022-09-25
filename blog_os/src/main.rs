//创建一个独立运行程序
//1.不依赖标准库
#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

//2.缺少panic处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {//散列函数，无返回
    println!("{}", _info);
    loop{}
}

static HELLO: &[u8] = b"Hello World ";

//需要一个开始语言项

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}
/* 
fn main() {
    //println!("Hello, world!");
    //不能这个宏，因为是在std中定义的

}
*/