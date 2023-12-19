static mut PARAM1: char = 'a';
static mut RESULT: i32 = 12;
mod guest_mem_wrapper;

use std::convert::TryInto;


#[derive(Copy, Clone, Debug)]
enum TaggedVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Undefined,
}

impl Default for TaggedVal {
    fn default() -> Self {
        TaggedVal::Undefined
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ValType {
    I32,
    I64,
    F32,
    F64,
    Undefined,
}

impl From<TaggedVal> for ValType {
    fn from(v: TaggedVal) -> Self {
        match v {
            TaggedVal::I32(_) => ValType::I32,
            TaggedVal::I64(_) => ValType::I64,
            TaggedVal::F32(_) => ValType::F32,
            TaggedVal::F64(_) => ValType::F64,
            TaggedVal::Undefined => ValType::Undefined,
        }
    }
}

macro_rules! tagged_value_conversion {
    ($ty:ty, $try_as:ident, $e:tt) => {
        impl TaggedVal {
            #[inline]
            #[allow(dead_code)]
            fn $try_as(&self) -> Option<$ty> {
                if let $e(v) = self {
                    Some(*v)
                } else {
                    None
                }
            }
        }

        impl From<$ty> for TaggedVal {
            #[inline]
            #[allow(dead_code)]
            fn from(v: $ty) -> Self {
                $e(v)
            }
        }
    };
}

tagged_value_conversion! {i32, try_as_i32, I32}
tagged_value_conversion! {i64, try_as_i64, I64}
tagged_value_conversion! {f32, try_as_f32, F32}
tagged_value_conversion! {f64, try_as_f64, F64}

impl From<u32> for TaggedVal {
    #[inline]
    #[allow(dead_code)]
    fn from(v: u32) -> Self {
        I32(v as i32)
    }
}

impl From<u64> for TaggedVal {
    #[inline]
    #[allow(dead_code)]
    fn from(v: u64) -> Self {
        I64(v as i64)
    }
}

trait SafeFloatConv<T> {
    fn try_to_int(self) -> Option<T>;
}
macro_rules! safe_float_conv {
    ($from:ty, $to:ty) => {
        impl SafeFloatConv<$to> for $from {
            fn try_to_int(self) -> Option<$to> {
                if self.is_finite() {
                    Some(self as $to)
                } else {
                    None
                }
            }
        }
    };
    ($to: ty) => {
        safe_float_conv! {f32, $to}
        safe_float_conv! {f64, $to}
    };
}
safe_float_conv! {i32}
safe_float_conv! {u32}
safe_float_conv! {i64}
safe_float_conv! {u64}

#[allow(unused_imports)]
use TaggedVal::*;


#[allow(dead_code)]
         pub struct WasmModule {
            memory: Vec<u8>,
            globals: Vec<TaggedVal>,
            indirect_call_table: Vec<Option<usize>>,
            context: wasi_common::WasiCtx,
         }

macro_rules! memory_accessors {
    ($ty:ty, $read:ident, $write:ident) => {
        #[inline]
        #[allow(dead_code)]
        fn $read(memory: &[u8], addr: usize) -> Option<$ty> {
            Some(<$ty>::from_le_bytes(
                memory.get(addr..addr + std::mem::size_of::<$ty>())?
                    .try_into()
                    .ok()?,
            ))
        }

        #[inline]
        #[allow(dead_code)]
        fn $write(memory: &mut [u8], addr: usize, value: $ty) -> Option<()> {
            memory.get_mut(addr..addr + std::mem::size_of::<$ty>())?
                .copy_from_slice(&value.to_le_bytes());
            Some(())
        }
    };
}

memory_accessors! {u8, read_mem_u8, write_mem_u8}
memory_accessors! {u16, read_mem_u16, write_mem_u16}
memory_accessors! {u32, read_mem_u32, write_mem_u32}
memory_accessors! {u64, read_mem_u64, write_mem_u64}

memory_accessors! {i8, read_mem_i8, write_mem_i8}
memory_accessors! {i16, read_mem_i16, write_mem_i16}
memory_accessors! {i32, read_mem_i32, write_mem_i32}
memory_accessors! {i64, read_mem_i64, write_mem_i64}

memory_accessors! {f32, read_mem_f32, write_mem_f32}
memory_accessors! {f64, read_mem_f64, write_mem_f64}


impl WasmModule {
             #[allow(unused_mut)]
             pub fn new() -> Self {
                 let mut m = WasmModule {
                     memory: vec![0u8; 131072],
                     globals: vec![],
                     indirect_call_table: vec![],
                     context: wasi_common::WasiCtx::new(std::env::args())
                  .expect("Unable to initialize WASI context"), };
                 m.globals.resize_with(1, Default::default);
                 m.globals[0] = TaggedVal::from(70032i32);
                 if m.indirect_call_table.len() < 6 { m.indirect_call_table.resize(6, None) }
m.indirect_call_table[1] = Some(62);
m.indirect_call_table[2] = Some(144);
m.indirect_call_table[3] = Some(131);
m.indirect_call_table[4] = Some(124);
m.indirect_call_table[5] = Some(115);
                 m.memory[1024..3818].copy_from_slice(&[99, 100, 0, 98, 97, 115, 105, 99, 95, 115, 116, 114, 105, 110, 103, 0, 115, 116, 111, 105, 0, 58, 32, 110, 111, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 0, 37, 115, 10, 0, 58, 32, 111, 117, 116, 32, 111, 102, 32, 114, 97, 110, 103, 101, 0, 97, 108, 108, 111, 99, 97, 116, 111, 114, 60, 84, 62, 58, 58, 97, 108, 108, 111, 99, 97, 116, 101, 40, 115, 105, 122, 101, 95, 116, 32, 110, 41, 32, 39, 110, 39, 32, 101, 120, 99, 101, 101, 100, 115, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 115, 105, 122, 101, 0, 240, 14, 0, 0, 83, 117, 99, 99, 101, 115, 115, 0, 73, 108, 108, 101, 103, 97, 108, 32, 98, 121, 116, 101, 32, 115, 101, 113, 117, 101, 110, 99, 101, 0, 68, 111, 109, 97, 105, 110, 32, 101, 114, 114, 111, 114, 0, 82, 101, 115, 117, 108, 116, 32, 110, 111, 116, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 98, 108, 101, 0, 78, 111, 116, 32, 97, 32, 116, 116, 121, 0, 80, 101, 114, 109, 105, 115, 115, 105, 111, 110, 32, 100, 101, 110, 105, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 110, 111, 116, 32, 112, 101, 114, 109, 105, 116, 116, 101, 100, 0, 78, 111, 32, 115, 117, 99, 104, 32, 102, 105, 108, 101, 32, 111, 114, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 78, 111, 32, 115, 117, 99, 104, 32, 112, 114, 111, 99, 101, 115, 115, 0, 70, 105, 108, 101, 32, 101, 120, 105, 115, 116, 115, 0, 86, 97, 108, 117, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 32, 102, 111, 114, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 0, 78, 111, 32, 115, 112, 97, 99, 101, 32, 108, 101, 102, 116, 32, 111, 110, 32, 100, 101, 118, 105, 99, 101, 0, 79, 117, 116, 32, 111, 102, 32, 109, 101, 109, 111, 114, 121, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 115, 121, 0, 73, 110, 116, 101, 114, 114, 117, 112, 116, 101, 100, 32, 115, 121, 115, 116, 101, 109, 32, 99, 97, 108, 108, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 116, 101, 109, 112, 111, 114, 97, 114, 105, 108, 121, 32, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 73, 110, 118, 97, 108, 105, 100, 32, 115, 101, 101, 107, 0, 67, 114, 111, 115, 115, 45, 100, 101, 118, 105, 99, 101, 32, 108, 105, 110, 107, 0, 82, 101, 97, 100, 45, 111, 110, 108, 121, 32, 102, 105, 108, 101, 32, 115, 121, 115, 116, 101, 109, 0, 68, 105, 114, 101, 99, 116, 111, 114, 121, 32, 110, 111, 116, 32, 101, 109, 112, 116, 121, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 115, 101, 116, 32, 98, 121, 32, 112, 101, 101, 114, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 116, 105, 109, 101, 100, 32, 111, 117, 116, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102, 117, 115, 101, 100, 0, 72, 111, 115, 116, 32, 105, 115, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 65, 100, 100, 114, 101, 115, 115, 32, 105, 110, 32, 117, 115, 101, 0, 66, 114, 111, 107, 101, 110, 32, 112, 105, 112, 101, 0, 73, 47, 79, 32, 101, 114, 114, 111, 114, 0, 78, 111, 32, 115, 117, 99, 104, 32, 100, 101, 118, 105, 99, 101, 32, 111, 114, 32, 97, 100, 100, 114, 101, 115, 115, 0, 78, 111, 32, 115, 117, 99, 104, 32, 100, 101, 118, 105, 99, 101, 0, 78, 111, 116, 32, 97, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 73, 115, 32, 97, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 84, 101, 120, 116, 32, 102, 105, 108, 101, 32, 98, 117, 115, 121, 0, 69, 120, 101, 99, 32, 102, 111, 114, 109, 97, 116, 32, 101, 114, 114, 111, 114, 0, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 0, 65, 114, 103, 117, 109, 101, 110, 116, 32, 108, 105, 115, 116, 32, 116, 111, 111, 32, 108, 111, 110, 103, 0, 83, 121, 109, 98, 111, 108, 105, 99, 32, 108, 105, 110, 107, 32, 108, 111, 111, 112, 0, 70, 105, 108, 101, 110, 97, 109, 101, 32, 116, 111, 111, 32, 108, 111, 110, 103, 0, 84, 111, 111, 32, 109, 97, 110, 121, 32, 111, 112, 101, 110, 32, 102, 105, 108, 101, 115, 32, 105, 110, 32, 115, 121, 115, 116, 101, 109, 0, 78, 111, 32, 102, 105, 108, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 66, 97, 100, 32, 102, 105, 108, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 78, 111, 32, 99, 104, 105, 108, 100, 32, 112, 114, 111, 99, 101, 115, 115, 0, 66, 97, 100, 32, 97, 100, 100, 114, 101, 115, 115, 0, 70, 105, 108, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 0, 84, 111, 111, 32, 109, 97, 110, 121, 32, 108, 105, 110, 107, 115, 0, 78, 111, 32, 108, 111, 99, 107, 115, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 100, 101, 97, 100, 108, 111, 99, 107, 32, 119, 111, 117, 108, 100, 32, 111, 99, 99, 117, 114, 0, 83, 116, 97, 116, 101, 32, 110, 111, 116, 32, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 0, 80, 114, 101, 118, 105, 111, 117, 115, 32, 111, 119, 110, 101, 114, 32, 100, 105, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 99, 97, 110, 99, 101, 108, 101, 100, 0, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 111, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100, 0, 78, 111, 32, 109, 101, 115, 115, 97, 103, 101, 32, 111, 102, 32, 100, 101, 115, 105, 114, 101, 100, 32, 116, 121, 112, 101, 0, 73, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 114, 101, 109, 111, 118, 101, 100, 0, 76, 105, 110, 107, 32, 104, 97, 115, 32, 98, 101, 101, 110, 32, 115, 101, 118, 101, 114, 101, 100, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 101, 114, 114, 111, 114, 0, 66, 97, 100, 32, 109, 101, 115, 115, 97, 103, 101, 0, 78, 111, 116, 32, 97, 32, 115, 111, 99, 107, 101, 116, 0, 68, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 100, 100, 114, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 100, 0, 77, 101, 115, 115, 97, 103, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 119, 114, 111, 110, 103, 32, 116, 121, 112, 101, 32, 102, 111, 114, 32, 115, 111, 99, 107, 101, 116, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 110, 111, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 110, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 0, 78, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 0, 65, 100, 100, 114, 101, 115, 115, 32, 102, 97, 109, 105, 108, 121, 32, 110, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 98, 121, 32, 112, 114, 111, 116, 111, 99, 111, 108, 0, 65, 100, 100, 114, 101, 115, 115, 32, 110, 111, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 78, 101, 116, 119, 111, 114, 107, 32, 105, 115, 32, 100, 111, 119, 110, 0, 78, 101, 116, 119, 111, 114, 107, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 115, 101, 116, 32, 98, 121, 32, 110, 101, 116, 119, 111, 114, 107, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 97, 98, 111, 114, 116, 101, 100, 0, 78, 111, 32, 98, 117, 102, 102, 101, 114, 32, 115, 112, 97, 99, 101, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 83, 111, 99, 107, 101, 116, 32, 105, 115, 32, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 83, 111, 99, 107, 101, 116, 32, 110, 111, 116, 32, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 114, 101, 97, 100, 121, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0, 83, 116, 97, 108, 101, 32, 102, 105, 108, 101, 32, 104, 97, 110, 100, 108, 101, 0, 81, 117, 111, 116, 97, 32, 101, 120, 99, 101, 101, 100, 101, 100, 0, 77, 117, 108, 116, 105, 104, 111, 112, 32, 97, 116, 116, 101, 109, 112, 116, 101, 100, 0, 67, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 105, 110, 115, 117, 102, 102, 105, 99, 105, 101, 110, 116, 0, 0, 0, 117, 2, 78, 0, 214, 1, 226, 4, 185, 4, 24, 1, 142, 5, 237, 2, 22, 4, 242, 0, 151, 3, 1, 3, 56, 5, 175, 1, 130, 1, 79, 3, 47, 4, 30, 0, 212, 5, 162, 0, 18, 3, 30, 3, 194, 1, 222, 3, 8, 0, 172, 5, 0, 1, 100, 2, 241, 1, 101, 5, 52, 2, 140, 2, 207, 2, 45, 3, 76, 4, 227, 5, 159, 2, 248, 4, 28, 5, 8, 5, 177, 2, 75, 5, 21, 2, 120, 0, 82, 2, 60, 3, 241, 3, 228, 0, 195, 3, 125, 4, 204, 0, 170, 3, 121, 5, 36, 2, 110, 1, 109, 3, 34, 4, 171, 4, 68, 0, 251, 1, 174, 0, 131, 3, 96, 0, 229, 1, 7, 4, 148, 4, 94, 4, 43, 0, 88, 1, 57, 1, 146, 0, 194, 5, 155, 1, 67, 2, 70, 1, 246, 5, 45, 43, 32, 32, 32, 48, 88, 48, 120, 0, 40, 110, 117, 108, 108, 41, 0, 0, 0, 0, 0, 0, 25, 0, 10, 0, 25, 25, 25, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 17, 10, 25, 25, 25, 3, 10, 7, 0, 1, 27, 9, 11, 24, 0, 0, 9, 6, 11, 0, 0, 11, 0, 6, 25, 0, 0, 0, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 10, 13, 25, 25, 25, 0, 13, 0, 0, 2, 0, 9, 14, 0, 0, 0, 9, 0, 14, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 19, 0, 0, 0, 0, 9, 12, 0, 0, 0, 0, 0, 12, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 4, 15, 0, 0, 0, 0, 9, 16, 0, 0, 0, 0, 0, 16, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 17, 0, 0, 0, 0, 9, 18, 0, 0, 0, 0, 0, 18, 0, 0, 18, 0, 0, 26, 0, 0, 0, 26, 26, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 26, 26, 26, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 23, 0, 0, 0, 0, 9, 20, 0, 0, 0, 0, 0, 20, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 21, 0, 0, 0, 0, 9, 22, 0, 0, 0, 0, 0, 22, 0, 0, 22, 0, 0, 83, 117, 112, 112, 111, 114, 116, 32, 102, 111, 114, 32, 102, 111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 108, 111, 110, 103, 32, 100, 111, 117, 98, 108, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 115, 32, 99, 117, 114, 114, 101, 110, 116, 108, 121, 32, 100, 105, 115, 97, 98, 108, 101, 100, 46, 10, 84, 111, 32, 101, 110, 97, 98, 108, 101, 32, 105, 116, 44, 32, 97, 100, 100, 32, 45, 108, 99, 45, 112, 114, 105, 110, 116, 115, 99, 97, 110, 45, 108, 111, 110, 103, 45, 100, 111, 117, 98, 108, 101, 32, 116, 111, 32, 116, 104, 101, 32, 108, 105, 110, 107, 32, 99, 111, 109, 109, 97, 110, 100, 46, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 45, 48, 88, 43, 48, 88, 32, 48, 88, 45, 48, 120, 43, 48, 120, 32, 48, 120, 0, 105, 110, 102, 0, 73, 78, 70, 0, 110, 97, 110, 0, 78, 65, 78, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 4, 7, 3, 6, 5, 0]);
m.memory[3824..3940].copy_from_slice(&[5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 100, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 14, 0, 0]);
                 m
             }
         }

impl WasmModule {
#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_0(&mut self, arg_0: i32) -> Option<()> {
std::process::exit(arg_0)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_1(&mut self, arg_0: i32, arg_1: i64, arg_2: i32, arg_3: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::fd_seek(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0, arg_1, arg_2, arg_3))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_2(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::fd_write(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0, arg_1, arg_2, arg_3))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_3(&mut self, arg_0: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::fd_close(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_5(&mut self, ) -> Option<()> {
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_6(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;self.func_4()?;
v0 = TaggedVal::from(self.func_12()?);
local_0 = v0.try_as_i32()?;
self.func_113()?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
self.func_0(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_7(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;
let mut local_48 : i32 = 0i32;
let mut local_49 : i32 = 0i32;
let mut local_50 : i32 = 0i32;
let mut local_51 : i32 = 0i32;
let mut local_52 : i32 = 0i32;
let mut local_53 : i32 = 0i32;
let mut local_54 : i32 = 0i32;
let mut local_55 : i32 = 0i32;
let mut local_56 : i32 = 0i32;
let mut local_57 : i32 = 0i32;
let mut local_58 : i32 = 0i32;
let mut local_59 : i32 = 0i32;
let mut local_60 : i32 = 0i32;
let mut local_61 : i32 = 0i32;
let mut local_62 : i32 = 0i32;
let mut local_63 : i32 = 0i32;
let mut local_64 : i32 = 0i32;
let mut local_65 : i32 = 0i32;
let mut local_66 : i32 = 0i32;
let mut local_67 : i32 = 0i32;
let mut local_68 : i32 = 0i32;
let mut local_69 : i32 = 0i32;
let mut local_70 : i32 = 0i32;
let mut local_71 : i32 = 0i32;
let mut local_72 : i32 = 0i32;
let mut local_73 : i32 = 0i32;
let mut local_74 : i32 = 0i32;
let mut local_75 : i32 = 0i32;
let mut local_76 : i32 = 0i32;
let mut local_77 : i32 = 0i32;
let mut local_78 : i32 = 0i32;
let mut local_79 : i32 = 0i32;
let mut local_80 : i32 = 0i32;
let mut local_81 : i32 = 0i32;
let mut local_82 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(3i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_10 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(24i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_18);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_rem(v1.try_as_i32()?)?);
local_21 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(0i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_22);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(24i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_31);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_38);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(24i32);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_46);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_49);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_4;
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(3i32);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_51);
v1 = TaggedVal::from(local_52);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_rem(v1.try_as_i32()?)?);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
local_54 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_53);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(0i32);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(10i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_56);
v1 = TaggedVal::from(local_57);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_60);
v1 = TaggedVal::from(local_61);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_rem(v1.try_as_i32()?)?);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_62);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_55);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
v1 = TaggedVal::from(local_64);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_65);
local_54 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_54);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_66);
v1 = TaggedVal::from(local_67);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_68);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(10i32);
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_69);
v2 = TaggedVal::from(local_70);
v0 = TaggedVal::from(self.func_73(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_71);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(12i32);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_72);
v1 = TaggedVal::from(local_73);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_rem(v1.try_as_i32()?)?);
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_74);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_69);
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_75);
v1 = TaggedVal::from(local_76);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_77);
v1 = TaggedVal::from(local_78);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_79);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_81);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_82);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_80);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_8(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_9(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_8);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_10(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_17(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_18(v0.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_9 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_11);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_11(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_21(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_22(v0.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_9 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_11);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_12(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(1024i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_7(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_62(v0.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_13(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(16i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_10);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v2 = TaggedVal::from(local_14);
self.func_64(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(32i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_11);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_14(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_29(v0.try_as_i32()?)?);

v0 = TaggedVal::from(32i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_15(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_135(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_16(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 11) as usize).and_then(|x| Some(x as i32))?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(255i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(128i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_16);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_17(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_18(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 11) as usize).and_then(|x| Some(x as i32))?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(255i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_8);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_19(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_20(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_20(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_21(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_22(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_24(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_23(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_25(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_24(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_26(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_25(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_26(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_27(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_28(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_29(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_30(v0.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_30(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_31(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_4);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_106(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(self.func_105()?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_15);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
{
                    let rets = self.indirect_call(v0.try_as_i32()? as usize, &[])?;
                    if rets.len() != 0 {
                        return None;
                    }
                 }
{

}
break 'label_3;
break;
}
{

}
break 'label_1;
break;
}
{

}
continue 'label_2;
break;}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_23);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_32(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.func_108(v0.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_33(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(1027i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
self.func_34(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_34(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
self.func_110()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_35(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_40(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(11i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_9 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_13);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_36(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_37(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
self.func_41(v0.try_as_i32()?, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_10);
self.func_42(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(16i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_38(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_6);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_39(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32, arg_5: i32, arg_6: i32, arg_7: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;
let mut local_4 : i32 = arg_4;
let mut local_5 : i32 = arg_5;
let mut local_6 : i32 = arg_6;
let mut local_7 : i32 = arg_7;let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;
let mut local_48 : i32 = 0i32;
let mut local_49 : i32 = 0i32;
let mut local_50 : i32 = 0i32;
let mut local_51 : i32 = 0i32;
let mut local_52 : i32 = 0i32;
let mut local_53 : i32 = 0i32;
let mut local_54 : i32 = 0i32;
let mut local_55 : i32 = 0i32;
let mut local_56 : i32 = 0i32;
let mut local_57 : i32 = 0i32;
let mut local_58 : i32 = 0i32;
let mut local_59 : i32 = 0i32;
let mut local_60 : i32 = 0i32;
let mut local_61 : i32 = 0i32;
let mut local_62 : i32 = 0i32;
let mut local_63 : i32 = 0i32;
let mut local_64 : i32 = 0i32;
let mut local_65 : i32 = 0i32;
let mut local_66 : i32 = 0i32;
let mut local_67 : i32 = 0i32;
let mut local_68 : i32 = 0i32;
let mut local_69 : i32 = 0i32;
let mut local_70 : i32 = 0i32;
let mut local_71 : i32 = 0i32;
let mut local_72 : i32 = 0i32;
let mut local_73 : i32 = 0i32;
let mut local_74 : i32 = 0i32;
let mut local_75 : i32 = 0i32;
let mut local_76 : i32 = 0i32;
let mut local_77 : i32 = 0i32;
let mut local_78 : i32 = 0i32;
let mut local_79 : i32 = 0i32;
let mut local_80 : i32 = 0i32;
let mut local_81 : i32 = 0i32;
let mut local_82 : i32 = 0i32;
let mut local_83 : i32 = 0i32;
let mut local_84 : i32 = 0i32;
let mut local_85 : i32 = 0i32;
let mut local_86 : i32 = 0i32;
let mut local_87 : i32 = 0i32;
let mut local_88 : i32 = 0i32;
let mut local_89 : i32 = 0i32;
let mut local_90 : i32 = 0i32;
let mut local_91 : i32 = 0i32;
let mut local_92 : i32 = 0i32;
let mut local_93 : i32 = 0i32;
let mut local_94 : i32 = 0i32;
let mut local_95 : i32 = 0i32;
let mut local_96 : i32 = 0i32;
let mut local_97 : i32 = 0i32;
let mut local_98 : i32 = 0i32;
let mut local_99 : i32 = 0i32;
let mut local_100 : i32 = 0i32;
let mut local_101 : i32 = 0i32;
let mut local_102 : i32 = 0i32;
let mut local_103 : i32 = 0i32;
let mut local_104 : i32 = 0i32;
let mut local_105 : i32 = 0i32;
let mut local_106 : i32 = 0i32;
let mut local_107 : i32 = 0i32;
let mut local_108 : i32 = 0i32;
let mut local_109 : i32 = 0i32;
let mut local_110 : i32 = 0i32;
let mut local_111 : i32 = 0i32;
let mut local_112 : i32 = 0i32;
let mut local_113 : i32 = 0i32;
let mut local_114 : i32 = 0i32;
let mut local_115 : i32 = 0i32;
let mut local_116 : i32 = 0i32;
let mut local_117 : i32 = 0i32;
let mut local_118 : i32 = 0i32;
let mut local_119 : i32 = 0i32;
let mut local_120 : i32 = 0i32;
let mut local_121 : i32 = 0i32;
let mut local_122 : i32 = 0i32;
let mut local_123 : i32 = 0i32;
let mut local_124 : i32 = 0i32;
let mut local_125 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(64i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 60) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 56) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 52) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 48) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 60) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_43(v0.try_as_i32()?)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_12);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 52) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_23 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_23);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_11);
self.func_33(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_35 = v0.try_as_i32()?;
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_35);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(16i32);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_36);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(12i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 52) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
v1 = TaggedVal::from(local_43);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_44);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_47);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(local_41);
v0 = TaggedVal::from(self.func_44(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_49);
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
local_51 = v0.try_as_i32()?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_52);
v1 = TaggedVal::from(local_53);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
local_51 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_51);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_55);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_57);
v1 = TaggedVal::from(local_58);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_56);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(self.func_47(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_60);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_11);
self.func_48(v0.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_61 = v0.try_as_i32()?;
'label_3: loop {
v0 = TaggedVal::from(local_61);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_62);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_64);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
v1 = TaggedVal::from(local_65);
v2 = TaggedVal::from(local_66);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_67 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_67);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_68);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_69);
v1 = TaggedVal::from(local_70);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_71);
v1 = TaggedVal::from(local_72);
v2 = TaggedVal::from(local_73);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 48) as usize)?);
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_74);
v1 = TaggedVal::from(local_75);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v1 = TaggedVal::from(local_77);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_78);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_79 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_79);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_80);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_81);
v1 = TaggedVal::from(local_82);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_83);
v1 = TaggedVal::from(local_84);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_86);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_87);
v1 = TaggedVal::from(local_88);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_89);
v1 = TaggedVal::from(local_90);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_85);
v1 = TaggedVal::from(local_91);
v2 = TaggedVal::from(local_92);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(11i32);
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_94);
v1 = TaggedVal::from(local_95);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_96 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_96);
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_93);
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_97);
v1 = TaggedVal::from(local_98);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_99);
v1 = TaggedVal::from(local_100);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_101 = v0.try_as_i32()?;
'label_6: loop {
v0 = TaggedVal::from(local_101);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_104);
v1 = TaggedVal::from(local_105);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_106 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_102);
v1 = TaggedVal::from(local_103);
v2 = TaggedVal::from(local_106);
self.func_50(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(3i32);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_107);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_108);
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_111);
self.func_51(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_112);
v1 = TaggedVal::from(local_113);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_114);
self.func_52(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_115 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_115);
v1 = TaggedVal::from(local_116);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_117);
v1 = TaggedVal::from(local_118);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_119);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 48) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 48) as usize)?);
local_120 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_120);
self.func_41(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_121 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 48) as usize)?);
local_122 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_121);
v1 = TaggedVal::from(local_122);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_123 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_110);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_123);
v1 = TaggedVal::from(local_109);
self.func_38(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(64i32);
local_124 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_124);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_125 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_125);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_40(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(2147483647i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_8);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_41(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_42(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_6);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 11) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_43(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_57(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_58(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_9);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_44(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_65(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_7);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_45(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(11i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_10 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(10i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(11i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v0 = TaggedVal::from(self.func_59(v0.try_as_i32()?)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_18);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_24 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_24);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_27);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_28);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_29);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_46(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_61(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_47(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_60(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_7);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_48(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_49(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_6 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_6);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_8 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_133(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_9);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_8);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_12);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_50(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_8);
self.func_63(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_51(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_52(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(-2147483648i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_53(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_54(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_55(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_54(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_90(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_91(v0.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_9 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_11);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_55(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_56(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;
let mut local_48 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_43(v0.try_as_i32()?)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_14 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_14);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_7);
self.func_33(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(11i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_22);
self.func_42(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_22(v0.try_as_i32()?)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_25);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(self.func_47(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_30);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_31);
self.func_51(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(local_33);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_34);
self.func_52(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_35);
self.func_41(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(7i32);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_36);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
v2 = TaggedVal::from(local_43);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_45);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_39);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 7) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_46);
v1 = TaggedVal::from(local_38);
self.func_38(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(32i32);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_47);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_57(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_95(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_58(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_94(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_59(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(15i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(-16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_60(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(-1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_11 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1080i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
self.func_34(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(1i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(self.func_99(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_17);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_61(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_98(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_62(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_21(v0.try_as_i32()?)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_40(v0.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_10);
self.func_50(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_11);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_63(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_6);
self.func_100(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_64(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_43(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_13 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_6);
self.func_33(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(11i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_20 = v0.try_as_i32()?;
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_21);
self.func_42(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_22(v0.try_as_i32()?)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_22);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(self.func_47(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_29);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_30);
self.func_51(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_33);
self.func_52(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_34);
self.func_41(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(11i32);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v1 = TaggedVal::from(local_41);
v2 = TaggedVal::from(local_42);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_38);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 11) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_37);
self.func_38(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(32i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_65(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(8i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_9);
v0 = TaggedVal::from(self.func_79(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_12 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
local_14 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
local_14 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_14);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_16);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_66(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;
let mut local_48 : i32 = 0i32;
let mut local_49 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;
let mut v7: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_35(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_17 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_18 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(7i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v2 = TaggedVal::from(local_29);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_33);
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_22);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 7) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_21);
self.func_38(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_40);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_38);
v2 = TaggedVal::from(local_43);
v3 = TaggedVal::from(local_44);
v4 = TaggedVal::from(local_45);
v5 = TaggedVal::from(local_37);
v6 = TaggedVal::from(local_46);
v7 = TaggedVal::from(local_47);
self.func_39(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?, v7.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(32i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_49);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_67(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(self.func_68(v0.try_as_i32()?)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(self.func_69(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_68(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_69(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_68(v0.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_70(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_57(v0.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_71(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_67(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_6);
self.func_72(v0.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_72(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(3i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_14);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_22);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_1;
break;}
break;
}
v0 = TaggedVal::from(16i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_73(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(8i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(1040i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_11);
v2 = TaggedVal::from(local_12);
v3 = TaggedVal::from(local_13);
v0 = TaggedVal::from(self.func_74(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
{
                    let rets = self.indirect_call(v1.try_as_i32()? as usize, &[v0])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

v0 = TaggedVal::from(32i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_14);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_74(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = self.globals[0];
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(2i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_10);
v3 = TaggedVal::from(local_11);
v4 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_75(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_12);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(self.func_76()?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_19 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_19);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(self.func_77()?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
self.func_78(v0.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_28);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_75(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;
let mut local_4 : i32 = arg_4;let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;
let mut local_34 : i32 = 0i32;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : i32 = 0i32;
let mut local_40 : i32 = 0i32;
let mut local_41 : i32 = 0i32;
let mut local_42 : i32 = 0i32;
let mut local_43 : i32 = 0i32;
let mut local_44 : i32 = 0i32;
let mut local_45 : i32 = 0i32;
let mut local_46 : i32 = 0i32;
let mut local_47 : i32 = 0i32;
let mut local_48 : i32 = 0i32;
let mut local_49 : i32 = 0i32;
let mut local_50 : i32 = 0i32;
let mut local_51 : i32 = 0i32;
let mut local_52 : i32 = 0i32;
let mut local_53 : i32 = 0i32;
let mut local_54 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(68i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(24i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_15);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v0 = TaggedVal::from(self.func_80(v0.try_as_i32()?)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(unsafe {PARAM1 as i32});
v0 = TaggedVal::from(self.func_81()?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_19);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(unsafe {PARAM1 as i32});
v0 = TaggedVal::from(self.func_81()?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_15);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_14);
v2 = TaggedVal::from(local_23);
v3 = TaggedVal::from(local_21);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(unsafe {PARAM1 as i32});
v0 = TaggedVal::from(self.func_81()?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_11);
self.func_82(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_31 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_31);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
self.func_78(v0.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(local_36);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_38);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_39 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_39);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
self.func_83(v0.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_47 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_47);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
v1 = TaggedVal::from(local_49);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_51);
v1 = TaggedVal::from(local_50);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_53);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_52);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_76(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(self.func_84()?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_77(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(self.func_85()?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_78(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1065i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_6);
self.func_86(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
self.func_87(v0.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
{
                    let rets = self.indirect_call(v1.try_as_i32()? as usize, &[v0])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_79(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_80(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_53(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_81(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
self.func_5()?;
break;
}
v0 = TaggedVal::from(4440i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_82(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(4i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(self.func_88(v0.try_as_i32()?)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_10);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_88(v0.try_as_i32()?)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_13);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(self.func_88(v0.try_as_i32()?)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_16);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_83(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1045i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_6);
self.func_86(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
self.func_89(v0.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
{
                    let rets = self.indirect_call(v1.try_as_i32()? as usize, &[v0])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_84(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(-2147483648i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_85(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(2147483647i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_86(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i32 = 0i32;
let mut local_33 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(1i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_12);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 19) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
self.func_70(v0.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(self.func_71(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_15);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from(self.func_53(v0.try_as_i32()?)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_20);
v3 = TaggedVal::from(local_23);
self.func_56(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_24);
v2 = TaggedVal::from(local_25);
v0 = TaggedVal::from(self.func_66(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(1i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_27);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 19) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 19) as usize).and_then(|x| Some(x as i32))?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_30 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_30);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_31);
{
                    let rets = self.indirect_call(v1.try_as_i32()? as usize, &[v0])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(32i32);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_87(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1148) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_80(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(1061i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
v0 = TaggedVal::from(self.func_129(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

self.func_110()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_88(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_89(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1148) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_80(v0.try_as_i32()?)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(1061i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
v0 = TaggedVal::from(self.func_129(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

self.func_110()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_90(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_91(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_92(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_92(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_93(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_93(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_94(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_96(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_95(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_97(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_96(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_97(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_98(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_99(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_31(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_100(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_8);
self.func_101(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_101(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
self.func_102(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_102(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.func_103(v0.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_103(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
self.func_32(v0.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_104(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_105(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(3940i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_104(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_106(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_107(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_107(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
'label_0: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
v1 = TaggedVal::from(70032i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(89i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4416) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 4428) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 4420) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4416) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4436) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4388) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4396) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(70032i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4392) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(70032i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3960) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3980) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3976) as usize, v1.try_as_i32()?)?;
'label_2: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3992i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3996i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(256i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_2;
}
break;}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(70040i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(70040i32);
v3 = TaggedVal::from(15i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(70036i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(-56i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4432) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3972) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(70032i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(69980i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(56i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
'label_3: loop {
'label_4: loop {
'label_5: loop {
'label_6: loop {
'label_7: loop {
'label_8: loop {
'label_9: loop {
'label_10: loop {
'label_11: loop {
'label_12: loop {
'label_13: loop {
'label_14: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(236i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
'label_15: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3944) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(19i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(-16i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(11i32);
v3 = TaggedVal::from(((v3.try_as_i32()? as u32) < (v4.try_as_i32()? as u32)) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_2 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_15;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(3992i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_16: loop {
'label_17: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
{

}
break 'label_16;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3952) as usize)?);
local_7 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
'label_18: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
'label_19: loop {
'label_20: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(2i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(5i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(3992i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_0);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_5 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
'label_21: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
local_4 = v0.try_as_i32()?;
'label_22: loop {
'label_23: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_8);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_23;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_8);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
local_8 = v0.try_as_i32()?;
{

}
break 'label_22;
break;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3948) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_9);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(5i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_0 = v0.try_as_i32()?;
'label_24: loop {
'label_25: loop {
'label_26: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_24;
}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) < (v3.try_as_i32()? as u32)) as i32);
local_0 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_0);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_0 = v0.try_as_i32()?;
{

}
continue 'label_25;
break;}
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_10 = v0.try_as_i32()?;
'label_27: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_27;
}
'label_28: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_6);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_28;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);

break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_4;
break;
}
'label_29: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_29;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
break;
}
'label_30: loop {
v0 = TaggedVal::from(local_0);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_30;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_30;
}
break;}
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
break 'label_4;
break;
}
v0 = TaggedVal::from(-1i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-65i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(19i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3948) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(0i32);
local_11 = v0.try_as_i32()?;
'label_31: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
v0 = TaggedVal::from(31i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(16777215i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1048320i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(520192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(245760i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(21i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_32: loop {
'label_33: loop {
'label_34: loop {
'label_35: loop {
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
{

}
break 'label_34;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(25i32);
v3 = TaggedVal::from(local_11);
v4 = TaggedVal::from(1i32);
v3 = TaggedVal::from((v3.try_as_i32()? as u32) >> (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(local_11);
v4 = TaggedVal::from(31i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
'label_36: loop {
'label_37: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_37;
}
v0 = TaggedVal::from(local_5);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
if v0.try_as_i32()? != 0 {
{

}
break 'label_37;
}
v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_3 = v0.try_as_i32()?;
{

}
break 'label_33;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(20i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(local_6);
v5 = TaggedVal::from(29i32);
v4 = TaggedVal::from((v4.try_as_i32()? as u32) >> (v5.try_as_i32()? % 32));
v5 = TaggedVal::from(4i32);
v4 = TaggedVal::from(v4.try_as_i32()? & v5.try_as_i32()?);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(16i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 0) as usize)?);
local_4 = v3.try_as_i32()?;
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_5);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(0i32);
v1 = TaggedVal::from((v1.try_as_i32()? != v2.try_as_i32()?) as i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_36;
}
break;}
break;
}
'label_38: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_38;
}
v0 = TaggedVal::from(2i32);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(5i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_32;
}
break;
}
'label_39: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_6 = v0.try_as_i32()?;
'label_40: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_40;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_6);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_6);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_39;
}
break;}
break;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3952) as usize)?);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_11 = v0.try_as_i32()?;
'label_41: loop {
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_41;
}
'label_42: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_8);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_42;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);

break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_5;
break;
}
'label_43: loop {
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_43;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_11;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
break;
}
'label_44: loop {
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_44;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_44;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
break 'label_5;
break;
}
'label_45: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3952) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_45;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
local_4 = v0.try_as_i32()?;
'label_46: loop {
'label_47: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_47;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_46;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
'label_48: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3956) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_48;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
'label_49: loop {
'label_50: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4416) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_50;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4424) as usize)?);
local_4 = v0.try_as_i32()?;
{

}
break 'label_49;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 4428) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 4420) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4416) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4436) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4388) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(65536i32);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
'label_51: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(71i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_7 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_11 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_51;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(48i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
'label_52: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4384) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
'label_53: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4376) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_53;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(48i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 4388) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
'label_54: loop {
'label_55: loop {
'label_56: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(4392i32);
local_3 = v0.try_as_i32()?;
'label_57: loop {
'label_58: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_58;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_55;
}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_57;
}
break;}
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_8);
local_5 = v0.try_as_i32()?;
'label_59: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4420) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_59;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(2147483646i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
'label_60: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4384) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_60;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4376) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
{

}
break 'label_7;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(2147483646i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(local_3);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_6);
local_3 = v0.try_as_i32()?;
break;
}
'label_61: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(72i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_61;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_61;
}
'label_62: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4424) as usize)?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(2147483646i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_62;
}
v0 = TaggedVal::from(local_3);
local_6 = v0.try_as_i32()?;
{

}
break 'label_7;
break;
}
'label_63: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_63;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_6 = v0.try_as_i32()?;
{

}
break 'label_7;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);

{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_3);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
{

}
break 'label_4;
break;
}
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
{

}
break 'label_5;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4388) as usize)?);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4388) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(2147483646i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(self.func_111(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(self.func_111(v1.try_as_i32()?)?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(56i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4376) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4376) as usize, v1.try_as_i32()?)?;
'label_64: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4380) as usize)?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_64;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4380) as usize, v1.try_as_i32()?)?;
break;
}
'label_65: loop {
'label_66: loop {
'label_67: loop {
'label_68: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_68;
}
v0 = TaggedVal::from(4392i32);
local_3 = v0.try_as_i32()?;
'label_69: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_3);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
local_8 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_67;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_69;
}
{

}
break 'label_66;
break;}
break;
}
'label_70: loop {
'label_71: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_71;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_70;
}
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3960) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4396) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4392) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3976) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4416) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3980) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4404) as usize, v1.try_as_i32()?)?;
'label_72: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3992i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3996i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(256i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_72;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-8i32);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_6);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(15i32);
v3 = TaggedVal::from(v3.try_as_i32()? & v4.try_as_i32()?);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(-56i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4432) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3972) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(56i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 12) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_66;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_66;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_66;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-8i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(15i32);
v3 = TaggedVal::from(v3.try_as_i32()? & v4.try_as_i32()?);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3956) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_11 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4432) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3972) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(56i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_65;
break;
}
'label_73: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3960) as usize)?);
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_73;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3960) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(4392i32);
local_3 = v0.try_as_i32()?;
'label_74: loop {
'label_75: loop {
'label_76: loop {
'label_77: loop {
'label_78: loop {
'label_79: loop {
'label_80: loop {
'label_81: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_80;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_81;
}
{

}
break 'label_79;
break;}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 12) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_78;
}
break;
}
v0 = TaggedVal::from(4392i32);
local_3 = v0.try_as_i32()?;
'label_82: loop {
'label_83: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_83;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_77;
}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
{

}
continue 'label_82;
break;}
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-8i32);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_6);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(15i32);
v3 = TaggedVal::from(v3.try_as_i32()? & v4.try_as_i32()?);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-8i32);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(15i32);
v3 = TaggedVal::from(v3.try_as_i32()? & v4.try_as_i32()?);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_84: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_84;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3956) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_75;
break;
}
'label_85: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_85;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3952) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
break 'label_75;
break;
}
'label_86: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_86;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_7 = v0.try_as_i32()?;
'label_87: loop {
'label_88: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_88;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_2 = v0.try_as_i32()?;
'label_89: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_9 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_89;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
'label_90: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_90;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3944) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_9);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
{

}
break 'label_87;
break;
}
'label_91: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_91;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_87;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_9 = v0.try_as_i32()?;
'label_92: loop {
'label_93: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_93;
}
'label_94: loop {
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_6);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_94;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);

break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_92;
break;
}
'label_95: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_95;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_95;
}
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
{

}
break 'label_92;
break;
}
'label_96: loop {
v0 = TaggedVal::from(local_4);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_96;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_96;
}
break;}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_9);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_87;
}
'label_97: loop {
'label_98: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_98;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
if v0.try_as_i32()? != 0 {
{

}
break 'label_97;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3948) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
{

}
break 'label_87;
break;
}
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(20i32);
v3 = TaggedVal::from(local_9);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 16) as usize)?);
v4 = TaggedVal::from(local_6);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_87;
}
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_99: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_99;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_87;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
'label_100: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_100;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_101: loop {
'label_102: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3944) as usize)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_102;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
local_4 = v0.try_as_i32()?;
{

}
break 'label_101;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_75;
break;
}
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
'label_103: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_103;
}
v0 = TaggedVal::from(31i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16777215i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_103;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(1048320i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(520192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(245760i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(21i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
'label_104: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3948) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_104;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_8);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_75;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(25i32);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(1i32);
v3 = TaggedVal::from((v3.try_as_i32()? as u32) >> (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(31i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
'label_105: loop {
v0 = TaggedVal::from(local_6);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_76;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(29i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_105;
}
break;}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_75;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-8i32);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_6);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(15i32);
v3 = TaggedVal::from(v3.try_as_i32()? & v4.try_as_i32()?);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(-56i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(56i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(55i32);
v3 = TaggedVal::from(local_0);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(15i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
v3 = TaggedVal::from(0i32);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(-55i32);
v4 = TaggedVal::from(v4.try_as_i32()?.wrapping_add(v5.try_as_i32()?));
v5 = TaggedVal::from(15i32);
v4 = TaggedVal::from(v4.try_as_i32()? & v5.try_as_i32()?);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-63i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(16i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) < (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(35i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4432) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3972) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 4400) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 4392) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4400) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4396) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4392) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4404) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(36i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_106: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(7i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_106;
}
break;}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_65;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_8);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_5 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
'label_107: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_107;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_108: loop {
'label_109: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3944) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_109;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
local_0 = v0.try_as_i32()?;
{

}
break 'label_108;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_0 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
'label_110: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_110;
}
v0 = TaggedVal::from(31i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(16777215i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_110;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1048320i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(520192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(245760i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(21i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_111: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3948) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_111;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_8);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(25i32);
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(1i32);
v3 = TaggedVal::from((v3.try_as_i32()? as u32) >> (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(31i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
'label_112: loop {
v0 = TaggedVal::from(local_6);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_74;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(29i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_6 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_112;
}
break;}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3956) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(48i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
'label_113: loop {
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_113;
}
'label_114: loop {
'label_115: loop {
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_8);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 28) as usize)?);
local_4 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4248i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_115;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
if v0.try_as_i32()? != 0 {
{

}
break 'label_114;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_7 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
{

}
break 'label_113;
break;
}
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(20i32);
v3 = TaggedVal::from(local_11);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 16) as usize)?);
v4 = TaggedVal::from(local_8);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_113;
}
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_116: loop {
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_116;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_113;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
'label_117: loop {
'label_118: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_118;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_117;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_119: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_119;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_120: loop {
'label_121: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3944) as usize)?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_121;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
local_4 = v0.try_as_i32()?;
{

}
break 'label_120;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_117;
break;
}
'label_122: loop {
'label_123: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_123;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
{

}
break 'label_122;
break;
}
v0 = TaggedVal::from(31i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(16777215i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_122;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(1048320i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(520192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(245760i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(21i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
'label_124: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_124;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_117;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(25i32);
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(1i32);
v3 = TaggedVal::from((v3.try_as_i32()? as u32) >> (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(31i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
'label_125: loop {
'label_126: loop {
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_125;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(29i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_126;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_117;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
'label_127: loop {
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_127;
}
'label_128: loop {
'label_129: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_6);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 28) as usize)?);
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4248i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_129;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
if v0.try_as_i32()? != 0 {
{

}
break 'label_128;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_0);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
{

}
break 'label_127;
break;
}
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(20i32);
v3 = TaggedVal::from(local_10);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 16) as usize)?);
v4 = TaggedVal::from(local_6);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_127;
}
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_130: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_130;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_127;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
'label_131: loop {
'label_132: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_132;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_131;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_133: loop {
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_133;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
local_3 = v0.try_as_i32()?;
'label_134: loop {
'label_135: loop {
v0 = TaggedVal::from(1i32);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_135;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
local_8 = v0.try_as_i32()?;
{

}
break 'label_134;
break;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_108(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
self.func_109(v0.try_as_i32()?)?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_109(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(-4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_2 = v1.try_as_i32()?;
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_0 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3960) as usize)?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_3: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_7 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
'label_5: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3944) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_7);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
'label_6: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_7 = v0.try_as_i32()?;
'label_7: loop {
'label_8: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
'label_9: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);

break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_7;
break;
}
'label_10: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
{

}
break 'label_7;
break;
}
'label_11: loop {
v0 = TaggedVal::from(local_2);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_11;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_11;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
'label_12: loop {
'label_13: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3948) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(20i32);
v3 = TaggedVal::from(local_7);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 16) as usize)?);
v4 = TaggedVal::from(local_1);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_14: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
'label_15: loop {
'label_16: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
'label_17: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3968) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3968) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3956) as usize)?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3956) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3964) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
'label_18: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3964) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3964) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3952) as usize)?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_19: loop {
'label_20: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
'label_21: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(3984i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
'label_22: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_22;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3944) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
'label_23: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_23;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_7 = v0.try_as_i32()?;
'label_24: loop {
'label_25: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_25;
}
'label_26: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3960) as usize)?);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);

break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_24;
break;
}
'label_27: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_27;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_27;
}
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
{

}
break 'label_24;
break;
}
'label_28: loop {
v0 = TaggedVal::from(local_2);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_28;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_28;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
'label_29: loop {
'label_30: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_30;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
if v0.try_as_i32()? != 0 {
{

}
break 'label_29;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3948) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(16i32);
v2 = TaggedVal::from(20i32);
v3 = TaggedVal::from(local_7);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 16) as usize)?);
v4 = TaggedVal::from(local_3);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_31: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3964) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_15;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3952) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
'label_32: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_32;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(3984i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_33: loop {
'label_34: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3944) as usize)?);
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_34;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3944) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
local_2 = v0.try_as_i32()?;
{

}
break 'label_33;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
'label_35: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
v0 = TaggedVal::from(31i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(16777215i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(1048320i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(520192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(245760i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_5 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(21i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(4248i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
'label_36: loop {
'label_37: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3948) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_3 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_37;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3948) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_36;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(25i32);
v3 = TaggedVal::from(local_2);
v4 = TaggedVal::from(1i32);
v3 = TaggedVal::from((v3.try_as_i32()? as u32) >> (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(local_2);
v4 = TaggedVal::from(31i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_5 = v0.try_as_i32()?;
'label_38: loop {
'label_39: loop {
v0 = TaggedVal::from(local_5);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_38;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(29i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_5 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_39;
}
break;}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_36;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 3976) as usize)?);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3976) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(4400i32);
local_1 = v0.try_as_i32()?;
'label_40: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_40;
}
break;}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3976) as usize, v1.try_as_i32()?)?;
break;
}Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_110(&mut self, ) -> Option<()> {
unreachable!("Reached a point explicitly marked unreachable in WASM module");
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_111(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;
let mut v1: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from((self.memory.len() / 65536) as i32);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
return Some(v0.try_as_i32()?);
break;
}
'label_1: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(65535i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
'label_2: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
{
                         let orig_size = (self.memory.len() / 65536);
                         self.memory.resize_with(self.memory.len() + (65536 * v0.try_as_i32()? as usize),
                                                 Default::default);
                         v0 = TaggedVal::from(orig_size as i32);
                     }
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(48i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
return Some(v0.try_as_i32()?);
break;
}
self.func_110()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_112(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_113(&mut self, ) -> Option<()> {
self.func_112()?;
self.func_121()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_114(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(255i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v0 = TaggedVal::from(self.func_1(v0.try_as_i32()?, v1.try_as_i64()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(70i32);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(76i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i64);
local_1 = v0.try_as_i64()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_1 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_1);Some(v0.try_as_i64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_115(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;
let mut local_2 : i32 = arg_2;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_114(v0.try_as_i32()?, v1.try_as_i64()?, v2.try_as_i32()?)?);Some(v0.try_as_i64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_116(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 60) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 60) as usize, v1.try_as_i32()?)?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(32i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 40) as usize)?);
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 44) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_117(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_116(v0.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_3 = v0.try_as_i32()?;
break;
}
'label_2: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
local_5 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(local_2);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
'label_3: loop {
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 64) as usize)?);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_4;
}
break;}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_6 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_2);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_133(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_118(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_5 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(self.func_116(v0.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_5 = v0.try_as_i32()?;
break;
}
'label_2: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(local_3);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
local_5 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_7 = v0.try_as_i32()?;
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 64) as usize)?);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
'label_5: loop {
'label_6: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_5;
}
break;}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(local_10);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_7 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_3);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_6 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_133(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_3);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
break;
}
'label_7: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_1);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? as u32).checked_div(v1.try_as_i32()? as u32)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_119(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4476) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(4452i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(4452i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4476) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(76i32);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) > (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(2704i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u16(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(1152i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
v0 = TaggedVal::from(self.func_138(v0.try_as_i32()?, v1.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_120(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_135(v0.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(1i32);
v5 = TaggedVal::from(local_2);
v6 = TaggedVal::from(local_1);
v3 = TaggedVal::from(self.func_118(v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?);
v2 = TaggedVal::from((v2.try_as_i32()? != v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_121(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(self.func_122()?);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
'label_3: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 36) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 52) as usize)?);
local_0 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_1;
}
break;}
break;
}
'label_4: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4480) as usize)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
'label_5: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 36) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
'label_6: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4480) as usize)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
'label_7: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 36) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
'label_8: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3936) as usize)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
'label_9: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 36) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_122(&mut self, ) -> Option<i32> {
let mut v0: TaggedVal;v0 = TaggedVal::from(4484i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_123(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(-1i32);
local_4 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(28i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
'label_2: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(12i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v0 = TaggedVal::from(self.func_2(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
local_4 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_124(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(2i32);
local_4 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 56) as usize)?);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(2i32);
v1 = TaggedVal::from(self.func_123(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_3);
local_1 = v0.try_as_i32()?;
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(32i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_1);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
local_7 = v2.try_as_i32()?;
v1 = TaggedVal::from(((v1.try_as_i32()? as u32) > (v2.try_as_i32()? as u32)) as i32);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_9);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(local_6);
v3 = TaggedVal::from(local_7);
v4 = TaggedVal::from(0i32);
v5 = TaggedVal::from(local_8);
if ValType::from(v3) != ValType::from(v4) {
                     return None;
                 }
                 if v5.try_as_i32()? != 0 {
                     v3 = v3;
                 } else {
                     v3 = v4;
                 }
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_7 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(12i32);
v2 = TaggedVal::from(4i32);
v3 = TaggedVal::from(local_8);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_9);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(local_7);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 56) as usize)?);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_1);
v4 = TaggedVal::from(local_8);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_1 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(local_8);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_sub(v4.try_as_i32()?));
local_4 = v3.try_as_i32()?;
v1 = TaggedVal::from(self.func_123(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_6 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_2;
}
break;}
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 40) as usize)?);
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 44) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
local_6 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_6);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_125(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 204) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(160i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(184i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(176i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 168) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 160) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 200) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(200i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(80i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_3);
v5 = TaggedVal::from(160i32);
v4 = TaggedVal::from(v4.try_as_i32()?.wrapping_add(v5.try_as_i32()?));
v0 = TaggedVal::from(self.func_126(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(-1i32);
local_0 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 60) as usize)?);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(-33i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
'label_3: loop {
'label_4: loop {
'label_5: loop {
'label_6: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(80i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
{

}
break 'label_5;
break;
}
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
break;
}
v0 = TaggedVal::from(-1i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_116(v0.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(200i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(80i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_3);
v5 = TaggedVal::from(160i32);
v4 = TaggedVal::from(v4.try_as_i32()?.wrapping_add(v5.try_as_i32()?));
v0 = TaggedVal::from(self.func_126(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_1 = v0.try_as_i32()?;
'label_7: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-1i32);
v2 = TaggedVal::from(local_5);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(32i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_0 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_126(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;
let mut local_4 : i32 = arg_4;let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;
let mut local_23 : i32 = 0i32;
let mut local_24 : i32 = 0i32;
let mut local_25 : i32 = 0i32;
let mut local_26 : i32 = 0i32;
let mut local_27 : i32 = 0i32;
let mut local_28 : i32 = 0i32;
let mut local_29 : i32 = 0i32;
let mut local_30 : i32 = 0i32;
let mut local_31 : i32 = 0i32;
let mut local_32 : i64 = 0i64;
let mut local_33 : i64 = 0i64;
let mut local_34 : f64 = 0f64;
let mut local_35 : i32 = 0i32;
let mut local_36 : i32 = 0i32;
let mut local_37 : i32 = 0i32;
let mut local_38 : i32 = 0i32;
let mut local_39 : f64 = 0f64;
let mut local_40 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(880i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(55i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(660i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(368i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(656i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(324i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(56i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_1);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(2147483647i32);
v2 = TaggedVal::from(local_16);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
'label_4: loop {
'label_5: loop {
'label_6: loop {
'label_7: loop {
'label_8: loop {
'label_9: loop {
'label_10: loop {
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_18);
local_1 = v0.try_as_i32()?;
'label_11: loop {
'label_12: loop {
'label_13: loop {
'label_14: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(37i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(local_1);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
local_17 = v0.try_as_i32()?;
'label_15: loop {
'label_16: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(37i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_17);
local_1 = v0.try_as_i32()?;
{

}
break 'label_13;
break;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(37i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_15;
}
{

}
break 'label_13;
break;}
break;
}
v0 = TaggedVal::from(local_1);
local_19 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(2147483647i32);
v2 = TaggedVal::from(local_16);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_20 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_17: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_17);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
'label_18: loop {
'label_19: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
v0 = TaggedVal::from(-1i32);
local_23 = v0.try_as_i32()?;
{

}
break 'label_18;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_1);
v2 = TaggedVal::from(read_mem_u8(&self.memory, (v2.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
v3 = TaggedVal::from(36i32);
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
local_19 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(-1i32);
v2 = TaggedVal::from(local_19);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
v1 = TaggedVal::from(local_15);
v2 = TaggedVal::from(local_19);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_19);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_21 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
'label_20: loop {
'label_21: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(local_17);
local_1 = v0.try_as_i32()?;
{

}
break 'label_20;
break;
}
'label_22: loop {
v0 = TaggedVal::from(1i32);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(75913i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_22;
}
v0 = TaggedVal::from(local_17);
local_1 = v0.try_as_i32()?;
{

}
break 'label_20;
break;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
'label_23: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_1);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(75913i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_23;
}
break;}
break;
}
'label_24: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(42i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_24;
}
'label_25: loop {
'label_26: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(36i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(10i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(-384i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_15 = v0.try_as_i32()?;
{

}
break 'label_25;
break;
}
v0 = TaggedVal::from(local_15);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
'label_27: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_27;
}
v0 = TaggedVal::from(0i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_24 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_15 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_19 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(0i32);
local_24 = v0.try_as_i32()?;
'label_28: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_28;
}
v0 = TaggedVal::from(local_1);
local_22 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(0i32);
local_24 = v0.try_as_i32()?;
'label_29: loop {
'label_30: loop {
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(214748364i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_30;
}
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(local_24);
v2 = TaggedVal::from(10i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_22 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(2147483647i32);
v4 = TaggedVal::from(local_22);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_sub(v4.try_as_i32()?));
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_29;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_29;
}
{

}
break 'label_2;
break;}
break;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
{

}
continue 'label_11;
break;}
break;
}
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
'label_31: loop {
v0 = TaggedVal::from(local_15);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
v0 = TaggedVal::from(0i32);
local_16 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
'label_32: loop {
'label_33: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_33;
}
v0 = TaggedVal::from(1i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_34: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_34;
}
v0 = TaggedVal::from(2i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_35: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
v0 = TaggedVal::from(3i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_36: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_36;
}
v0 = TaggedVal::from(4i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_37: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_37;
}
v0 = TaggedVal::from(5i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(40i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_38: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_38;
}
v0 = TaggedVal::from(6i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_39: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_39;
}
v0 = TaggedVal::from(7i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(56i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_40: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_40;
}
v0 = TaggedVal::from(8i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
'label_41: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_41;
}
v0 = TaggedVal::from(9i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_32;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(72i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(1i32);
local_16 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_1 = v0.try_as_i32()?;
'label_42: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(40i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_42;
}
break;}
v0 = TaggedVal::from(1i32);
local_16 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_21 = v0.try_as_i32()?;
'label_43: loop {
'label_44: loop {
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(46i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_44;
}
v0 = TaggedVal::from(local_22);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_25 = v0.try_as_i32()?;
{

}
break 'label_43;
break;
}
'label_45: loop {
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(42i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_45;
}
'label_46: loop {
'label_47: loop {
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_47;
}
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(36i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_47;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(10i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(-384i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_21 = v0.try_as_i32()?;
{

}
break 'label_46;
break;
}
v0 = TaggedVal::from(local_15);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_48: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_48;
}
v0 = TaggedVal::from(0i32);
local_21 = v0.try_as_i32()?;
{

}
break 'label_46;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_22 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_21 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_25 = v0.try_as_i32()?;
{

}
break 'label_43;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_49: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_49;
}
v0 = TaggedVal::from(1i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_21 = v0.try_as_i32()?;
{

}
break 'label_43;
break;
}
v0 = TaggedVal::from(0i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
local_22 = v0.try_as_i32()?;
'label_50: loop {
v0 = TaggedVal::from(-1i32);
local_21 = v0.try_as_i32()?;
'label_51: loop {
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(214748364i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_51;
}
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(10i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_26);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_26);
v3 = TaggedVal::from(2147483647i32);
v4 = TaggedVal::from(local_1);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_sub(v4.try_as_i32()?));
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_21 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(1i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_50;
}
break;}
break;
}
'label_52: loop {
v0 = TaggedVal::from(local_17);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(-65i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(57i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(58i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(2880i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_52;
}
break;}
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(27i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(28i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_23);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_23);
v3 = TaggedVal::from(3i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 56) as usize, v1.try_as_i64()?)?;
break;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
{

}
break 'label_4;
break;
}
'label_53: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_53;
}
v0 = TaggedVal::from(0i32);
local_16 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(56i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_2);
self.func_127(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-65537i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_28 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_19);
v3 = TaggedVal::from(8192i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_23 = v0.try_as_i32()?;
'label_54: loop {
'label_55: loop {
'label_56: loop {
'label_57: loop {
'label_58: loop {
'label_59: loop {
'label_60: loop {
'label_61: loop {
'label_62: loop {
'label_63: loop {
'label_64: loop {
'label_65: loop {
'label_66: loop {
'label_67: loop {
'label_68: loop {
'label_69: loop {
'label_70: loop {
'label_71: loop {
'label_72: loop {
'label_73: loop {
'label_74: loop {
'label_75: loop {
'label_76: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(-33i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(15i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
v3 = TaggedVal::from(3i32);
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_22);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_29 = v0.try_as_i32()?;
v1 = TaggedVal::from(-65i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
match v0.try_as_i32()? {
0 => {
{

}
break 'label_60;
},
1 => {
{

}
break 'label_59;
},
2 => {
{

}
break 'label_63;
},
3 => {
{

}
break 'label_59;
},
4 => {
{

}
break 'label_60;
},
5 => {
{

}
break 'label_60;
},
6 => {
{

}
break 'label_60;
},
7 => {
{

}
break 'label_59;
},
8 => {
{

}
break 'label_59;
},
9 => {
{

}
break 'label_59;
},
10 => {
{

}
break 'label_59;
},
11 => {
{

}
break 'label_59;
},
12 => {
{

}
break 'label_59;
},
13 => {
{

}
break 'label_59;
},
14 => {
{

}
break 'label_59;
},
15 => {
{

}
break 'label_59;
},
16 => {
{

}
break 'label_59;
},
17 => {
{

}
break 'label_59;
},
18 => {
{

}
break 'label_64;
},
19 => {
{

}
break 'label_59;
},
20 => {
{

}
break 'label_59;
},
21 => {
{

}
break 'label_59;
},
22 => {
{

}
break 'label_59;
},
23 => {
{

}
break 'label_73;
},
24 => {
{

}
break 'label_59;
},
25 => {
{

}
break 'label_59;
},
26 => {
{

}
break 'label_59;
},
27 => {
{

}
break 'label_59;
},
28 => {
{

}
break 'label_59;
},
29 => {
{

}
break 'label_59;
},
30 => {
{

}
break 'label_59;
},
31 => {
{

}
break 'label_59;
},
32 => {
{

}
break 'label_60;
},
33 => {
{

}
break 'label_59;
},
34 => {
{

}
break 'label_68;
},
35 => {
{

}
break 'label_71;
},
36 => {
{

}
break 'label_60;
},
37 => {
{

}
break 'label_60;
},
38 => {
{

}
break 'label_60;
},
39 => {
{

}
break 'label_59;
},
40 => {
{

}
break 'label_71;
},
41 => {
{

}
break 'label_59;
},
42 => {
{

}
break 'label_59;
},
43 => {
{

}
break 'label_59;
},
44 => {
{

}
break 'label_67;
},
45 => {
{

}
break 'label_75;
},
46 => {
{

}
break 'label_72;
},
47 => {
{

}
break 'label_74;
},
48 => {
{

}
break 'label_59;
},
49 => {
{

}
break 'label_59;
},
50 => {
{

}
break 'label_66;
},
51 => {
{

}
break 'label_59;
},
52 => {
{

}
break 'label_76;
},
53 => {
{

}
break 'label_59;
},
54 => {
{

}
break 'label_59;
},
55 => {
{

}
break 'label_73;
},
_ => {
{

}
break 'label_59;
},
}
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_32 = v0.try_as_i64()?;
{

}
break 'label_70;
break;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
'label_77: loop {
'label_78: loop {
'label_79: loop {
'label_80: loop {
'label_81: loop {
'label_82: loop {
'label_83: loop {
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
match v0.try_as_i32()? {
0 => {
{

}
break 'label_83;
},
1 => {
{

}
break 'label_82;
},
2 => {
{

}
break 'label_81;
},
3 => {
{

}
break 'label_80;
},
4 => {
{

}
break 'label_79;
},
5 => {
{

}
continue 'label_3;
},
6 => {
{

}
break 'label_78;
},
7 => {
{

}
break 'label_77;
},
_ => {
{

}
continue 'label_3;
},
}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
write_mem_u16(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u16)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_16);
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(8i32);
v2 = TaggedVal::from(local_21);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) > (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(120i32);
local_29 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
'label_84: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_32 = v0.try_as_i64()?;
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_84;
}
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
{

}
break 'label_69;
break;
}
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
'label_85: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_32);
v1 = TaggedVal::from(v1.try_as_i64()? as i32);
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(3488i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(4i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
local_32 = v0.try_as_i64()?;
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_85;
}
break;}
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_69;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_69;
}
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(2858i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_30 = v0.try_as_i32()?;
{

}
break 'label_69;
break;
}
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
'label_86: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_32 = v0.try_as_i64()?;
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_86;
}
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
'label_87: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_32);
v1 = TaggedVal::from(v1.try_as_i64()? as i32);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(3i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
local_32 = v0.try_as_i64()?;
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_87;
}
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_69;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_14);
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_21);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_21 = v0.try_as_i32()?;
{

}
break 'label_69;
break;
}
'label_88: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_32 = v0.try_as_i64()?;
v1 = TaggedVal::from(-1i64);
v0 = TaggedVal::from((v0.try_as_i64()? > v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_88;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(0i64);
v2 = TaggedVal::from(local_32);
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_sub(v2.try_as_i64()?));
local_32 = v1.try_as_i64()?;
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 56) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(1i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
{

}
break 'label_70;
break;
}
'label_89: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(2048i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_89;
}
v0 = TaggedVal::from(1i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2859i32);
local_31 = v0.try_as_i32()?;
{

}
break 'label_70;
break;
}
v0 = TaggedVal::from(2860i32);
v1 = TaggedVal::from(2858i32);
v2 = TaggedVal::from(local_23);
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
local_30 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_31 = v0.try_as_i32()?;
break;
}
'label_90: loop {
'label_91: loop {
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(4294967296i64);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) >= (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_91;
}
v0 = TaggedVal::from(local_32);
local_33 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
{

}
break 'label_90;
break;
}
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
'label_92: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_32);
v2 = TaggedVal::from(local_32);
v3 = TaggedVal::from(10i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64).checked_div(v3.try_as_i64()? as u64)?);
local_33 = v2.try_as_i64()?;
v3 = TaggedVal::from(10i64);
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_mul(v3.try_as_i64()?));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_sub(v2.try_as_i64()?));
v1 = TaggedVal::from(v1.try_as_i64()? as i32);
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(42949672959i64);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) > (v1.try_as_i64()? as u64)) as i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
local_32 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_17);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_92;
}
break;}
break;
}
v0 = TaggedVal::from(local_33);
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_69;
}
'label_93: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_93;
}
break;}
break;
}
'label_94: loop {
v0 = TaggedVal::from(local_25);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_94;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
break;
}
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(-65537i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_23);
v2 = TaggedVal::from(local_25);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_32 = v0.try_as_i64()?;
'label_95: loop {
v0 = TaggedVal::from(local_21);
if v0.try_as_i32()? != 0 {
{

}
break 'label_95;
}
v0 = TaggedVal::from(0i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_95;
}
v0 = TaggedVal::from(local_14);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
{

}
break 'label_58;
break;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_14);
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(local_32);
v2 = TaggedVal::from((v2.try_as_i64()? == 0) as i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_21);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
{

}
break 'label_58;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 56) as usize)?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 55) as usize, v1.try_as_i64()? as u8)?;
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
{

}
break 'label_58;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4440) as usize)?);
v0 = TaggedVal::from(self.func_119(v0.try_as_i32()?)?);
local_18 = v0.try_as_i32()?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(2868i32);
v2 = TaggedVal::from(local_17);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(2147483647i32);
v3 = TaggedVal::from(local_21);
v4 = TaggedVal::from(local_21);
v5 = TaggedVal::from(0i32);
v4 = TaggedVal::from((v4.try_as_i32()? < v5.try_as_i32()?) as i32);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v1 = TaggedVal::from(self.func_132(v1.try_as_i32()?, v2.try_as_i32()?)?);
local_27 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_58;
}
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_58;
}
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
if v0.try_as_i32()? != 0 {
{

}
break 'label_62;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
{

}
break 'label_61;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 56) as usize)?);
write_mem_u32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()? as u32)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 56) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_19 = v0.try_as_i32()?;
'label_96: loop {
'label_97: loop {
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_96;
}
'label_98: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(self.func_145(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_22 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_98;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_98;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_20);
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_17 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_97;
}
{

}
break 'label_96;
break;
}
break;}
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
break;
}
'label_99: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(73728i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_99;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_99;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_27 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_100: loop {
v0 = TaggedVal::from(local_27);
if v0.try_as_i32()? != 0 {
{

}
break 'label_100;
}
'label_101: loop {
'label_102: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_102;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_101;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_99;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_103: loop {
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_103;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
'label_104: loop {
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_103;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(self.func_145(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_103;
}
'label_105: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_105;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_20);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_104;
}
break;}
break;
}
'label_106: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_106;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_106;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_107: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_107;
}
'label_108: loop {
'label_109: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_109;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_108;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_106;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
{

}
continue 'label_3;
break;
}
'label_110: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_110;
}
v0 = TaggedVal::from(local_25);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_f64(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_34 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 364) as usize, v1.try_as_i32()?)?;
'label_111: loop {
'label_112: loop {
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
v1 = TaggedVal::from(-1i64);
v0 = TaggedVal::from((v0.try_as_i64()? > v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_112;
}
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(-v0.try_as_f64()?);
local_34 = v0.try_as_f64()?;
v0 = TaggedVal::from(1i32);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(3504i32);
local_37 = v0.try_as_i32()?;
{

}
break 'label_111;
break;
}
'label_113: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(2048i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_113;
}
v0 = TaggedVal::from(1i32);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(3507i32);
local_37 = v0.try_as_i32()?;
{

}
break 'label_111;
break;
}
v0 = TaggedVal::from(3510i32);
v1 = TaggedVal::from(3505i32);
v2 = TaggedVal::from(local_23);
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
local_35 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_36 = v0.try_as_i32()?;
break;
}
'label_114: loop {
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_f64()?.abs());
v1 = TaggedVal::from(f64::INFINITY);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_114;
}
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
'label_115: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_115;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_115;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_116: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_116;
}
'label_117: loop {
'label_118: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_118;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_117;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_115;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_119: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_119;
}
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_35);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_17 = v0.try_as_i32()?;
break;
}
'label_120: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_120;
}
v0 = TaggedVal::from(3531i32);
v1 = TaggedVal::from(3535i32);
v2 = TaggedVal::from(local_29);
v3 = TaggedVal::from(32i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
local_17 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(3523i32);
v2 = TaggedVal::from(3527i32);
v3 = TaggedVal::from(local_17);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(local_34);
v3 = TaggedVal::from(local_34);
v2 = TaggedVal::from((v2.try_as_f64()? != v3.try_as_f64()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(3i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_121: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(73728i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_121;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_121;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_122: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_122;
}
'label_123: loop {
'label_124: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_124;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_123;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_121;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
{

}
break 'label_54;
break;
}
'label_125: loop {
'label_126: loop {
'label_127: loop {
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(364i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(self.func_147(v0.try_as_f64()?, v1.try_as_i32()?)?);
local_34 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_34 = v0.try_as_f64()?;
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from((v0.try_as_f64()? == v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_127;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 364) as usize)?);
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 364) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_31 = v0.try_as_i32()?;
v1 = TaggedVal::from(97i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_126;
}
{

}
break 'label_55;
break;
}
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_31 = v0.try_as_i32()?;
v1 = TaggedVal::from(97i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_55;
}
v0 = TaggedVal::from(6i32);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(local_21);
v3 = TaggedVal::from(0i32);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 364) as usize)?);
local_18 = v0.try_as_i32()?;
{

}
break 'label_125;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(-29i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 364) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(6i32);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(local_21);
v3 = TaggedVal::from(0i32);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(268435456f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_34 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(368i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_11);
v2 = TaggedVal::from(local_18);
v3 = TaggedVal::from(0i32);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
local_38 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_30 = v0.try_as_i32()?;
local_20 = v0.try_as_i32()?;
'label_128: loop {
'label_129: loop {
'label_130: loop {
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(4294967296f64);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
v1 = TaggedVal::from(local_34);
v2 = TaggedVal::from(0f64);
v1 = TaggedVal::from((v1.try_as_f64()? >= v2.try_as_f64()?) as i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_130;
}
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(<_ as SafeFloatConv<u32>>::try_to_int(v0.try_as_f64()?.trunc())?);
local_17 = v0.try_as_i32()?;
{

}
break 'label_129;
break;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_17);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as f64));
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
v1 = TaggedVal::from(1000000000f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_34 = v0.try_as_f64()?;
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from((v0.try_as_f64()? != v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_128;
}
break;}
'label_131: loop {
'label_132: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_132;
}
v0 = TaggedVal::from(local_20);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
local_19 = v0.try_as_i32()?;
{

}
break 'label_131;
break;
}
v0 = TaggedVal::from(local_30);
local_19 = v0.try_as_i32()?;
'label_133: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(29i32);
v2 = TaggedVal::from(local_18);
v3 = TaggedVal::from(29i32);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_18 = v0.try_as_i32()?;
'label_134: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_134;
}
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_33 = v0.try_as_i64()?;
v0 = TaggedVal::from(0i64);
local_32 = v0.try_as_i64()?;
'label_135: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_17);
v1 = TaggedVal::from(read_mem_u32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
v2 = TaggedVal::from(local_33);
v1 = TaggedVal::from(v1.try_as_i64()? << (v2.try_as_i64()? % 64));
v2 = TaggedVal::from(local_32);
v3 = TaggedVal::from(4294967295i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
local_32 = v1.try_as_i64()?;
v2 = TaggedVal::from(local_32);
v3 = TaggedVal::from(1000000000i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64).checked_div(v3.try_as_i64()? as u64)?);
local_32 = v2.try_as_i64()?;
v3 = TaggedVal::from(1000000000i64);
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_mul(v3.try_as_i64()?));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_sub(v2.try_as_i64()?));
write_mem_u32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()? as u32)?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_135;
}
break;}
v0 = TaggedVal::from(local_32);
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_134;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
'label_136: loop {
'label_137: loop {
v0 = TaggedVal::from(local_20);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_136;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_137;
}
break;}
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 364) as usize)?);
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 364) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_17);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_133;
}
break;}
break;
}
'label_138: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_138;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(25i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32).checked_div(v1.try_as_i32()? as u32)?);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_25 = v0.try_as_i32()?;
'label_139: loop {
v0 = TaggedVal::from(9i32);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(local_18);
v3 = TaggedVal::from(-9i32);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_21 = v0.try_as_i32()?;
'label_140: loop {
'label_141: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_141;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_19);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_19 = v0.try_as_i32()?;
{

}
break 'label_140;
break;
}
v0 = TaggedVal::from(1000000000i32);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_20 = v0.try_as_i32()?;
'label_142: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_20);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_22 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_21);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_142;
}
break;}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_19);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_140;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_5);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 364) as usize)?);
v2 = TaggedVal::from(local_21);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 364) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_31);
v3 = TaggedVal::from(102i32);
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_25);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_20);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v3 = TaggedVal::from(2i32);
v2 = TaggedVal::from(v2.try_as_i32()? >> (v3.try_as_i32()? % 32));
v3 = TaggedVal::from(local_25);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_139;
}
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_20 = v0.try_as_i32()?;
'label_143: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_143;
}
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_143;
}
v0 = TaggedVal::from(10i32);
local_18 = v0.try_as_i32()?;
'label_144: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(10i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_144;
}
break;}
break;
}
'label_145: loop {
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_20);
v3 = TaggedVal::from(local_31);
v4 = TaggedVal::from(102i32);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_28);
v2 = TaggedVal::from(0i32);
v1 = TaggedVal::from((v1.try_as_i32()? != v2.try_as_i32()?) as i32);
v2 = TaggedVal::from(local_31);
v3 = TaggedVal::from(103i32);
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
local_25 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_30);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(9i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
v2 = TaggedVal::from(-9i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_145;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(9216i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_9);
v3 = TaggedVal::from(local_38);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_38 = v0.try_as_i32()?;
v1 = TaggedVal::from(-4096i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(10i32);
local_22 = v0.try_as_i32()?;
'label_146: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(9i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(7i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_146;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(10i32);
local_22 = v0.try_as_i32()?;
'label_147: loop {
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_147;
}
break;}
break;
}
v0 = TaggedVal::from(local_26);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(local_22);
v1 = TaggedVal::from((v1.try_as_i32()? as u32).checked_div(v2.try_as_i32()? as u32)?);
local_27 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_22);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
'label_148: loop {
'label_149: loop {
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_31 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_149;
}
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_148;
}
break;
}
'label_150: loop {
'label_151: loop {
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_151;
}
v0 = TaggedVal::from(9007199254740992f64);
local_34 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_150;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(1000000000i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_150;
}
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_150;
}
break;
}
v0 = TaggedVal::from(9007199254740994f64);
local_34 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(0.5f64);
v1 = TaggedVal::from(1f64);
v2 = TaggedVal::from(1.5f64);
v3 = TaggedVal::from(local_18);
v4 = TaggedVal::from(local_22);
v5 = TaggedVal::from(1i32);
v4 = TaggedVal::from((v4.try_as_i32()? as u32) >> (v5.try_as_i32()? % 32));
local_27 = v4.try_as_i32()?;
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(1.5f64);
v3 = TaggedVal::from(local_31);
v4 = TaggedVal::from(local_17);
v3 = TaggedVal::from((v3.try_as_i32()? == v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(local_18);
v3 = TaggedVal::from(local_27);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) < (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_39 = v0.try_as_f64()?;
'label_152: loop {
v0 = TaggedVal::from(local_36);
if v0.try_as_i32()? != 0 {
{

}
break 'label_152;
}
v0 = TaggedVal::from(local_37);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(45i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_152;
}
v0 = TaggedVal::from(local_39);
v0 = TaggedVal::from(-v0.try_as_f64()?);
local_39 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(-v0.try_as_f64()?);
local_34 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(local_18);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from((v0.try_as_f64()? == v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_148;
}
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_22);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_20 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_153: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1000000000i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_153;
}
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(-4100i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
'label_154: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_155: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_155;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_20);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(999999999i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_154;
}
break;}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_148;
}
v0 = TaggedVal::from(10i32);
local_18 = v0.try_as_i32()?;
'label_156: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(10i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_156;
}
break;}
break;
}
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) > (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
break;
}
'label_157: loop {
'label_158: loop {
'label_159: loop {
v0 = TaggedVal::from(local_17);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_159;
}
v0 = TaggedVal::from(0i32);
local_31 = v0.try_as_i32()?;
{

}
break 'label_157;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_158;
}
break;}
v0 = TaggedVal::from(1i32);
local_31 = v0.try_as_i32()?;
break;
}
'label_160: loop {
'label_161: loop {
v0 = TaggedVal::from(local_25);
if v0.try_as_i32()? != 0 {
{

}
break 'label_161;
}
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_26 = v0.try_as_i32()?;
{

}
break 'label_160;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
v1 = TaggedVal::from(-1i32);
v2 = TaggedVal::from(local_28);
v3 = TaggedVal::from(1i32);
v4 = TaggedVal::from(local_28);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_20);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
v3 = TaggedVal::from(local_20);
v4 = TaggedVal::from(-5i32);
v3 = TaggedVal::from((v3.try_as_i32()? > v4.try_as_i32()?) as i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
local_18 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(-2i32);
v2 = TaggedVal::from(local_18);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_26 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_160;
}
v0 = TaggedVal::from(9i32);
local_17 = v0.try_as_i32()?;
'label_162: loop {
v0 = TaggedVal::from(local_31);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_162;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_162;
}
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32).checked_rem(v1.try_as_i32()? as u32)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_162;
}
v0 = TaggedVal::from(10i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_17 = v0.try_as_i32()?;
'label_163: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(10i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_18 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? as u32).checked_rem(v1.try_as_i32()? as u32)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_163;
}
break;}
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
v1 = TaggedVal::from(-9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
'label_164: loop {
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(-33i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(70i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_164;
}
v0 = TaggedVal::from(0i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_17);
v4 = TaggedVal::from(0i32);
v3 = TaggedVal::from((v3.try_as_i32()? > v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_28);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
{

}
break 'label_160;
break;
}
v0 = TaggedVal::from(0i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_20);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_17);
v4 = TaggedVal::from(0i32);
v3 = TaggedVal::from((v3.try_as_i32()? > v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_17 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_28);
v3 = TaggedVal::from(local_17);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(-1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(2147483645i32);
v2 = TaggedVal::from(2147483646i32);
v3 = TaggedVal::from(local_28);
v4 = TaggedVal::from(local_26);
v3 = TaggedVal::from(v3.try_as_i32()? | v4.try_as_i32()?);
local_25 = v3.try_as_i32()?;
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_25);
v2 = TaggedVal::from(0i32);
v1 = TaggedVal::from((v1.try_as_i32()? != v2.try_as_i32()?) as i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_36 = v0.try_as_i32()?;
'label_165: loop {
'label_166: loop {
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(-33i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(70i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_40 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_166;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(2147483647i32);
v2 = TaggedVal::from(local_36);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_20);
v3 = TaggedVal::from(0i32);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_20 = v0.try_as_i32()?;
{

}
break 'label_165;
break;
}
v0 = TaggedVal::from(local_12);
local_18 = v0.try_as_i32()?;
'label_167: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_20);
v2 = TaggedVal::from(31i32);
v1 = TaggedVal::from(v1.try_as_i32()? >> (v2.try_as_i32()? % 32));
local_17 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_167;
}
'label_168: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_21 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_168;
}
break;}
break;
}
'label_169: loop {
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_169;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
'label_170: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(48i32);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_170;
}
break;}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_38 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_29);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(-1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(45i32);
v2 = TaggedVal::from(43i32);
v3 = TaggedVal::from(local_20);
v4 = TaggedVal::from(0i32);
v3 = TaggedVal::from((v3.try_as_i32()? < v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_38);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(2147483647i32);
v2 = TaggedVal::from(local_36);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
break;
}
v0 = TaggedVal::from(-1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_36);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_35);
v2 = TaggedVal::from(2147483647i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
'label_171: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(73728i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_23 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_171;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_171;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_29);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_21 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_172: loop {
v0 = TaggedVal::from(local_21);
if v0.try_as_i32()? != 0 {
{

}
break 'label_172;
}
'label_173: loop {
'label_174: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_174;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_173;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_171;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_175: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_175;
}
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_35);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_176: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(65536i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_176;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_176;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_29);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_21 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_177: loop {
v0 = TaggedVal::from(local_21);
if v0.try_as_i32()? != 0 {
{

}
break 'label_177;
}
'label_178: loop {
'label_179: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_179;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_178;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_176;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_40);
if v0.try_as_i32()? != 0 {
{

}
break 'label_57;
}
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_19);
v3 = TaggedVal::from(local_30);
v2 = TaggedVal::from(((v2.try_as_i32()? as u32) > (v3.try_as_i32()? as u32)) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_27 = v0.try_as_i32()?;
local_21 = v0.try_as_i32()?;
'label_180: loop {
'label_181: loop {
'label_182: loop {
'label_183: loop {
'label_184: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_184;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
'label_185: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_20 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_185;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
'label_186: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_186;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_181;
}
{

}
break 'label_182;
break;
}
v0 = TaggedVal::from(local_19);
if v0.try_as_i32()? != 0 {
{

}
break 'label_181;
}
{

}
break 'label_183;
break;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_182;
}
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(48i32);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_181;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_19);
v3 = TaggedVal::from(9i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
break;
}
'label_187: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_187;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_17);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_180;
}
break;}
'label_188: loop {
v0 = TaggedVal::from(local_25);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_188;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_188;
}
v0 = TaggedVal::from(3539i32);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_189: loop {
'label_190: loop {
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_190;
}
v0 = TaggedVal::from(local_28);
local_17 = v0.try_as_i32()?;
{

}
break 'label_189;
break;
}
'label_191: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_191;
}
v0 = TaggedVal::from(local_28);
local_17 = v0.try_as_i32()?;
{

}
break 'label_189;
break;
}
'label_192: loop {
v0 = TaggedVal::from(local_8);
local_17 = v0.try_as_i32()?;
'label_193: loop {
'label_194: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_194;
}
v0 = TaggedVal::from(local_8);
local_17 = v0.try_as_i32()?;
'label_195: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_19);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_20 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_195;
}
break;}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_193;
}
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_13);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

