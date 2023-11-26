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
                 m.globals[0] = TaggedVal::from(66560i32);
                 
                 
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
fn func_3(&mut self, arg_0: i32) -> Option<i32> {
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
let mut local_12 : f64 = 0f64;
let mut local_13 : f64 = 0f64;
let mut local_14 : f64 = 0f64;
let mut local_15 : f64 = 0f64;
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
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(9i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(19i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_10);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from((v0.try_as_i32()? as f64));
local_12 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from(self.func_7(v0.try_as_f64()?)?);
local_13 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_f64()?.abs());
local_14 = v0.try_as_f64()?;
v0 = TaggedVal::from(2147483648f64);
local_15 = v0.try_as_f64()?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from((v0.try_as_f64()? < v1.try_as_f64()?) as i32);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_17 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_17);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_13);
v0 = TaggedVal::from(<_ as SafeFloatConv<i32>>::try_to_int(v0.try_as_f64()?.trunc())?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_19 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(-2147483648i32);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_19 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_19);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(9i32);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_28);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
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
fn func_4(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;let mut v0: TaggedVal;v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_3(v0.try_as_i32()?)?);

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
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;
let mut local_4 : i32 = 0i32;
let mut local_5 : f64 = 0f64;
let mut local_6 : f64 = 0f64;
let mut local_7 : f64 = 0f64;
let mut local_8 : f64 = 0f64;
let mut local_9 : f64 = 0f64;
let mut local_10 : f64 = 0f64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;
let mut v7: TaggedVal;
let mut v8: TaggedVal;
let mut v9: TaggedVal;
let mut v10: TaggedVal;'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
local_1 = v0.try_as_i64()?;
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? < v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(32i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(1048575i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
break;
}
'label_5: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(9223372036854775807i64);
v0 = TaggedVal::from(v0.try_as_i64()? & v1.try_as_i64()?);
v1 = TaggedVal::from(0i64);
v0 = TaggedVal::from((v0.try_as_i64()? != v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(-1f64);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? / v1.try_as_f64()?);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-1i64);
v0 = TaggedVal::from((v0.try_as_i64()? > v1.try_as_i64()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_f64()? - v1.try_as_f64()?);
v1 = TaggedVal::from(0f64);
v0 = TaggedVal::from(v0.try_as_f64()? / v1.try_as_f64()?);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(2146435071i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1072693248i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1023i32);
local_4 = v0.try_as_i32()?;
'label_6: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(1072693248i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_2);
local_3 = v0.try_as_i32()?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0f64);
return Some(v0.try_as_f64()?);
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(18014398509481984f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
v0 = TaggedVal::from((v0.try_as_f64()?.to_bits()));
local_1 = v0.try_as_i64()?;
v1 = TaggedVal::from(32i64);
v0 = TaggedVal::from((v0.try_as_i64()? as u64) >> (v1.try_as_i64()? % 64));
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1077i32);
local_4 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(614242i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_2 = v1.try_as_i32()?;
v2 = TaggedVal::from(20i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()? as f64));
local_5 = v0.try_as_f64()?;
v1 = TaggedVal::from(0.30102999566361177f64);
v0 = TaggedVal::from(v0.try_as_f64()? * v1.try_as_f64()?);
local_6 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(1048575i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1072079006i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
v2 = TaggedVal::from(32i64);
v1 = TaggedVal::from(v1.try_as_i64()? << (v2.try_as_i64()? % 64));
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(4294967295i64);
v2 = TaggedVal::from(v2.try_as_i64()? & v3.try_as_i64()?);
v1 = TaggedVal::from(v1.try_as_i64()? | v2.try_as_i64()?);
v1 = TaggedVal::from(f64::from_bits(v1.try_as_i64()? as u64));
v2 = TaggedVal::from(-1f64);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
local_0 = v1.try_as_f64()?;
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(0.5f64);
v3 = TaggedVal::from(v3.try_as_f64()? * v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
local_7 = v2.try_as_f64()?;
v1 = TaggedVal::from(v1.try_as_f64()? - v2.try_as_f64()?);
v1 = TaggedVal::from((v1.try_as_f64()?.to_bits()));
v2 = TaggedVal::from(-4294967296i64);
v1 = TaggedVal::from(v1.try_as_i64()? & v2.try_as_i64()?);
v1 = TaggedVal::from(f64::from_bits(v1.try_as_i64()? as u64));
local_8 = v1.try_as_f64()?;
v2 = TaggedVal::from(0.4342944818781689f64);
v1 = TaggedVal::from(v1.try_as_f64()? * v2.try_as_f64()?);
local_9 = v1.try_as_f64()?;
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
local_10 = v0.try_as_f64()?;
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_6);
v3 = TaggedVal::from(local_10);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(local_8);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v3 = TaggedVal::from(local_7);
v2 = TaggedVal::from(v2.try_as_f64()? - v3.try_as_f64()?);
v3 = TaggedVal::from(local_0);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(2f64);
v4 = TaggedVal::from(v4.try_as_f64()? + v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? / v4.try_as_f64()?);
local_0 = v3.try_as_f64()?;
v4 = TaggedVal::from(local_7);
v5 = TaggedVal::from(local_0);
v6 = TaggedVal::from(local_0);
v5 = TaggedVal::from(v5.try_as_f64()? * v6.try_as_f64()?);
local_6 = v5.try_as_f64()?;
v6 = TaggedVal::from(local_6);
v5 = TaggedVal::from(v5.try_as_f64()? * v6.try_as_f64()?);
local_0 = v5.try_as_f64()?;
v6 = TaggedVal::from(local_0);
v7 = TaggedVal::from(local_0);
v8 = TaggedVal::from(0.15313837699209373f64);
v7 = TaggedVal::from(v7.try_as_f64()? * v8.try_as_f64()?);
v8 = TaggedVal::from(0.22222198432149784f64);
v7 = TaggedVal::from(v7.try_as_f64()? + v8.try_as_f64()?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v7 = TaggedVal::from(0.3999999999940942f64);
v6 = TaggedVal::from(v6.try_as_f64()? + v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? * v6.try_as_f64()?);
v6 = TaggedVal::from(local_6);
v7 = TaggedVal::from(local_0);
v8 = TaggedVal::from(local_0);
v9 = TaggedVal::from(local_0);
v10 = TaggedVal::from(0.14798198605116586f64);
v9 = TaggedVal::from(v9.try_as_f64()? * v10.try_as_f64()?);
v10 = TaggedVal::from(0.1818357216161805f64);
v9 = TaggedVal::from(v9.try_as_f64()? + v10.try_as_f64()?);
v8 = TaggedVal::from(v8.try_as_f64()? * v9.try_as_f64()?);
v9 = TaggedVal::from(0.2857142874366239f64);
v8 = TaggedVal::from(v8.try_as_f64()? + v9.try_as_f64()?);
v7 = TaggedVal::from(v7.try_as_f64()? * v8.try_as_f64()?);
v8 = TaggedVal::from(0.6666666666666735f64);
v7 = TaggedVal::from(v7.try_as_f64()? + v8.try_as_f64()?);
v6 = TaggedVal::from(v6.try_as_f64()? * v7.try_as_f64()?);
v5 = TaggedVal::from(v5.try_as_f64()? + v6.try_as_f64()?);
v4 = TaggedVal::from(v4.try_as_f64()? + v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? * v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
local_0 = v2.try_as_f64()?;
v3 = TaggedVal::from(0.4342944818781689f64);
v2 = TaggedVal::from(v2.try_as_f64()? * v3.try_as_f64()?);
v3 = TaggedVal::from(local_5);
v4 = TaggedVal::from(0.0000000000003694239077158931f64);
v3 = TaggedVal::from(v3.try_as_f64()? * v4.try_as_f64()?);
v4 = TaggedVal::from(local_0);
v5 = TaggedVal::from(local_8);
v4 = TaggedVal::from(v4.try_as_f64()? + v5.try_as_f64()?);
v5 = TaggedVal::from(0.000000000025082946711645275f64);
v4 = TaggedVal::from(v4.try_as_f64()? * v5.try_as_f64()?);
v3 = TaggedVal::from(v3.try_as_f64()? + v4.try_as_f64()?);
v2 = TaggedVal::from(v2.try_as_f64()? + v3.try_as_f64()?);
v1 = TaggedVal::from(v1.try_as_f64()? + v2.try_as_f64()?);
v0 = TaggedVal::from(v0.try_as_f64()? + v1.try_as_f64()?);
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
                         let a0 = args[0].try_as_i32()?;
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