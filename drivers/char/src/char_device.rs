use kernel::*;
use alloc::boxed::Box;
use libc::{c_int, c_void, EINVAL};
use core::{mem, ptr};
use core::cmp::min;
use crate::char_ffi;
use crate::echo_msg::{EchoMsg, BUFFERSIZE};

pub struct CharacterDevice {
    cdevsw_ptr: *mut cdevsw,
    echo_dev: *mut cdev,
    echo_buf: Box<EchoMsg>,
}

impl CharacterDevice {
    pub fn new() -> Result<Box<Self>, c_int> {
        let echo_buf = Box::new(EchoMsg::new());

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
		        cstr_ptr!("echo"),
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
        let me_ptr = &mut *me as *mut CharacterDevice as *mut c_void;

        unsafe { 
            (*echo_dev).si_drv1 = me_ptr.cast()
        };
        me.echo_dev = echo_dev;

        Ok(me)
    }

    fn cdevsw_init() -> cdevsw {
        cdevsw {
            d_version: D_VERSION,
            d_name: cstr_ptr!("echo"),
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

impl Drop for CharacterDevice {
    fn drop(&mut self) {
        unsafe {
            destroy_dev(self.echo_dev);
            let _ = Box::from_raw(self.cdevsw_ptr);
        }
    }
}

impl Cdev for CharacterDevice {
    fn open(&mut self, dev: *mut cdev, _oflags: c_int, _devtype: c_int, _td: *mut thread) -> Result<(), c_int> {
        unsafe { dev_ref(dev) };

        println!("[char_device.rs] character device opened");
        Ok(())
    }

    fn close(&mut self, dev: *mut cdev, _oflags: c_int, _devtype: c_int, _td: *mut thread) -> Result<(), c_int> {
        unsafe { dev_rel(dev) };

        println!("[char_device.rs] character device closed");
        Ok(())
    }

    fn write(&mut self, _dev: *mut cdev, uio_ptr: *mut uio, _ioflag: c_int) -> Result<c_int, c_int> {
        let safe_uio = Uio::new(uio_ptr);

        let resid = safe_uio.get_resid();
        let offset = safe_uio.get_offset();

        let length = self.echo_buf.get_length();

        if offset != 0 && offset != length {
            return Err(EINVAL);
        }

        if offset == 0 {
            self.echo_buf.set_length(0);
        }
        let amt = min(resid, BUFFERSIZE - length);
        let error = unsafe { 
            uiomove(self.echo_buf.get_msg().as_mut_ptr().add(offset) as *mut c_void,
                amt as c_int,
                uio_ptr,
            )
        };

        self.echo_buf.set_length(offset + amt);
        self.echo_buf.reset_msg(self.echo_buf.get_length());

        match error {
            error if error < 0 => Err(error),
            error => Ok(error),
        }
    }

    fn read(&mut self, _dev: *mut cdev, uio_ptr: *mut uio, _ioflag: c_int) -> Result<c_int, c_int> {
        let safe_uio = Uio::new(uio_ptr);

        let resid = safe_uio.get_resid();
        let offset = safe_uio.get_offset();

        let length = self.echo_buf.get_length();

        let remain: usize;

        if offset >= length + 1 {
            remain = 0;
        } else {
            remain = length + 1 - offset;
        }

        let amt = min(resid, remain);

        let error = unsafe { 
            uiomove(self.echo_buf.get_msg().as_mut_ptr() as *mut c_void,
                amt as c_int,
                uio_ptr,
            )
        };

        // we return 0 on success but some char drivers return the amount of bytes read/written
        match error {
            error if error < 0 => Err(error),
            error => Ok(error),
        }    
    }
}
