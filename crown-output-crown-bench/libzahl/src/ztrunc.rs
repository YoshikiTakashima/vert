use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor59 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ztrunc(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut bits: size_t,
) {
    let mut mask = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = 0;
    let mut i: size_t = 0;
    if zzero(b) != 0 {
        (*a).sign= 0 as libc::c_int;
        return;
    }
    chars= bits.wrapping_add((32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        >> 5 as libc::c_int;
    (*a).sign= (*b).sign;
    (*a).used= if chars < (*b).used { chars } else { (*b).used };
    if (*a).used < chars {
        bits= 0 as libc::c_int as size_t;
    }
    if a != b {
        if (*a).alloced < (*a).used {
            {let crown_promoted_local_0 = (*a).used;crate::src::allocator::libzahl_realloc(a.as_mut(), crown_promoted_local_0)};
        }
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*a).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    bits= bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    if bits != 0 {
        mask<<= bits;
        mask= (mask as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as zahl_char_t as zahl_char_t;
        *(*a).chars
            .offset(
                (*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) &= mask;
    }
    i= (*a).used;
    loop {
        let fresh1 = i;
        i= i.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if *(*a).chars.offset(i as isize) != 0 {
            return;
        }
    }
    (*a).sign= 0 as libc::c_int;
}
