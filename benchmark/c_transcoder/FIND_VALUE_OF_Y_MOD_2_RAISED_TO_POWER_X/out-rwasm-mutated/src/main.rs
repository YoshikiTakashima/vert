static mut PARAM1: i32 = 12;
static mut PARAM2: i32 = 12;
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
                 m.globals[0] = TaggedVal::from(68752i32);
                 
                 m.memory[1024..3216].copy_from_slice(&[0, 0, 32, 101, 71, 21, 247, 63, 0, 162, 239, 46, 252, 5, 231, 61, 57, 131, 43, 101, 71, 21, 231, 191, 190, 4, 58, 220, 9, 199, 222, 63, 251, 47, 112, 100, 71, 21, 215, 191, 72, 76, 3, 80, 108, 119, 210, 63, 188, 146, 234, 40, 179, 199, 206, 191, 46, 249, 23, 225, 37, 98, 202, 63, 254, 130, 43, 101, 71, 21, 231, 191, 247, 3, 58, 220, 9, 199, 222, 63, 63, 124, 43, 101, 71, 21, 215, 191, 228, 91, 240, 80, 108, 119, 210, 63, 229, 143, 118, 221, 9, 199, 206, 191, 54, 231, 196, 30, 118, 97, 202, 63, 155, 167, 100, 188, 63, 21, 199, 191, 74, 27, 240, 84, 209, 132, 196, 63, 60, 56, 44, 167, 228, 137, 194, 191, 102, 238, 90, 40, 47, 179, 192, 63, 248, 172, 177, 107, 40, 36, 247, 63, 0, 176, 205, 238, 95, 9, 225, 191, 161, 204, 210, 102, 247, 225, 246, 63, 0, 208, 118, 189, 148, 132, 224, 191, 138, 212, 48, 14, 61, 161, 246, 63, 0, 248, 232, 174, 67, 1, 224, 191, 133, 108, 208, 50, 236, 97, 246, 63, 0, 64, 11, 54, 197, 254, 222, 191, 248, 152, 17, 149, 250, 35, 246, 63, 0, 224, 183, 26, 217, 253, 221, 191, 108, 2, 207, 164, 91, 231, 245, 63, 0, 144, 199, 12, 174, 255, 220, 191, 184, 79, 33, 90, 5, 172, 245, 63, 0, 160, 253, 17, 56, 4, 220, 191, 30, 110, 22, 15, 237, 113, 245, 63, 0, 224, 58, 50, 103, 11, 219, 191, 53, 248, 11, 89, 9, 57, 245, 63, 0, 176, 45, 90, 47, 21, 218, 191, 221, 173, 97, 237, 79, 1, 245, 63, 0, 96, 248, 90, 127, 33, 217, 191, 208, 123, 72, 142, 184, 202, 244, 63, 0, 144, 113, 176, 77, 48, 216, 191, 238, 79, 51, 180, 57, 149, 244, 63, 0, 224, 169, 249, 137, 65, 215, 191, 105, 213, 175, 223, 203, 96, 244, 63, 0, 144, 25, 181, 43, 85, 214, 191, 83, 185, 228, 78, 102, 45, 244, 63, 0, 16, 155, 162, 35, 107, 213, 191, 166, 216, 29, 17, 1, 251, 243, 63, 0, 160, 95, 15, 101, 131, 212, 191, 54, 88, 12, 183, 149, 201, 243, 63, 0, 160, 246, 55, 233, 157, 211, 191, 74, 253, 182, 74, 28, 153, 243, 63, 0, 96, 141, 83, 161, 186, 210, 191, 181, 153, 224, 12, 142, 105, 243, 63, 0, 64, 202, 64, 131, 217, 209, 191, 178, 231, 19, 130, 228, 58, 243, 63, 0, 224, 64, 58, 133, 250, 208, 191, 177, 189, 133, 25, 25, 13, 243, 63, 0, 48, 231, 50, 156, 29, 208, 191, 215, 113, 178, 202, 37, 224, 242, 63, 0, 96, 250, 162, 125, 133, 206, 191, 130, 205, 19, 207, 4, 180, 242, 63, 0, 128, 61, 99, 200, 211, 204, 191, 80, 203, 124, 44, 176, 136, 242, 63, 0, 160, 20, 76, 3, 38, 203, 191, 229, 77, 148, 99, 34, 94, 242, 63, 0, 224, 79, 47, 28, 124, 201, 191, 177, 21, 134, 61, 86, 52, 242, 63, 0, 0, 128, 63, 2, 214, 199, 191, 56, 175, 62, 227, 70, 11, 242, 63, 0, 224, 5, 26, 167, 51, 198, 191, 221, 163, 205, 253, 238, 226, 241, 63, 0, 0, 87, 233, 245, 148, 196, 191, 48, 57, 11, 88, 74, 187, 241, 63, 0, 160, 224, 36, 228, 249, 194, 191, 0, 34, 127, 132, 83, 148, 241, 63, 0, 192, 253, 90, 89, 98, 193, 191, 60, 215, 213, 192, 6, 110, 241, 63, 0, 128, 189, 117, 154, 156, 191, 191, 194, 228, 183, 71, 95, 72, 241, 63, 0, 192, 249, 91, 87, 123, 188, 191, 209, 133, 0, 173, 88, 35, 241, 63, 0, 128, 244, 15, 198, 96, 185, 191, 39, 34, 83, 15, 240, 254, 240, 63, 0, 0, 182, 71, 226, 76, 182, 191, 143, 58, 208, 119, 32, 219, 240, 63, 0, 64, 1, 178, 120, 63, 179, 191, 217, 128, 89, 214, 230, 183, 240, 63, 0, 192, 66, 26, 125, 56, 176, 191, 141, 64, 123, 254, 62, 149, 240, 63, 0, 0, 181, 8, 146, 111, 170, 191, 131, 59, 197, 202, 37, 115, 240, 63, 0, 0, 119, 79, 149, 122, 164, 191, 92, 27, 13, 228, 151, 81, 240, 63, 0, 0, 12, 197, 168, 35, 157, 191, 162, 142, 32, 193, 145, 48, 240, 63, 0, 0, 120, 41, 38, 106, 145, 191, 33, 126, 179, 37, 16, 16, 240, 63, 0, 0, 232, 216, 248, 32, 119, 191, 107, 167, 202, 249, 126, 192, 239, 63, 0, 0, 80, 177, 83, 254, 134, 63, 132, 241, 246, 211, 101, 68, 239, 63, 0, 128, 15, 225, 204, 28, 161, 63, 127, 16, 132, 159, 7, 204, 238, 63, 0, 128, 139, 140, 252, 77, 172, 63, 232, 90, 151, 153, 58, 87, 238, 63, 0, 64, 87, 30, 50, 170, 179, 63, 230, 61, 189, 240, 214, 229, 237, 63, 0, 128, 139, 208, 160, 24, 185, 63, 179, 56, 255, 129, 182, 119, 237, 63, 0, 64, 4, 218, 233, 114, 190, 63, 67, 233, 77, 114, 181, 12, 237, 63, 0, 96, 127, 80, 210, 220, 193, 63, 99, 117, 14, 220, 178, 164, 236, 63, 0, 160, 222, 3, 171, 118, 196, 63, 81, 203, 214, 232, 142, 63, 236, 63, 0, 32, 226, 119, 67, 7, 199, 63, 76, 12, 2, 79, 43, 221, 235, 63, 0, 64, 169, 139, 222, 142, 201, 63, 202, 21, 96, 0, 108, 125, 235, 63, 0, 224, 210, 106, 184, 13, 204, 63, 143, 51, 46, 110, 54, 32, 235, 63, 0, 224, 206, 175, 10, 132, 206, 63, 57, 80, 41, 38, 112, 197, 234, 63, 0, 128, 103, 180, 10, 121, 208, 63, 221, 49, 39, 188, 1, 109, 234, 63, 0, 192, 1, 104, 5, 172, 209, 63, 139, 241, 63, 188, 211, 22, 234, 63, 0, 224, 254, 212, 17, 219, 210, 63, 173, 254, 103, 73, 209, 194, 233, 63, 0, 128, 197, 78, 70, 6, 212, 63, 2, 153, 124, 244, 228, 112, 233, 63, 0, 240, 58, 9, 190, 45, 213, 63, 242, 188, 130, 57, 251, 32, 233, 63, 0, 208, 80, 32, 144, 81, 214, 63, 241, 89, 247, 135, 1, 211, 232, 63, 0, 240, 234, 205, 210, 113, 215, 63, 109, 246, 185, 235, 229, 134, 232, 63, 0, 144, 125, 133, 156, 142, 216, 63, 148, 185, 88, 182, 151, 60, 232, 63, 0, 96, 225, 85, 1, 168, 217, 63, 34, 16, 198, 255, 5, 244, 231, 63, 0, 208, 211, 110, 24, 190, 218, 63, 202, 21, 20, 24, 34, 173, 231, 63, 0, 224, 160, 174, 242, 208, 219, 63, 140, 255, 158, 249, 220, 103, 231, 63, 0, 64, 191, 61, 164, 224, 220, 63, 142, 10, 185, 18, 0, 32, 230, 63, 5, 182, 68, 6, 171, 4, 137, 60, 166, 52, 87, 4, 0, 96, 230, 63, 169, 247, 98, 234, 155, 255, 97, 60, 197, 242, 37, 195, 255, 159, 230, 63, 186, 144, 60, 203, 207, 126, 130, 60, 4, 90, 185, 56, 0, 224, 230, 63, 38, 147, 115, 86, 136, 255, 136, 60, 227, 148, 153, 224, 255, 31, 231, 63, 177, 130, 95, 39, 64, 253, 138, 60, 16, 14, 89, 21, 0, 96, 231, 63, 65, 131, 35, 180, 117, 253, 114, 188, 213, 91, 101, 18, 0, 160, 231, 63, 118, 43, 36, 124, 230, 8, 120, 60, 166, 233, 89, 50, 0, 224, 231, 63, 183, 34, 246, 38, 228, 8, 98, 188, 210, 178, 180, 237, 255, 31, 232, 63, 47, 201, 165, 30, 70, 2, 132, 188, 195, 252, 250, 45, 0, 96, 232, 63, 31, 154, 242, 162, 244, 247, 109, 60, 80, 107, 140, 247, 255, 159, 232, 63, 253, 149, 73, 9, 83, 4, 142, 188, 102, 21, 103, 57, 0, 224, 232, 63, 69, 123, 199, 190, 243, 4, 138, 188, 69, 23, 191, 226, 255, 31, 233, 63, 60, 32, 14, 64, 52, 250, 119, 188, 209, 159, 92, 204, 255, 95, 233, 63, 93, 105, 160, 5, 128, 255, 118, 188, 103, 71, 186, 59, 0, 160, 233, 63, 3, 126, 236, 196, 196, 248, 112, 60, 165, 45, 185, 231, 255, 223, 233, 63, 2, 70, 140, 71, 217, 127, 142, 60, 175, 253, 46, 215, 255, 31, 234, 63, 126, 174, 205, 77, 85, 12, 106, 188, 149, 255, 4, 222, 255, 95, 234, 63, 107, 178, 233, 140, 169, 125, 134, 60, 43, 141, 94, 202, 255, 159, 234, 63, 222, 19, 76, 181, 201, 132, 130, 188, 234, 3, 173, 221, 255, 223, 234, 63, 60, 46, 96, 234, 200, 18, 88, 60, 77, 61, 13, 241, 255, 31, 235, 63, 156, 120, 39, 173, 221, 250, 142, 188, 90, 22, 33, 206, 255, 95, 235, 63, 55, 18, 198, 25, 23, 203, 83, 60, 116, 230, 80, 217, 255, 159, 235, 63, 0, 206, 148, 65, 217, 247, 115, 60, 175, 168, 156, 19, 0, 224, 235, 63, 192, 155, 93, 33, 196, 10, 117, 60, 153, 223, 70, 91, 0, 32, 236, 63, 201, 193, 233, 83, 166, 238, 107, 60, 174, 247, 185, 64, 0, 96, 236, 63, 214, 112, 74, 39, 159, 7, 124, 188, 138, 253, 85, 98, 0, 160, 236, 63, 31, 76, 232, 118, 64, 11, 122, 188, 93, 9, 76, 217, 255, 223, 236, 63, 215, 181, 154, 249, 51, 249, 136, 60, 207, 214, 117, 249, 255, 31, 237, 63, 190, 225, 95, 102, 8, 44, 88, 188, 147, 28, 86, 162, 255, 95, 237, 63, 243, 149, 210, 155, 40, 4, 123, 188, 12, 139, 34, 157, 255, 159, 237, 63, 54, 162, 15, 52, 81, 2, 135, 60, 22, 126, 188, 101, 0, 224, 237, 63, 12, 216, 164, 22, 30, 1, 117, 188, 145, 71, 246, 2, 0, 32, 238, 63, 224, 98, 239, 9, 47, 128, 137, 60, 216, 166, 215, 87, 0, 96, 238, 63, 250, 247, 12, 88, 117, 11, 126, 188, 12, 192, 237, 39, 0, 160, 238, 63, 17, 152, 69, 9, 131, 132, 140, 188, 124, 203, 245, 108, 0, 224, 238, 63, 244, 118, 21, 149, 39, 128, 143, 188, 204, 125, 43, 120, 0, 32, 239, 63, 143, 83, 116, 114, 217, 129, 143, 188, 10, 69, 12, 38, 0, 96, 239, 63, 220, 255, 39, 39, 0, 113, 64, 188, 51, 213, 140, 232, 255, 159, 239, 63, 176, 168, 253, 225, 220, 27, 88, 188, 137, 134, 15, 213, 255, 223, 239, 63, 110, 142, 145, 203, 26, 249, 135, 60, 103, 35, 41, 4, 0, 32, 240, 63, 129, 70, 50, 101, 243, 127, 155, 60, 104, 214, 227, 227, 255, 95, 240, 63, 123, 149, 174, 221, 8, 250, 134, 60, 87, 167, 133, 10, 0, 160, 240, 63, 145, 251, 211, 128, 222, 226, 87, 188, 204, 63, 95, 26, 0, 224, 240, 63, 20, 240, 197, 5, 51, 130, 145, 188, 245, 186, 175, 248, 255, 31, 241, 63, 194, 186, 128, 102, 187, 250, 139, 188, 173, 145, 77, 229, 255, 95, 241, 63, 239, 231, 55, 23, 18, 127, 157, 188, 225, 54, 172, 17, 0, 160, 241, 63, 255, 245, 22, 5, 10, 0, 156, 60, 72, 66, 200, 25, 0, 224, 241, 63, 160, 93, 218, 228, 251, 130, 144, 188, 110, 94, 254, 15, 0, 32, 242, 63, 67, 251, 156, 76, 208, 253, 136, 188, 145, 216, 159, 38, 0, 96, 242, 63, 130, 209, 148, 121, 42, 254, 140, 60, 218, 230, 166, 41, 0, 160, 242, 63, 197, 139, 94, 113, 115, 2, 112, 188, 57, 62, 41, 224, 255, 223, 242, 63, 249, 166, 178, 218, 57, 124, 155, 60, 130, 240, 220, 247, 255, 31, 243, 63, 84, 82, 220, 110, 51, 241, 125, 60, 96, 139, 90, 240, 255, 95, 243, 63, 235, 49, 205, 76, 86, 3, 158, 188, 204, 174, 14, 46, 0, 160, 243, 63, 119, 164, 211, 75, 231, 240, 117, 60, 54, 178, 59, 4, 0, 224, 243, 63, 51, 136, 157, 20, 203, 125, 156, 60, 255, 135, 209, 2, 0, 32, 244, 63, 40, 61, 45, 207, 175, 8, 126, 60, 177, 124, 56, 13, 0, 96, 244, 63, 166, 153, 101, 133, 55, 8, 130, 60, 137, 159, 86, 4, 0, 160, 244, 63, 210, 188, 79, 144, 92, 250, 137, 188, 243, 67, 53, 4, 0, 224, 244, 63, 41, 83, 23, 237, 37, 17, 120, 188, 15, 127, 2, 204, 255, 31, 245, 63, 220, 84, 119, 132, 216, 131, 152, 60, 111, 179, 135, 253, 255, 95, 245, 63, 7, 40, 208, 49, 231, 9, 135, 188, 186, 247, 29, 242, 255, 159, 245, 63, 2, 123, 114, 104, 159, 247, 135, 60, 129, 52, 252, 235, 255, 223, 245, 63, 62, 233, 48, 46, 144, 128, 145, 188]);
                 m
             }
         }

