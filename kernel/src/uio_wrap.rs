use crate::bindings::imports::{uio, uiomove};
use libc::{c_int, c_void};

pub struct Uio<'a>(&'a mut uio);

impl<'a> Uio<'a> {
    pub fn new(uio: &'a mut uio) -> Self {
        Self(uio)
    }

    pub fn get_resid(&self) -> usize{
        self.0.uio_resid as usize 
    }
    
    pub fn get_offset(&self) -> usize {
        self.0.uio_offset as usize
    }
    
    pub fn uio_move(&mut self, buff: *mut u8, amt: usize, offset: usize) -> c_int {
        unsafe {
            uiomove(buff.add(offset) as *mut c_void, amt as c_int, self.0)
        }
    }
}
