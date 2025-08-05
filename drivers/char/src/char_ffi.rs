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
/*
pub extern "C" fn echo_read(
    dev: *mut cdev,
    uio_ptr: *mut uio,
    _ioflag: c_int
) -> c_int {
    if uio_ptr.is_null() {
        println!("[echo_read] uio_ptr is NULL");
        return EFAULT;
    }
    unsafe {
        let state = &mut *(*dev).si_drv1.cast::<CharacterDevice>();

        let resid = (*uio_ptr).uio_resid as usize;
        let offset = (*uio_ptr).uio_offset as usize;
        let length = state.echo_buf.len;

        let remain: usize;

        if offset >= length + 1 {
            remain = 0;
        } else {
            remain = length + 1 - offset;
        }

        let amt = min(resid, remain);

        let error = uiomove(state.echo_buf.msg.as_mut_ptr() as *mut c_void,
                amt as c_int,
                uio_ptr,
        );

        error
    }
}

pub extern "C" fn echo_write(
    dev: *mut cdev,
    uio_ptr: *mut uio,
    _ioflag: c_int,
) -> c_int {
    if uio_ptr.is_null() {
        println!("[echo_write] uio_ptr is NULL");
        return EFAULT;
    }
    unsafe {
        let state = &mut *(*dev).si_drv1.cast::<CharacterDevice>();

        let offset = (*uio_ptr).uio_offset as usize;
        let length = state.echo_buf.get_len();
        let resid = (*uio_ptr).uio_resid as usize;

        if offset != 0 && offset != length {
            return EINVAL;
        }

        if offset == 0 {
            state.echo_buf.set_len(0); 
        } 
        let amt = min(resid, BUFFERSIZE - length);
        let error = uiomove(state.echo_buf.msg.as_mut_ptr().add(offset) as *mut c_void,
                amt as c_int,
                uio_ptr,
        );

        state.echo_buf.set_len(offset + amt);
        state.echo_buf.reset_msg(state.echo_buf.get_len());

        error
    }
}
*/
