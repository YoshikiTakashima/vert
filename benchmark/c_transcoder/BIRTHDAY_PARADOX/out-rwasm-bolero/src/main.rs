static mut PARAM1: f32 = 12.0;
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
                 m.globals[0] = TaggedVal::from(70800i32);
                 
                 m.memory[1024..5264].copy_from_slice(&[0, 56, 250, 254, 66, 46, 230, 63, 48, 103, 199, 147, 87, 243, 46, 61, 1, 0, 0, 0, 0, 0, 224, 191, 91, 48, 81, 85, 85, 85, 213, 63, 144, 69, 235, 255, 255, 255, 207, 191, 17, 1, 241, 36, 179, 153, 201, 63, 159, 200, 6, 229, 117, 85, 197, 191, 0, 0, 0, 0, 0, 0, 224, 191, 119, 85, 85, 85, 85, 85, 213, 63, 203, 253, 255, 255, 255, 255, 207, 191, 12, 221, 149, 153, 153, 153, 201, 63, 167, 69, 103, 85, 85, 85, 197, 191, 48, 222, 68, 163, 36, 73, 194, 63, 101, 61, 66, 164, 255, 255, 191, 191, 202, 214, 42, 40, 132, 113, 188, 63, 255, 104, 176, 67, 235, 153, 185, 191, 133, 208, 175, 247, 130, 129, 183, 63, 205, 69, 209, 117, 19, 82, 181, 191, 159, 222, 224, 195, 240, 52, 247, 63, 0, 144, 230, 121, 127, 204, 215, 191, 31, 233, 44, 106, 120, 19, 247, 63, 0, 0, 13, 194, 238, 111, 215, 191, 160, 181, 250, 8, 96, 242, 246, 63, 0, 224, 81, 19, 227, 19, 215, 191, 125, 140, 19, 31, 166, 209, 246, 63, 0, 120, 40, 56, 91, 184, 214, 191, 209, 180, 197, 11, 73, 177, 246, 63, 0, 120, 128, 144, 85, 93, 214, 191, 186, 12, 47, 51, 71, 145, 246, 63, 0, 0, 24, 118, 208, 2, 214, 191, 35, 66, 34, 24, 159, 113, 246, 63, 0, 144, 144, 134, 202, 168, 213, 191, 217, 30, 165, 153, 79, 82, 246, 63, 0, 80, 3, 86, 67, 79, 213, 191, 196, 36, 143, 170, 86, 51, 246, 63, 0, 64, 107, 195, 55, 246, 212, 191, 20, 220, 157, 107, 179, 20, 246, 63, 0, 80, 168, 253, 167, 157, 212, 191, 76, 92, 198, 82, 100, 246, 245, 63, 0, 168, 137, 57, 146, 69, 212, 191, 79, 44, 145, 181, 103, 216, 245, 63, 0, 184, 176, 57, 244, 237, 211, 191, 222, 144, 91, 203, 188, 186, 245, 63, 0, 112, 143, 68, 206, 150, 211, 191, 120, 26, 217, 242, 97, 157, 245, 63, 0, 160, 189, 23, 30, 64, 211, 191, 135, 86, 70, 18, 86, 128, 245, 63, 0, 128, 70, 239, 226, 233, 210, 191, 211, 107, 231, 206, 151, 99, 245, 63, 0, 224, 48, 56, 27, 148, 210, 191, 147, 127, 167, 226, 37, 71, 245, 63, 0, 136, 218, 140, 197, 62, 210, 191, 131, 69, 6, 66, 255, 42, 245, 63, 0, 144, 39, 41, 225, 233, 209, 191, 223, 189, 178, 219, 34, 15, 245, 63, 0, 248, 72, 43, 109, 149, 209, 191, 215, 222, 52, 71, 143, 243, 244, 63, 0, 248, 185, 154, 103, 65, 209, 191, 64, 40, 222, 207, 67, 216, 244, 63, 0, 152, 239, 148, 208, 237, 208, 191, 200, 163, 120, 192, 62, 189, 244, 63, 0, 16, 219, 24, 165, 154, 208, 191, 138, 37, 224, 195, 127, 162, 244, 63, 0, 184, 99, 82, 230, 71, 208, 191, 52, 132, 212, 36, 5, 136, 244, 63, 0, 240, 134, 69, 34, 235, 207, 191, 11, 45, 25, 27, 206, 109, 244, 63, 0, 176, 23, 117, 74, 71, 207, 191, 84, 24, 57, 211, 217, 83, 244, 63, 0, 48, 16, 61, 68, 164, 206, 191, 90, 132, 180, 68, 39, 58, 244, 63, 0, 176, 233, 68, 13, 2, 206, 191, 251, 248, 21, 65, 181, 32, 244, 63, 0, 240, 119, 41, 162, 96, 205, 191, 177, 244, 62, 218, 130, 7, 244, 63, 0, 144, 149, 4, 1, 192, 204, 191, 143, 254, 87, 93, 143, 238, 243, 63, 0, 16, 137, 86, 41, 32, 204, 191, 233, 76, 11, 160, 217, 213, 243, 63, 0, 16, 129, 141, 23, 129, 203, 191, 43, 193, 16, 192, 96, 189, 243, 63, 0, 208, 211, 204, 201, 226, 202, 191, 184, 218, 117, 43, 36, 165, 243, 63, 0, 144, 18, 46, 64, 69, 202, 191, 2, 208, 159, 205, 34, 141, 243, 63, 0, 240, 29, 104, 119, 168, 201, 191, 28, 122, 132, 197, 91, 117, 243, 63, 0, 48, 72, 105, 109, 12, 201, 191, 226, 54, 173, 73, 206, 93, 243, 63, 0, 192, 69, 166, 32, 113, 200, 191, 64, 212, 77, 152, 121, 70, 243, 63, 0, 48, 20, 180, 143, 214, 199, 191, 36, 203, 255, 206, 92, 47, 243, 63, 0, 112, 98, 60, 184, 60, 199, 191, 73, 13, 161, 117, 119, 24, 243, 63, 0, 96, 55, 155, 154, 163, 198, 191, 144, 57, 62, 55, 200, 1, 243, 63, 0, 160, 183, 84, 49, 11, 198, 191, 65, 248, 149, 187, 78, 235, 242, 63, 0, 48, 36, 118, 125, 115, 197, 191, 209, 169, 25, 2, 10, 213, 242, 63, 0, 48, 194, 143, 123, 220, 196, 191, 42, 253, 183, 168, 249, 190, 242, 63, 0, 0, 210, 81, 44, 70, 196, 191, 171, 27, 12, 122, 28, 169, 242, 63, 0, 0, 131, 188, 138, 176, 195, 191, 48, 181, 20, 96, 114, 147, 242, 63, 0, 0, 73, 107, 153, 27, 195, 191, 245, 161, 87, 87, 250, 125, 242, 63, 0, 64, 164, 144, 84, 135, 194, 191, 191, 59, 29, 155, 179, 104, 242, 63, 0, 160, 121, 248, 185, 243, 193, 191, 189, 245, 143, 131, 157, 83, 242, 63, 0, 160, 44, 37, 200, 96, 193, 191, 59, 8, 201, 170, 183, 62, 242, 63, 0, 32, 247, 87, 127, 206, 192, 191, 182, 64, 169, 43, 1, 42, 242, 63, 0, 160, 254, 73, 220, 60, 192, 191, 50, 65, 204, 150, 121, 21, 242, 63, 0, 128, 75, 188, 189, 87, 191, 191, 155, 252, 210, 29, 32, 1, 242, 63, 0, 64, 64, 150, 8, 55, 190, 191, 11, 72, 77, 73, 244, 236, 241, 63, 0, 64, 249, 62, 152, 23, 189, 191, 105, 101, 143, 82, 245, 216, 241, 63, 0, 160, 216, 78, 103, 249, 187, 191, 124, 126, 87, 17, 35, 197, 241, 63, 0, 96, 47, 32, 121, 220, 186, 191, 233, 38, 203, 116, 124, 177, 241, 63, 0, 128, 40, 231, 195, 192, 185, 191, 182, 26, 44, 12, 1, 158, 241, 63, 0, 192, 114, 179, 70, 166, 184, 191, 189, 112, 182, 123, 176, 138, 241, 63, 0, 0, 172, 179, 1, 141, 183, 191, 182, 188, 239, 37, 138, 119, 241, 63, 0, 0, 56, 69, 241, 116, 182, 191, 218, 49, 76, 53, 141, 100, 241, 63, 0, 128, 135, 109, 14, 94, 181, 191, 221, 95, 39, 144, 185, 81, 241, 63, 0, 224, 161, 222, 92, 72, 180, 191, 76, 210, 50, 164, 14, 63, 241, 63, 0, 160, 106, 77, 217, 51, 179, 191, 218, 249, 16, 114, 139, 44, 241, 63, 0, 96, 197, 248, 121, 32, 178, 191, 49, 181, 236, 40, 48, 26, 241, 63, 0, 32, 98, 152, 70, 14, 177, 191, 175, 52, 132, 218, 251, 7, 241, 63, 0, 0, 210, 106, 108, 250, 175, 191, 179, 107, 78, 15, 238, 245, 240, 63, 0, 64, 119, 74, 141, 218, 173, 191, 206, 159, 42, 93, 6, 228, 240, 63, 0, 0, 133, 228, 236, 188, 171, 191, 33, 165, 44, 99, 68, 210, 240, 63, 0, 192, 18, 64, 137, 161, 169, 191, 26, 152, 226, 124, 167, 192, 240, 63, 0, 192, 2, 51, 88, 136, 167, 191, 209, 54, 198, 131, 47, 175, 240, 63, 0, 128, 214, 103, 94, 113, 165, 191, 57, 19, 160, 152, 219, 157, 240, 63, 0, 128, 101, 73, 138, 92, 163, 191, 223, 231, 82, 175, 171, 140, 240, 63, 0, 64, 21, 100, 227, 73, 161, 191, 251, 40, 78, 47, 159, 123, 240, 63, 0, 128, 235, 130, 192, 114, 158, 191, 25, 143, 53, 140, 181, 106, 240, 63, 0, 128, 82, 82, 241, 85, 154, 191, 44, 249, 236, 165, 238, 89, 240, 63, 0, 128, 129, 207, 98, 61, 150, 191, 144, 44, 209, 205, 73, 73, 240, 63, 0, 0, 170, 140, 251, 40, 146, 191, 169, 173, 240, 198, 198, 56, 240, 63, 0, 0, 249, 32, 123, 49, 140, 191, 169, 50, 121, 19, 101, 40, 240, 63, 0, 0, 170, 93, 53, 25, 132, 191, 72, 115, 234, 39, 36, 24, 240, 63, 0, 0, 236, 194, 3, 18, 120, 191, 149, 177, 20, 6, 4, 8, 240, 63, 0, 0, 36, 121, 9, 4, 96, 191, 26, 250, 38, 247, 31, 224, 239, 63, 0, 0, 144, 132, 243, 239, 111, 63, 116, 234, 97, 194, 28, 161, 239, 63, 0, 0, 61, 53, 65, 220, 135, 63, 46, 153, 129, 176, 16, 99, 239, 63, 0, 128, 194, 196, 163, 206, 147, 63, 205, 173, 238, 60, 246, 37, 239, 63, 0, 0, 137, 20, 193, 159, 155, 63, 231, 19, 145, 3, 200, 233, 238, 63, 0, 0, 17, 206, 216, 176, 161, 63, 171, 177, 203, 120, 128, 174, 238, 63, 0, 192, 1, 208, 91, 138, 165, 63, 155, 12, 157, 162, 26, 116, 238, 63, 0, 128, 216, 64, 131, 92, 169, 63, 181, 153, 10, 131, 145, 58, 238, 63, 0, 128, 87, 239, 106, 39, 173, 63, 86, 154, 96, 9, 224, 1, 238, 63, 0, 192, 152, 229, 152, 117, 176, 63, 152, 187, 119, 229, 1, 202, 237, 63, 0, 32, 13, 227, 245, 83, 178, 63, 3, 145, 124, 11, 242, 146, 237, 63, 0, 0, 56, 139, 221, 46, 180, 63, 206, 92, 251, 102, 172, 92, 237, 63, 0, 192, 87, 135, 89, 6, 182, 63, 157, 222, 94, 170, 44, 39, 237, 63, 0, 0, 106, 53, 118, 218, 183, 63, 205, 44, 107, 62, 110, 242, 236, 63, 0, 96, 28, 78, 67, 171, 185, 63, 2, 121, 167, 162, 109, 190, 236, 63, 0, 96, 13, 187, 199, 120, 187, 63, 109, 8, 55, 109, 38, 139, 236, 63, 0, 32, 231, 50, 19, 67, 189, 63, 4, 88, 93, 189, 148, 88, 236, 63, 0, 96, 222, 113, 49, 10, 191, 63, 140, 159, 187, 51, 181, 38, 236, 63, 0, 64, 145, 43, 21, 103, 192, 63, 63, 231, 236, 238, 131, 245, 235, 63, 0, 176, 146, 130, 133, 71, 193, 63, 193, 150, 219, 117, 253, 196, 235, 63, 0, 48, 202, 205, 110, 38, 194, 63, 40, 74, 134, 12, 30, 149, 235, 63, 0, 80, 197, 166, 215, 3, 195, 63, 44, 62, 239, 197, 226, 101, 235, 63, 0, 16, 51, 60, 195, 223, 195, 63, 139, 136, 201, 103, 72, 55, 235, 63, 0, 128, 122, 107, 54, 186, 196, 63, 74, 48, 29, 33, 75, 9, 235, 63, 0, 240, 209, 40, 57, 147, 197, 63, 126, 239, 242, 133, 232, 219, 234, 63, 0, 240, 24, 36, 205, 106, 198, 63, 162, 61, 96, 49, 29, 175, 234, 63, 0, 144, 102, 236, 248, 64, 199, 63, 167, 88, 211, 63, 230, 130, 234, 63, 0, 240, 26, 245, 192, 21, 200, 63, 139, 115, 9, 239, 64, 87, 234, 63, 0, 128, 246, 84, 41, 233, 200, 63, 39, 75, 171, 144, 42, 44, 234, 63, 0, 64, 248, 2, 54, 187, 201, 63, 209, 242, 147, 19, 160, 1, 234, 63, 0, 0, 44, 28, 237, 139, 202, 63, 27, 60, 219, 36, 159, 215, 233, 63, 0, 208, 1, 92, 81, 91, 203, 63, 144, 177, 199, 5, 37, 174, 233, 63, 0, 192, 188, 204, 103, 41, 204, 63, 47, 206, 151, 242, 46, 133, 233, 63, 0, 96, 72, 213, 53, 246, 204, 63, 117, 75, 164, 238, 186, 92, 233, 63, 0, 192, 70, 52, 189, 193, 205, 63, 56, 72, 231, 157, 198, 52, 233, 63, 0, 224, 207, 184, 1, 140, 206, 63, 230, 82, 103, 47, 79, 13, 233, 63, 0, 144, 23, 192, 9, 85, 207, 63, 157, 215, 255, 142, 82, 230, 232, 63, 0, 184, 31, 18, 108, 14, 208, 63, 124, 0, 204, 159, 206, 191, 232, 63, 0, 208, 147, 14, 184, 113, 208, 63, 14, 195, 190, 218, 192, 153, 232, 63, 0, 112, 134, 158, 107, 212, 208, 63, 251, 23, 35, 170, 39, 116, 232, 63, 0, 208, 75, 51, 135, 54, 209, 63, 8, 154, 179, 172, 0, 79, 232, 63, 0, 72, 35, 103, 13, 152, 209, 63, 85, 62, 101, 232, 73, 42, 232, 63, 0, 128, 204, 224, 255, 248, 209, 63, 96, 2, 244, 149, 1, 6, 232, 63, 0, 104, 99, 215, 95, 89, 210, 63, 41, 163, 224, 99, 37, 226, 231, 63, 0, 168, 20, 9, 48, 185, 210, 63, 173, 181, 220, 119, 179, 190, 231, 63, 0, 96, 67, 16, 114, 24, 211, 63, 194, 37, 151, 103, 170, 155, 231, 63, 0, 24, 236, 109, 38, 119, 211, 63, 87, 6, 23, 242, 7, 121, 231, 63, 0, 48, 175, 251, 79, 213, 211, 63, 12, 19, 214, 219, 202, 86, 231, 63, 0, 224, 47, 227, 238, 50, 212, 63, 107, 182, 79, 1, 0, 16, 230, 63, 60, 91, 66, 145, 108, 2, 126, 60, 149, 180, 77, 3, 0, 48, 230, 63, 65, 93, 0, 72, 234, 191, 141, 60, 120, 212, 148, 13, 0, 80, 230, 63, 183, 165, 214, 134, 167, 127, 142, 60, 173, 111, 78, 7, 0, 112, 230, 63, 76, 37, 84, 107, 234, 252, 97, 60, 174, 15, 223, 254, 255, 143, 230, 63, 253, 14, 89, 76, 39, 126, 124, 188, 188, 197, 99, 7, 0, 176, 230, 63, 1, 218, 220, 72, 104, 193, 138, 188, 246, 193, 92, 30, 0, 208, 230, 63, 17, 147, 73, 157, 28, 63, 131, 60, 62, 246, 5, 235, 255, 239, 230, 63, 83, 45, 226, 26, 4, 128, 126, 188, 128, 151, 134, 14, 0, 16, 231, 63, 82, 121, 9, 113, 102, 255, 123, 60, 18, 233, 103, 252, 255, 47, 231, 63, 36, 135, 189, 38, 226, 0, 140, 60, 106, 17, 129, 223, 255, 79, 231, 63, 210, 1, 241, 110, 145, 2, 110, 188, 144, 156, 103, 15, 0, 112, 231, 63, 116, 156, 84, 205, 113, 252, 103, 188, 53, 200, 126, 250, 255, 143, 231, 63, 131, 4, 245, 158, 193, 190, 129, 60, 230, 194, 32, 254, 255, 175, 231, 63, 101, 100, 204, 41, 23, 126, 112, 188, 0, 201, 63, 237, 255, 207, 231, 63, 28, 139, 123, 8, 114, 128, 128, 188, 118, 26, 38, 233, 255, 239, 231, 63, 174, 249, 157, 109, 40, 192, 141, 60, 232, 163, 156, 4, 0, 16, 232, 63, 51, 76, 229, 81, 210, 127, 137, 60, 143, 44, 147, 23, 0, 48, 232, 63, 129, 243, 48, 182, 233, 254, 138, 188, 156, 115, 51, 6, 0, 80, 232, 63, 188, 53, 101, 107, 191, 191, 137, 60, 198, 137, 66, 32, 0, 112, 232, 63, 117, 123, 17, 243, 101, 191, 139, 188, 4, 121, 245, 235, 255, 143, 232, 63, 87, 203, 61, 162, 110, 0, 137, 188, 223, 4, 188, 34, 0, 176, 232, 63, 10, 75, 224, 56, 223, 0, 125, 188, 138, 27, 12, 229, 255, 207, 232, 63, 5, 159, 255, 70, 113, 0, 136, 188, 67, 142, 145, 252, 255, 239, 232, 63, 56, 112, 122, 208, 123, 129, 131, 60, 199, 95, 250, 30, 0, 16, 233, 63, 3, 180, 223, 118, 145, 62, 137, 60, 185, 123, 70, 19, 0, 48, 233, 63, 118, 2, 152, 75, 78, 128, 127, 60, 111, 7, 238, 230, 255, 79, 233, 63, 46, 98, 255, 217, 240, 126, 143, 188, 209, 18, 60, 222, 255, 111, 233, 63, 186, 56, 38, 150, 170, 130, 112, 188, 13, 138, 69, 244, 255, 143, 233, 63, 239, 168, 100, 145, 27, 128, 135, 188, 62, 46, 152, 221, 255, 175, 233, 63, 55, 147, 90, 138, 224, 64, 135, 188, 102, 251, 73, 237, 255, 207, 233, 63, 0, 224, 155, 193, 8, 206, 63, 60, 81, 156, 241, 32, 0, 240, 233, 63, 10, 91, 136, 39, 170, 63, 138, 188, 6, 176, 69, 17, 0, 16, 234, 63, 86, 218, 88, 153, 72, 255, 116, 60, 250, 246, 187, 7, 0, 48, 234, 63, 24, 109, 43, 138, 171, 190, 140, 60, 121, 29, 151, 16, 0, 80, 234, 63, 48, 121, 120, 221, 202, 254, 136, 60, 72, 46, 245, 29, 0, 112, 234, 63, 219, 171, 216, 61, 118, 65, 143, 188, 82, 51, 89, 28, 0, 144, 234, 63, 18, 118, 194, 132, 2, 191, 142, 188, 75, 62, 79, 42, 0, 176, 234, 63, 95, 63, 255, 60, 4, 253, 105, 188, 209, 30, 174, 215, 255, 207, 234, 63, 180, 112, 144, 18, 231, 62, 130, 188, 120, 4, 81, 238, 255, 239, 234, 63, 163, 222, 14, 224, 62, 6, 106, 60, 91, 13, 101, 219, 255, 15, 235, 63, 185, 10, 31, 56, 200, 6, 90, 60, 87, 202, 170, 254, 255, 47, 235, 63, 29, 60, 35, 116, 30, 1, 121, 188, 220, 186, 149, 217, 255, 79, 235, 63, 159, 42, 134, 104, 16, 255, 121, 188, 156, 101, 158, 36, 0, 112, 235, 63, 62, 79, 134, 208, 69, 255, 138, 60, 64, 22, 135, 249, 255, 143, 235, 63, 249, 195, 194, 150, 119, 254, 124, 60, 79, 203, 4, 210, 255, 175, 235, 63, 196, 43, 242, 238, 39, 255, 99, 188, 69, 92, 65, 210, 255, 207, 235, 63, 33, 234, 59, 238, 183, 255, 108, 188, 223, 9, 99, 248, 255, 239, 235, 63, 92, 11, 46, 151, 3, 65, 129, 188, 83, 118, 181, 225, 255, 15, 236, 63, 25, 106, 183, 148, 100, 193, 139, 60, 227, 87, 250, 241, 255, 47, 236, 63, 237, 198, 48, 141, 239, 254, 100, 188, 36, 228, 191, 220, 255, 79, 236, 63, 117, 71, 236, 188, 104, 63, 132, 188, 247, 185, 84, 237, 255, 111, 236, 63, 236, 224, 83, 240, 163, 126, 132, 60, 213, 143, 153, 235, 255, 143, 236, 63, 241, 146, 249, 141, 6, 131, 115, 60, 154, 33, 37, 33, 0, 176, 236, 63, 4, 14, 24, 100, 142, 253, 104, 188, 156, 70, 148, 221, 255, 207, 236, 63, 114, 234, 199, 28, 190, 126, 142, 60, 118, 196, 253, 234, 255, 239, 236, 63, 254, 136, 159, 173, 57, 190, 142, 60, 43, 248, 154, 22, 0, 16, 237, 63, 113, 90, 185, 168, 145, 125, 117, 60, 29, 247, 15, 13, 0, 48, 237, 63, 218, 199, 112, 105, 144, 193, 137, 60, 196, 15, 121, 234, 255, 79, 237, 63, 12, 254, 88, 197, 55, 14, 88, 188, 229, 135, 220, 46, 0, 112, 237, 63, 68, 15, 193, 77, 214, 128, 127, 188, 170, 130, 220, 33, 0, 144, 237, 63, 92, 92, 253, 148, 143, 124, 116, 188, 131, 2, 107, 216, 255, 175, 237, 63, 126, 97, 33, 197, 29, 127, 140, 60, 57, 71, 108, 41, 0, 208, 237, 63, 83, 177, 255, 178, 158, 1, 136, 60, 245, 144, 68, 229, 255, 239, 237, 63, 137, 204, 82, 198, 210, 0, 110, 60, 148, 246, 171, 205, 255, 15, 238, 63, 210, 105, 45, 32, 64, 131, 127, 188, 221, 200, 82, 219, 255, 47, 238, 63, 100, 8, 27, 202, 193, 0, 123, 60, 239, 22, 66, 242, 255, 79, 238, 63, 81, 171, 148, 176, 168, 255, 114, 60, 17, 94, 138, 232, 255, 111, 238, 63, 89, 190, 239, 177, 115, 246, 87, 188, 13, 255, 158, 17, 0, 144, 238, 63, 1, 200, 11, 94, 141, 128, 132, 188, 68, 23, 165, 223, 255, 175, 238, 63, 181, 32, 67, 213, 6, 0, 120, 60, 161, 127, 18, 26, 0, 208, 238, 63, 146, 92, 86, 96, 248, 2, 80, 188, 196, 188, 186, 7, 0, 240, 238, 63, 17, 230, 53, 93, 68, 64, 133, 188, 2, 141, 122, 245, 255, 15, 239, 63, 5, 145, 239, 57, 49, 251, 79, 188, 199, 138, 229, 30, 0, 48, 239, 63, 85, 17, 115, 242, 172, 129, 138, 60, 148, 52, 130, 245, 255, 79, 239, 63, 67, 199, 215, 212, 65, 63, 138, 60, 107, 76, 169, 252, 255, 111, 239, 63, 117, 120, 152, 28, 244, 2, 98, 188, 65, 196, 249, 225, 255, 143, 239, 63, 75, 231, 119, 244, 209, 125, 119, 60, 126, 227, 224, 210, 255, 175, 239, 63, 49, 163, 124, 154, 25, 1, 111, 188, 158, 228, 119, 28, 0, 208, 239, 63, 177, 172, 206, 75, 238, 129, 113, 60, 49, 195, 224, 247, 255, 239, 239, 63, 90, 135, 112, 1, 55, 5, 110, 188, 110, 96, 101, 244, 255, 15, 240, 63, 218, 10, 28, 73, 173, 126, 138, 188, 88, 122, 134, 243, 255, 47, 240, 63, 224, 178, 252, 195, 105, 127, 151, 188, 23, 13, 252, 253, 255, 79, 240, 63, 91, 148, 203, 52, 254, 191, 151, 60, 130, 77, 205, 3, 0, 112, 240, 63, 203, 86, 228, 192, 131, 0, 130, 60, 232, 203, 242, 249, 255, 143, 240, 63, 26, 117, 55, 190, 223, 255, 109, 188, 101, 218, 12, 1, 0, 176, 240, 63, 235, 38, 230, 174, 127, 63, 145, 188, 56, 211, 164, 1, 0, 208, 240, 63, 247, 159, 72, 121, 250, 125, 128, 60, 253, 253, 218, 250, 255, 239, 240, 63, 192, 107, 214, 112, 5, 4, 119, 188, 150, 253, 186, 11, 0, 16, 241, 63, 98, 11, 109, 132, 212, 128, 142, 60, 93, 244, 229, 250, 255, 47, 241, 63, 239, 54, 253, 100, 250, 191, 157, 60, 217, 154, 213, 13, 0, 80, 241, 63, 174, 80, 18, 112, 119, 0, 154, 60, 154, 85, 33, 15, 0, 112, 241, 63, 238, 222, 227, 226, 249, 253, 141, 60, 38, 84, 39, 252, 255, 143, 241, 63, 115, 114, 59, 220, 48, 0, 145, 60, 89, 60, 61, 18, 0, 176, 241, 63, 136, 1, 3, 128, 121, 127, 153, 60, 183, 158, 41, 248, 255, 207, 241, 63, 103, 140, 159, 171, 50, 249, 101, 188, 0, 212, 138, 244, 255, 239, 241, 63, 235, 91, 167, 157, 191, 127, 147, 60, 164, 134, 139, 12, 0, 16, 242, 63, 34, 91, 253, 145, 107, 128, 159, 60, 3, 67, 133, 3, 0, 48, 242, 63, 51, 191, 159, 235, 194, 255, 147, 60, 132, 246, 188, 255, 255, 79, 242, 63, 114, 46, 46, 126, 231, 1, 118, 60, 217, 33, 41, 245, 255, 111, 242, 63, 97, 12, 127, 118, 187, 252, 127, 60, 60, 58, 147, 20, 0, 144, 242, 63, 43, 65, 2, 60, 202, 2, 114, 188, 19, 99, 85, 20, 0, 176, 242, 63, 2, 31, 242, 51, 130, 128, 146, 188, 59, 82, 254, 235, 255, 207, 242, 63, 242, 220, 79, 56, 126, 255, 136, 188, 150, 173, 184, 11, 0, 240, 242, 63, 197, 65, 48, 80, 81, 255, 133, 188, 175, 226, 122, 251, 255, 15, 243, 63, 157, 40, 94, 136, 113, 0, 129, 188, 127, 95, 172, 254, 255, 47, 243, 63, 21, 183, 183, 63, 93, 255, 145, 188, 86, 103, 166, 12, 0, 80, 243, 63, 189, 130, 139, 34, 130, 127, 149, 60, 33, 247, 251, 17, 0, 112, 243, 63, 204, 213, 13, 196, 186, 0, 128, 60, 185, 47, 89, 249, 255, 143, 243, 63, 81, 167, 178, 45, 157, 63, 148, 188, 66, 210, 221, 4, 0, 176, 243, 63, 225, 56, 118, 112, 107, 127, 133, 60, 87, 201, 178, 245, 255, 207, 243, 63, 49, 18, 191, 16, 58, 2, 122, 60, 24, 180, 176, 234, 255, 239, 243, 63, 176, 82, 177, 102, 109, 127, 152, 60, 244, 175, 50, 21, 0, 16, 244, 63, 36, 133, 25, 95, 55, 248, 103, 60, 41, 139, 71, 23, 0, 48, 244, 63, 67, 81, 220, 114, 230, 1, 131, 60, 99, 180, 149, 231, 255, 79, 244, 63, 90, 137, 178, 184, 105, 255, 137, 60, 224, 117, 4, 232, 255, 111, 244, 63, 84, 242, 194, 155, 177, 192, 149, 188, 231, 193, 111, 239, 255, 143, 244, 63, 114, 42, 58, 242, 9, 64, 155, 60, 4, 167, 190, 229, 255, 175, 244, 63, 69, 125, 13, 191, 183, 255, 148, 188, 222, 39, 16, 23, 0, 208, 244, 63, 61, 106, 220, 113, 100, 192, 153, 188, 226, 62, 240, 15, 0, 240, 244, 63, 28, 83, 133, 11, 137, 127, 151, 60, 209, 75, 220, 18, 0, 16, 245, 63, 54, 164, 102, 113, 101, 4, 96, 60, 122, 39, 5, 22, 0, 48, 245, 63, 9, 50, 35, 206, 206, 191, 150, 188, 76, 112, 219, 236, 255, 79, 245, 63, 215, 161, 5, 5, 114, 2, 137, 188, 169, 84, 95, 239, 255, 111, 245, 63, 18, 100, 201, 14, 230, 191, 155, 60, 18, 16, 230, 23, 0, 144, 245, 63, 144, 239, 175, 129, 197, 126, 136, 60, 146, 62, 201, 3, 0, 176, 245, 63, 192, 12, 191, 10, 8, 65, 159, 188, 188, 25, 73, 29, 0, 208, 245, 63, 41, 71, 37, 251, 42, 129, 152, 188, 137, 122, 184, 231, 255, 239, 245, 63, 4, 105, 237, 128, 183, 126, 148, 188]);
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
fn func_3(&mut self, arg_0: f64) -> Option<i32> {
let mut local_0 : f64 = arg_0;let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : f64 = 0f64;
let mut local_5 : f64 = 0f64;
let mut local_6 : f64 = 0f64;
let mut local_7 : f64 = 0f64;
let mut local_8 : f64 = 0f64;
let mut local_9 : f64 = 0f64;
let mut local_10 : f64 = 0f64;
let mut local_11 : f64 = 0f64;
let mut local_12 : f64 = 0f64;
let mut local_13 : f64 = 0f64;
let mut local_14 : f64 = 0f64;
let mut local_15 : i32 = 0i32;
let mut local_16 : i32 = 0i32;
let mut local_17 : i32 = 0i32;
let mut local_18 : i32 = 0i32;
let mut local_19 : i32 = 0i32;
let mut local_20 : i32 = 0i32;
let mut local_21 : i32 = 0i32;
let mut local_22 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(730f64);
local_4 = v0.try_as_f64()?;
v0 = TaggedVal::from(1f64);
local_5 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_f64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_f64()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_f64(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_6 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
local_7 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_f64()? / v1.try_as_f64()?);
local_8 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(self.func_7(v0.try_as_f64()?)?);
local_9 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_10 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_f64()?.sqrt());
local_11 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(v0.try_as_f64()?.ceil());
local_12 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_f64()?.abs());
local_13 = v0.try_as_f64()?;
v0 = TaggedVal::from(2147483648f64);
local_14 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_16 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_16);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from(<_ as SafeFloatConv<i32>>::try_to_int(v0.try_as_f64()?.trunc())?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
local_18 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(-2147483648i32);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
local_18 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_18);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_20);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : f64 = 0f64;let mut v0: TaggedVal;v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(29f64);
local_1 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(unsafe {PARAM1});
v0 = TaggedVal::from(self.func_3(v0.try_as_f64()?)?);

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
let mut local_6 : i32 = 0i32;
let mut local_7 : i64 = 0i64;let mut v0: TaggedVal;
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
v1 = TaggedVal::from(-4606619468846596096i64);
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_add(v1.try_as_i64()?));
v1 = TaggedVal::from(854320534781951i64);
v0 = TaggedVal::from(((v0.try_as_i64()? as u64) > (v1.try_as_i64()? as u64)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-1f64);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_0 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(134217728f64);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_2 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_f64()? - v2.try_as_f64()?);
local_2 = v1.try_as_f64()?;
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v2 = TaggedVal::from(0i32);
v2 = TaggedVal::from(read_mem_f64(&self.memory, (v2.try_as_i32()? + 1080) as usize)?);
local_3 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_4 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_5 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(local_5);
v3 = TaggedVal::from(v3.try_as_f64()? - v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(local_0);
v3 = TaggedVal::from(v3.try_as_f64()? * v4.try_as_f64()?);
local_2 = v3.try_as_f64()?;
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_3 = v2.try_as_f64()?;
v3 = TaggedVal::from(0i32);
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 1088) as usize)?);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1096) as usize)?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_2);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1104) as usize)?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_3);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1112) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1120) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v6 = TaggedVal::from(local_2);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1128) as usize)?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v6 = TaggedVal::from(local_3);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1136) as usize)?);
v8 = TaggedVal::from(local_0);
v9 = TaggedVal::from(0i32);
v9 = TaggedVal::from(read_mem_f64(&self.memory, (v9.try_as_i32()? + 1144) as usize)?);
v8 = TaggedVal::from(v8.try_as_f64()? * v9.try_as_f64()?);
v7 = TaggedVal::from(v7.try_as_f64()? + v8.try_as_f64()?);
v8 = TaggedVal::from(local_2);
v9 = TaggedVal::from(0i32);
v9 = TaggedVal::from(read_mem_f64(&self.memory, (v9.try_as_i32()? + 1152) as usize)?);
v8 = TaggedVal::from(v8.try_as_f64()? * v9.try_as_f64()?);
v7 = TaggedVal::from(v7.try_as_f64()? + v8.try_as_f64()?);
v8 = TaggedVal::from(local_3);
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
local_6 = v0.try_as_i32()?;
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
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(32768i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_6);
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
local_7 = v0.try_as_i64()?;
v1 = TaggedVal::from(45i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
v1 = TaggedVal::from(127i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(1176i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_f64(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_f64(&self.memory, (v1.try_as_i32()? + 1024) as usize)?);
v2 = TaggedVal::from(local_7);
v3 = TaggedVal::from(52i64);
v2 = TaggedVal::from(v2.try_as_i64()? >> (v3.try_as_i64()? % 64));
v2 = TaggedVal::from(v2.try_as_i64()? as i32);
v2 = TaggedVal::from((v2.try_as_i32()? as f64));
local_3 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_4 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(1168i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_f64(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(local_7);
v4 = TaggedVal::from(-4503599627370496i64);
v3 = TaggedVal::from(v3.try_as_i64()? & v4.try_as_i64()?);
v2 = TaggedVal::from(v2.try_as_i64()?.wrapping_sub(v3.try_as_i64()?));
v2 = TaggedVal::from(f64::from_bits(v2.try_as_i64()? as u64));
v3 = TaggedVal::from(local_6);
v4 = TaggedVal::from(3216i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v3 = TaggedVal::from(local_6);
v4 = TaggedVal::from(3224i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_0 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_5 = v0.try_as_f64()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_f64(&self.memory, (v1.try_as_i32()? + 1040) as usize)?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_0);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_2 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v2 = TaggedVal::from(0i32);
v2 = TaggedVal::from(read_mem_f64(&self.memory, (v2.try_as_i32()? + 1032) as usize)?);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(local_4);
v5 = TaggedVal::from(local_5);
v4 = TaggedVal::from(v4.try_as_f64()? - v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v3 = TaggedVal::from(0i32);
v3 = TaggedVal::from(read_mem_f64(&self.memory, (v3.try_as_i32()? + 1048) as usize)?);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1056) as usize)?);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v4 = TaggedVal::from(local_2);
v5 = TaggedVal::from(0i32);
v5 = TaggedVal::from(read_mem_f64(&self.memory, (v5.try_as_i32()? + 1064) as usize)?);
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(0i32);
v7 = TaggedVal::from(read_mem_f64(&self.memory, (v7.try_as_i32()? + 1072) as usize)?);
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
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_f64()?;
                         let rets = self.func_3(a0)?;
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


fn f_gold(p: f32 ) -> i32 {
    (std::f32::consts::SQRT_2 * 365. * (std::f32::consts::E.ln() / (1. - p))).ceil() as i32
}////// LLM Output //////


use proptest::prelude::*;
proptest!{
  #[test]
  fn check_eq(
    PARAM_1: f32
  ) {
     
		unsafe {
		PARAM1 = PARAM_1;

		}
		let result = f_gold(unsafe{PARAM1}.into());
		let result_prime = f_gold_wasm_thread_unsafe();
		assert_eq!(result, result_prime);
	
  }
}