'label_196: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_196;
}
break;}
break;
}
'label_197: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_197;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_28);
v2 = TaggedVal::from(9i32);
v3 = TaggedVal::from(local_28);
v4 = TaggedVal::from(9i32);
v3 = TaggedVal::from((v3.try_as_i32()? < v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(-9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_189;
}
v0 = TaggedVal::from(local_17);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_192;
}
break;}
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_18 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_198: loop {
v0 = TaggedVal::from(local_18);
if v0.try_as_i32()? != 0 {
{

}
break 'label_198;
}
'label_199: loop {
'label_200: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_200;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_199;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

{

}
break 'label_56;
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2858i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
local_27 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(local_27);
v3 = TaggedVal::from(local_26);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_23 = v0.try_as_i32()?;
v1 = TaggedVal::from(2147483647i32);
v2 = TaggedVal::from(local_30);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_24);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_21);
v2 = TaggedVal::from((v2.try_as_i32()? < v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_201: loop {
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(73728i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_25 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_201;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_201;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_21);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_28 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_202: loop {
v0 = TaggedVal::from(local_28);
if v0.try_as_i32()? != 0 {
{

}
break 'label_202;
}
'label_203: loop {
'label_204: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_204;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_203;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_201;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_205: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_205;
}
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_30);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_206: loop {
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(65536i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_206;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_206;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_21);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_28 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_207: loop {
v0 = TaggedVal::from(local_28);
if v0.try_as_i32()? != 0 {
{

}
break 'label_207;
}
'label_208: loop {
'label_209: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_209;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_208;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_206;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_210: loop {
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_210;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_23);
v3 = TaggedVal::from(local_26);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_27 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_211: loop {
v0 = TaggedVal::from(local_27);
if v0.try_as_i32()? != 0 {
{

}
break 'label_211;
}
'label_212: loop {
'label_213: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_213;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_22 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_212;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_210;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_214: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_214;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_26);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(local_21);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_19 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_19);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
'label_215: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_215;
}
'label_216: loop {
'label_217: loop {
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_217;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_216;
}
break;}
break;
}
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

