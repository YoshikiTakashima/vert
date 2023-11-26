static mut PARAM1: [i32 ; 2] = [12,12];
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
                 m.globals[0] = TaggedVal::from(66576i32);
                 
                 m.memory[1024..1032].copy_from_slice(&[11, 0, 0, 0, 129, 0, 0, 0]);
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
let mut local_211 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = self.globals[0];
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(48i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(local_3);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(-2147483648i32);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 44) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 40) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_9);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 36) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(2i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(15i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(-16i32);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_13);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_16);
v1 = TaggedVal::from(local_15);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_8);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 32) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_12);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_20);
v1 = TaggedVal::from(local_14);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_22);
v1 = TaggedVal::from(local_21);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_18);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_7);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_24);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_25);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_27);
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_28);
v1 = TaggedVal::from(local_29);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_32);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
v1 = TaggedVal::from(local_35);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_33);
v1 = TaggedVal::from(local_36);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_37);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v1 = TaggedVal::from(local_41);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
v1 = TaggedVal::from(local_43);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_39);
v1 = TaggedVal::from(local_44);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_45);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_38);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_46);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_47);
v1 = TaggedVal::from(local_48);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_49);
v1 = TaggedVal::from(local_50);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_51 = v0.try_as_i32()?;
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_51);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_3;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_52);
v1 = TaggedVal::from(local_53);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
v1 = TaggedVal::from(local_55);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_56);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_57);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_60);
v1 = TaggedVal::from(local_61);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_59);
v1 = TaggedVal::from(local_62);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_58);
v1 = TaggedVal::from(local_64);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_65 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_66);
v1 = TaggedVal::from(local_67);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_68);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_69);
v1 = TaggedVal::from(local_65);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
break 'label_2;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v1 = TaggedVal::from(local_77);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_78);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_79);
v1 = TaggedVal::from(local_75);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_80);
v1 = TaggedVal::from(local_81);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_82);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_1;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_84);
v1 = TaggedVal::from(local_85);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_86);
v1 = TaggedVal::from(local_87);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_83);
v1 = TaggedVal::from(local_88);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_89);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_91);
v1 = TaggedVal::from(local_92);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_93);
v1 = TaggedVal::from(local_94);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_95);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_96 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_96);
v1 = TaggedVal::from(local_90);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_97);
v1 = TaggedVal::from(local_98);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_99);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(0i32);
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_101 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_101);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_100);
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_102);
v1 = TaggedVal::from(local_103);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_104);
v1 = TaggedVal::from(local_105);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_106 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_106);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_108);
v1 = TaggedVal::from(local_109);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_107);
v1 = TaggedVal::from(local_110);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_111);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_115 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_114);
v1 = TaggedVal::from(local_115);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_116);
v1 = TaggedVal::from(local_117);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_113);
v1 = TaggedVal::from(local_118);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_119);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_120 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_112);
local_121 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_120);
local_122 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_121);
v1 = TaggedVal::from(local_122);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_123 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_124 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_123);
v1 = TaggedVal::from(local_124);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_125 = v0.try_as_i32()?;
'label_6: loop {
'label_7: loop {
v0 = TaggedVal::from(local_125);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_126 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_127 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_126);
v1 = TaggedVal::from(local_127);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_128 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_129 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_128);
v1 = TaggedVal::from(local_129);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_130 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_130);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_131 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_131);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_132 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_133 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_134 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_135 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_134);
v1 = TaggedVal::from(local_135);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_136 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_133);
v1 = TaggedVal::from(local_136);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_137 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_137);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_138 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_132);
v1 = TaggedVal::from(local_138);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_139 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_140 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_141 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_140);
v1 = TaggedVal::from(local_141);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_142 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_142);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_143 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_143);
v1 = TaggedVal::from(local_139);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
{

}
break 'label_6;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_144 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_145 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_146 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_145);
v1 = TaggedVal::from(local_146);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_147 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_144);
v1 = TaggedVal::from(local_147);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_148 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_148);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_149 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_150 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_151 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_150);
v1 = TaggedVal::from(local_151);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_152 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_152);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_153 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_153);
v1 = TaggedVal::from(local_149);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_154 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_155 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_154);
v1 = TaggedVal::from(local_155);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_156 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_156);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_5;
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_157 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_157);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
'label_8: loop {
'label_9: loop {
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_158 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 40) as usize)?);
local_159 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_158);
local_160 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_159);
local_161 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_160);
v1 = TaggedVal::from(local_161);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_162 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_163 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_162);
v1 = TaggedVal::from(local_163);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_164 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_164);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_8;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_165 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_166 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_167 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_166);
v1 = TaggedVal::from(local_167);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_168 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_168);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_169 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_169);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_170 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_171 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_172 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_171);
v1 = TaggedVal::from(local_172);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_173 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_173);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_174 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_174);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_175 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_170);
v1 = TaggedVal::from(local_175);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_176 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_177 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_178 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_179 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_178);
v1 = TaggedVal::from(local_179);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_180 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_177);
v1 = TaggedVal::from(local_180);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_181 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_181);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_182 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_176);
v1 = TaggedVal::from(local_182);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_183 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_165);
local_184 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_183);
local_185 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_184);
v1 = TaggedVal::from(local_185);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_186 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_187 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_186);
v1 = TaggedVal::from(local_187);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_188 = v0.try_as_i32()?;
'label_10: loop {
v0 = TaggedVal::from(local_188);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_10;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_189 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_190 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_189);
v1 = TaggedVal::from(local_190);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_191 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
v1 = TaggedVal::from(local_191);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_192 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_192);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_193 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_194 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_195 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_194);
v1 = TaggedVal::from(local_195);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_196 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v1 = TaggedVal::from(local_196);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_197 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_197);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_198 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_193);
v1 = TaggedVal::from(local_198);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_199 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 44) as usize)?);
local_200 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_201 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_202 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_201);
v1 = TaggedVal::from(local_202);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_203 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_200);
v1 = TaggedVal::from(local_203);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_204 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_204);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_205 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_199);
v1 = TaggedVal::from(local_205);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_206 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_206);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_207 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_208 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_207);
v1 = TaggedVal::from(local_208);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_209 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v1 = TaggedVal::from(local_209);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_9;
break;}
break;
}
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_210 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_5);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 36) as usize)?);
local_211 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_211);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_210);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_4(&mut self, ) -> Option<i32> {
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
v0 = TaggedVal::from(29i32);
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
v0 = TaggedVal::from(unsafe {
	PARAM1[0] = kani::any();
	kani::assume((0..2).contains(&PARAM1[0]));
	PARAM1[0]
});
v0 = TaggedVal::from(self.func_3(v0.try_as_i32()?, v1.try_as_i32()?)?);

let retval = v0.try_as_i32()?;
unsafe {
RESULT = retval;
}

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
fn func_5(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_6(&mut self, ) -> Option<()> {
self.func_5()?;
self.func_5()?;Some(())}

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


fn f_gold(arr: [i32;2], n: i32) -> i32 {
    let mut msis = vec![0; n as usize];
    let mut msds = vec![0; n as usize];
    let mut max_sum = i32::MIN;
    msis[0] = arr[0];
    for i in 1..n as usize {
        if arr[i] > arr[i - 1] {
            msis[i] = msis[i - 1] + arr[i]; 
        } else {
            msis[i] = arr[i];
        }
    }
    msds[n as usize - 1] = arr[n as usize - 1];
    for i in (0..n as usize - 1).rev() {
        if arr[i] > arr[i + 1] {
            msds[i] = msds[i + 1] + arr[i];
        } else {
            msds[i] = arr[i];
        }
    }
    for i in 0..n as usize {
        if max_sum < (msis[i] + msds[i] - arr[i]) {
            max_sum = msis[i] + msds[i] - arr[i];
        }
    }
    max_sum
}////// LLM Output //////

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
fn kani_wasm_eq(){ 
		let result = f_gold([unsafe{PARAM1}[0], unsafe{PARAM1}[1]],unsafe{PARAM2}.into());
		let result_prime = f_gold_wasm_thread_unsafe();
		assert_eq!(result, result_prime);
}