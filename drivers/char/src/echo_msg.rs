pub const BUFFERSIZE: usize = 256;

#[repr(C)]
pub struct EchoMsg {
    length: usize,
    msg: [u8; BUFFERSIZE],
}
impl EchoMsg {
    pub fn new() -> Self {
        EchoMsg {
            length: 0,
            msg: [0; BUFFERSIZE],
        }
    }
     
    pub fn get_length(&self) -> usize {
        self.length   
    }

    pub fn get_msg(&self) -> [u8; BUFFERSIZE] {
        self.msg
    }
    
    pub fn reset_msg(&mut self, pos: usize) {
        self.msg[pos] = 0;
    }

    pub fn set_length(&mut self, new_length: usize) {
        self.length = new_length 
    }
}
