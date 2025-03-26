#![no_std]
#![no_main]


extern crate user_lib;

use user_lib::{
    TaskInfo, TaskStatus,
    get_time, println, sleep, task_info,
};

use user_lib::syscall::*;

#[unsafe(no_mangle)]
pub fn main() -> usize {
    println!("test task info...");
    let t1 = get_time() as usize;
    let info = TaskInfo::new();
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    // 注意本次 task info 调用也计入
    assert_eq!(0, task_info(&info));
    let t3 = get_time() as usize;
    assert!(3 <= info.syscall_times[SYS_GETTIMEOFDAY]);
    assert_eq!(1, info.syscall_times[SYS_TASK_INFO]);
    assert_eq!(0, info.syscall_times[SYS_WRITE]);
    assert!(0 < info.syscall_times[SYS_YIELD]);
    assert_eq!(0, info.syscall_times[SYS_EXIT]);
    assert!(t2 - t1 <= info.time + 1);
    assert!(info.time < t3 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    // 想想为什么 write 调用是两次
    println!("string from task info test\n");
    let t4 = get_time() as usize;
    assert_eq!(0, task_info(&info));
    let t5 = get_time() as usize;
    assert!(5 <= info.syscall_times[SYS_GETTIMEOFDAY]);
    assert_eq!(2, info.syscall_times[SYS_TASK_INFO]);
    assert_eq!(1, info.syscall_times[SYS_WRITE]);
    assert!(0 < info.syscall_times[SYS_YIELD]);
    assert_eq!(0, info.syscall_times[SYS_EXIT]);
    assert!(t4 - t1 <= info.time + 1);
    assert!(info.time < t5 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    println!("Test task info OK!");
    0
}
