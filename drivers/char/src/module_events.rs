use kernel::*;
use core::mem::take;
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
            let dev = take(&mut &raw mut ECHO_DEVICE);
            drop(dev);
        }
        println!("Echo device unloaded");
    }
    
    pub fn quiesce() {
        println!("Quiesce from CharacterDevice.rs!");
    }
    
    pub fn shutdown() {
        println!("Shutdown from CharacterDevice.rs!");
    }
}
