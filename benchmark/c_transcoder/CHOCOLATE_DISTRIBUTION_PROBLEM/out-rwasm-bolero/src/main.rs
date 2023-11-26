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
                 m.globals[0] = TaggedVal::from(66576i32);
                 if m.indirect_call_table.len() < 2 { m.indirect_call_table.resize(2, None) }
m.indirect_call_table[1] = Some(3);
                 m.memory[1024..1032].copy_from_slice(&[11, 0, 0, 0, 12, 0, 0, 0]);
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
v0 = TaggedVal::from(self.func_6()?);
local_0 = v0.try_as_i32()?;
self.func_8()?;
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
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
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
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;v0 = self.globals[0];
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
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(local_6);
self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?;
v0 = TaggedVal::from(16i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
return Some(());// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_5(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
let mut local_85 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_6 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
v0 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
break;
}
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_14);
self.func_4(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
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
'label_3: loop {
v0 = TaggedVal::from(local_21);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(-1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_22);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(2147483647i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_23);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_26);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_31);
v1 = TaggedVal::from(local_32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v1 = TaggedVal::from(local_34);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v1 = TaggedVal::from(local_38);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_40);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_43);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_46);
v1 = TaggedVal::from(local_49);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_52);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_53);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_55);
v1 = TaggedVal::from(local_56);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_57);
v1 = TaggedVal::from(local_58);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_59 = v0.try_as_i32()?;
'label_6: loop {
v0 = TaggedVal::from(local_59);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_60);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_61);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 32) as usize)?);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_62);
v1 = TaggedVal::from(local_63);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_64);
v1 = TaggedVal::from(local_65);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_66);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_67);
v1 = TaggedVal::from(local_68);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_69);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_5;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_71);
v1 = TaggedVal::from(local_72);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_70);
v1 = TaggedVal::from(local_73);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_74);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_77);
v1 = TaggedVal::from(local_78);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v1 = TaggedVal::from(local_79);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_80);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_75);
v1 = TaggedVal::from(local_81);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_82);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_84);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_85);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_83);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_6(&mut self, ) -> Option<i32> {
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
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 1024) as usize)?);
local_10 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(local_5);
v0 = TaggedVal::from(unsafe {PARAM1}[0]);
v0 = TaggedVal::from(self.func_5(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

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
fn func_7(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_8(&mut self, ) -> Option<()> {
self.func_7()?;
self.func_7()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_9(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
fn func_10(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<()> {
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
let mut local_17 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;
let mut v6: TaggedVal;
let mut v7: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(704i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()?)?;
'label_0: loop {
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_mul(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(8i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_8 = v0.try_as_i32()?;
'label_1: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_2);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_7);
local_8 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_7 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_1;
}
break;}
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_9 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
'label_4: loop {
'label_5: loop {
'label_6: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(3i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 208) as usize, v1.try_as_i32()?)?;
'label_7: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_14 = v0.try_as_i32()?;
'label_8: loop {
'label_9: loop {
'label_10: loop {
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_14);
v4 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
local_7 = v3.try_as_i32()?;
v4 = TaggedVal::from(2i32);
v3 = TaggedVal::from(v3.try_as_i32()? << (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
break;
}
'label_11: loop {
'label_12: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
{

}
break 'label_11;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
local_1 = v0.try_as_i32()?;
break;
}
'label_13: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 208) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_14 = v0.try_as_i32()?;
{

}
continue 'label_9;
break;
}
break;}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_12);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_15 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(448i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(448i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_16 = v0.try_as_i32()?;
'label_14: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 208) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_16);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_16);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_7 = v2.try_as_i32()?;
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
local_5 = v0.try_as_i32()?;
'label_15: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(4i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_13 = v2.try_as_i32()?;
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
local_14 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_7);
v1 = TaggedVal::from(self.func_9(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
v2 = TaggedVal::from(local_7);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_15;
}
break;}
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_15);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_1 = v0.try_as_i32()?;
{

}
continue 'label_14;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 12) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(30i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_4);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 8) as usize)?);
v3 = TaggedVal::from(2i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
{

}
break 'label_5;
break;
}
'label_16: loop {
'label_17: loop {
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(-1i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_15 = v1.try_as_i32()?;
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_10);
v5 = TaggedVal::from(0i32);
v6 = TaggedVal::from(local_4);
v7 = TaggedVal::from(16i32);
v6 = TaggedVal::from(v6.try_as_i32()?.wrapping_add(v7.try_as_i32()?));
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?;
{

}
break 'label_16;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 208) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(1i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_14 = v0.try_as_i32()?;
'label_18: loop {
'label_19: loop {
'label_20: loop {
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_6);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(16i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v3 = TaggedVal::from(local_14);
v4 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
local_7 = v3.try_as_i32()?;
v4 = TaggedVal::from(2i32);
v3 = TaggedVal::from(v3.try_as_i32()? << (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_20;
}
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_18;
}
break;
}
'label_21: loop {
'label_22: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_3);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_22;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
{

}
break 'label_21;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
local_1 = v0.try_as_i32()?;
break;
}
'label_23: loop {
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_23;
}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 208) as usize)?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
local_14 = v0.try_as_i32()?;
{

}
continue 'label_19;
break;
}
break;}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_12);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(448i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(448i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
local_16 = v0.try_as_i32()?;
'label_24: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 208) as usize)?);
local_8 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_16);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_16);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_7 = v2.try_as_i32()?;
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(208i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
local_5 = v0.try_as_i32()?;
'label_25: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(4i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_13 = v2.try_as_i32()?;
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
local_14 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_7);
v1 = TaggedVal::from(self.func_9(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
v2 = TaggedVal::from(local_7);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_25;
}
break;}
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_17);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_1 = v0.try_as_i32()?;
{

}
continue 'label_24;
break;}
break;
}
'label_26: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_26;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_7 = v1.try_as_i32()?;
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(31i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_4);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 12) as usize)?);
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_10 = v0.try_as_i32()?;
{

}
break 'label_5;
break;
}
'label_27: loop {
'label_28: loop {
v0 = TaggedVal::from(local_15);
v1 = TaggedVal::from(31i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) > (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_28;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_8 = v0.try_as_i32()?;
{

}
break 'label_27;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_8 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(-33i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_15 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(local_15);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(32i32);
v3 = TaggedVal::from(local_15);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_8);
v3 = TaggedVal::from(local_15);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
local_1 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_9);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_4;
}
break;}
break;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_10);
v5 = TaggedVal::from(0i32);
v6 = TaggedVal::from(local_4);
v7 = TaggedVal::from(16i32);
v6 = TaggedVal::from(v6.try_as_i32()?.wrapping_add(v7.try_as_i32()?));
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
'label_29: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_1 = v0.try_as_i32()?;
'label_30: loop {
'label_31: loop {
'label_32: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_32;
}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_32;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_31;
}
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_30;
}
break;
}
'label_33: loop {
'label_34: loop {
'label_35: loop {
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()?.trailing_zeros() as i32));
local_8 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_35;
}
'label_36: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()?.trailing_zeros() as i32));
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_36;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
{

}
break 'label_34;
break;
}
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_7 = v0.try_as_i32()?;
{

}
break 'label_33;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_33;
}
break;
}
v0 = TaggedVal::from(local_5);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(local_7);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(32i32);
v3 = TaggedVal::from(local_7);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_sub(v3.try_as_i32()?));
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(local_7);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
{

}
continue 'label_29;
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 12) as usize)?);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(30i32);
v2 = TaggedVal::from((v2.try_as_i32()? as u32) >> (v3.try_as_i32()? % 32));
local_7 = v2.try_as_i32()?;
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(2147483646i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(local_7);
v3 = TaggedVal::from(31i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
v2 = TaggedVal::from(3i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_4);
v2 = TaggedVal::from(16i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(local_10);
v3 = TaggedVal::from(-2i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_10 = v2.try_as_i32()?;
v3 = TaggedVal::from(2i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_1);
v5 = TaggedVal::from(1i32);
v6 = TaggedVal::from(local_4);
v7 = TaggedVal::from(16i32);
v6 = TaggedVal::from(v6.try_as_i32()?.wrapping_add(v7.try_as_i32()?));
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_4);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 8) as usize)?);
local_1 = v1.try_as_i32()?;
v2 = TaggedVal::from(31i32);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(local_4);
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 12) as usize)?);
v3 = TaggedVal::from(1i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v2 = TaggedVal::from(1i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_2);
v2 = TaggedVal::from(local_3);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(8i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
v4 = TaggedVal::from(local_10);
v5 = TaggedVal::from(1i32);
v6 = TaggedVal::from(local_4);
v7 = TaggedVal::from(16i32);
v6 = TaggedVal::from(v6.try_as_i32()?.wrapping_add(v7.try_as_i32()?));
self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?, v6.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
{

}
continue 'label_29;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(704i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_11(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32, arg_5: i32, arg_6: i32) -> Option<()> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i32 = arg_1;
let mut local_2 : i32 = arg_2;
let mut local_3 : i32 = arg_3;
let mut local_4 : i32 = arg_4;
let mut local_5 : i32 = arg_5;
let mut local_6 : i32 = arg_6;let mut local_7 : i32 = 0i32;
let mut local_8 : i32 = 0i32;
let mut local_9 : i32 = 0i32;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;
let mut local_12 : i32 = 0i32;
let mut local_13 : i32 = 0i32;
let mut local_14 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;
let mut v4: TaggedVal;
let mut v5: TaggedVal;v0 = self.globals[0];
v1 = TaggedVal::from(736i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_7 = v0.try_as_i32()?;
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_8 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
'label_2: loop {
'label_3: loop {
'label_4: loop {
v0 = TaggedVal::from(local_3);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_9 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_0);
local_11 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
'label_5: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(2i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_5;
}
v0 = TaggedVal::from(local_0);
local_11 = v0.try_as_i32()?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
'label_6: loop {
v0 = TaggedVal::from(local_3);
local_11 = v0.try_as_i32()?;
'label_7: loop {
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(-8i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_3 = v0.try_as_i32()?;
'label_8: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_14 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_11);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_0);
local_11 = v0.try_as_i32()?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(local_11);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_0);
local_11 = v0.try_as_i32()?;
{

}
break 'label_1;
break;
}
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_9: loop {
'label_10: loop {
'label_11: loop {
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from((v0.try_as_i32()?.trailing_zeros() as i32));
local_3 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_11;
}
'label_12: loop {
v0 = TaggedVal::from(local_9);
v0 = TaggedVal::from((v0.try_as_i32()?.trailing_zeros() as i32));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_12;
}
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_10;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_0 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(32i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_9);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_0 = v0.try_as_i32()?;
{

}
break 'label_9;
break;
}
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(-32i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_14 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(32i32);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_0);
v1 = TaggedVal::from((v1.try_as_i32()? as u32) >> (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
'label_13: loop {
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_0);
v0 = TaggedVal::from((v0.try_as_i32()? as u32) >> (v1.try_as_i32()? % 32));
local_9 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? == v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
break;
}
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(local_4);
v3 = TaggedVal::from(2i32);
v2 = TaggedVal::from(v2.try_as_i32()? << (v3.try_as_i32()? % 32));
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_7);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
continue 'label_6;
}
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_10 = v0.try_as_i32()?;
break;
}
'label_14: loop {
v0 = TaggedVal::from(local_10);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_10);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_12 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(480i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(480i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_1);
local_5 = v0.try_as_i32()?;
'label_15: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_7);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 0) as usize)?);
local_9 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_5);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_5);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_3 = v2.try_as_i32()?;
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_7);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_10);
local_8 = v0.try_as_i32()?;
'label_16: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(4i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_13 = v2.try_as_i32()?;
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
local_14 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_3);
v1 = TaggedVal::from(self.func_9(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_16;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_12);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
{

}
continue 'label_15;
break;}
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 240) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from((v0.try_as_i32()? <= v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(240i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()? | v1.try_as_i32()?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
local_0 = v0.try_as_i32()?;
'label_17: loop {
'label_18: loop {
'label_19: loop {
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_0);
v2 = TaggedVal::from(local_13);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_9 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_6);
v3 = TaggedVal::from(local_4);
v4 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(v3.try_as_i32()?.wrapping_add(v4.try_as_i32()?));
local_3 = v3.try_as_i32()?;
v4 = TaggedVal::from(2i32);
v3 = TaggedVal::from(v3.try_as_i32()? << (v4.try_as_i32()? % 32));
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_sub(v2.try_as_i32()?));
local_0 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_19;
}
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_17;
}
break;
}
'label_20: loop {
'label_21: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_2);
{
                    let rets = self.indirect_call(v2.try_as_i32()? as usize, &[v0, v1])?;
                    if rets.len() != 1 {
                        return None;
                    }v0 = rets[0];
                 }
