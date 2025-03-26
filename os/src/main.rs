//! The main module and entrypoint

#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate bitflags;

mod boards;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod logging;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

use core::arch::global_asm;
use log::*;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    info!("[kernel] Hello, world!");
    mm::init();
    info!("[kernel] back to world!");
    mm::remap_test();
    trap::init();
    // trap::enable_interrupt();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
