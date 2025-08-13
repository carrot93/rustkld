#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod module_events;
mod char_device;

use core::ffi::c_void;
use libc::{c_int, EOPNOTSUPP};
use kernel::*;
use module_events::Events;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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
