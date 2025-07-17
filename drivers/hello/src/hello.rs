use kernel::*;

pub struct helloWorld;

impl helloWorld {
    pub fn load() {
        println!("Greetings from hello.rs!");
    }

    pub fn unload() {
        println!("Goodbye from hello.rs!");
    }
}