{

}
continue 'label_3;
break;
}
'label_218: loop {
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_218;
}
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_31);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_21 = v0.try_as_i32()?;
'label_219: loop {
v0 = TaggedVal::from(local_8);
local_18 = v0.try_as_i32()?;
'label_220: loop {
'label_221: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_221;
}
v0 = TaggedVal::from(0i32);
local_20 = v0.try_as_i32()?;
'label_222: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_18 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_222;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
if v0.try_as_i32()? != 0 {
{

}
break 'label_220;
}
break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(48i32);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
break;
}
'label_223: loop {
'label_224: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_224;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_223;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_18);
v3 = TaggedVal::from(local_13);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

'label_225: loop {
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_225;
}
{

}
break 'label_223;
break;}
break;
}
'label_226: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_226;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
'label_227: loop {
v0 = TaggedVal::from(local_26);
if v0.try_as_i32()? != 0 {
{

}
break 'label_227;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_223;
}
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_223;
}
v0 = TaggedVal::from(3539i32);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
'label_228: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_228;
}
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_28);
v3 = TaggedVal::from(local_28);
v4 = TaggedVal::from(local_17);
v3 = TaggedVal::from((v3.try_as_i32()? > v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_28 = v0.try_as_i32()?;
'label_229: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_21 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_229;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_219;
}
break;
}
break;}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_218;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_28);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_28);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_20 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_17 = v0.try_as_i32()?;
'label_230: loop {
v0 = TaggedVal::from(local_20);
if v0.try_as_i32()? != 0 {
{

}
break 'label_230;
}
'label_231: loop {
'label_232: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_232;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_19 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_28 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_231;
}
break;}
break;
}
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_218;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_28);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(local_12);
v2 = TaggedVal::from(local_38);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_233: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_233;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_233;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_29);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_18 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_234: loop {
v0 = TaggedVal::from(local_18);
if v0.try_as_i32()? != 0 {
{

}
break 'label_234;
}
'label_235: loop {
'label_236: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_236;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_235;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_233;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_29);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_29);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
{

}
break 'label_54;
break;
}
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_37);
v2 = TaggedVal::from(local_29);
v3 = TaggedVal::from(32i32);
v2 = TaggedVal::from(v2.try_as_i32()? & v3.try_as_i32()?);
local_22 = v2.try_as_i32()?;
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_28 = v0.try_as_i32()?;
'label_237: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(11i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_237;
}
v0 = TaggedVal::from(12i32);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_237;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(-12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(16f64);
local_39 = v0.try_as_f64()?;
'label_238: loop {
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(16f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_39 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_238;
}
break;}
'label_239: loop {
v0 = TaggedVal::from(local_28);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(45i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_239;
}
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_34);
v1 = TaggedVal::from(-v1.try_as_f64()?);
v2 = TaggedVal::from(local_39);
v1 = TaggedVal::from(v1.try_as_f64()? - v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
v0 = TaggedVal::from(-v0.try_as_f64()?);
local_34 = v0.try_as_f64()?;
{

}
break 'label_237;
break;
}
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
local_34 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(local_12);
local_20 = v0.try_as_i32()?;
'label_240: loop {
'label_241: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 364) as usize)?);
local_27 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(31i32);
v1 = TaggedVal::from(v1.try_as_i32()? >> (v2.try_as_i32()? % 32));
local_17 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_241;
}
v0 = TaggedVal::from(0i32);
local_19 = v0.try_as_i32()?;
'label_242: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(324i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(11i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32).checked_div(v3.try_as_i32()? as u32)?);
local_20 = v2.try_as_i32()?;
v3 = TaggedVal::from(10i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_mul(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(48i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_242;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(324i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
if v0.try_as_i32()? != 0 {
{

}
break 'label_240;
}
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(48i32);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
break;
}
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_25 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_29);
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(45i32);
v2 = TaggedVal::from(43i32);
v3 = TaggedVal::from(local_27);
v4 = TaggedVal::from(0i32);
v3 = TaggedVal::from((v3.try_as_i32()? < v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
'label_243: loop {
v0 = TaggedVal::from(local_19);
local_17 = v0.try_as_i32()?;
'label_244: loop {
'label_245: loop {
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_f64()?.abs());
v1 = TaggedVal::from(2147483648f64);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_245;
}
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from(<_ as SafeFloatConv<i32>>::try_to_int(v0.try_as_f64()?.trunc())?);
local_19 = v0.try_as_i32()?;
{

}
break 'label_244;
break;
}
v0 = TaggedVal::from(-2147483648i32);
local_19 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_19);
v2 = TaggedVal::from(3488i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v2 = TaggedVal::from(local_22);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_19);
v1 = TaggedVal::from((v1.try_as_i32()? as f64));
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
v1 = TaggedVal::from(16f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_34 = v0.try_as_f64()?;
'label_246: loop {
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(336i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_246;
}
'label_247: loop {
v0 = TaggedVal::from(local_20);
if v0.try_as_i32()? != 0 {
{

}
break 'label_247;
}
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_247;
}
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from((v0.try_as_f64()? == v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_246;
}
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(46i32);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_19 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from((v0.try_as_f64()? != v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_243;
}
break;}
v0 = TaggedVal::from(-1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(2147483645i32);
v1 = TaggedVal::from(local_26);
v2 = TaggedVal::from(local_12);
v3 = TaggedVal::from(local_25);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_27 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_20 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_54;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_19);
v3 = TaggedVal::from(local_5);
v4 = TaggedVal::from(336i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_22 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_7);
v4 = TaggedVal::from(local_19);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_21);
v3 = TaggedVal::from((v3.try_as_i32()? < v4.try_as_i32()?) as i32);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
v2 = TaggedVal::from(local_22);
v3 = TaggedVal::from(local_21);
if ValType::from(v1) != ValType::from(v2) {
                     return None;
                 }
                 if v3.try_as_i32()? != 0 {
                     v1 = v1;
                 } else {
                     v1 = v2;
                 }
