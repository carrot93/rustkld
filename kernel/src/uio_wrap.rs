use crate::bindings::imports::{uio, uiomove};
use crate::Read;
use core::cmp::min;
use alloc::vec::Vec;
use libc::{c_int, c_void, EINVAL};

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
    
    /// # Safety
    ///
    /// Tiny wrapper for the uiomove call
    pub unsafe fn uio_move(&mut self, buff: *mut u8, amt: usize, offset: usize) -> c_int {
        unsafe {
            uiomove(buff.add(offset) as *mut c_void, amt as c_int, self.0)
        }
    }
}
impl<'a> Read for Uio<'a> {
    fn read(&mut self, buf: &mut Vec<u8>) -> Result<libc::c_int, libc::c_int> {
        self.read_buf(buf) 
    }   
    fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<libc::c_int, libc::c_int> {
        let resid = self.get_resid();
        let offset = self.get_offset();

        let length = buf.len();

        if offset != 0 && offset != length {
            return Err(EINVAL);
        }

        if offset == 0 {
            buf.clear();
        }
        let amt = min(resid, buf.capacity() - length);

        let error = unsafe {
            self.uio_move(buf.as_mut_ptr(), amt, offset)
        };
        
        unsafe {
            buf.set_len(offset + amt) 
        };
    
        match error {
            error if error < 0 => Err(error),
            error => Ok(error),
        }
    }   
}
