
use std::fs::File;
use std::io::SeekFrom;

fn file_size(path: &str) -> i64 {
    let f = File::open(path).unwrap();
    let size = f.seek(SeekFrom::End(0)).unwrap(); 
    f.rewind().unwrap();
    size
}
fn main(){
}