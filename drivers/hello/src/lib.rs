#![no_std]
#![no_main]

mod hello;

use core::ffi::c_void;
use libc::{c_int, EOPNOTSUPP};
use kernel::*;
use hello::HelloWorld;

/// # Safety
///
/// This function is in charge of dealing with any incomming module event
#[unsafe(no_mangle)]
pub unsafe extern "C" fn module_event(
    _mod: *mut module,
    event: i32,
    _arg: *mut c_void,
) -> c_int {
    let mut error = 0;
    match ModEventType::from(event) {
        ModEventType::Load => {
            HelloWorld::load();
        },
        ModEventType::Unload => {
            HelloWorld::unload();
        },
        _ => {
            error = EOPNOTSUPP;
        }
    }
    error
}

#[unsafe(no_mangle)]
pub static mut hello_mod: moduledata_t = moduledata_t {
    name: c"hello".as_ptr(),
    evhand: Some(module_event),
    priv_: core::ptr::null_mut(),
};
