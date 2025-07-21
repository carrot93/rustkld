use kernel::*;
use crate::char_device::CharacterDevice;

static mut ECHO_DEVICE: Option<CharacterDevice> = None;

pub struct Events;

impl Events {
    pub fn load() {
        match CharacterDevice::new() {
            Ok(dev) => unsafe {
                ECHO_DEVICE = Some(dev);
                println!("Echo device loaded");
            },
            Err(_err) => println!("Echo device make failed"),
        }
    }
    pub fn unload() {
        unsafe {
            // deref raw ptr to get pointed Option<CharacterDevice>
            let ptr: *mut Option<CharacterDevice> = &raw mut ECHO_DEVICE;

            // call Option::take() to move Some(dev) out, leaving nothing behind
            let dev_out = (*ptr).take();

            // drop dev
            if let Some(dev) = dev_out {
                drop(dev);
            }
        }
        println!("Echo device unloaded");
    }
    
    pub fn quiesce() {
        println!("Quiesce from module_events.rs!");
    }
    
    pub fn shutdown() {
        println!("Shutdown from module_events.rs!");
    }
}