v1 = TaggedVal::from(0i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_21;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
{

}
break 'label_20;
break;
}
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_9);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_9);
local_0 = v0.try_as_i32()?;
break;
}
'label_22: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_22;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(4i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 240) as usize)?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_3);
local_4 = v0.try_as_i32()?;
{

}
continue 'label_18;
break;
}
break;}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_5 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(2i32);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(240i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_5);
v2 = TaggedVal::from(2i32);
v1 = TaggedVal::from(v1.try_as_i32()? << (v2.try_as_i32()? % 32));
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_7);
v2 = TaggedVal::from(480i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_1);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(480i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_23: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_7);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 240) as usize)?);
local_9 = v1.try_as_i32()?;
v2 = TaggedVal::from(local_1);
v3 = TaggedVal::from(256i32);
v4 = TaggedVal::from(local_1);
v5 = TaggedVal::from(256i32);
v4 = TaggedVal::from(((v4.try_as_i32()? as u32) < (v5.try_as_i32()? as u32)) as i32);
if ValType::from(v2) != ValType::from(v3) {
                     return None;
                 }
                 if v4.try_as_i32()? != 0 {
                     v2 = v2;
                 } else {
                     v2 = v3;
                 }
local_3 = v2.try_as_i32()?;
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);

v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(240i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
local_8 = v0.try_as_i32()?;
'label_24: loop {
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_0);
v3 = TaggedVal::from(4i32);
v2 = TaggedVal::from(v2.try_as_i32()?.wrapping_add(v3.try_as_i32()?));
local_13 = v2.try_as_i32()?;
v2 = TaggedVal::from(read_mem_i32(&self.memory, (v2.try_as_i32()? + 0) as usize)?);
local_14 = v2.try_as_i32()?;
v3 = TaggedVal::from(local_3);
v1 = TaggedVal::from(self.func_9(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_13);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_14);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(-1i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_8 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
continue 'label_24;
}
break;}
v0 = TaggedVal::from(local_1);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_0 = v0.try_as_i32()?;
{

}
continue 'label_23;
break;}
break;
}
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(736i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);Some(())}

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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_4(a0, a1)?;
                         Some(vec![])
                     }
5 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_5(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
6 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_6()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
7 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_7()?;
                         Some(vec![])
                     }
8 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_8()?;
                         Some(vec![])
                     }
9 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_9(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
10 => {
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         self.func_10(a0, a1, a2, a3)?;
                         Some(vec![])
                     }
11 => {
                         if args.len() != 7 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
let a5 = args[5].try_as_i32()?;
let a6 = args[6].try_as_i32()?;
                         self.func_11(a0, a1, a2, a3, a4, a5, a6)?;
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

use std::cmp::min;
use std::mem;

fn f_gold(mut arr: [i32; n: i32], m: i32) -> i32 {
    if m == 0 || n == 0 {
        return 0;
    }
    arr.sort();
    let mut min_diff = i32::MAX;
    let mut first = 0;
    let mut last = 0;
    for i in 0..n - m + 1 {
        let diff = arr[i + m - 1] - arr[i];
        if diff < min_diff {
            min_diff = diff;
            first = i;
            last = i + m - 1;
        }
    }
    arr[last] - arr[first]
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