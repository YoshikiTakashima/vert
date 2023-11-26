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
                 m.globals[0] = TaggedVal::from(67216i32);
                 if m.indirect_call_table.len() < 4 { m.indirect_call_table.resize(4, None) }
m.indirect_call_table[1] = Some(25);
m.indirect_call_table[2] = Some(21);
m.indirect_call_table[3] = Some(19);
                 m.memory[1024..1032].copy_from_slice(&[11, 0, 0, 0, 12, 0, 0, 0]);
m.memory[1032..1148].copy_from_slice(&[5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 124, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 4, 0, 0]);
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
let mut local_0 : i32 = 0i32;let mut v0: TaggedVal;self.func_4()?;
v0 = TaggedVal::from(self.func_7()?);
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
fn func_6(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
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
let mut local_263 : i32 = 0i32;let mut v0: TaggedVal;
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
v0 = TaggedVal::from(0i32);
local_5 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_6 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 28) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_1);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 24) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_7 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_8 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_8);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_9 = v0.try_as_i32()?;
v0 = TaggedVal::from(1073741823i32);
local_10 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_7);
v1 = TaggedVal::from(local_10);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_11 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_11);
v1 = TaggedVal::from(local_7);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_12 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_13 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_12);
v1 = TaggedVal::from(local_13);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_14 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_9);
v2 = TaggedVal::from(local_14);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_15 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_15);
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?)?);
local_16 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_16);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 12) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_0: loop {
'label_1: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_17 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_18 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_17);
local_19 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_18);
local_20 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_19);
v1 = TaggedVal::from(local_20);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_21 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_22 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_21);
v1 = TaggedVal::from(local_22);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_23 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_23);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(1i32);
local_24 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_25 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_26 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_27 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_26);
v1 = TaggedVal::from(local_27);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_28 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_25);
v1 = TaggedVal::from(local_28);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_29 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_29);
v1 = TaggedVal::from(local_24);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_30 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_31 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_30);
v1 = TaggedVal::from(local_31);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_32 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_1;
break;}
break;
}
v0 = TaggedVal::from(1i32);
local_33 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_33);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_2: loop {
'label_3: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_34 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_35 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_34);
local_36 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_35);
local_37 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_36);
v1 = TaggedVal::from(local_37);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_38 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_39 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_38);
v1 = TaggedVal::from(local_39);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_40 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_40);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_2;
}
v0 = TaggedVal::from(0i32);
local_41 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_41);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_4: loop {
'label_5: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_42 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_43 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_42);
local_44 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_43);
local_45 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_44);
v1 = TaggedVal::from(local_45);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_46 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_47 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_46);
v1 = TaggedVal::from(local_47);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_48 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_48);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_4;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_49 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_50 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_51 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_50);
v1 = TaggedVal::from(local_51);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_52 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_49);
v1 = TaggedVal::from(local_52);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_53 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_53);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_54 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_55 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_56 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_57 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_56);
v1 = TaggedVal::from(local_57);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_58 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_55);
v1 = TaggedVal::from(local_58);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_59 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_59);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_60 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_54);
local_61 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_60);
local_62 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_61);
v1 = TaggedVal::from(local_62);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_63 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_64 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_63);
v1 = TaggedVal::from(local_64);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_65 = v0.try_as_i32()?;
'label_6: loop {
v0 = TaggedVal::from(local_65);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_66 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_67 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_68 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_67);
v1 = TaggedVal::from(local_68);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_69 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_66);
v1 = TaggedVal::from(local_69);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_70 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_70);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_71 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_72 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_73 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_74 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_73);
v1 = TaggedVal::from(local_74);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_75 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_72);
v1 = TaggedVal::from(local_75);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_76 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_76);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_77 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_78 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_77);
v1 = TaggedVal::from(local_78);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_79 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_71);
local_80 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_79);
local_81 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_80);
v1 = TaggedVal::from(local_81);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_82 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_83 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_82);
v1 = TaggedVal::from(local_83);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_84 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_84);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_85 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_86 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_87 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_86);
v1 = TaggedVal::from(local_87);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_88 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_85);
v1 = TaggedVal::from(local_88);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_89 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_89);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_90 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_91 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_90);
v1 = TaggedVal::from(local_91);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_92 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_93 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_94 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_95 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_94);
v1 = TaggedVal::from(local_95);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_96 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_93);
v1 = TaggedVal::from(local_96);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_97 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_97);
v1 = TaggedVal::from(local_92);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_98 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_99 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_98);
v1 = TaggedVal::from(local_99);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_100 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_100);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_5;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_101 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_102 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_101);
v1 = TaggedVal::from(local_102);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_103 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_103);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_3;
break;}
break;
}
v0 = TaggedVal::from(0i32);
local_104 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_105 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_106 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_107 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_106);
v1 = TaggedVal::from(local_107);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_108 = v0.try_as_i32()?;
v0 = TaggedVal::from(1073741823i32);
local_109 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_106);
v1 = TaggedVal::from(local_109);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_110 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_110);
v1 = TaggedVal::from(local_106);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
local_111 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_112 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_111);
v1 = TaggedVal::from(local_112);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_113 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_105);
v1 = TaggedVal::from(local_108);
v2 = TaggedVal::from(local_113);
if ValType::from(v0) != ValType::from(v1) {
                     return None;
                 }
                 if v2.try_as_i32()? != 0 {
                     v0 = v0;
                 } else {
                     v0 = v1;
                 }
