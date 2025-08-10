use kernel::*;
use libc::{c_int, EFAULT};
use crate::char_device::EchoDevice;

pub extern "C" fn echo_open(
    dev: *mut cdev,
    oflags: c_int,
    devtype: c_int,
    td: *mut thread,
) -> c_int {
    let charDev = unsafe {
        &mut *((*dev).si_drv1 as *mut EchoDevice)
    };

    let cdevr = unsafe {&mut *dev};
    let safe_dev = Cdev::new(cdevr);

    match charDev.open(safe_dev, oflags, devtype, td) {
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
        &mut *((*dev).si_drv1 as *mut EchoDevice)
    };

    let cdevr = unsafe {&mut *dev};
    let safe_dev = Cdev::new(cdevr);

    match charDev.close(safe_dev, oflags, devtype, td) {
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
        &mut *((*dev).si_drv1 as *mut EchoDevice)
    };

    let cdevr = unsafe {&mut *dev};
    let safe_dev = Cdev::new(cdevr);

    let uior = unsafe {&mut *uio_ptr};
    let safe_uio = Uio::new(uior);

    match charDev.read(safe_dev, safe_uio, ioflag) {
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
        &mut *((*dev).si_drv1 as *mut EchoDevice)
    };

    let cdevr = unsafe {&mut *dev};
    let safe_dev = Cdev::new(cdevr);

    let uior = unsafe {&mut *uio_ptr};
    let safe_uio = Uio::new(uior);

    match charDev.write(safe_dev, safe_uio, ioflag) {
        Ok(n) => n,
        Err(error) => error,
    }
}