impl WasmModule {
#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_0(&mut self, arg_0: i32) -> Option<()> {
std::process::exit(arg_0)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_1(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_2(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;self.func_1()?;
v0 = TaggedVal::from(self.func_4()?);
local_0 = v0.try_as_i32()?;
self.func_6()?;
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
fn func_3(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : i32 = 0i32;
let mut local_6 : f64 = 0f64;
let mut local_7 : f64 = 0f64;
let mut local_8 : i32 = 0i32;
let mut local_9 : f64 = 0f64;
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
let mut local_29 : i32 = 0i32;let mut v0: TaggedVal;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? as f64));
local_6 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from(self.func_7(v0.try_as_f64()?)?);
local_7 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? as f64));
local_9 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_13);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(63i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_17);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_19);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_20 = v0.try_as_i32()?;
'label_2: loop {
v0 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_21);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_24);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_25);
v0 = TaggedVal::from(v0.try_as_i32()?.checked_rem(v1.try_as_i32()?)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_26);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_27);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(29i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(unsafe {
	PARAM1 = kani::any();
	kani::assume((0..2).contains(&PARAM1));
	PARAM1
});
v0 = TaggedVal::from(self.func_3(v0.try_as_i32()?, v1.try_as_i32()?)?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}

v0 = TaggedVal::from(local_0);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_5(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_6(&mut self, ) -> Option<()> {
self.func_5()?;
self.func_5()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_7(&mut self, arg_0: f64) -> Option<f64> {
let mut local_0 : f64 = arg_0;let mut local_1 : i64 = 0i64;
let mut local_2 : f64 = 0f64;
let mut local_3 : f64 = 0f64;
let mut local_4 : f64 = 0f64;
let mut local_5 : f64 = 0f64;
let mut local_6 : f64 = 0f64;
let mut local_7 : f64 = 0f64;
let mut local_8 : i32 = 0i32;
let mut local_9 : i64 = 0i64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;
let mut v7: TaggedVal;
let mut v8: TaggedVal;
let mut v9: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
local_1 = v0.try_as_i64()?;
v1 = TaggedVal::from(-4606800540372828160i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
v1 = TaggedVal::from(581272283906047i64);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) > (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_f64(&self.memory, (v0.try_as_i32()? + 1024) as usize)?);
local_2 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(-1f64);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
local_0 = v1.try_as_f64()?;
v1 = TaggedVal::from((v1.try_as_f64()?.to_bits()));
v2 = TaggedVal::from(-4294967296i64);
v1 = TaggedVal::from(v1.try_as_i64()? & v2.try_as_i64()?);
v1 = TaggedVal::from(f64::from_bits(v1.try_as_i64()? as u64));
local_3 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_4 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_5 = v1.try_as_f64()?;
v2 = TaggedVal::from(0i32);
v2 = TaggedVal::from(read_mem_f64(&self.memory, (v2.try_as_i32()? + 1088) as usize)?);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(0i32);
v4 = TaggedVal::from(read_mem_f64(&self.memory, (v4.try_as_i32()? + 1096) as usize)?);
v3 = TaggedVal::from(v3.try_as_f64()? * v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_6 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_7 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_f64()? - v2.try_as_f64()?);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(0i32);
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 1032) as usize)?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_6);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(local_7);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(local_5);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_2 = v2.try_as_f64()?;
v3 = TaggedVal::from(0i32);
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 1104) as usize)?);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1112) as usize)?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_5);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1120) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1128) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_2);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1136) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1144) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v6 = TaggedVal::from(local_5);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1152) as usize)?);
v8 = TaggedVal::from(local_0);
v9 = TaggedVal::from(0i32);
v9 = TaggedVal::from(read_mem_f64(&self.memory, (v9.try_as_i32()? + 1160) as usize)?);
v8 = TaggedVal::from(v8.try_as_f64()? * v9.try_as_f64()?);
v7 = TaggedVal::from(v7.try_as_f64()? + v8.try_as_f64()?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
return Some(v0.try_as_f64()?);
break;
}
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(48i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(-16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(32736i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
'label_3: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(9223372036854775807i64);
v0 = TaggedVal::from(v0.try_as_i64()? & v1.try_as_i64()?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(1i32);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(9218868437227405312i64);
v0 = TaggedVal::from((v0.try_as_i64()? == v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(32768i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(32752i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(32752i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_9(v0.try_as_f64()?)?);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(4503599627370496f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
v1 = TaggedVal::from(-234187180623265792i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_1 = v0.try_as_i64()?;
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-4604367669032910848i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
local_9 = v0.try_as_i64()?;
v1 = TaggedVal::from(46i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
v1 = TaggedVal::from(63i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_8 = v0.try_as_i32()?;
v1 = TaggedVal::from(1176i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_f64(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(52i64);
v1 = TaggedVal::from(v1.try_as_i64()? >> (v2.try_as_i64()? % 64));
v1 = TaggedVal::from(v1.try_as_i64()? as i32);
v1 = TaggedVal::from((v1.try_as_i32()? as f64));
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_5 = v0.try_as_f64()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_f64(&self.memory, (v1.try_as_i32()? + 1024) as usize)?);
local_2 = v1.try_as_f64()?;
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(1168i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v2 = TaggedVal::from(read_mem_f64(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
v3 = TaggedVal::from(local_1);
v4 = TaggedVal::from(local_9);
v5 = TaggedVal::from(-4503599627370496i64);
v4 = TaggedVal::from(v4.try_as_i64()? & v5.try_as_i64()?);
v3 = TaggedVal::from(v3.try_as_i64()?.wrapping_sub(v4.try_as_i64()?));
v3 = TaggedVal::from(f64::from_bits(v3.try_as_i64()? as u64));
v4 = TaggedVal::from(local_8);
v5 = TaggedVal::from(2192i32);
v4 = TaggedVal::from(v4.try_as_i32()?.wrapping_add(v5.try_as_i32()?));
v4 = TaggedVal::from(read_mem_f64(&self.memory, (v4.try_as_i32()? + 0) as usize)?);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v4 = TaggedVal::from(local_8);
v5 = TaggedVal::from(2200i32);
v4 = TaggedVal::from(v4.try_as_i32()?.wrapping_add(v5.try_as_i32()?));
v4 = TaggedVal::from(read_mem_f64(&self.memory, (v4.try_as_i32()? + 0) as usize)?);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_0 = v2.try_as_f64()?;
v2 = TaggedVal::from((v2.try_as_f64()?.to_bits()));
v3 = TaggedVal::from(-4294967296i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v2 = TaggedVal::from(f64::from_bits(v2.try_as_i64()? as u64));
local_3 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_4 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_6 = v0.try_as_f64()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_f64(&self.memory, (v1.try_as_i32()? + 1032) as usize)?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v2 = TaggedVal::from(local_2);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(local_3);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(local_5);
v4 = TaggedVal::from(local_6);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_0);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_5 = v2.try_as_f64()?;
v3 = TaggedVal::from(0i32);
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 1040) as usize)?);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1048) as usize)?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_5);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1056) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1064) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_5);
v5 = TaggedVal::from(local_5);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1072) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1080) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_0 = v0.try_as_f64()?;
break;
}
v0 = TaggedVal::from(local_0);Some(v0.try_as_f64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_8(&mut self, arg_0: i32) -> Option<f64> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(f64::NEG_INFINITY);
v1 = TaggedVal::from(f64::INFINITY);
v2 = TaggedVal::from(local_0);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }Some(v0.try_as_f64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_9(&mut self, arg_0: f64) -> Option<f64> {
let mut local_0 : f64 = arg_0;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
local_0 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_f64()? / v1.try_as_f64()?);Some(v0.try_as_f64()?)}

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
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_1()?;
                         Some(vec![])
                     }
2 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_2()?;
                         Some(vec![])
                     }
3 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_3(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
4 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_4()?;
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
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_f64()?;
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
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_f64()?;
                         let rets = self.func_9(a0)?;
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
                         self.func_2()
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

use std::f32::consts::LOG2_E;

fn f_gold(y: i32, x: i32) -> i32 {
    if (y as f32).log2() < x as f32 {
        return y;
    } 
    if x > 63 {
        return y;
    }
    y % (1 << x) 
}////// LLM Output //////

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
fn kani_wasm_eq(){ 
		let result = f_gold(unsafe{PARAM1}.into(),unsafe{PARAM2}.into());
		let result_prime = f_gold_wasm_thread_unsafe();
		assert_eq!(result, result_prime);
}