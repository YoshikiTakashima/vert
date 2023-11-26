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
self.func_17()?;
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
let mut local_63 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(3i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_9);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_11 = v0.try_as_i32()?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_11);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_12);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(0i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
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
v0 = TaggedVal::from(local_14);
v1 = TaggedVal::from(local_19);
self.func_4(v0.try_as_i32()?, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_13);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v1 = TaggedVal::from(local_25);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(local_32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
v1 = TaggedVal::from(local_40);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_41);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_42);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v1 = TaggedVal::from(local_46);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_49);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_51);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_52);
v1 = TaggedVal::from(local_53);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_55);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_56 = v0.try_as_i32()?;
'label_4: loop {
v0 = TaggedVal::from(local_56);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(1i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_57);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
break 'label_0;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_59);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_60);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;}
break;
}
unreachable!("Reached a point explicitly marked unreachable in WASM module");
unreachable!("Reached a point explicitly marked unreachable in WASM module");
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(16i32);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_62);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_61);
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
self.func_5(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
fn func_5(&mut self, arg_0: i32, arg_1: i32) -> Option<()> {
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
self.func_7(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
let mut local_9 : i64 = 0i64;
let mut local_10 : i32 = 0i32;
let mut local_11 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(2i32);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(8i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v0 = TaggedVal::from(read_mem_i64(&self.memory, (v0.try_as_i32()? + 1024) as usize)?);
local_9 = v0.try_as_i64()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_9);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_4);
v0 = TaggedVal::from(self.func_3(v0.try_as_i32()?, v1.try_as_i32()?)?);

v0 = TaggedVal::from(16i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_3);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_7(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);

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
self.func_13(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_14(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_7(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
self.func_7(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?;
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
fn func_8(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
fn func_10(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
fn func_11(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
fn func_12(&mut self, arg_0: i32, arg_1: i32, arg_2: i32, arg_3: i32, arg_4: i32, arg_5: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
fn func_13(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<()> {
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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
self.func_9(v0.try_as_i32()?, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_11(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?, v4.try_as_i32()?, v5.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?, v1.try_as_i32()?, v2.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
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
fn func_15(&mut self, arg_0: i32) -> Option<i32> {
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
fn func_16(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_17(&mut self, ) -> Option<()> {
self.func_16()?;
self.func_16()?;Some(())}

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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         self.func_5(a0, a1)?;
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
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_7(a0, a1, a2)?;
                         Some(vec![])
                     }
8 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_8(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
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
                         if args.len() != 4 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
                         let rets = self.func_10(a0, a1, a2, a3)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
11 => {
                         if args.len() != 5 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
                         let rets = self.func_11(a0, a1, a2, a3, a4)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
12 => {
                         if args.len() != 6 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
let a3 = args[3].try_as_i32()?;
let a4 = args[4].try_as_i32()?;
let a5 = args[5].try_as_i32()?;
                         let rets = self.func_12(a0, a1, a2, a3, a4, a5)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
13 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         self.func_13(a0, a1, a2)?;
                         Some(vec![])
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
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_16()?;
                         Some(vec![])
                     }
17 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_17()?;
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