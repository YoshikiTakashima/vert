
use std::mem;

struct Buffer {
    len: usize,
    alloc: String,
    data: String,
}

impl Buffer {
    fn resize(&mut self, n: usize) -> Result<(), std::alloc::AllocError> {
        n = n.next_multiple_of(1024);
        self.len = n;
        self.alloc = " ".repeat(n);
        self.data = &self.alloc[..];
        Ok(())
    }

    fn append_n(&mut self, str: &str, len: usize) -> Result<(), std::alloc::AllocError> {
        let prev = self.data.len();
        let needed = len + prev;

        if self.len > needed {
            self.data.push_str(&str[..len]);
            Ok(())
        } else {
            self.resize(needed)?;
            self.data.push_str(&str[..len]);
            Ok(())
        }
    }

    fn prepend(&mut self, str: &str) -> Result<(), std::alloc::AllocError> {
        let len = str.len();
        let prev = self.data.len();
        let needed = len + prev;

        if self.len > needed {
            unsafe {
                mem::transmute::<&mut str, &mut [u8]>(&mut self.data);
                let slice = &mut self.data;
                slice.copy_within(len.., 0);
                let (first, rest) = slice.split_at_mut(len);
                first.copy_from_slice(str.as_bytes());
            }
            Ok(())
        } else {
            self.resize(needed)?;
            unsafe {
                let slice = &mut self.data;
                slice.copy_within(len.., 0);
                let (first, rest) = slice.split_at_mut(len);
                first.copy_from_slice(str.as_bytes());
            }
            Ok(())
        }
    }
}
fn main(){
}