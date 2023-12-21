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
                 m.globals[0] = TaggedVal::from(67072i32);
                 
                 
                 m
             }
         }

impl WasmModule {
#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_0(&mut self, arg_0: i32) -> Option<()> {
std::process::exit(arg_0)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_1(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::args_sizes_get(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0, arg_1))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_2(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
Some(wasi_common::wasi::wasi_snapshot_preview1::args_get(&self.context, &guest_mem_wrapper::GuestMemWrapper::from(&mut self.memory), arg_0, arg_1))}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_3(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_5(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;self.func_3()?;
v0 = TaggedVal::from(self.func_6()?);
local_0 = v0.try_as_i32()?;
self.func_18()?;
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
fn func_6(&mut self, ) -> Option<i32> {
let mut v0: TaggedVal;v0 = TaggedVal::from(self.func_13()?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_7(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_8(&mut self, arg_0: i32) -> Option<i32> {
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
v1 = TaggedVal::from(67072i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1500) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1512) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1504) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1500) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1520) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1472) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1480) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(67072i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1476) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(67072i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1044) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1064) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1060) as usize, v1.try_as_i32()?)?;
'label_2: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1076i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1068i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1080i32);
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
v1 = TaggedVal::from(67080i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(67080i32);
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
v1 = TaggedVal::from(67076i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1516) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1056) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(67072i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(67020i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1028) as usize)?);
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
v1 = TaggedVal::from(1076i32);
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
v2 = TaggedVal::from(1068i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
{

}
break 'label_16;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1036) as usize)?);
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
v1 = TaggedVal::from(1076i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(1068i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1032) as usize)?);
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
v1 = TaggedVal::from(1332i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1032) as usize)?);
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
v1 = TaggedVal::from(1332i32);
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
v1 = TaggedVal::from(1332i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1036) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1036) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_45;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1040) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_48;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1500) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_50;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1508) as usize)?);
local_4 = v0.try_as_i32()?;
{

}
break 'label_49;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1512) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1504) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1500) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1520) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1472) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1024) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
'label_52: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1468) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
'label_53: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1460) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1024) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 1472) as usize).and_then(|x| Some(x as i32))?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(1476i32);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1504) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1468) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_60;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1460) as usize)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1508) as usize)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);

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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1472) as usize)?);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1472) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(self.func_15(v1.try_as_i32()?)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1460) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1460) as usize, v1.try_as_i32()?)?;
'label_64: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1464) as usize)?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_64;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1464) as usize, v1.try_as_i32()?)?;
break;
}
'label_65: loop {
'label_66: loop {
'label_67: loop {
'label_68: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_68;
}
v0 = TaggedVal::from(1476i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1044) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1480) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1476) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1060) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1500) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1064) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1488) as usize, v1.try_as_i32()?)?;
'label_72: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1076i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1068i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1080i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1516) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1056) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1040) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1516) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1056) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1044) as usize)?);
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_73;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1044) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(1476i32);
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
v0 = TaggedVal::from(1476i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1040) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_85;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1036) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1068i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1028) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_9);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1032) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_101: loop {
'label_102: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1028) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
'label_104: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1032) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1516) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1056) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 1484) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 1476) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1484) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1480) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1476) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1488) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_108: loop {
'label_109: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1028) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_111: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1032) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1040) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1024) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1332i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_120: loop {
'label_121: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1028) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1332i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
fn func_9(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
self.func_10(v0.try_as_i32()?)?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_10(&mut self, arg_0: i32) -> Option<()> {
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1044) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
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
v2 = TaggedVal::from(1068i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1028) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_7);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1032) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1052) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1052) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1040) as usize)?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1040) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1048) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
return Some(());
break;
}
'label_18: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1048) as usize)?);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1048) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1036) as usize)?);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1068i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_2 = v1.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1028) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_3);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1044) as usize)?);
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
v1 = TaggedVal::from(1332i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1032) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_4);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1048) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_15;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1036) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1068i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_33: loop {
'label_34: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1028) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1028) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1332i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
'label_36: loop {
'label_37: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1032) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1032) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1060) as usize)?);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1060) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1484i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1060) as usize, v1.try_as_i32()?)?;
break;
}Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_11(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut local_2 : i32 = 0i32;
let mut local_3 : i64 = 0i64;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_0);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
local_2 = v0.try_as_i32()?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? as u32 as u64 as i64));
v1 = TaggedVal::from(local_1);
v1 = TaggedVal::from((v1.try_as_i32()? as u32 as u64 as i64));
v0 = TaggedVal::from(v0.try_as_i64()?.wrapping_mul(v1.try_as_i64()?));
local_3 = v0.try_as_i64()?;
v0 = TaggedVal::from(v0.try_as_i64()? as i32);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
v1 = TaggedVal::from(65536i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(32i64);
v2 = TaggedVal::from((v2.try_as_i64()? as u64) >> (v3.try_as_i64()? % 64));
v2 = TaggedVal::from(v2.try_as_i64()? as i32);
v3 = TaggedVal::from(0i32);
v2 = TaggedVal::from((v2.try_as_i32()? != v3.try_as_i32()?) as i32);
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
'label_2: loop {
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(-4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 0) as usize).and_then(|x| Some(x as i32))?);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_19(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

break;
}
v0 = TaggedVal::from(local_0);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_12(&mut self, arg_0: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
self.func_0(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_13(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;
let mut local_3 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v0 = TaggedVal::from(self.func_1(v0.try_as_i32()?, v1.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_1 = v0.try_as_i32()?;
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
v0 = TaggedVal::from(self.func_7(v0.try_as_i32()?)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(self.func_2(v0.try_as_i32()?, v1.try_as_i32()?)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_16(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_1);
return Some(v0.try_as_i32()?);
break;
}
v0 = TaggedVal::from(71i32);
self.func_12(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(70i32);
self.func_12(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(70i32);
self.func_12(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(local_3);
self.func_9(v0.try_as_i32()?)?;
v0 = TaggedVal::from(70i32);
self.func_12(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(local_3);
self.func_9(v0.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
self.func_9(v0.try_as_i32()?)?;
v0 = TaggedVal::from(71i32);
self.func_12(v0.try_as_i32()?)?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_14(&mut self, ) -> Option<()> {
unreachable!("Reached a point explicitly marked unreachable in WASM module");
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_15(&mut self, arg_0: i32) -> Option<i32> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1024) as usize, v1.try_as_i32()?)?;
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
self.func_14()?;
unreachable!("Reached a point explicitly marked unreachable in WASM module");// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_16(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_4(v0.try_as_i32()?, v1.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_17(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_18(&mut self, ) -> Option<()> {
self.func_17()?;
self.func_17()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_19(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_1(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
2 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_2(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
3 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_3()?;
                         Some(vec![])
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
                         
                         let rets = self.func_6()?;
                         Some(vec![TaggedVal::from(rets)])
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
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_9(a0)?;
                         Some(vec![])
                     }
10 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_10(a0)?;
                         Some(vec![])
                     }
11 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_11(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
12 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         self.func_12(a0)?;
                         Some(vec![])
                     }
13 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_13()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
14 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_14()?;
                         Some(vec![])
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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_16(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
17 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_17()?;
                         Some(vec![])
                     }
18 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_18()?;
                         Some(vec![])
                     }
19 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_19(a0, a1, a2)?;
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
                         self.func_5()
                     }
                 }
fn main() {
                         let mut wasm_module = WasmModule::new();
                         wasm_module._start().unwrap();
                     }