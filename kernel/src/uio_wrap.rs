use crate::bindings::imports::uio;

pub struct Uio(uio);

impl Uio {
    pub fn get_resid(&self) -> usize{
        self.0.uio_resid as usize 
    }
    
    pub fn get_offset(&self) -> usize {
        self.0.uio_offset as usize
    }
}
