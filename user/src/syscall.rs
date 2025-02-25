


use core::arch::asm;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

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
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}
