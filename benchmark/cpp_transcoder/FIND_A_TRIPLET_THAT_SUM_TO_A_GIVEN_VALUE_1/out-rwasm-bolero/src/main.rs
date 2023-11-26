static mut PARAM1: [i32 ; 2] = [12,12];
static mut PARAM2: i32 = 12;
static mut PARAM3: i32 = 12;
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
                 m.globals[0] = TaggedVal::from(70336i32);
                 if m.indirect_call_table.len() < 5 { m.indirect_call_table.resize(5, None) }
m.indirect_call_table[1] = Some(40);
m.indirect_call_table[2] = Some(42);
m.indirect_call_table[3] = Some(24);
m.indirect_call_table[4] = Some(32);
                 m.memory[1024..3445].copy_from_slice(&[84, 114, 105, 112, 108, 101, 116, 32, 105, 115, 32, 37, 100, 44, 32, 37, 100, 44, 32, 37, 100, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 83, 117, 99, 99, 101, 115, 115, 0, 73, 108, 108, 101, 103, 97, 108, 32, 98, 121, 116, 101, 32, 115, 101, 113, 117, 101, 110, 99, 101, 0, 68, 111, 109, 97, 105, 110, 32, 101, 114, 114, 111, 114, 0, 82, 101, 115, 117, 108, 116, 32, 110, 111, 116, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 98, 108, 101, 0, 78, 111, 116, 32, 97, 32, 116, 116, 121, 0, 80, 101, 114, 109, 105, 115, 115, 105, 111, 110, 32, 100, 101, 110, 105, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 110, 111, 116, 32, 112, 101, 114, 109, 105, 116, 116, 101, 100, 0, 78, 111, 32, 115, 117, 99, 104, 32, 102, 105, 108, 101, 32, 111, 114, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 78, 111, 32, 115, 117, 99, 104, 32, 112, 114, 111, 99, 101, 115, 115, 0, 70, 105, 108, 101, 32, 101, 120, 105, 115, 116, 115, 0, 86, 97, 108, 117, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 32, 102, 111, 114, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 0, 78, 111, 32, 115, 112, 97, 99, 101, 32, 108, 101, 102, 116, 32, 111, 110, 32, 100, 101, 118, 105, 99, 101, 0, 79, 117, 116, 32, 111, 102, 32, 109, 101, 109, 111, 114, 121, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 115, 121, 0, 73, 110, 116, 101, 114, 114, 117, 112, 116, 101, 100, 32, 115, 121, 115, 116, 101, 109, 32, 99, 97, 108, 108, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 116, 101, 109, 112, 111, 114, 97, 114, 105, 108, 121, 32, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 73, 110, 118, 97, 108, 105, 100, 32, 115, 101, 101, 107, 0, 67, 114, 111, 115, 115, 45, 100, 101, 118, 105, 99, 101, 32, 108, 105, 110, 107, 0, 82, 101, 97, 100, 45, 111, 110, 108, 121, 32, 102, 105, 108, 101, 32, 115, 121, 115, 116, 101, 109, 0, 68, 105, 114, 101, 99, 116, 111, 114, 121, 32, 110, 111, 116, 32, 101, 109, 112, 116, 121, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 115, 101, 116, 32, 98, 121, 32, 112, 101, 101, 114, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 116, 105, 109, 101, 100, 32, 111, 117, 116, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 102, 117, 115, 101, 100, 0, 72, 111, 115, 116, 32, 105, 115, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 65, 100, 100, 114, 101, 115, 115, 32, 105, 110, 32, 117, 115, 101, 0, 66, 114, 111, 107, 101, 110, 32, 112, 105, 112, 101, 0, 73, 47, 79, 32, 101, 114, 114, 111, 114, 0, 78, 111, 32, 115, 117, 99, 104, 32, 100, 101, 118, 105, 99, 101, 32, 111, 114, 32, 97, 100, 100, 114, 101, 115, 115, 0, 78, 111, 32, 115, 117, 99, 104, 32, 100, 101, 118, 105, 99, 101, 0, 78, 111, 116, 32, 97, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 73, 115, 32, 97, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 84, 101, 120, 116, 32, 102, 105, 108, 101, 32, 98, 117, 115, 121, 0, 69, 120, 101, 99, 32, 102, 111, 114, 109, 97, 116, 32, 101, 114, 114, 111, 114, 0, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 0, 65, 114, 103, 117, 109, 101, 110, 116, 32, 108, 105, 115, 116, 32, 116, 111, 111, 32, 108, 111, 110, 103, 0, 83, 121, 109, 98, 111, 108, 105, 99, 32, 108, 105, 110, 107, 32, 108, 111, 111, 112, 0, 70, 105, 108, 101, 110, 97, 109, 101, 32, 116, 111, 111, 32, 108, 111, 110, 103, 0, 84, 111, 111, 32, 109, 97, 110, 121, 32, 111, 112, 101, 110, 32, 102, 105, 108, 101, 115, 32, 105, 110, 32, 115, 121, 115, 116, 101, 109, 0, 78, 111, 32, 102, 105, 108, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 66, 97, 100, 32, 102, 105, 108, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 78, 111, 32, 99, 104, 105, 108, 100, 32, 112, 114, 111, 99, 101, 115, 115, 0, 66, 97, 100, 32, 97, 100, 100, 114, 101, 115, 115, 0, 70, 105, 108, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 0, 84, 111, 111, 32, 109, 97, 110, 121, 32, 108, 105, 110, 107, 115, 0, 78, 111, 32, 108, 111, 99, 107, 115, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 82, 101, 115, 111, 117, 114, 99, 101, 32, 100, 101, 97, 100, 108, 111, 99, 107, 32, 119, 111, 117, 108, 100, 32, 111, 99, 99, 117, 114, 0, 83, 116, 97, 116, 101, 32, 110, 111, 116, 32, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 0, 80, 114, 101, 118, 105, 111, 117, 115, 32, 111, 119, 110, 101, 114, 32, 100, 105, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 99, 97, 110, 99, 101, 108, 101, 100, 0, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 111, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100, 0, 78, 111, 32, 109, 101, 115, 115, 97, 103, 101, 32, 111, 102, 32, 100, 101, 115, 105, 114, 101, 100, 32, 116, 121, 112, 101, 0, 73, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 114, 101, 109, 111, 118, 101, 100, 0, 76, 105, 110, 107, 32, 104, 97, 115, 32, 98, 101, 101, 110, 32, 115, 101, 118, 101, 114, 101, 100, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 101, 114, 114, 111, 114, 0, 66, 97, 100, 32, 109, 101, 115, 115, 97, 103, 101, 0, 78, 111, 116, 32, 97, 32, 115, 111, 99, 107, 101, 116, 0, 68, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 100, 100, 114, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 100, 0, 77, 101, 115, 115, 97, 103, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103, 101, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 119, 114, 111, 110, 103, 32, 116, 121, 112, 101, 32, 102, 111, 114, 32, 115, 111, 99, 107, 101, 116, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 110, 111, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 80, 114, 111, 116, 111, 99, 111, 108, 32, 110, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 0, 78, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 0, 65, 100, 100, 114, 101, 115, 115, 32, 102, 97, 109, 105, 108, 121, 32, 110, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 98, 121, 32, 112, 114, 111, 116, 111, 99, 111, 108, 0, 65, 100, 100, 114, 101, 115, 115, 32, 110, 111, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 78, 101, 116, 119, 111, 114, 107, 32, 105, 115, 32, 100, 111, 119, 110, 0, 78, 101, 116, 119, 111, 114, 107, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 114, 101, 115, 101, 116, 32, 98, 121, 32, 110, 101, 116, 119, 111, 114, 107, 0, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 97, 98, 111, 114, 116, 101, 100, 0, 78, 111, 32, 98, 117, 102, 102, 101, 114, 32, 115, 112, 97, 99, 101, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 0, 83, 111, 99, 107, 101, 116, 32, 105, 115, 32, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 83, 111, 99, 107, 101, 116, 32, 110, 111, 116, 32, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 114, 101, 97, 100, 121, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0, 79, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0, 83, 116, 97, 108, 101, 32, 102, 105, 108, 101, 32, 104, 97, 110, 100, 108, 101, 0, 81, 117, 111, 116, 97, 32, 101, 120, 99, 101, 101, 100, 101, 100, 0, 77, 117, 108, 116, 105, 104, 111, 112, 32, 97, 116, 116, 101, 109, 112, 116, 101, 100, 0, 67, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 105, 110, 115, 117, 102, 102, 105, 99, 105, 101, 110, 116, 0, 0, 0, 117, 2, 78, 0, 214, 1, 226, 4, 185, 4, 24, 1, 142, 5, 237, 2, 22, 4, 242, 0, 151, 3, 1, 3, 56, 5, 175, 1, 130, 1, 79, 3, 47, 4, 30, 0, 212, 5, 162, 0, 18, 3, 30, 3, 194, 1, 222, 3, 8, 0, 172, 5, 0, 1, 100, 2, 241, 1, 101, 5, 52, 2, 140, 2, 207, 2, 45, 3, 76, 4, 227, 5, 159, 2, 248, 4, 28, 5, 8, 5, 177, 2, 75, 5, 21, 2, 120, 0, 82, 2, 60, 3, 241, 3, 228, 0, 195, 3, 125, 4, 204, 0, 170, 3, 121, 5, 36, 2, 110, 1, 109, 3, 34, 4, 171, 4, 68, 0, 251, 1, 174, 0, 131, 3, 96, 0, 229, 1, 7, 4, 148, 4, 94, 4, 43, 0, 88, 1, 57, 1, 146, 0, 194, 5, 155, 1, 67, 2, 70, 1, 246, 5, 45, 43, 32, 32, 32, 48, 88, 48, 120, 0, 40, 110, 117, 108, 108, 41, 0, 0, 0, 0, 0, 0, 25, 0, 10, 0, 25, 25, 25, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 17, 10, 25, 25, 25, 3, 10, 7, 0, 1, 27, 9, 11, 24, 0, 0, 9, 6, 11, 0, 0, 11, 0, 6, 25, 0, 0, 0, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 10, 13, 25, 25, 25, 0, 13, 0, 0, 2, 0, 9, 14, 0, 0, 0, 9, 0, 14, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 19, 0, 0, 0, 0, 9, 12, 0, 0, 0, 0, 0, 12, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 4, 15, 0, 0, 0, 0, 9, 16, 0, 0, 0, 0, 0, 16, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 17, 0, 0, 0, 0, 9, 18, 0, 0, 0, 0, 0, 18, 0, 0, 18, 0, 0, 26, 0, 0, 0, 26, 26, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 26, 26, 26, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 23, 0, 0, 0, 0, 9, 20, 0, 0, 0, 0, 0, 20, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 21, 0, 0, 0, 0, 9, 22, 0, 0, 0, 0, 0, 22, 0, 0, 22, 0, 0, 83, 117, 112, 112, 111, 114, 116, 32, 102, 111, 114, 32, 102, 111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 108, 111, 110, 103, 32, 100, 111, 117, 98, 108, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 115, 32, 99, 117, 114, 114, 101, 110, 116, 108, 121, 32, 100, 105, 115, 97, 98, 108, 101, 100, 46, 10, 84, 111, 32, 101, 110, 97, 98, 108, 101, 32, 105, 116, 44, 32, 97, 100, 100, 32, 45, 108, 99, 45, 112, 114, 105, 110, 116, 115, 99, 97, 110, 45, 108, 111, 110, 103, 45, 100, 111, 117, 98, 108, 101, 32, 116, 111, 32, 116, 104, 101, 32, 108, 105, 110, 107, 32, 99, 111, 109, 109, 97, 110, 100, 46, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 45, 48, 88, 43, 48, 88, 32, 48, 88, 45, 48, 120, 43, 48, 120, 32, 48, 120, 0, 105, 110, 102, 0, 73, 78, 70, 0, 110, 97, 110, 0, 78, 65, 78, 0, 46, 0]);