local_30 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
'label_248: loop {
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(73728i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_248;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_248;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_23 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_249: loop {
v0 = TaggedVal::from(local_23);
if v0.try_as_i32()? != 0 {
{

}
break 'label_249;
}
'label_250: loop {
'label_251: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_251;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_250;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_248;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_252: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_252;
}
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_26);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_253: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(65536i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_253;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_253;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_26 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_254: loop {
v0 = TaggedVal::from(local_26);
if v0.try_as_i32()? != 0 {
{

}
break 'label_254;
}
'label_255: loop {
'label_256: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_256;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_255;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_253;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_257: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_257;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(336i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_22);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_258: loop {
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_258;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(48i32);
v2 = TaggedVal::from(local_17);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_259: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_259;
}
'label_260: loop {
'label_261: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_261;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_260;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_258;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_262: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_262;
}
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
'label_263: loop {
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(8192i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_263;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_263;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
local_17 = v2.try_as_i32()?;
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_17);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
local_22 = v4.try_as_i32()?;
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
v0 = TaggedVal::from(self.func_134(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
'label_264: loop {
v0 = TaggedVal::from(local_22);
if v0.try_as_i32()? != 0 {
{

}
break 'label_264;
}
'label_265: loop {
'label_266: loop {
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_266;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(256i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(-256i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_265;
}
break;}
break;
}
v0 = TaggedVal::from(local_19);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_263;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_117(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_24);
v3 = TaggedVal::from(local_18);
v2 = TaggedVal::from((v2.try_as_i32()? > v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_17 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
break;}
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(61i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(-1i32);
local_16 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(880i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_16);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_127(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
'label_5: loop {
'label_6: loop {
'label_7: loop {
'label_8: loop {
'label_9: loop {
'label_10: loop {
'label_11: loop {
'label_12: loop {
'label_13: loop {
'label_14: loop {
'label_15: loop {
'label_16: loop {
'label_17: loop {
'label_18: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
match v0.try_as_i32()? {
0 => {
{

}
break 'label_1;
},
1 => {
{

}
break 'label_18;
},
2 => {
{

}
break 'label_17;
},
3 => {
{

}
break 'label_14;
},
4 => {
{

}
break 'label_16;
},
5 => {
{

}
break 'label_15;
},
6 => {
{

}
break 'label_13;
},
7 => {
{

}
break 'label_12;
},
8 => {
{

}
break 'label_11;
},
9 => {
{

}
break 'label_10;
},
10 => {
{

}
break 'label_9;
},
11 => {
{

}
break 'label_8;
},
12 => {
{

}
break 'label_7;
},
13 => {
{

}
break 'label_6;
},
14 => {
{

}
break 'label_5;
},
15 => {
{

}
break 'label_4;
},
16 => {
{

}
break 'label_3;
},
17 => {
{

}
break 'label_2;
},
_ => {
{

}
break 'label_0;
},
}
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i16(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u16(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u32(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i64))?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(7i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-8i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
return Some(());
break;
}
self.func_128()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_128(&mut self, ) -> Option<()> {
let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(3344i32);
v1 = TaggedVal::from(3824i32);
v0 = TaggedVal::from(self.func_120(v0.try_as_i32()?, v1.try_as_i32()?)?);

self.func_110()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_129(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_125(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_2);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_130(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;
let mut v1: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_3(v0.try_as_i32()?)?);
local_0 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_131(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v0 = TaggedVal::from(self.func_130(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_132(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_136(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_133(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_0);
local_3 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_4);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_2;
}
{

}
break 'label_0;
break;}
break;
}
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_3 = v0.try_as_i32()?;
break;
}
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_2 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
'label_5: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
'label_6: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_6;
}
break;}
break;
}
'label_7: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
'label_8: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
'label_9: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);
break;
}
'label_10: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
'label_11: loop {
'label_12: loop {
'label_13: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
match v0.try_as_i32()? {
0 => {
{

}
break 'label_13;
},
1 => {
{

}
break 'label_12;
},
2 => {
{

}
break 'label_11;
},
_ => {
{

}
break 'label_10;
},
}
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
'label_14: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_7 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(24i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(24i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(24i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(24i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_14;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
{

}
break 'label_10;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
'label_15: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_7 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(17i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_15;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
{

}
break 'label_10;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
'label_16: loop {
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_7 = v1.try_as_i32()?;
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(24i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(24i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(24i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(24i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(8i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(18i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_16;
}
break;}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
'label_17: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u16(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u16(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u16)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 4) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 5) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 5) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 6) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 6) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 7) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 7) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 8) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 9) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 9) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 10) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 10) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 11) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 11) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 12) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 13) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 13) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 14) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 14) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 15) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 15) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
'label_18: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 4) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 5) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 5) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 6) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 6) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 7) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 7) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
'label_19: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 2) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
'label_20: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
break;
}
v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_134(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i64 = 0i64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-3i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(7i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(255i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(16843009i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_4);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v2 = TaggedVal::from(-4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-12i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(25i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-20i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-28i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(24i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
local_5 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_6 = v0.try_as_i64()?;
v1 = TaggedVal::from(32i64);
v0 = TaggedVal::from(v0.try_as_i64()? << (v1.try_as_i64()? % 64));
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i64()? | v1.try_as_i64()?);
local_6 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_6);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(24i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_6);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_6);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_6);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_1;
}
break;}
break;
}
v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_135(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
local_1 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_3: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
{

}
continue 'label_4;
break;}
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(-16843009i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(-2139062144i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_5;
}
break;}
'label_6: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
return Some(v0.try_as_i32()?);
break;
}
'label_7: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 1) as usize).and_then(|x| Some(x as i32))?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_7;
}
{

}
break 'label_0;
break;}
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_136(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_3 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_2);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
{

}
break 'label_2;
break;
}
'label_4: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_5 = v0.try_as_i32()?;
'label_5: loop {
'label_6: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_2);
local_4 = v0.try_as_i32()?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_4);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_5;
}
break;}
break;
}
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
break;
}
'label_7: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(255i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(16843009i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_8: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(-16843009i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(-2139062144i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_8;
}
break;}
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(255i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_2 = v0.try_as_i32()?;
'label_9: loop {
'label_10: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_9;
}
break;}
break;
}
v0 = TaggedVal::from(0i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_137(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_138(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_137(v0.try_as_i32()?, v1.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_139(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 60) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 60) as usize, v1.try_as_i32()?)?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 24) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(0i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 32) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }

