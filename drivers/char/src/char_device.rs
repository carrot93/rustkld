use kernel::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use libc::{c_int, c_void, EINVAL};
use core::{mem, ptr};
use core::cmp::min;
use crate::char_ffi;

const BUFFERSIZE: usize = 256;

pub struct EchoDevice {
    cdevsw_ptr: *mut cdevsw,
    echo_dev: *mut cdev,
    echo_buf: Vec<u8>,
}

impl EchoDevice {
    pub fn new() -> Result<Box<Self>, c_int> {
        let echo_buf = Vec::with_capacity(BUFFERSIZE);

        // move cdevsw to the heap using Box
        let boxed_cdevsw = Box::new(Self::cdevsw_init());

        // get raw pointer to give to kernel during cdev call
        let cdevsw_ptr: *mut cdevsw = Box::into_raw(boxed_cdevsw);

        let mut echo_dev: *mut cdev = ptr::null_mut();
        let error = unsafe {
            make_dev_p(MAKEDEV_CHECKNAME | MAKEDEV_WAITOK,
		        &mut echo_dev,
                cdevsw_ptr,
                core::ptr::null_mut(),
		        UID_ROOT.try_into().unwrap(),
		        GID_WHEEL.try_into().unwrap(),
		        0600,
		        c"echo".as_ptr(),
            )
        };

        if error != 0 {
            unsafe { let _ = Box::from_raw(cdevsw_ptr); } // reclaim and free cdevsw on failure
            return Err(error);
        }

        let mut me = Box::new(Self {
            cdevsw_ptr,
            echo_dev: ptr::null_mut(),
            echo_buf,
        });
        let me_ptr = &mut *me as *mut EchoDevice as *mut c_void;

        unsafe { 
            (*echo_dev).si_drv1 = me_ptr.cast()
        };
        me.echo_dev = echo_dev;

        Ok(me)
    }

    fn cdevsw_init() -> cdevsw {
        cdevsw {
            d_version: D_VERSION,
            d_name: c"echo".as_ptr(),
            d_open: Some(char_ffi::echo_open),
            d_close: Some(char_ffi::echo_close),
            d_read: Some(char_ffi::echo_read),
            d_write: Some(char_ffi::echo_write),
            .. unsafe { mem::zeroed() }
        }
    }

    pub fn get_usecount(&self) -> usize {
        unsafe {
            (*self.echo_dev).si_usecount as usize  
        }
    }
}

impl Drop for EchoDevice {
    fn drop(&mut self) {
        unsafe {
            destroy_dev(self.echo_dev);
            let _ = Box::from_raw(self.cdevsw_ptr);
        }
    }
}

impl Cdevsw for EchoDevice {
    fn open(&mut self, mut dev: Cdev, _oflags: c_int, _devtype: c_int, _td: *mut thread) -> Result<(), c_int> {
        dev.cdev_ref();

        println!("[char_device.rs] character device opened");
        Ok(())
    }

    fn close(&mut self, mut dev: Cdev, _oflags: c_int, _devtype: c_int, _td: *mut thread) -> Result<(), c_int> {
        dev.cdev_rel();

        println!("[char_device.rs] character device closed");
        Ok(())
    }

    fn write(&mut self, _dev: Cdev, mut safe_uio: Uio, _ioflag: c_int) -> Result<c_int, c_int> {
        let resid = safe_uio.get_resid();
        let offset = safe_uio.get_offset();

        let length = self.echo_buf.len();

        if offset != 0 && offset != length {
            return Err(EINVAL);
        }

        if offset == 0 {
            self.echo_buf.resize(0, 0);
        }
        let amt = min(resid, self.echo_buf.capacity() - length);

        let error = safe_uio.uio_move(self.echo_buf.as_mut_ptr(), amt, offset);
        
        unsafe {
            self.echo_buf.set_len(offset + amt) 
        };
    
        match error {
            error if error < 0 => {
                return Err(error);
            }
            error => Ok(error),
        }
    }

    fn read(&mut self, _dev: Cdev, mut safe_uio: Uio, _ioflag: c_int) -> Result<c_int, c_int> {
        let resid = safe_uio.get_resid();
        let offset = safe_uio.get_offset();

        let length = self.echo_buf.len();

        let remain: usize;

        if offset >= length - 1 {
            remain = 0;
        } else {
            remain = length - 1 - offset;
        } 

        let amt = min(resid, remain);

        let error = safe_uio.uio_move(self.echo_buf.as_mut_ptr(), amt, offset);

        // we return 0 on success, some char drivers return the amount of bytes read/written
        match error {
            error if error < 0 => Err(error),
            error => Ok(error),
        }    
    }
}