m.memory[3448..3684].copy_from_slice(&[5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 136, 14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 13, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 180, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 13, 0, 0]);
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
fn func_4(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::fd_fdstat_get(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0, arg_1))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_5(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_6(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;self.func_5()?;
v0 = TaggedVal::from(self.func_10()?);
local_0 = v0.try_as_i32()?;
self.func_21()?;
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
fn func_7(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
let mut local_119 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(0i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_12);
self.func_8(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_27);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(local_33);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(local_38);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
v1 = TaggedVal::from(local_43);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_49);
v1 = TaggedVal::from(local_50);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_52);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_53);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_55);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_56);
v1 = TaggedVal::from(local_57);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_60 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_60);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_62);
v1 = TaggedVal::from(local_63);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_61);
v1 = TaggedVal::from(local_64);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_65);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_68);
v1 = TaggedVal::from(local_69);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_67);
v1 = TaggedVal::from(local_70);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_71);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_74);
v1 = TaggedVal::from(local_75);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_73);
v1 = TaggedVal::from(local_76);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_77);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_78);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_72);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_66);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(1024i32);
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_79);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(self.func_22(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(1i32);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_80);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_82);
v1 = TaggedVal::from(local_83);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_81);
v1 = TaggedVal::from(local_84);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_85);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_88);
v1 = TaggedVal::from(local_89);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_87);
v1 = TaggedVal::from(local_90);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_91);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_86);
v1 = TaggedVal::from(local_92);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_96 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_95);
v1 = TaggedVal::from(local_96);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_94);
v1 = TaggedVal::from(local_97);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_98);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_93);
v1 = TaggedVal::from(local_99);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_101 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_100);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_101);
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_102);
v1 = TaggedVal::from(local_103);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_104);
v1 = TaggedVal::from(local_105);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_106 = v0.try_as_i32()?;
'label_6: loop {
'label_7: loop {
v0 = TaggedVal::from(local_106);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_107);
v1 = TaggedVal::from(local_108);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_109);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_6;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_110);
v1 = TaggedVal::from(local_111);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_112);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
{

}
continue 'label_4;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_113);
v1 = TaggedVal::from(local_114);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_115 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_115);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_2;
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_116);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_118);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_119);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_117);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_8(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
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
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_9(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
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
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_7);
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
fn func_10(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : i32 = 0i32;
let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i64 = 0i64;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
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
v0 = TaggedVal::from(2i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(29i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
local_10 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_5);
v0 = TaggedVal::from(unsafe {PARAM1}[0]);
v0 = TaggedVal::from(self.func_7(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}

v0 = TaggedVal::from(16i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_11(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
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
let mut local_125 : i32 = 0i32;
let mut local_126 : i32 = 0i32;
let mut local_127 : i32 = 0i32;
let mut local_128 : i32 = 0i32;
let mut local_129 : i32 = 0i32;
let mut local_130 : i32 = 0i32;
let mut local_131 : i32 = 0i32;
let mut local_132 : i32 = 0i32;
let mut local_133 : i32 = 0i32;
let mut local_134 : i32 = 0i32;
let mut local_135 : i32 = 0i32;
let mut local_136 : i32 = 0i32;
let mut local_137 : i32 = 0i32;
let mut local_138 : i32 = 0i32;
let mut local_139 : i32 = 0i32;
let mut local_140 : i32 = 0i32;
let mut local_141 : i32 = 0i32;
let mut local_142 : i32 = 0i32;
let mut local_143 : i32 = 0i32;
let mut local_144 : i32 = 0i32;
let mut local_145 : i32 = 0i32;
let mut local_146 : i32 = 0i32;
let mut local_147 : i32 = 0i32;
let mut local_148 : i32 = 0i32;
let mut local_149 : i32 = 0i32;
let mut local_150 : i32 = 0i32;
let mut local_151 : i32 = 0i32;
let mut local_152 : i32 = 0i32;
let mut local_153 : i32 = 0i32;
let mut local_154 : i32 = 0i32;
let mut local_155 : i32 = 0i32;
let mut local_156 : i32 = 0i32;
let mut local_157 : i32 = 0i32;
let mut local_158 : i32 = 0i32;
let mut local_159 : i32 = 0i32;
let mut local_160 : i32 = 0i32;
let mut local_161 : i32 = 0i32;
let mut local_162 : i32 = 0i32;
let mut local_163 : i32 = 0i32;
let mut local_164 : i32 = 0i32;
let mut local_165 : i32 = 0i32;
let mut local_166 : i32 = 0i32;
let mut local_167 : i32 = 0i32;
let mut local_168 : i32 = 0i32;
let mut local_169 : i32 = 0i32;
let mut local_170 : i32 = 0i32;
let mut local_171 : i32 = 0i32;
let mut local_172 : i32 = 0i32;
let mut local_173 : i32 = 0i32;
let mut local_174 : i32 = 0i32;
let mut local_175 : i32 = 0i32;
let mut local_176 : i32 = 0i32;
let mut local_177 : i32 = 0i32;
let mut local_178 : i32 = 0i32;
let mut local_179 : i32 = 0i32;
let mut local_180 : i32 = 0i32;
let mut local_181 : i32 = 0i32;
let mut local_182 : i32 = 0i32;
let mut local_183 : i32 = 0i32;
let mut local_184 : i32 = 0i32;
let mut local_185 : i32 = 0i32;
let mut local_186 : i32 = 0i32;
let mut local_187 : i32 = 0i32;
let mut local_188 : i32 = 0i32;
let mut local_189 : i32 = 0i32;
let mut local_190 : i32 = 0i32;
let mut local_191 : i32 = 0i32;
let mut local_192 : i32 = 0i32;
let mut local_193 : i32 = 0i32;
let mut local_194 : i32 = 0i32;
let mut local_195 : i32 = 0i32;
let mut local_196 : i32 = 0i32;
let mut local_197 : i32 = 0i32;
let mut local_198 : i32 = 0i32;
let mut local_199 : i32 = 0i32;
let mut local_200 : i32 = 0i32;
let mut local_201 : i32 = 0i32;
let mut local_202 : i32 = 0i32;
let mut local_203 : i32 = 0i32;
let mut local_204 : i32 = 0i32;
let mut local_205 : i32 = 0i32;
let mut local_206 : i32 = 0i32;
let mut local_207 : i32 = 0i32;
let mut local_208 : i32 = 0i32;
let mut local_209 : i32 = 0i32;
let mut local_210 : i32 = 0i32;
let mut local_211 : i32 = 0i32;
let mut local_212 : i32 = 0i32;
let mut local_213 : i32 = 0i32;
let mut local_214 : i32 = 0i32;
let mut local_215 : i32 = 0i32;
let mut local_216 : i32 = 0i32;
let mut local_217 : i32 = 0i32;
let mut local_218 : i32 = 0i32;
let mut local_219 : i32 = 0i32;
let mut local_220 : i32 = 0i32;
let mut local_221 : i32 = 0i32;
let mut local_222 : i32 = 0i32;
let mut local_223 : i32 = 0i32;
let mut local_224 : i32 = 0i32;
let mut local_225 : i32 = 0i32;
let mut local_226 : i32 = 0i32;
let mut local_227 : i32 = 0i32;
let mut local_228 : i32 = 0i32;
let mut local_229 : i32 = 0i32;
let mut local_230 : i32 = 0i32;
let mut local_231 : i32 = 0i32;
let mut local_232 : i32 = 0i32;
let mut local_233 : i32 = 0i32;
let mut local_234 : i32 = 0i32;
let mut local_235 : i32 = 0i32;
let mut local_236 : i32 = 0i32;
let mut local_237 : i32 = 0i32;
let mut local_238 : i32 = 0i32;
let mut local_239 : i32 = 0i32;
let mut local_240 : i32 = 0i32;
let mut local_241 : i32 = 0i32;
let mut local_242 : i32 = 0i32;
let mut local_243 : i32 = 0i32;
let mut local_244 : i32 = 0i32;
let mut local_245 : i32 = 0i32;
let mut local_246 : i32 = 0i32;
let mut local_247 : i32 = 0i32;
let mut local_248 : i32 = 0i32;
let mut local_249 : i32 = 0i32;
let mut local_250 : i32 = 0i32;
let mut local_251 : i32 = 0i32;
let mut local_252 : i32 = 0i32;
let mut local_253 : i32 = 0i32;
let mut local_254 : i32 = 0i32;
let mut local_255 : i32 = 0i32;
let mut local_256 : i32 = 0i32;
let mut local_257 : i32 = 0i32;
let mut local_258 : i32 = 0i32;
let mut local_259 : i32 = 0i32;
let mut local_260 : i32 = 0i32;
let mut local_261 : i32 = 0i32;
let mut local_262 : i32 = 0i32;
let mut local_263 : i32 = 0i32;
let mut local_264 : i32 = 0i32;
let mut local_265 : i32 = 0i32;
let mut local_266 : i32 = 0i32;
let mut local_267 : i32 = 0i32;
let mut local_268 : i32 = 0i32;
let mut local_269 : i32 = 0i32;
let mut local_270 : i32 = 0i32;
let mut local_271 : i32 = 0i32;
let mut local_272 : i32 = 0i32;
let mut local_273 : i32 = 0i32;
let mut local_274 : i32 = 0i32;
let mut local_275 : i32 = 0i32;
let mut local_276 : i32 = 0i32;
let mut local_277 : i32 = 0i32;
let mut local_278 : i32 = 0i32;
let mut local_279 : i32 = 0i32;
let mut local_280 : i32 = 0i32;
let mut local_281 : i32 = 0i32;
let mut local_282 : i32 = 0i32;
let mut local_283 : i32 = 0i32;
let mut local_284 : i32 = 0i32;
let mut local_285 : i32 = 0i32;
let mut local_286 : i32 = 0i32;
let mut local_287 : i32 = 0i32;
let mut local_288 : i32 = 0i32;
let mut local_289 : i32 = 0i32;
let mut local_290 : i32 = 0i32;
let mut local_291 : i32 = 0i32;
let mut local_292 : i32 = 0i32;
let mut local_293 : i32 = 0i32;
let mut local_294 : i32 = 0i32;
let mut local_295 : i32 = 0i32;
let mut local_296 : i32 = 0i32;
let mut local_297 : i32 = 0i32;
let mut local_298 : i32 = 0i32;
let mut local_299 : i32 = 0i32;
let mut local_300 : i32 = 0i32;
let mut local_301 : i32 = 0i32;
let mut local_302 : i32 = 0i32;
let mut local_303 : i32 = 0i32;
let mut local_304 : i32 = 0i32;
let mut local_305 : i32 = 0i32;
let mut local_306 : i32 = 0i32;
let mut local_307 : i32 = 0i32;
let mut local_308 : i32 = 0i32;
let mut local_309 : i32 = 0i32;
let mut local_310 : i32 = 0i32;
let mut local_311 : i32 = 0i32;
let mut local_312 : i32 = 0i32;
let mut local_313 : i32 = 0i32;
let mut local_314 : i32 = 0i32;
let mut local_315 : i32 = 0i32;
let mut local_316 : i32 = 0i32;
let mut local_317 : i32 = 0i32;
let mut local_318 : i32 = 0i32;
let mut local_319 : i32 = 0i32;
let mut local_320 : i32 = 0i32;
let mut local_321 : i32 = 0i32;
let mut local_322 : i32 = 0i32;
let mut local_323 : i32 = 0i32;
let mut local_324 : i32 = 0i32;
let mut local_325 : i32 = 0i32;
let mut local_326 : i32 = 0i32;
let mut local_327 : i32 = 0i32;
let mut local_328 : i32 = 0i32;
let mut local_329 : i32 = 0i32;
let mut local_330 : i32 = 0i32;
let mut local_331 : i32 = 0i32;
let mut local_332 : i32 = 0i32;
let mut local_333 : i32 = 0i32;
let mut local_334 : i32 = 0i32;
let mut local_335 : i32 = 0i32;
let mut local_336 : i32 = 0i32;
let mut local_337 : i32 = 0i32;
let mut local_338 : i32 = 0i32;
let mut local_339 : i32 = 0i32;
let mut local_340 : i32 = 0i32;
let mut local_341 : i32 = 0i32;
let mut local_342 : i32 = 0i32;
let mut local_343 : i32 = 0i32;
let mut local_344 : i32 = 0i32;
let mut local_345 : i32 = 0i32;
let mut local_346 : i32 = 0i32;
let mut local_347 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(30i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(5i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

'label_4: loop {
'label_5: loop {
'label_6: loop {
'label_7: loop {
'label_8: loop {
'label_9: loop {
v0 = TaggedVal::from(local_12);
match v0.try_as_i32()? {
0 => {
{

}
break 'label_9;
},
1 => {
{

}
break 'label_9;
},
2 => {
{

}
break 'label_8;
},
3 => {
{

}
break 'label_7;
},
4 => {
{

}
break 'label_6;
},
5 => {
{

}
break 'label_5;
},
_ => {
{

}
break 'label_4;
},
}
break;
}
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_17);
v2 = TaggedVal::from(local_18);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
'label_10: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_30);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(local_30);
v3 = TaggedVal::from(local_31);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_40);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_41);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(local_35);
v2 = TaggedVal::from(local_38);
v3 = TaggedVal::from(local_41);
v4 = TaggedVal::from(local_42);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);

{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_45);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(12i32);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_53);
v1 = TaggedVal::from(local_54);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_55);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_46);
v2 = TaggedVal::from(local_49);
v3 = TaggedVal::from(local_52);
v4 = TaggedVal::from(local_55);
v5 = TaggedVal::from(local_56);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);

