use kernel::*;
use alloc:: boxed::Box;
use libc::c_int;
use core::{mem, ptr};
use crate::char_ffi;

#[repr(C)]
struct EchoMsg {
    len: usize,
    msg: [u8; 256],
}

pub struct CharacterDevice {
    cdevsw_ptr: *mut cdevsw,
    echo_dev: *mut cdev,
    echo_buf: Box<EchoMsg>,
}

impl CharacterDevice {
    fn cdevsw_init() -> cdevsw {
        cdevsw {
            d_version: D_VERSION,
            d_name: cstr_ptr!("echo"),
            d_open: Some(char_ffi::echo_open),
            d_close: Some(char_ffi::echo_close),
            d_read: None,
            d_write: None,
            .. unsafe { mem::zeroed() }
        }
    }
    
    pub fn new() -> Result<Self, c_int> {
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

        let echo_buf = Box::new(EchoMsg { len: 0, msg: [0; 256] });

        Ok(CharacterDevice { 
            cdevsw_ptr,
            echo_dev, 
            echo_buf, 
        })
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
}
