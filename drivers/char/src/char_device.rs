use kernel::*;

pub struct CharacterDevice;

impl CharacterDevice {
    pub fn load() {
        println!("Greetings from CharacterDevice.rs!");
    }

    pub fn unload() {
        println!("Goodbye from CharacterDevice.rs!");
    }
    
    pub fn quiesce() {
        println!("Quiesce from CharacterDevice.rs!");
    }
    
    pub fn shutdown() {
        println!("Shutdown from CharacterDevice.rs!");
    }
}
