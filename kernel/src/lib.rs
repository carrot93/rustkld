#![no_std]
#![feature(alloc_error_handler)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]                                                                                                

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub extern crate libc;

#[macro_use]
mod macros;

mod bindings;
pub use bindings::imports::*;

mod io;
pub use io::KernelDebugWriter;

mod allocator;
pub use allocator::*;
#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

mod char_ffi;
pub use char_ffi::*;

mod uio_wrap;
pub use uio_wrap::Uio;

mod cdev_wrap;
pub use cdev_wrap::Cdev;

/*
pub struct BorrowedFoo<'a>(&'a [u8]);

impl<'a> BorrowedFoo<'a> {

}
*/

pub trait Cdevsw {    
    fn open(&mut self, dev: Cdev, oflags: libc::c_int, devtype: libc::c_int, td: *mut thread) -> Result<(), libc::c_int>;
    fn close(&mut self, dev: Cdev, oflags: libc::c_int, devtype: libc::c_int, td: *mut thread) -> Result<(), libc::c_int>;
    fn write(&mut self, dev: Cdev, uio_ptr: Uio, ioflag: libc::c_int) -> Result<libc::c_int, libc::c_int>;
    fn read(&mut self, dev: Cdev, uio_ptr: Uio, ioflag: libc::c_int) -> Result<libc::c_int, libc::c_int>;
/*
    fn ioctl(...);
    fn poll(...);
    fn mmap(...);
    fn strategy(...);
    fn kqfilter(...);
    fn purge(...);
    fn mmap_single(...);
*/
}

/*
pub trait Read {
    pub fn read(&mut self, buf:   
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
