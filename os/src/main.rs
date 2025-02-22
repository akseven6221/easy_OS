//! The main module and entrypoint

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod sync;
mod syscall;
mod trap;
mod batch;
mod logging;


use core::arch::global_asm;
use log::*;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of read-only data segment
        fn erodata();   // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of bss segment
        fn ebss(); // end addr of bss segment
        fn boot_stack_lower_bound(); // lower bound of boot stack
        fn boot_stack_top(); // top of boot stack
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack [{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        sbss as usize, ebss as usize
    );
    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}



