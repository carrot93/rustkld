use crate::bindings::imports::uio;

pub struct Uio<'a>(&'a mut uio);

impl<'a> Uio<'a> {
    pub fn new(uio: &'a mut uio) -> Self {
        Self(uio)
    }

    pub fn get_resid(&self) -> usize{
        (*self.0).uio_resid as usize 
    }
    
    pub fn get_offset(&self) -> usize {
        (*self.0).uio_offset as usize
    }
}
