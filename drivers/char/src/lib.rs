#![no_std]
#![no_main]

mod module_events;
mod char_device;

use core::ffi::c_void;
use libc::{c_int, EOPNOTSUPP};
use kernel::*;
use module_events::Events;

extern crate alloc;

/// # Safety
///
/// This function is in charge of dealing with any incomming module event
#[unsafe(no_mangle)]
pub unsafe extern "C" fn module_event(
    _mod: *mut module,
    event: i32,
    _arg: *mut c_void,
) -> c_int {
    let error: c_int = match ModEventType::from(event) {
        ModEventType::Load => {
            Events::load()
        },
        ModEventType::Unload => {
            Events::unload()
        },
        ModEventType::Quiesce => {
            Events::quiesce()
        },
        ModEventType::Shutdown => {
            Events::shutdown()
        },
        _ => {
            EOPNOTSUPP
        }
    };

    error
}

#[unsafe(no_mangle)]
pub static mut char_mod: moduledata_t = moduledata_t {
    name: c"CharacterDevice".as_ptr(),
    evhand: Some(module_event),
    priv_: core::ptr::null_mut(),
};
