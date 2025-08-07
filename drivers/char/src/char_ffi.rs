use kernel::*;
use libc::{c_int};
use crate::char_device::CharacterDevice;

pub extern "C" fn echo_open(
    dev: *mut cdev,
    oflags: c_int,
    devtype: c_int,
    td: *mut thread,
) -> c_int {
    let charDev = unsafe {
        &mut *((*dev).si_drv1 as *mut CharacterDevice)
    };

    match charDev.open(dev, oflags, devtype, td) {
        Ok(()) => 0,
        Err(error) => error,
    }
}

pub extern "C" fn echo_close(
    dev: *mut cdev,
    oflags: c_int,
    devtype: c_int,
    td: *mut thread,
) -> c_int {
    let charDev = unsafe {
        &mut *((*dev).si_drv1 as *mut CharacterDevice)
    };

    match charDev.close(dev, oflags, devtype, td) {
        Ok(()) => 0,
        Err(error) => error,
    }
}

pub extern "C" fn echo_read(
    dev: *mut cdev,
    uio_ptr: *mut uio,
    ioflag: c_int
) -> c_int {
    if uio_ptr.is_null() {
        println!("[char_ffi.rs] uio_ptr is NULL");
        return EFAULT;
    }
    let charDev = unsafe {
        &mut *((*dev).si_drv1 as *mut CharacterDevice)
    };

    match charDev.read(dev, uio_ptr, ioflag) {
        Ok(n) => n,
        Err(error) => error,
    }
}

pub extern "C" fn echo_write(
    dev: *mut cdev,
    uio_ptr: *mut uio,
    ioflag: c_int,
) -> c_int {
    if uio_ptr.is_null() {
        println!("[char_ffi.rs] uio_ptr is NULL");
        return EFAULT;
    }
    let charDev = unsafe {
        &mut *((*dev).si_drv1 as *mut CharacterDevice)
    };

    match charDev.write(dev, uio_ptr, ioflag) {
        Ok(n) => n,
        Err(error) => error,
    }
}
