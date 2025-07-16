#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

//include!("bindings/imports/freebsd-bindings.rs");
#[path = "bindings/imports/freebsd-bindings.rs"]
mod freebsd_bindings;
pub use freebsd_bindings::*;

pub extern crate libc;

#[macro_use]
pub mod macros;

pub mod io;


/*
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
*/

pub enum ModEventType {
    Load = 0,
    Unload = 1,
    Shutdown = 2,
    Quiesce = 3,
    Unknown = 4,
}
impl From<i32> for ModEventType {
    fn from(i: i32) -> Self {
        match i {
            0 => ModEventType::Load,
            1 => ModEventType::Unload,
            2 => ModEventType::Shutdown,
            3 => ModEventType::Quiesce,
            _ => ModEventType::Unknown,
        }
    }
}
