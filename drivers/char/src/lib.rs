#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod char_device;

use core::ffi::c_void;
use libc::{c_int, EOPNOTSUPP};
use kernel::*;
use char_device::*;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn module_event(
    _mod: *mut module,
    event: i32,
    _arg: *mut c_void,
) -> c_int {
    let mut error = 0;

    match ModEventType::from(event) {
        ModEventType::Load => {
            CharacterDevice::load();
        },
        ModEventType::Unload => {
            CharacterDevice::unload();
        },
        ModEventType::Quiesce => {
            CharacterDevice::quiesce();
        },
        ModEventType::Shutdown => {
            CharacterDevice::shutdown();
        },
        _ => {
            error = EOPNOTSUPP;
        }
    }

    error
}

#[unsafe(no_mangle)]
pub static mut char_mod: moduledata_t = moduledata_t {
    name: cstr_ptr!("CharacterDevice"),
    evhand: Some(module_event),
    priv_: core::ptr::null_mut(),
};
