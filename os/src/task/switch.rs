use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));


// in switch.S, we can see "current" need write, "next" only need read, that's why *mut and *const int switch
extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
