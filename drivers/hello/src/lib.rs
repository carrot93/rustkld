#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;
use libc::{c_int, EOPNOTSUPP};
use kernel::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn module_event(
    _mod: *mut module,
    event: i32,
    _arg: *mut c_void,
) -> c_int {
    let mut error = 0;
    match ModEventType::from(event) {
        ModEventType::Load => unsafe {
            uprintf(b"Hello loaded\n\0".as_ptr() as *const i8);
        },
        ModEventType::Unload => unsafe {
            uprintf(b"Hello Unloaded\n\0".as_ptr() as *const i8);
        },
        _ => {
            error = EOPNOTSUPP;
        }
    }
    error.try_into().unwrap()
}
