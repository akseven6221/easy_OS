


use core::arch::asm;

use crate::TaskInfo;

pub const SYS_GETTIMEOFDAY: usize = 169;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_YIELD: usize = 124;
pub const SYS_GET_TIME: usize = 169;
pub const SYS_TASK_INFO: usize = 410;
pub const SYS_SBRK: usize = 214;

fn syscall(id: usize, args: [usize; 3]) ->isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret, // a0(x10) 作为输入寄存器，也作为输出寄存器。 args[0]为输入变量, ret为输出变量
            in("x11") args[1],  //将输入参数 args[1] 绑定到 ecall 的输入寄存器 x11 即 a1 中
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

/// &[u8] is a fat pointer, which contains both the pointer to the data and the length of the data.
/// use as_ptr to get the start address of the data, and use len to get the length of the data.
pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYS_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYS_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYS_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYS_GET_TIME, [0, 0, 0])
}

pub fn sys_task_info(info: &TaskInfo) -> isize {
    syscall(SYS_TASK_INFO, [info as *const _ as usize, 0, 0])
}

pub fn sys_sbrk(size: i32) -> isize {
    syscall(SYS_SBRK, [size as usize, 0, 0])
}
