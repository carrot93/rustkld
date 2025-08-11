use crate::bindings::imports::{cdev, dev_ref, dev_rel};

pub struct Cdev<'a>(&'a mut cdev);

impl<'a> Cdev<'a> {
    pub fn new(cdev: &'a mut cdev) -> Self {
        Self(cdev)
    }

    pub fn get_usecount(&self) -> usize {
        self.0.si_usecount as usize
    }
    
    pub fn cdev_ref(&mut self) {
        unsafe {
            dev_ref(&mut *self.0)
        }
    }
    
    pub fn cdev_rel(&mut self) {
        unsafe {
            dev_rel(&mut *self.0)
        }
    }
}
