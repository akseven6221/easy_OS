mod context;
mod switch;
pub mod task;

use crate::loader::{get_app_data, get_num_app};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::vec::Vec;
pub use context::TaskContext;
use lazy_static::lazy_static;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        println!("init TASK_MANAGER");
        let num_app = get_num_app();
        println!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(
                get_app_data(i),
                i,
            ));
        }
        // we do not need to tag TaskManager to Sync, compiler can induce TaskManager is Sync according to TaskManager's segment
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;

            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_cx as *mut TaskContext;
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn run_first_task(&self) -> ! {
        let mut inner = TASK_MANAGER.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut __unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut __unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!")
    }

    fn get_current_task_status(&self) -> TaskStatus {
        let inner: core::cell::RefMut<'_, TaskManagerInner> = self.inner.exclusive_access();
        inner.tasks[inner.current_task].task_status
    }

    fn add_call_times(&self, syscall_id: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].call_times[syscall_id] += 1;
    }

    fn get_call_times(&self) -> [u32; 500] {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let cloned = inner.tasks[current].call_times.clone();
        cloned
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }

    pub fn change_current_program_brk(&self, size: i32) -> Option<usize> {
        let mut inner = self.inner.exclusive_access();
        let cur = inner.current_task;
        inner.tasks[cur].change_program_brk(size)
    }
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn get_current_task_status() -> TaskStatus {
    TASK_MANAGER.get_current_task_status()
}

pub fn add_call_times(syscall_id: usize) {
    TASK_MANAGER.add_call_times(syscall_id);
}

pub fn get_call_times() -> [u32; 500] {
    TASK_MANAGER.get_call_times()
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

/// Change the current 'Running' task's program break
pub fn change_program_brk(size: i32) -> Option<usize> {
    TASK_MANAGER.change_current_program_brk(size)
}