{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(30i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_57);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_59);
v1 = TaggedVal::from(local_60);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_61);
v1 = TaggedVal::from(local_62);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_63 = v0.try_as_i32()?;
'label_11: loop {
v0 = TaggedVal::from(local_63);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_11;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_64);
v1 = TaggedVal::from(local_65);
v2 = TaggedVal::from(local_66);
self.func_17(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(1000i32);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_68);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_69);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_70);
v1 = TaggedVal::from(local_71);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_72);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_73);
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_67);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_74);
v1 = TaggedVal::from(local_75);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v1 = TaggedVal::from(local_77);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_78 = v0.try_as_i32()?;
'label_12: loop {
'label_13: loop {
v0 = TaggedVal::from(local_78);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_79);
v1 = TaggedVal::from(local_80);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_81);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_82);
v1 = TaggedVal::from(local_84);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_83);
v1 = TaggedVal::from(local_85);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_86);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_87);
v1 = TaggedVal::from(local_88);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_89);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_92);
v1 = TaggedVal::from(local_93);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_91);
v1 = TaggedVal::from(local_94);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_96 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_98);
v1 = TaggedVal::from(local_99);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_97);
v1 = TaggedVal::from(local_100);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_101 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_90);
v1 = TaggedVal::from(local_95);
v2 = TaggedVal::from(local_96);
v3 = TaggedVal::from(local_101);
v4 = TaggedVal::from(local_102);
v5 = TaggedVal::from(local_103);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_104);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
break 'label_12;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_106 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_105);
v1 = TaggedVal::from(local_106);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_107);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_108);
v1 = TaggedVal::from(local_110);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_109);
v1 = TaggedVal::from(local_111);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_112);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_115 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_113);
v1 = TaggedVal::from(local_114);
v2 = TaggedVal::from(local_115);
v3 = TaggedVal::from(local_116);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_117);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_118);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_119);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_120 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_121 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_122 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_120);
v1 = TaggedVal::from(local_121);
v2 = TaggedVal::from(local_122);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_123 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_124 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_123);
v1 = TaggedVal::from(local_124);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_125 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_125);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_14: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_126 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_127 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_128 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_127);
v1 = TaggedVal::from(local_128);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_129 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_129);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_126);
local_130 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_129);
local_131 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_130);
v1 = TaggedVal::from(local_131);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_132 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_133 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_132);
v1 = TaggedVal::from(local_133);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_134 = v0.try_as_i32()?;
'label_15: loop {
v0 = TaggedVal::from(local_134);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_15;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_135 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_136 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_135);
v1 = TaggedVal::from(local_136);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_137 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_137);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_138 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_138);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_139 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_140 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_141 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_142 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_141);
v1 = TaggedVal::from(local_142);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_143 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_143);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_139);
v1 = TaggedVal::from(local_140);
v2 = TaggedVal::from(local_143);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_144 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_145 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_144);
v1 = TaggedVal::from(local_145);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_146 = v0.try_as_i32()?;
'label_16: loop {
v0 = TaggedVal::from(local_146);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
'label_17: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_147 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_148 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_147);
local_149 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_148);
local_150 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_149);
v1 = TaggedVal::from(local_150);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_151 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_152 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_151);
v1 = TaggedVal::from(local_152);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_153 = v0.try_as_i32()?;
'label_18: loop {
v0 = TaggedVal::from(local_153);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_154 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_155 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_156 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_154);
v1 = TaggedVal::from(local_155);
v2 = TaggedVal::from(local_156);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_157 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_158 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_157);
v1 = TaggedVal::from(local_158);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_159 = v0.try_as_i32()?;
'label_19: loop {
'label_20: loop {
v0 = TaggedVal::from(local_159);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_160 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_161 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_160);
v1 = TaggedVal::from(local_161);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_162 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_163 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_162);
v1 = TaggedVal::from(local_163);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_164 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_164);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_165 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_166 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_165);
v1 = TaggedVal::from(local_166);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_167 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_167);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_168 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_169 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_168);
v1 = TaggedVal::from(local_169);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_170 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_170);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_17;
break;
}
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_171 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_172 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_171);
local_173 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_172);
local_174 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_173);
v1 = TaggedVal::from(local_174);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_175 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_176 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_175);
v1 = TaggedVal::from(local_176);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_177 = v0.try_as_i32()?;
'label_21: loop {
v0 = TaggedVal::from(local_177);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
{

}
break 'label_1;
break;
}
'label_22: loop {
'label_23: loop {
'label_24: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_178 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_179 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_180 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_178);
v1 = TaggedVal::from(local_179);
v2 = TaggedVal::from(local_180);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_181 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_182 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_181);
v1 = TaggedVal::from(local_182);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_183 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_184 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_183);
v1 = TaggedVal::from(local_184);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_185 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_185);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_23;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_186 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_187 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_186);
v1 = TaggedVal::from(local_187);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_188 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_188);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_24;
break;}
break;
}
'label_25: loop {
'label_26: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_189 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_190 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_191 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_192 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_191);
v1 = TaggedVal::from(local_192);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_193 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_193);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_189);
v1 = TaggedVal::from(local_190);
v2 = TaggedVal::from(local_193);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_194 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_195 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_194);
v1 = TaggedVal::from(local_195);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_196 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_196);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_25;
}
{

}
continue 'label_26;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_197 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_198 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_197);
local_199 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_198);
local_200 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_199);
v1 = TaggedVal::from(local_200);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
local_201 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_202 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_201);
v1 = TaggedVal::from(local_202);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_203 = v0.try_as_i32()?;
'label_27: loop {
'label_28: loop {
v0 = TaggedVal::from(local_203);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_28;
}
{

}
break 'label_27;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_204 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_205 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_204);
v1 = TaggedVal::from(local_205);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_206 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_207 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_206);
v1 = TaggedVal::from(local_207);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_208 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_208);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_209 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_210 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_209);
v1 = TaggedVal::from(local_210);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_211 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_211);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_22;
break;
}
break;}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_212 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_212);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_213 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_214 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_215 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_213);
v1 = TaggedVal::from(local_214);
v2 = TaggedVal::from(local_215);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_216 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_217 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_216);
v1 = TaggedVal::from(local_217);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_218 = v0.try_as_i32()?;
'label_29: loop {
'label_30: loop {
v0 = TaggedVal::from(local_218);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_30;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_219 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_220 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_219);
v1 = TaggedVal::from(local_220);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_221 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_222 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_221);
v1 = TaggedVal::from(local_222);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_223 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_223);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
break 'label_29;
break;
}
{

}
continue 'label_14;
break;
}
break;}
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_224 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_225 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_224);
v1 = TaggedVal::from(local_225);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_226 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_226);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_227 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_228 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_227);
local_229 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_228);
local_230 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_229);
v1 = TaggedVal::from(local_230);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
local_231 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_232 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_231);
v1 = TaggedVal::from(local_232);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_233 = v0.try_as_i32()?;
'label_31: loop {
v0 = TaggedVal::from(local_233);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
'label_32: loop {
'label_33: loop {
'label_34: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_234 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_235 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_236 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_234);
v1 = TaggedVal::from(local_235);
v2 = TaggedVal::from(local_236);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_237 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_238 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_237);
v1 = TaggedVal::from(local_238);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_239 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_239);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_33;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_240 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_241 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_240);
v1 = TaggedVal::from(local_241);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_242 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_242);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_34;
break;}
break;
}
'label_35: loop {
'label_36: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_243 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_244 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_245 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_244);
v1 = TaggedVal::from(local_245);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_246 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_246);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_247 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_243);
v1 = TaggedVal::from(local_246);
v2 = TaggedVal::from(local_247);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_248 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_249 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_248);
v1 = TaggedVal::from(local_249);
v0 = TaggedVal::from(v0.try_as_i32()? ^ v1.try_as_i32()?);
local_250 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_251 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_250);
v1 = TaggedVal::from(local_251);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_252 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_252);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
{

}
continue 'label_36;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_253 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_254 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_253);
local_255 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_254);
local_256 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_255);
v1 = TaggedVal::from(local_256);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
local_257 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_258 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_257);
v1 = TaggedVal::from(local_258);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_259 = v0.try_as_i32()?;
'label_37: loop {
'label_38: loop {
v0 = TaggedVal::from(local_259);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_38;
}
{

}
break 'label_37;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_260 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_261 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_260);
v1 = TaggedVal::from(local_261);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_262 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_263 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_262);
v1 = TaggedVal::from(local_263);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_264 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_264);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_265 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_266 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_265);
local_267 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_266);
local_268 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_267);
v1 = TaggedVal::from(local_268);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_269 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_270 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_269);
v1 = TaggedVal::from(local_270);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_271 = v0.try_as_i32()?;
'label_39: loop {
v0 = TaggedVal::from(local_271);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_39;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_272 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_272);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_273 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_274 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_273);
v1 = TaggedVal::from(local_274);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_275 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_275);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_32;
break;
}
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_276 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_277 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_276);
local_278 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_277);
local_279 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_278);
v1 = TaggedVal::from(local_279);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_280 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_281 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_280);
v1 = TaggedVal::from(local_281);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_282 = v0.try_as_i32()?;
'label_40: loop {
v0 = TaggedVal::from(local_282);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_40;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_283 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_284 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_285 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_283);
v1 = TaggedVal::from(local_284);
v2 = TaggedVal::from(local_285);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_286 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_287 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_286);
v1 = TaggedVal::from(local_287);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_288 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_288);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_40;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_289 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_290 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_289);
v1 = TaggedVal::from(local_290);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_291 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_292 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_291);
v1 = TaggedVal::from(local_292);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_293 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_293);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_294 = v0.try_as_i32()?;
'label_41: loop {
v0 = TaggedVal::from(local_294);
if v0.try_as_i32()? != 0 {
{

}
break 'label_41;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_295 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_296 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_297 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_295);
v1 = TaggedVal::from(local_296);
v2 = TaggedVal::from(local_297);
v0 = TaggedVal::from(self.func_18(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_298 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_299 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_298);
v1 = TaggedVal::from(local_299);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_300 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_300);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 3) as usize, v1.try_as_i32()? as u8)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_301 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_302 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_301);
v1 = TaggedVal::from(local_302);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_303 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_304 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_305 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_303);
v1 = TaggedVal::from(local_304);
v2 = TaggedVal::from(local_305);
v0 = TaggedVal::from(self.func_18(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_306 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_307 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_306);
v1 = TaggedVal::from(local_307);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_308 = v0.try_as_i32()?;
'label_42: loop {
v0 = TaggedVal::from(local_308);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_42;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
local_309 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_310 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_309);
v1 = TaggedVal::from(local_310);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_311 = v0.try_as_i32()?;
'label_43: loop {
v0 = TaggedVal::from(local_311);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_43;
}
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_312 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_312);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 3) as usize).and_then(|x| Some(x as i32))?);
local_313 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_314 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_313);
v1 = TaggedVal::from(local_314);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_315 = v0.try_as_i32()?;
'label_44: loop {
v0 = TaggedVal::from(local_315);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_44;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_316 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_317 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_316);
v1 = TaggedVal::from(local_317);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_318 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_318);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_318);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_0;
break;
}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_319 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_320 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_319);
v1 = TaggedVal::from(local_320);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_321 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_322 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_321);
v1 = TaggedVal::from(local_322);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
local_323 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_324 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_325 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_324);
v1 = TaggedVal::from(local_325);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_326 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_327 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_326);
v1 = TaggedVal::from(local_327);
v0 = TaggedVal::from(v0.try_as_i32()? >> (v1.try_as_i32()? % 32));
local_328 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_323);
local_329 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_328);
local_330 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_329);
v1 = TaggedVal::from(local_330);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_331 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_332 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_331);
v1 = TaggedVal::from(local_332);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_333 = v0.try_as_i32()?;
'label_45: loop {
'label_46: loop {
v0 = TaggedVal::from(local_333);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_46;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_334 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_335 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_336 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_334);
v1 = TaggedVal::from(local_335);
v2 = TaggedVal::from(local_336);
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_337 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_338 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_337);
v1 = TaggedVal::from(local_338);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_339 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_339);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_339);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
break 'label_45;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_340 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_341 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_340);
v1 = TaggedVal::from(local_341);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_342 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_343 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_344 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_342);
v1 = TaggedVal::from(local_343);
v2 = TaggedVal::from(local_344);
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_345 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_345);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
break;
}
{

}
continue 'label_0;
break;
}
break;}
v0 = TaggedVal::from(48i32);
local_346 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_346);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_347 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_347);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_12(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
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
fn func_13(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
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
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
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
fn func_14(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
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
let mut local_59 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(0i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_10);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_13 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_13);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v2 = TaggedVal::from(local_16);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_18);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_19 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_19);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_20);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_21);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v2 = TaggedVal::from(local_26);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_29 = v0.try_as_i32()?;
'label_3: loop {
v0 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(2i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_32);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_30);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_33);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_35);
v2 = TaggedVal::from(local_36);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_38);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_39 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_39);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(1i32);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_40);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_43);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_46);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_44);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v2 = TaggedVal::from(local_49);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_52 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_52);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(2i32);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_55);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_53);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_56);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_58);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_59);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_57);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_15(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32) -> Option<i32> {
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
let mut local_48 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_10);
v3 = TaggedVal::from(local_11);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_12);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_14);
v2 = TaggedVal::from(local_15);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_18 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_18);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v2 = TaggedVal::from(local_26);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_29 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(local_36);
v2 = TaggedVal::from(local_37);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_40 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_40);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_45);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
break;
}
break;
}
break;
}
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_47);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_46);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_16(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32, arg_5: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;
let mut local_4 : i32 = arg_4;
let mut local_5 : i32 = arg_5;let mut local_6 : i32 = 0i32;
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
let mut local_61 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;v0 = self.globals[0];
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(local_11);
v3 = TaggedVal::from(local_12);
v4 = TaggedVal::from(local_13);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_14);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v2 = TaggedVal::from(local_17);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_20 = v0.try_as_i32()?;
'label_0: loop {
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_25);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v2 = TaggedVal::from(local_28);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(local_30);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_31 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_31);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v1 = TaggedVal::from(local_33);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_36);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_38);
v2 = TaggedVal::from(local_39);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v1 = TaggedVal::from(local_41);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_42 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_42);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
v1 = TaggedVal::from(local_44);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_47);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
v1 = TaggedVal::from(local_49);
v2 = TaggedVal::from(local_50);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_51);
v1 = TaggedVal::from(local_52);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_53 = v0.try_as_i32()?;
'label_3: loop {
v0 = TaggedVal::from(local_53);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_55);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_56);
v1 = TaggedVal::from(local_57);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_58);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
break;
}
break;
}
break;
}
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_60);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_61);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_59);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_17(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
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
let mut local_72 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(8i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_12);
v2 = TaggedVal::from(local_13);
v3 = TaggedVal::from(local_14);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_16);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_17);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_23);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v2 = TaggedVal::from(local_27);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_30 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_30);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_33);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_34);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_35);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_3: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_38);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_40);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_45);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_46);
v1 = TaggedVal::from(local_47);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
local_49 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_48);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(8i32);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_50);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_51);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_55);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_56);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_53);
v1 = TaggedVal::from(local_52);
v2 = TaggedVal::from(local_56);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_57);
local_49 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_49);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_60);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_3;
}
break;}
v0 = TaggedVal::from(8i32);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_61);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_62);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_64);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_66);
v1 = TaggedVal::from(local_65);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_67);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_68);
v1 = TaggedVal::from(local_69);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_70);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_1;
break;}
break;
}
v0 = TaggedVal::from(32i32);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_71);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_72);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_18(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
let mut local_125 : i32 = 0i32;
let mut local_126 : i32 = 0i32;
let mut local_127 : i32 = 0i32;
let mut local_128 : i32 = 0i32;
let mut local_129 : i32 = 0i32;
let mut local_130 : i32 = 0i32;
let mut local_131 : i32 = 0i32;
let mut local_132 : i32 = 0i32;
let mut local_133 : i32 = 0i32;
let mut local_134 : i32 = 0i32;
let mut local_135 : i32 = 0i32;
let mut local_136 : i32 = 0i32;
let mut local_137 : i32 = 0i32;
let mut local_138 : i32 = 0i32;
let mut local_139 : i32 = 0i32;
let mut local_140 : i32 = 0i32;
let mut local_141 : i32 = 0i32;
let mut local_142 : i32 = 0i32;
let mut local_143 : i32 = 0i32;
let mut local_144 : i32 = 0i32;
let mut local_145 : i32 = 0i32;
let mut local_146 : i32 = 0i32;
let mut local_147 : i32 = 0i32;
let mut local_148 : i32 = 0i32;
let mut local_149 : i32 = 0i32;
let mut local_150 : i32 = 0i32;
let mut local_151 : i32 = 0i32;
let mut local_152 : i32 = 0i32;
let mut local_153 : i32 = 0i32;
let mut local_154 : i32 = 0i32;
let mut local_155 : i32 = 0i32;
let mut local_156 : i32 = 0i32;
let mut local_157 : i32 = 0i32;
let mut local_158 : i32 = 0i32;
let mut local_159 : i32 = 0i32;
let mut local_160 : i32 = 0i32;
let mut local_161 : i32 = 0i32;
let mut local_162 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_div(v1.try_as_i32()?)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(5i32);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_11);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);

