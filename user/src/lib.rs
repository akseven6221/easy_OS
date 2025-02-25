//! standered library for user programs


#![no_std]
// to support the link action below
#![feature(linkage)]
// #![feature(panic_info_message)]

#[macro_use]
mod syscall;
pub mod console;
mod lang_items;


#[unsafe(no_mangle)]
// 将这段代码编译后的asm code放到.text.entry段, 方便后续链接的时候调整它的位置使得它能够作为用户库的入口。
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit");
}

// 弱链接的main函数, 如果用户程序没有定义main函数, 则调用这个函数，然后报错
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("cannot find main");
}


fn clear_bss() {
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf :&[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}