break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i64()?)?;
'label_1: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(32i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 40) as usize)?);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 44) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_2 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(27i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_140(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(-1i32);
local_2 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_139(v0.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(15i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(local_0);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 28) as usize)?);
{
                    let rets = self.indirect_call(v3.try_as_i32()? as usize, &[v0, v1, v2])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 15) as usize).and_then(|x| Some(x as i32))?);
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_2);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_141(&mut self, arg_0: i32, arg_1: i64) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 88) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 40) as usize)?);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
local_2 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 96) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? as i64));
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i64()? <= v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_1);
v2 = TaggedVal::from(v2.try_as_i64()? as i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 84) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 84) as usize, v1.try_as_i32()?)?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_142(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i64 = 0i64;
let mut local_4 : i64 = 0i64;
let mut local_5 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 96) as usize)?);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 40) as usize)?);
local_2 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_3 = v0.try_as_i64()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 88) as usize)?);
local_4 = v0.try_as_i64()?;
v0 = TaggedVal::from((v0.try_as_i64()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from((v0.try_as_i64()? >= v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_140(v0.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_2 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 88) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 84) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_1);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v2 = TaggedVal::from((v2.try_as_i32()? as i64));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 96) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(-1i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_3 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
'label_3: loop {
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 88) as usize)?);
local_4 = v0.try_as_i64()?;
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_1 = v0.try_as_i32()?;
{

}
break 'label_4;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_sub(v1.try_as_i64()?));
local_4 = v0.try_as_i64()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
local_1 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
v0 = TaggedVal::from((v0.try_as_i64()? >= v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i64()? as i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 84) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 84) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(local_0);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 40) as usize)?);
local_5 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_1);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v2 = TaggedVal::from((v2.try_as_i32()? as i64));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 96) as usize, v1.try_as_i64()?)?;
'label_6: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
break;
}
v0 = TaggedVal::from(local_2);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_143(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i64) -> Option<i64> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i64 = arg_3;let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i64 = 0i64;
let mut local_8 : i64 = 0i64;
let mut local_9 : i64 = 0i64;
let mut local_10 : i32 = 0i32;
let mut local_11 : i64 = 0i64;
let mut local_12 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(36i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
'label_6: loop {
'label_7: loop {
'label_8: loop {
'label_9: loop {
'label_10: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-9i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(5i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_8;
}
'label_11: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
match v0.try_as_i32()? {
0 => {
{

}
continue 'label_8;
},
1 => {
{

}
break 'label_7;
},
2 => {
{

}
break 'label_7;
},
3 => {
{

}
break 'label_7;
},
4 => {
{

}
break 'label_7;
},
5 => {
{

}
break 'label_7;
},
6 => {
{

}
break 'label_7;
},
7 => {
{

}
break 'label_7;
},
8 => {
{

}
break 'label_7;
},
9 => {
{

}
break 'label_7;
},
10 => {
{

}
break 'label_7;
},
11 => {
{

}
break 'label_11;
},
12 => {
{

}
break 'label_7;
},
13 => {
{

}
break 'label_11;
},
_ => {
{

}
break 'label_7;
},
}
break;
}
break;}
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(45i32);
v2 = TaggedVal::from((v2.try_as_i32()? == v3.try_as_i32()?) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_6 = v0.try_as_i32()?;
'label_12: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_6;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_6;
break;
}
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
break;
}
'label_13: loop {
'label_14: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-17i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(48i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
'label_15: loop {
'label_16: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_15;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
'label_17: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-33i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(88i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
'label_18: loop {
'label_19: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_18;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(16i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(3553i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(0i64);
local_3 = v0.try_as_i64()?;
'label_20: loop {
'label_21: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 88) as usize)?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? < v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
local_5 = v1.try_as_i32()?;
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(-2i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_2);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
break;
}
v0 = TaggedVal::from(0i64);
local_3 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
self.func_141(v0.try_as_i32()?, v1.try_as_i64()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_1);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(8i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(10i32);
v2 = TaggedVal::from(local_1);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(0i64);
local_3 = v0.try_as_i64()?;
'label_22: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 88) as usize)?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? < v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_22;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i64);
self.func_141(v0.try_as_i32()?, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(28i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(0i64);
local_7 = v0.try_as_i64()?;
'label_23: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_23;
}
v0 = TaggedVal::from(0i32);
local_1 = v0.try_as_i32()?;
'label_24: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(10i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_25: loop {
'label_26: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_25;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
'label_27: loop {
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_27;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(429496729i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_24;
}
break;
}
break;}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_7 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(10i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_mul(v1.try_as_i64()?));
local_8 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_9 = v0.try_as_i64()?;
'label_28: loop {
'label_29: loop {
'label_30: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_30;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_29;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_7 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-48i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(1844674407370955162i64);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) >= (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(10i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_mul(v1.try_as_i64()?));
local_8 = v0.try_as_i64()?;
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
local_9 = v1.try_as_i64()?;
v2 = TaggedVal::from(-1i64);
v1 = TaggedVal::from(v1.try_as_i64()? ^ v2.try_as_i64()?);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) <= (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_28;
}
break;}
v0 = TaggedVal::from(10i32);
local_1 = v0.try_as_i32()?;
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(28i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i64);
local_3 = v0.try_as_i64()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(10i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(9i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
{

}
break 'label_1;
break;
}
'label_31: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
v0 = TaggedVal::from(0i64);
local_7 = v0.try_as_i64()?;
'label_32: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_32;
}
v0 = TaggedVal::from(0i32);
local_10 = v0.try_as_i32()?;
'label_33: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_mul(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
'label_34: loop {
'label_35: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_34;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(3553i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v0.try_as_i32()?;
'label_36: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(119304646i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_36;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_33;
}
break;
}
break;}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_7 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_8 = v0.try_as_i64()?;
'label_37: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_mul(v1.try_as_i64()?));
local_9 = v0.try_as_i64()?;
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
v2 = TaggedVal::from(255i64);
v1 = TaggedVal::from(v1.try_as_i64()? & v2.try_as_i64()?);
local_11 = v1.try_as_i64()?;
v2 = TaggedVal::from(-1i64);
v1 = TaggedVal::from(v1.try_as_i64()? ^ v2.try_as_i64()?);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) > (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_38: loop {
'label_39: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_39;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_38;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_7 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(0i64);
v3 = TaggedVal::from(local_7);
v4 = TaggedVal::from(0i64);
self.func_148(v0.try_as_i32()?, v1.try_as_i64()?, v2.try_as_i64()?, v3.try_as_i64()?, v4.try_as_i64()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
{

}
continue 'label_37;
break;}
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(23i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
v1 = TaggedVal::from(5i32);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(7i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(3809i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i64);
local_7 = v0.try_as_i64()?;
'label_40: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_40;
}
v0 = TaggedVal::from(0i32);
local_10 = v0.try_as_i32()?;
'label_41: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_12);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_10 = v0.try_as_i32()?;
'label_42: loop {
'label_43: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_43;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_42;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(3553i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v0.try_as_i32()?;
'label_44: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(134217727i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_44;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_41;
}
break;
}
break;}
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
local_7 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(-1i64);
v1 = TaggedVal::from(local_12);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
local_8 = v1.try_as_i64()?;
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
local_9 = v0.try_as_i64()?;
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) < (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_45: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i64()? << (v1.try_as_i64()? % 64));
v1 = TaggedVal::from(local_2);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
v2 = TaggedVal::from(255i64);
v1 = TaggedVal::from(v1.try_as_i64()? & v2.try_as_i64()?);
v0 = TaggedVal::from(v0.try_as_i64()? | v1.try_as_i64()?);
local_7 = v0.try_as_i64()?;
'label_46: loop {
'label_47: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_47;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_46;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) > (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_45;
}
break;}
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
'label_48: loop {
'label_49: loop {
'label_50: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_5 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 84) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_50;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
local_5 = v0.try_as_i32()?;
{

}
break 'label_49;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_142(v0.try_as_i32()?)?);
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(3553i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_u8(&self.memory, (v1.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_48;
}
break;}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(68i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(1i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v2 = TaggedVal::from((v2.try_as_i64()? == 0) as i32);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_7 = v0.try_as_i64()?;
break;
}
'label_51: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 88) as usize)?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? < v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_51;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 4) as usize)?);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
'label_52: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) < (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
'label_53: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_53;
}
v0 = TaggedVal::from(local_6);
if v0.try_as_i32()? != 0 {
{

}
break 'label_53;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(68i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_3 = v0.try_as_i64()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) <= (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(68i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_6);
v1 = TaggedVal::from((v1.try_as_i32()? as i64));
local_3 = v1.try_as_i64()?;
v0 = TaggedVal::from(v0.try_as_i64()? ^ v1.try_as_i64()?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_sub(v1.try_as_i64()?));
local_3 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);Some(v0.try_as_i64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_144(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;
let mut local_4 : i64 = 0i64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(112i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i64);
self.func_141(v0.try_as_i32()?, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(1i32);
v3 = TaggedVal::from(2147483648i64);
v0 = TaggedVal::from(self.func_143(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i64()?)?);
local_4 = v0.try_as_i64()?;
'label_0: loop {
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_3);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 4) as usize)?);
v3 = TaggedVal::from(local_3);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 96) as usize)?);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_3);
v3 = TaggedVal::from(read_mem_i32(&self.memory, (v3.try_as_i32()? + 40) as usize)?);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(112i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i64()? as i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_145(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(0i32);
v0 = TaggedVal::from(self.func_146(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_146(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(1i32);
local_3 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
'label_1: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(127i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(1i32);
return Some(v0.try_as_i32()?);
break;
}
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4452) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
'label_4: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-128i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(57216i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(25i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(1i32);
return Some(v0.try_as_i32()?);
break;
}
'label_5: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(2047i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(6i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(192i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(2i32);
return Some(v0.try_as_i32()?);
break;
}
'label_6: loop {
'label_7: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(55296i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-8192i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(57344i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(224i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(6i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(3i32);
return Some(v0.try_as_i32()?);
break;
}
'label_8: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-65536i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(1048575i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(18i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(240i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(6i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 2) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(63i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(128i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 1) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(4i32);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(25i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4440) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(-1i32);
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_147(&mut self, arg_0: f64, arg_1: i32) -> Option<f64> {
let mut local_0 : f64 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i64 = 0i64;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
local_2 = v0.try_as_i64()?;
v1 = TaggedVal::from(52i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
v1 = TaggedVal::from(2047i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(2047i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
'label_1: loop {
v0 = TaggedVal::from(local_3);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
'label_2: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from((v0.try_as_f64()? != v1.try_as_f64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(18446744073709552000f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_147(v0.try_as_f64()?, v1.try_as_i32()?)?);
local_0 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(-64i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(-1022i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(-9218868437227405313i64);
v0 = TaggedVal::from(v0.try_as_i64()? & v1.try_as_i64()?);
v1 = TaggedVal::from(4602678819172646912i64);
v0 = TaggedVal::from(v0.try_as_i64()? | v1.try_as_i64()?);
v0 = TaggedVal::from(f64::from_bits(v0.try_as_i64()? as u64));
local_0 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(local_0);Some(v0.try_as_f64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_148(&mut self, arg_0: i32, arg_1: i64, arg_2: i64, arg_3: i64, arg_4: i64) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;
let mut local_2 : i64 = arg_2;
let mut local_3 : i64 = arg_3;
let mut local_4 : i64 = arg_4;let mut local_5 : i64 = 0i64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_1);
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_mul(v2.try_as_i64()?));
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_mul(v3.try_as_i64()?));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(32i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64) >> (v3.try_as_i64()? % 64));
local_4 = v2.try_as_i64()?;
v3 = TaggedVal::from(local_1);
v4 = TaggedVal::from(32i64);
v3 = TaggedVal::from((v3.try_as_i64()? as u64) >> (v4.try_as_i64()? % 64));
local_2 = v3.try_as_i64()?;
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_mul(v3.try_as_i64()?));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(4294967295i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
local_3 = v2.try_as_i64()?;
v3 = TaggedVal::from(local_1);
v4 = TaggedVal::from(4294967295i64);
v3 = TaggedVal::from(v3.try_as_i64()? & v4.try_as_i64()?);
local_1 = v3.try_as_i64()?;
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_mul(v3.try_as_i64()?));
local_5 = v2.try_as_i64()?;
v3 = TaggedVal::from(32i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64) >> (v3.try_as_i64()? % 64));
v3 = TaggedVal::from(local_3);
v4 = TaggedVal::from(local_2);
v3 = TaggedVal::from(v3.try_as_i64()?.wrapping_mul(v4.try_as_i64()?));
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_add(v3.try_as_i64()?));
local_3 = v2.try_as_i64()?;
v3 = TaggedVal::from(32i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64) >> (v3.try_as_i64()? % 64));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(4294967295i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(local_1);
v3 = TaggedVal::from(v3.try_as_i64()?.wrapping_mul(v4.try_as_i64()?));
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_add(v3.try_as_i64()?));
local_3 = v2.try_as_i64()?;
v3 = TaggedVal::from(32i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64) >> (v3.try_as_i64()? % 64));
v1 = TaggedVal::from(v1.try_as_i64()?.wrapping_add(v2.try_as_i64()?));
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(32i64);
v1 = TaggedVal::from(v1.try_as_i64()? << (v2.try_as_i64()? % 64));
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(4294967295i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v1 = TaggedVal::from(v1.try_as_i64()? | v2.try_as_i64()?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;Some(())}

}

