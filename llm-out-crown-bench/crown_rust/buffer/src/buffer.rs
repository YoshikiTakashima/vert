
use std::mem;

pub struct Buffer {
    len: usize,
    data: Box<[u8]>, 
}

impl Buffer {
    fn new() -> Buffer {
        Buffer::with_size(1024)
    }

    fn with_size(size: usize) -> Buffer {
        let mut data = vec![0; size + 1];
        Buffer {
            len: size,
            data: data.into_boxed_slice(),
        }
    }
}
fn main(){
}