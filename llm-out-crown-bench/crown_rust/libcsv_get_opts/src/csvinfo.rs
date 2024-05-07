
struct CsvParser {
    pstate: i32,            // Parser state 
    quoted: bool,          // Is the current field a quoted field?
    spaces: usize,         // Number of continious spaces after quote or in a non-quoted field
    entry_buf: Vec<u8>,   // Entry buffer  
    entry_pos: usize,      // Current position in entry_buf (and current size of entry)
    entry_size: usize,     // Size of entry buffer
    status: i32,           // Operation status
    options: u8,
    quote_char: u8,
    delim_char: u8,
    is_space: fn(u8) -> bool, 
    is_term: fn(u8) -> bool,
    blk_size: usize,
    malloc_func: fn(usize) -> *mut std::ffi::c_void,  
    realloc_func: fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void,
    free_func: fn(*mut std::ffi::c_void), 
}

fn csv_get_opts(parser: &CsvParser) -> i32 {
    if parser.is_null() {
        return -1;
    }
    parser.options
} 

fn main(){
}