impl WasmModule {
             #[allow(dead_code)]
             fn indirect_call(&mut self, idx: usize, args: &[TaggedVal]) ->
                     Option<Vec<TaggedVal>> {
                 let call_target = (*self.indirect_call_table.get(idx)?)?;
                 match call_target {
                     0 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_0(a0)?;
                         Some(vec![])
                     }
1 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_1(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
2 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_2(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
3 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_3(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
4 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_4()?;
                         Some(vec![])
                     }
5 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_5()?;
                         Some(vec![])
                     }
6 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_6()?;
                         Some(vec![])
                     }
7 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_7(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
8 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_8(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
9 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_9(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
10 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_10(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
11 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_11(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
12 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_12()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
13 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_13(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
14 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_14(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
15 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_15(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
16 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_16(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
17 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_17(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
18 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_18(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
19 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_19(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
20 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_20(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
21 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_21(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
22 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_22(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
23 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_23(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
24 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_24(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
25 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_25(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
26 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_26(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
27 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_27(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
28 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_28(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
29 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_29(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
30 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_30(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
31 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_31(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
32 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_32(a0)?;
                         Some(vec![])
                     }
33 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_33(a0)?;
                         Some(vec![])
                     }
34 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_34(a0)?;
                         Some(vec![])
                     }
35 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_35(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
36 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_36(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
37 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_37(a0, a1)?;
                         Some(vec![])
                     }
38 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_38(a0, a1)?;
                         Some(vec![])
                     }
39 => {
                         if args.len() != 8 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
let a5 = args[5].try_as_i32()?;
let a6 = args[6].try_as_i32()?;
let a7 = args[7].try_as_i32()?;
                         self.func_39(a0, a1, a2, a3, a4, a5, a6, a7)?;
                         Some(vec![])
                     }
40 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_40(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
41 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_41(a0, a1)?;
                         Some(vec![])
                     }
42 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_42(a0, a1)?;
                         Some(vec![])
                     }
43 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_43(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
44 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_44(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
45 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_45(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
46 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_46(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
47 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_47(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
48 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_48(a0)?;
                         Some(vec![])
                     }
49 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_49(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
50 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_50(a0, a1, a2)?;
                         Some(vec![])
                     }
51 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_51(a0, a1)?;
                         Some(vec![])
                     }
52 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_52(a0, a1)?;
                         Some(vec![])
                     }
53 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_53(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
54 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_54(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
55 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_55(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
56 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         self.func_56(a0, a1, a2, a3)?;
                         Some(vec![])
                     }
57 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_57(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
58 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_58(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
59 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_59(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
60 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_60(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
61 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_61(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
62 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_62(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
63 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_63(a0, a1, a2)?;
                         Some(vec![])
                     }
64 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_64(a0, a1, a2)?;
                         Some(vec![])
                     }
65 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_65(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
66 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_66(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
67 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_67(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
68 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_68(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
69 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_69(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
70 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_70(a0)?;
                         Some(vec![])
                     }
71 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_71(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
72 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_72(a0)?;
                         Some(vec![])
                     }
73 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_73(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
74 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_74(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
75 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
                         let rets = self.func_75(a0, a1, a2, a3, a4)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
76 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_76()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
77 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_77()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
78 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_78(a0)?;
                         Some(vec![])
                     }
79 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_79(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
80 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_80(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
81 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_81()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
82 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_82(a0, a1)?;
                         Some(vec![])
                     }
83 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_83(a0)?;
                         Some(vec![])
                     }
84 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_84()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
85 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_85()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
86 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_86(a0, a1, a2)?;
                         Some(vec![])
                     }
87 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_87(a0)?;
                         Some(vec![])
                     }
88 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_88(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
89 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_89(a0)?;
                         Some(vec![])
                     }
90 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_90(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
91 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_91(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
92 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_92(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
93 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_93(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
94 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_94(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
95 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_95(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
96 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_96(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
97 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_97(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
98 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_98(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
99 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_99(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
100 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_100(a0, a1, a2)?;
                         Some(vec![])
                     }
101 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_101(a0, a1, a2)?;
                         Some(vec![])
                     }
102 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_102(a0, a1)?;
                         Some(vec![])
                     }
103 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_103(a0)?;
                         Some(vec![])
                     }
104 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_104(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
105 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_105()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
106 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_106(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
107 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_107(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
108 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_108(a0)?;
                         Some(vec![])
                     }
109 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_109(a0)?;
                         Some(vec![])
                     }
110 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_110()?;
                         Some(vec![])
                     }
111 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_111(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
112 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_112()?;
                         Some(vec![])
                     }
113 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_113()?;
                         Some(vec![])
                     }
114 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_114(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
115 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_115(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
116 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_116(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
117 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_117(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
118 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_118(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
119 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_119(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
120 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_120(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
121 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_121()?;
                         Some(vec![])
                     }
122 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_122()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
123 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_123(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
124 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_124(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
125 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_125(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
126 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
                         let rets = self.func_126(a0, a1, a2, a3, a4)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
127 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_127(a0, a1, a2)?;
                         Some(vec![])
                     }
128 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_128()?;
                         Some(vec![])
                     }
129 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_129(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
130 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_130(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
131 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_131(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
132 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_132(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
133 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_133(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
134 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_134(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
135 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_135(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
136 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_136(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
137 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_137(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
138 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_138(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
139 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_139(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
140 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_140(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
141 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
                         self.func_141(a0, a1)?;
                         Some(vec![])
                     }
142 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_142(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
143 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i64()?;
                         let rets = self.func_143(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
144 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_144(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
145 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_145(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
146 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_146(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
147 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_f64()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_147(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
148 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i64()?;
let a3 = args[3].try_as_i64()?;
let a4 = args[4].try_as_i64()?;
                         self.func_148(a0, a1, a2, a3, a4)?;
                         Some(vec![])
                     }
                     _ => None,
                 }
             }
         }

impl WasmModule {
                    #[allow(dead_code)]
                    pub fn get_memory(&mut self) -> *mut u8 {
                        self.memory.as_mut_ptr()
                    }
                }

impl WasmModule {
                     pub fn _start(&mut self, ) -> Option<()> {
                         self.func_6()
                     }
                 }
fn main() {
                         let mut wasm_module = WasmModule::new();
                         wasm_module._start().unwrap();
                     }

fn f_gold_wasm_thread_unsafe() -> i32 {
	let mut wasm_module = WasmModule::new();
	wasm_module._start().unwrap();
	unsafe { RESULT }
}

////// LLM Output //////
fn min(x: i32, y: i32) -> i32 {if x < y { x } else { y } }
fn max(x: i32, y: i32) -> i32 {if x > y { x } else { y }}
fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {a.cmp(b)}
fn len(arr: &[i32]) -> usize {arr.len()}
fn sort(arr: &mut [i32]) {arr.sort_by(cmpfunc);}


fn f_gold(num: String) -> i32 {
    if num.len() >= 3 {
        let d1:i32 = num.chars().last().unwrap() as i32;
        if d1 % 2 != 0 { return 0; }
        let d2:i32 = num.chars().nth(num.len() - 2).unwrap() as i32;
        let mut sum:i32 = 0;
        for c in num.chars() {
            sum += c as i32;
        }
        if sum % 3 == 0 && (d2 * 10 + d1) % 4 == 0 {
            return 1;
        } else {
            return 0;
        }
    } else {
        let number:i32 = num.parse().unwrap();
        if number % 12 == 0 {
            return 1;
        } else {
            return 0;
        }
    }
}////// LLM Output //////


use proptest::prelude::*;
proptest!{
  #[test]
  fn check_eq(
    PARAM_1: String
  ) {
     
		if let Some(param1_0) = PARAM_1.chars().nth(0){
		unsafe {
		PARAM1 = param1_0;

		}
		let result = f_gold(unsafe{PARAM1}.into());
		let result_prime = f_gold_wasm_thread_unsafe();
		assert_eq!(result, result_prime);
	}
  }
}
