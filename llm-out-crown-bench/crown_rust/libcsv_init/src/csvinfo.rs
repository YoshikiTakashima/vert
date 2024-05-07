
use std::mem;

pub struct CSVParser {
    state: CSVState,
    quoted: bool,
    spaces: usize,
    entry_buf: Vec<u8>,
    entry_pos: usize,
    entry_size: usize,
    status: i32,
    options: u8,
    quote_char: u8,
    delim_char: u8,
    is_space: Option<extern "C" fn(u8) -> i32>,
    is_term: Option<extern "C" fn(u8) -> i32>,
    blk_size: usize,
    malloc_func: Option<extern "C" fn(usize) -> *mut std::ffi::c_void>,
    realloc_func: Option<extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void>, 
    free_func: Option<extern "C" fn(*mut std::ffi::c_void)>
}

enum CSVState {
    RowNotBegun,
    FieldBegins,
    QuotedField,
    Field,
    BetweenFields 
}

struct Counts {
    fields: usize,
    rows: usize
}

extern "C" fn cb1(s: *mut std::ffi::c_void, len: usize, data: *mut std::ffi::c_void) {
    let counts = unsafe { &mut *(data as *mut Counts) };
    counts.fields += 1;
}

extern "C" fn cb2(c: i32, data: *mut std::ffi::c_void) {
    let counts = unsafe { &mut *(data as *mut Counts) };
    counts.rows += 1; 
}

impl CSVParser {
    fn new(options: u8) -> CSVParser {
        CSVParser {
            state: CSVState::RowNotBegun,
            quoted: false,
            spaces: 0,
            entry_buf: Vec::new(),
            entry_pos: 0,
            entry_size: 0,
            status: 0,
            options,
            quote_char: b'"',
            delim_char: b',',
            is_space: None,
            is_term: None,
            blk_size: 4096,
            malloc_func: None,
            realloc_func: None,
            free_func: None
        }
    }
}
fn main(){
}