'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
'label_5: loop {
'label_6: loop {
v0 = TaggedVal::from(local_10);
match v0.try_as_i32()? {
0 => {
{

}
break 'label_6;
},
1 => {
{

}
break 'label_6;
},
2 => {
{

}
break 'label_5;
},
3 => {
{

}
break 'label_4;
},
4 => {
{

}
break 'label_3;
},
5 => {
{

}
break 'label_2;
},
_ => {
{

}
break 'label_1;
},
}
break;
}
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_14);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_18);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(local_18);
v2 = TaggedVal::from(local_19);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_22 = v0.try_as_i32()?;
'label_7: loop {
v0 = TaggedVal::from(local_22);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_24);
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(1i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_27);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_35);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(local_32);
v2 = TaggedVal::from(local_35);
v3 = TaggedVal::from(local_36);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

v0 = TaggedVal::from(1i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_38);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_45);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_49);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v1 = TaggedVal::from(local_43);
v2 = TaggedVal::from(local_46);
v3 = TaggedVal::from(local_49);
v4 = TaggedVal::from(local_50);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);

v0 = TaggedVal::from(1i32);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_52);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_55);
v1 = TaggedVal::from(local_56);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(12i32);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_61);
v1 = TaggedVal::from(local_62);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_64);
v1 = TaggedVal::from(local_65);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_66);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_57);
v2 = TaggedVal::from(local_60);
v3 = TaggedVal::from(local_63);
v4 = TaggedVal::from(local_66);
v5 = TaggedVal::from(local_67);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);