local_114 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_114);
v0 = TaggedVal::from(self.func_9(v0.try_as_i32()?)?);
local_115 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_115);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_104);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_7: loop {
'label_8: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_116 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_117 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_116);
local_118 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_117);
local_119 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_118);
v1 = TaggedVal::from(local_119);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_120 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_121 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_120);
v1 = TaggedVal::from(local_121);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_122 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_122);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_7;
}
v0 = TaggedVal::from(1i32);
local_123 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_124 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_125 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_126 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_125);
v1 = TaggedVal::from(local_126);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_127 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_124);
v1 = TaggedVal::from(local_127);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_128 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_128);
v1 = TaggedVal::from(local_123);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_129 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_130 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_129);
v1 = TaggedVal::from(local_130);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_131 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_131);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_8;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_132 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_133 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_132);
v1 = TaggedVal::from(local_133);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_134 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_134);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_9: loop {
'label_10: loop {
v0 = TaggedVal::from(0i32);
local_135 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_136 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_136);
local_137 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_135);
local_138 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_137);
v1 = TaggedVal::from(local_138);
v0 = TaggedVal::from((v0.try_as_i32()? >= v1.try_as_i32()?) as i32);
local_139 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_140 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_139);
v1 = TaggedVal::from(local_140);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_141 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_141);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_9;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_142 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_143 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_142);
v1 = TaggedVal::from(local_143);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_144 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_144);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
'label_11: loop {
'label_12: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_145 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_146 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_145);
local_147 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_146);
local_148 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_147);
v1 = TaggedVal::from(local_148);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_149 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_150 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_149);
v1 = TaggedVal::from(local_150);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_151 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_151);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_11;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_152 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_153 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_154 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_153);
v1 = TaggedVal::from(local_154);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_155 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_152);
v1 = TaggedVal::from(local_155);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_156 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_156);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_157 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 28) as usize)?);
local_158 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_159 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_160 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_159);
v1 = TaggedVal::from(local_160);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_161 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_158);
v1 = TaggedVal::from(local_161);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_162 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_162);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_163 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_157);
local_164 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_163);
local_165 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_164);
v1 = TaggedVal::from(local_165);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_166 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_167 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_166);
v1 = TaggedVal::from(local_167);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_168 = v0.try_as_i32()?;
'label_13: loop {
v0 = TaggedVal::from(local_168);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_169 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_170 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_171 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_170);
v1 = TaggedVal::from(local_171);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_172 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_169);
v1 = TaggedVal::from(local_172);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_173 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_173);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_174 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_175 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_176 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_177 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_176);
v1 = TaggedVal::from(local_177);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_178 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_175);
v1 = TaggedVal::from(local_178);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_179 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_179);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_180 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_181 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_180);
v1 = TaggedVal::from(local_181);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_182 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_174);
local_183 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_182);
local_184 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_183);
v1 = TaggedVal::from(local_184);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_185 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_186 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_185);
v1 = TaggedVal::from(local_186);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_187 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_187);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_13;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_188 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_189 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_190 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_189);
v1 = TaggedVal::from(local_190);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_191 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_188);
v1 = TaggedVal::from(local_191);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_192 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_192);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_193 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_194 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_193);
v1 = TaggedVal::from(local_194);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_195 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_196 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_197 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_198 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_197);
v1 = TaggedVal::from(local_198);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_199 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_196);
v1 = TaggedVal::from(local_199);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_200 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_200);
v1 = TaggedVal::from(local_195);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 16) as usize)?);
local_201 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_202 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_201);
v1 = TaggedVal::from(local_202);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_203 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_203);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 16) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_12;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_204 = v0.try_as_i32()?;
v0 = TaggedVal::from(-1i32);
local_205 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_204);
v1 = TaggedVal::from(local_205);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_206 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_206);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_10;
break;}
break;
}
v0 = TaggedVal::from(1i32);
local_207 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_208 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_208);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_209 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_210 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_210);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_211 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_209);
v1 = TaggedVal::from(local_211);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_212 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_213 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_212);
v1 = TaggedVal::from(local_213);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_214 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_214);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_207);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
'label_14: loop {
'label_15: loop {
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_215 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 24) as usize)?);
local_216 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_215);
local_217 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_216);
local_218 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_217);
v1 = TaggedVal::from(local_218);
v0 = TaggedVal::from((v0.try_as_i32()? < v1.try_as_i32()?) as i32);
local_219 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_220 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_219);
v1 = TaggedVal::from(local_220);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_221 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_221);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_14;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_222 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_223 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_224 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_223);
v1 = TaggedVal::from(local_224);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_225 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_222);
v1 = TaggedVal::from(local_225);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_226 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_226);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_227 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_228 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_229 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_230 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_229);
v1 = TaggedVal::from(local_230);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_231 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_228);
v1 = TaggedVal::from(local_231);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_232 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_232);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_233 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_227);
v1 = TaggedVal::from(local_233);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_234 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_235 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_234);
v1 = TaggedVal::from(local_235);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_236 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_237 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_236);
local_238 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_237);
local_239 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_238);
v1 = TaggedVal::from(local_239);
v0 = TaggedVal::from((v0.try_as_i32()? > v1.try_as_i32()?) as i32);
local_240 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_241 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_240);
v1 = TaggedVal::from(local_241);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
local_242 = v0.try_as_i32()?;
'label_16: loop {
v0 = TaggedVal::from(local_242);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_16;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 12) as usize)?);
local_243 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_244 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_245 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_244);
v1 = TaggedVal::from(local_245);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_246 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_243);
v1 = TaggedVal::from(local_246);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_247 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_247);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_248 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_249 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_250 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_251 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_250);
v1 = TaggedVal::from(local_251);
v0 = TaggedVal::from(v0.try_as_i32()? << (v1.try_as_i32()? % 32));
local_252 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_249);
v1 = TaggedVal::from(local_252);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_253 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_253);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_254 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_248);
v1 = TaggedVal::from(local_254);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_255 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_256 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_255);
v1 = TaggedVal::from(local_256);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
local_257 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_257);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 4) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 20) as usize)?);
local_258 = v0.try_as_i32()?;
v0 = TaggedVal::from(1i32);
local_259 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_258);
v1 = TaggedVal::from(local_259);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_260 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_260);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 20) as usize, v1.try_as_i32()?)?;
{

}
continue 'label_15;
break;}
break;
}
v0 = TaggedVal::from(local_4);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 4) as usize)?);
local_261 = v0.try_as_i32()?;
v0 = TaggedVal::from(32i32);
local_262 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_4);
v1 = TaggedVal::from(local_262);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_263 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_263);
self.globals[0] = TaggedVal::from(v0.try_as_i32()?);
v0 = TaggedVal::from(local_261);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_7(&mut self, ) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_6(v0.try_as_i32()?, v1.try_as_i32()?)?);

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
v0 = TaggedVal::from(self.func_12(v0.try_as_i32()?)?);
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
v0 = TaggedVal::from(self.func_11()?);
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
fn func_9(&mut self, arg_0: i32) -> Option<i32> {
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
v0 = TaggedVal::from(self.func_8(v0.try_as_i32()?)?);
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
fn func_10(&mut self, arg_0: i32, arg_1: i32) -> Option<i32> {
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
fn func_11(&mut self, ) -> Option<i32> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;v0 = TaggedVal::from(1148i32);
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(2i32);
local_1 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_0);
v1 = TaggedVal::from(local_1);
v0 = TaggedVal::from(self.func_10(v0.try_as_i32()?, v1.try_as_i32()?)?);
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(local_2);
return Some(v0.try_as_i32()?);// no implicit return
}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_12(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(self.func_13(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_13(&mut self, arg_0: i32) -> Option<i32> {
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1176) as usize)?);
if v0.try_as_i32()? != 0 {
{

}
break 'label_0;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(self.func_15(v0.try_as_i32()?)?);
v1 = TaggedVal::from(67216i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1624) as usize)?);
local_4 = v0.try_as_i32()?;
if v0.try_as_i32()? != 0 {
{

}
break 'label_1;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1636) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1628) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1624) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1644) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1596) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_2);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1604) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(67216i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1600) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(67216i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1168) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1188) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1184) as usize, v1.try_as_i32()?)?;
'label_2: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1200i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1204i32);
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
v1 = TaggedVal::from(67224i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_sub(v1.try_as_i32()?));
v1 = TaggedVal::from(15i32);
v0 = TaggedVal::from(v0.try_as_i32()? & v1.try_as_i32()?);
v1 = TaggedVal::from(0i32);
v2 = TaggedVal::from(67224i32);
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
v1 = TaggedVal::from(67220i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1640) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1180) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(67216i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_2);
v1 = TaggedVal::from(67164i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1152) as usize)?);
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
v1 = TaggedVal::from(1200i32);
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
v2 = TaggedVal::from(1192i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
{

}
break 'label_16;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1168) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1160) as usize)?);
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
v1 = TaggedVal::from(1200i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 0) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 8) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_6);
v2 = TaggedVal::from(1192i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
{

}
break 'label_19;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1168) as usize)?);
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
v1 = TaggedVal::from(1192i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1172) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1172) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1160) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1156) as usize)?);
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
v1 = TaggedVal::from(1456i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1168) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1156) as usize)?);
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
v1 = TaggedVal::from(1456i32);
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
v1 = TaggedVal::from(1456i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1160) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1168) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1160) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) < (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_45;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1172) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1160) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1172) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1172) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1160) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1164) as usize)?);
local_6 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_48;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1176) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1624) as usize)?);
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_50;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1632) as usize)?);
local_4 = v0.try_as_i32()?;
{

}
break 'label_49;
break;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1636) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(281474976776192i64);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 1628) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(12i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
v2 = TaggedVal::from(-16i32);
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
v2 = TaggedVal::from(1431655768i32);
v1 = TaggedVal::from(v1.try_as_i32()? ^ v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1624) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1644) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1596) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
'label_52: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1592) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_52;
}
'label_53: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1584) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
{

}
break 'label_3;
break;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_u8(&self.memory, (v0.try_as_i32()? + 1596) as usize).and_then(|x| Some(x as i32))?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1176) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_56;
}
v0 = TaggedVal::from(1600i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1628) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1592) as usize)?);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_60;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1584) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1632) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1596) as usize)?);
v2 = TaggedVal::from(4i32);
v1 = TaggedVal::from(v1.try_as_i32()? | v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1596) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1584) as usize)?);
v2 = TaggedVal::from(local_5);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1584) as usize, v1.try_as_i32()?)?;
'label_64: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1588) as usize)?);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_64;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1588) as usize, v1.try_as_i32()?)?;
break;
}
'label_65: loop {
'label_66: loop {
'label_67: loop {
'label_68: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1176) as usize)?);
local_4 = v0.try_as_i32()?;
v0 = TaggedVal::from((v0.try_as_i32()? == 0) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_68;
}
v0 = TaggedVal::from(1600i32);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1168) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1168) as usize, v1.try_as_i32()?)?;
break;
}
v0 = TaggedVal::from(0i32);
local_3 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1604) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1600) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(-1i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1184) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1624) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1188) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1612) as usize, v1.try_as_i32()?)?;
'label_72: loop {
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1200i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(local_3);
v2 = TaggedVal::from(1192i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_4 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_3);
v1 = TaggedVal::from(1204i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1640) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1180) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1164) as usize)?);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1640) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1180) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1168) as usize)?);
local_8 = v1.try_as_i32()?;
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) >= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_73;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1168) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_6);
local_8 = v0.try_as_i32()?;
break;
}
v0 = TaggedVal::from(local_6);
v1 = TaggedVal::from(local_5);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
v0 = TaggedVal::from(1600i32);
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
v0 = TaggedVal::from(1600i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1164) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1172) as usize)?);
v1 = TaggedVal::from(local_6);
v0 = TaggedVal::from((v0.try_as_i32()? != v1.try_as_i32()?) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_85;
}
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1172) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1160) as usize)?);
v2 = TaggedVal::from(local_3);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
local_3 = v1.try_as_i32()?;
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1160) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1192i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1152) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_9);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1456i32);
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1156) as usize)?);
v2 = TaggedVal::from(-2i32);
v3 = TaggedVal::from(local_2);
v2 = TaggedVal::from(v2.try_as_i32()?.rotate_left(v3.try_as_i32()? as u32));
v1 = TaggedVal::from(v1.try_as_i32()? & v2.try_as_i32()?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1192i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_101: loop {
'label_102: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1152) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1456i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
'label_104: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1156) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(read_mem_i32(&self.memory, (v1.try_as_i32()? + 1640) as usize)?);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1180) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_3);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_11);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(16i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 1608) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 0) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(local_8);
v1 = TaggedVal::from(0i32);
v1 = TaggedVal::from(read_mem_i64(&self.memory, (v1.try_as_i32()? + 1600) as usize)?);
write_mem_i64(&mut self.memory, (v0.try_as_i32()? + 8) as usize, v1.try_as_i64()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_8);
v2 = TaggedVal::from(8i32);
v1 = TaggedVal::from(v1.try_as_i32()?.wrapping_add(v2.try_as_i32()?));
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1608) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_5);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1604) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_6);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1600) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(0i32);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1612) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1192i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_108: loop {
'label_109: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1152) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1456i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_0 = v0.try_as_i32()?;
'label_111: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1156) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1164) as usize)?);
local_3 = v0.try_as_i32()?;
v1 = TaggedVal::from(local_2);
v0 = TaggedVal::from(((v0.try_as_i32()? as u32) <= (v1.try_as_i32()? as u32)) as i32);
if v0.try_as_i32()? != 0 {
{

}
break 'label_6;
}
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1176) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1164) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_0);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1176) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1456i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1192i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_3 = v0.try_as_i32()?;
'label_120: loop {
'label_121: loop {
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1152) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1456i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v2 = TaggedVal::from(1456i32);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1156) as usize, v1.try_as_i32()?)?;
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
v1 = TaggedVal::from(1192i32);
v0 = TaggedVal::from(v0.try_as_i32()?.wrapping_add(v1.try_as_i32()?));
local_2 = v0.try_as_i32()?;
v0 = TaggedVal::from(0i32);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1172) as usize)?);
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1152) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1172) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(0i32);
v1 = TaggedVal::from(local_4);
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1160) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
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
fn func_16(&mut self, ) -> Option<()> {
Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_17(&mut self, ) -> Option<()> {
self.func_16()?;
self.func_23()?;Some(())}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_18(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
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
fn func_19(&mut self, arg_0: i32, arg_1: i64, arg_2: i32) -> Option<i64> {
let mut local_0 : i32 = arg_0;
let mut local_1 : i64 = arg_1;
let mut local_2 : i32 = arg_2;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v1 = TaggedVal::from(local_1);
v2 = TaggedVal::from(local_2);
v0 = TaggedVal::from(self.func_18(v0.try_as_i32()?, v1.try_as_i64()?, v2.try_as_i32()?)?);Some(v0.try_as_i64()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_20(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
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
fn func_21(&mut self, arg_0: i32, arg_1: i32, arg_2: i32) -> Option<i32> {
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
v1 = TaggedVal::from(self.func_20(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
v1 = TaggedVal::from(self.func_20(v1.try_as_i32()?, v2.try_as_i32()?, v3.try_as_i32()?)?);
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
fn func_22(&mut self, ) -> Option<i32> {
let mut v0: TaggedVal;v0 = TaggedVal::from(1660i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_23(&mut self, ) -> Option<()> {
let mut local_0 : i32 = 0i32;
let mut local_1 : i32 = 0i32;
let mut local_2 : i32 = 0i32;let mut v0: TaggedVal;
let mut v1: TaggedVal;
let mut v2: TaggedVal;
let mut v3: TaggedVal;'label_0: loop {
v0 = TaggedVal::from(self.func_22()?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1664) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1664) as usize)?);
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
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 1144) as usize)?);
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
fn func_24(&mut self, arg_0: i32) -> Option<i32> {
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
write_mem_i32(&mut self.memory, (v0.try_as_i32()? + 1648) as usize, v1.try_as_i32()?)?;
v0 = TaggedVal::from(-1i32);Some(v0.try_as_i32()?)}

#[allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_labels)]
fn func_25(&mut self, arg_0: i32) -> Option<i32> {
let mut local_0 : i32 = arg_0;let mut v0: TaggedVal;v0 = TaggedVal::from(local_0);
v0 = TaggedVal::from(read_mem_i32(&self.memory, (v0.try_as_i32()? + 56) as usize)?);
v0 = TaggedVal::from(self.func_24(v0.try_as_i32()?)?);Some(v0.try_as_i32()?)}

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
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_6(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
7 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_7()?;
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
                         let rets = self.func_9(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
10 => {
                         if args.len() != 2 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
                         let rets = self.func_10(a0, a1)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
11 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_11()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
12 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_12(a0)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
13 => {
                         if args.len() != 1 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
                         let rets = self.func_13(a0)?;
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
18 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_18(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
19 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i64()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_19(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
20 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_20(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
21 => {
                         if args.len() != 3 {
                             return None;
                         }
                         let a0 = args[0].try_as_i32()?;
let a1 = args[1].try_as_i32()?;
let a2 = args[2].try_as_i32()?;
                         let rets = self.func_21(a0, a1, a2)?;
                         Some(vec![TaggedVal::from(rets)])
                     }
22 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         let rets = self.func_22()?;
                         Some(vec![TaggedVal::from(rets)])
                     }
23 => {
                         if args.len() != 0 {
                             return None;
                         }
                         
                         self.func_23()?;
                         Some(vec![])
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