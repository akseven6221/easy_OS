//! standered library for user programs


#![no_std]
// to support the link action below
#![feature(linkage)]
// #![feature(panic_info_message)]

#[macro_use]
pub mod syscall;
pub mod console;
mod lang_items;


#[unsafe(no_mangle)]
// 将这段代码编译后的asm code放到.text.entry段, 方便后续链接的时候调整它的位置使得它能够作为用户库的入口。
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    exit(main());
    panic!("unreachable after sys_exit");
}

// 弱链接的main函数, 如果用户程序没有定义main函数, 则调用这个函数，然后报错
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("cannot find main");
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone, Debug)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}

const MAX_SYSCALL_NUM: usize = 500;

pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize
}

impl TaskInfo {
    pub fn new() -> Self {
        Self {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0
        }
    }
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

pub fn sleep(period_ms: usize) {
    let start = get_time() as usize;
    while get_time() as usize - start < period_ms {
        sys_yield();
    }
}

pub fn task_info(info: &TaskInfo) -> isize {
    sys_task_info(info)
}
pub fn sbrk(size: i32) -> isize {
    sys_sbrk(size)
}
