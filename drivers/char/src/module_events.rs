use kernel::*;
use libc::{c_int, ENXIO};
use alloc::boxed::Box;
use crate::char_device::EchoDevice;

static mut CDEVSW: Option<Box<Box<dyn Cdevsw>>> = None;

pub struct Events;

impl Events {
    pub fn load() -> c_int{
        match EchoDevice::new() {
            Ok(dev) => unsafe {
                CDEVSW = Some(dev);
                println!("[module_events.rs] Echo device loaded");
                0
            },
            Err(_err) => {
                println!("[module_events.rs] Echo device make failed");
                ENXIO
            },
        }
    }
    pub fn unload() -> c_int {
        unsafe {
            // deref raw ptr to get pointed Option<Box<Box<dyn Cdevsw>>>
            let ptr: *mut Option<Box<Box<dyn Cdevsw>>> = &raw mut CDEVSW;

            // call Option::take() to move Some(dev) out, leaving nothing behind
            let dev_out = (*ptr).take();

            // drop dev
            if let Some(dev) = dev_out {
                drop(dev);
            }
        }
        println!("[module_events.rs] Echo device unloaded");
        0
    }
    
    pub fn quiesce() -> c_int {
        println!("[module_events.rs] Quiesce!");

        // this is really ugly, I might need to change how I go about this
        /*
        let refcount = unsafe {
            (&raw mut ECHO_DEVICE)
                .as_ref().unwrap()
                .as_ref().unwrap()
                .get_usecount()
        };         

        if refcount > 0 {
             return EBUSY;
        }
        */

        0
    }
    
    pub fn shutdown() -> c_int {
        println!("[module_events.rs] Shutdown!");
        0
    }
}