v0 = TaggedVal::from(1i32);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_53);
v1 = TaggedVal::from(local_68);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_69);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_72);
v1 = TaggedVal::from(local_73);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_74);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v1 = TaggedVal::from(local_77);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_75);
v1 = TaggedVal::from(local_78);
v2 = TaggedVal::from(local_79);
v3 = TaggedVal::from(local_80);
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_71);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_70);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_81);
v1 = TaggedVal::from(local_82);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_83);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_8: loop {
'label_9: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_84);
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_85);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_86);
v1 = TaggedVal::from(local_87);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_88);
v1 = TaggedVal::from(local_89);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_90);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_91);
v1 = TaggedVal::from(local_92);
v2 = TaggedVal::from(local_93);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_94);
v1 = TaggedVal::from(local_95);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_96 = v0.try_as_i32()?;
'label_10: loop {
v0 = TaggedVal::from(local_96);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_97);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_98);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_99);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_100);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_101 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_101);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
'label_11: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_102);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_103);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_105);
v1 = TaggedVal::from(local_104);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_106 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_106);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_108);
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_109);
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_110);
v1 = TaggedVal::from(local_111);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_112);
v1 = TaggedVal::from(local_113);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_107);
local_115 = v0.try_as_i32()?;
'label_12: loop {
v0 = TaggedVal::from(local_114);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(12i32);
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_116);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_117);
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_120 = v0.try_as_i32()?;
v0 = TaggedVal::from(-4i32);
local_121 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_120);
v1 = TaggedVal::from(local_121);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_122 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_122);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_119);
v1 = TaggedVal::from(local_118);
v2 = TaggedVal::from(local_122);
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_123 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_123);
local_115 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_115);
local_124 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_125 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_124);
v1 = TaggedVal::from(local_125);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_126 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_126);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_11;
}
break;}
v0 = TaggedVal::from(8i32);
local_127 = v0.try_as_i32()?;
v0 = TaggedVal::from(12i32);
local_128 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_128);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_129 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_129);
local_130 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_130);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?)?);
local_131 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_131);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_132 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_133 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_133);
v1 = TaggedVal::from(local_132);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_134 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_135 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_134);
v1 = TaggedVal::from(local_135);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_136 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_136);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_136);
local_137 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_127);
local_138 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_137);
v1 = TaggedVal::from(local_138);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_139 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_140 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_139);
v1 = TaggedVal::from(local_140);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_141 = v0.try_as_i32()?;
'label_13: loop {
v0 = TaggedVal::from(local_141);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_142 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_143 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_142);
v1 = TaggedVal::from(local_143);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_144 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_144);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_145 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_144);
local_146 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_145);
local_147 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_146);
v1 = TaggedVal::from(local_147);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
local_148 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_149 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_148);
v1 = TaggedVal::from(local_149);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_150 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_150);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
{

}
break 'label_0;
break;
}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_151 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_151);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_152 = v0.try_as_i32()?;
v0 = TaggedVal::from(4i32);
local_153 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_152);
v1 = TaggedVal::from(local_153);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_154 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_154);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_9;
break;}
break;
}
v0 = TaggedVal::from(1i32);
local_155 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_156 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_155);
v1 = TaggedVal::from(local_156);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_157 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_157);
write_mem_u8(&mut self.memory, (v0.try_as_i32()? + 47) as usize, v1.try_as_i32()? as u8)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 47) as usize).and_then(|x| Some(x as i32))?);
local_158 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_159 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_158);
v1 = TaggedVal::from(local_159);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_160 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_161 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_161);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_162 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_162);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_160);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_19(&mut self, arg_0: i32) -> Option<i32> {
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
fn func_20(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_21(&mut self, ) -> Option<()> {
self.func_20()?;
self.func_26()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_22(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(3448i32);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_35(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_1);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_23(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
fn func_24(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;
let mut local_2 : i32 = arg_2;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_23(v0.try_as_i32()?, v1.try_as_i64()?, v2.try_as_i32()?)?);Some(v0.try_as_i64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_25(&mut self, ) -> Option<i32> {
let mut v0: TaggedVal;v0 = TaggedVal::from(4744i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_26(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(self.func_25()?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4748) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3560) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3680) as usize)?);
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
fn func_27(&mut self, arg_0: i32) -> Option<i32> {
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
fn func_28(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_44(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
fn func_29(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_44(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
fn func_30(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4776) as usize)?);
local_1 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(4752i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(4752i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4776) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(2608i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u16(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(1056i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 20) as usize)?);
v0 = TaggedVal::from(self.func_49(v0.try_as_i32()?, v1.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_31(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
fn func_32(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v1 = TaggedVal::from(self.func_31(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
v1 = TaggedVal::from(self.func_31(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
fn func_33(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_46(v0.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(1i32);
v5 = TaggedVal::from(local_2);
v6 = TaggedVal::from(local_1);
v3 = TaggedVal::from(self.func_29(v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?);
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
fn func_34(&mut self, ) -> Option<()> {
unreachable!("Reached a point explicitly marked unreachable in WASM module");
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_35(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_27(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_36(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
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
fn func_36(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
v1 = TaggedVal::from(2784i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
self.func_37(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(2762i32);
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
v2 = TaggedVal::from(3392i32);
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
v1 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(2763i32);
local_31 = v0.try_as_i32()?;
{

}
break 'label_70;
break;
}
v0 = TaggedVal::from(2764i32);
v1 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 3696) as usize)?);
v0 = TaggedVal::from(self.func_30(v0.try_as_i32()?)?);
local_18 = v0.try_as_i32()?;
{

}
break 'label_65;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(2772i32);
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
v1 = TaggedVal::from(self.func_43(v1.try_as_i32()?, v2.try_as_i32()?)?);
local_27 = v1.try_as_i32()?;
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(self.func_50(v0.try_as_i32()?, v1.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_50(v0.try_as_i32()?, v1.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(3408i32);
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
v0 = TaggedVal::from(3411i32);
local_37 = v0.try_as_i32()?;
{

}
break 'label_111;
break;
}
v0 = TaggedVal::from(3414i32);
v1 = TaggedVal::from(3409i32);
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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(3435i32);
v1 = TaggedVal::from(3439i32);
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
v1 = TaggedVal::from(3427i32);
v2 = TaggedVal::from(3431i32);
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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_52(v0.try_as_f64()?, v1.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(3443i32);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

{

}
break 'label_56;
break;
}
v0 = TaggedVal::from(0i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2762i32);
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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(3443i32);
v1 = TaggedVal::from(1i32);
v2 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v2 = TaggedVal::from(3392i32);
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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_45(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_28(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
fn func_37(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
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
self.func_38()?;
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
fn func_38(&mut self, ) -> Option<()> {
let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(3248i32);
v1 = TaggedVal::from(3568i32);
v0 = TaggedVal::from(self.func_33(v0.try_as_i32()?, v1.try_as_i32()?)?);

self.func_34()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_39(&mut self, arg_0: i32) -> Option<i32> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_40(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v0 = TaggedVal::from(self.func_39(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_41(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(self.func_4(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_0 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(59i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 8) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 16) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(36i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(1i32);
local_2 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_2);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_42(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(4i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(64i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v0 = TaggedVal::from(self.func_41(v0.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 64) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_32(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_43(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_47(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
fn func_44(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
fn func_45(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
fn func_46(&mut self, arg_0: i32) -> Option<i32> {
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
fn func_47(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
fn func_48(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_49(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_48(v0.try_as_i32()?, v1.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_50(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_51(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_51(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4752) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 3696) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(-1i32);
local_3 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_3);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_52(&mut self, arg_0: f64, arg_1: i32) -> Option<f64> {
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
v0 = TaggedVal::from(self.func_52(v0.try_as_f64()?, v1.try_as_i32()?)?);
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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_4(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
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
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_7(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
8 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_8(a0, a1)?;
                         Some(vec![])
                     }
9 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_9(a0, a1)?;
                         Some(vec![])
                     }
10 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_10()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
11 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_11(a0, a1, a2)?;
                         Some(vec![])
                     }
12 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_12(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
13 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_13(a0, a1)?;
                         Some(vec![])
                     }
14 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_14(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
15 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
                         let rets = self.func_15(a0, a1, a2, a3, a4)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
16 => {
                         if args.len() != 6 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
let a5 = args[5].try_as_i32()?;
                         let rets = self.func_16(a0, a1, a2, a3, a4, a5)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
17 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_17(a0, a1, a2)?;
                         Some(vec![])
                     }
18 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_18(a0, a1, a2)?;
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
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_20()?;
                         Some(vec![])
                     }
21 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_21()?;
                         Some(vec![])
                     }
22 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_22(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
23 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_23(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
24 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_24(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
25 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_25()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
26 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_26()?;
                         Some(vec![])
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
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_28(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
29 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_29(a0, a1, a2, a3)?;
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
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_31(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
32 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_32(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
33 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_33(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
34 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_34()?;
                         Some(vec![])
                     }
35 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_35(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
36 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
                         let rets = self.func_36(a0, a1, a2, a3, a4)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
37 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_37(a0, a1, a2)?;
                         Some(vec![])
                     }
38 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_38()?;
                         Some(vec![])
                     }
39 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_39(a0)?;
                         Some(vec![TaggedVal::from(rets)])
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
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_41(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
42 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_42(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
43 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_43(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
44 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_44(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
45 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_45(a0, a1, a2)?;
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
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_47(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
48 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_48(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
49 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_49(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
50 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_50(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
51 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_51(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
52 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_f64()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_52(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
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


fn f_gold(a: i32[i32], sum: i32) -> i32 {
    let mut a = a.to_vec();
    a.sort();
    let (mut l, mut r) = (0, a.len() - 1);
    for i in 0..a.len() - 2 {
        while l < r {
            if a[i] + a[l] + a[r] == sum {
                return 1; 
            } else if a[i] + a[l] + a[r] < sum {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    0
}////// LLM Output //////

use bolero::check;
#[test]
fn bolero_wasm_eq(){
	bolero::check!().with_type::<([i32;2], i32, i32)>().cloned().for_each(|(PARAM_1,PARAM_2,PARAM_3)|{ 
		unsafe {
		PARAM1 = PARAM_1;
		PARAM2 = PARAM_2;
		PARAM3 = PARAM_3;

		}
		let result = f_gold([unsafe{PARAM1}[0], unsafe{PARAM1}[1]],unsafe{PARAM2}.into(),unsafe{PARAM3}.into());
		let result_prime = f_gold_wasm_thread_unsafe();
		assert_eq!(result, result_prime);
	});
}