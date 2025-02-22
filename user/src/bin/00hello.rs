#![no_std]
#![no_main]

extern crate user_lib; // cargo.toml里设置的name = user_lib

use user_lib::println;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("6666");
    0
}