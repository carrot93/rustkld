use crate::bindings::imports::uio;

pub struct Uio(*mut uio);

impl Uio {
    pub fn new(uio_ptr: *mut uio) -> Self {
        Uio(uio_ptr)
    }

    pub fn get_resid(&self) -> usize{
        unsafe {
            (*self.0).uio_resid as usize 
        }
    }
    
    pub fn get_offset(&self) -> usize {
        unsafe {
            (*self.0).uio_offset as usize
        }
    }
}
