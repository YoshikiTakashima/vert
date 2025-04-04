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

struct ErasedByPreprocessor2 { dummy: () }
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zand(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut current_block: u64;
    let mut n: size_t = 0;
    if zzero(b) != 0 || zzero(c) != 0 {
        (*a.as_deref_mut().unwrap()).sign= 0 as libc::c_int;
        return;
    }
    n= if (*b).used < (*c).used { (*b).used } else { (*c).used };
    loop {
        let fresh0 = n;
        n= n.wrapping_sub(1);
        if !(fresh0 != 0) {
            current_block= 2473556513754201174;
            break;
        }
        if *(*b).chars.offset(n as isize) & *(*c).chars.offset(n as isize) != 0 {
            current_block= 8876057388401302592;
            break;
        }
    }
    match current_block {
        2473556513754201174 => {
            (*a.as_deref_mut().unwrap()).sign= 0 as libc::c_int;
            return;
        }
        _ => {
            n= n.wrapping_add(1);
            (*a.as_deref_mut().unwrap()).used= n;
            if a.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()) == b {
                loop {
                    let fresh1 = n;
                    n= n.wrapping_sub(1);
                    if !(fresh1 != 0) {
                        break;
                    }
                    *(*a.as_deref().unwrap()).chars.offset(n as isize) &= *(*c).chars.offset(n as isize);
                }
            } else if a.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()) == c {
                loop {
                    let fresh3 = n;
                    n= n.wrapping_sub(1);
                    if !(fresh3 != 0) {
                        break;
                    }
                    *(*a.as_deref().unwrap()).chars.offset(n as isize) &= *(*b).chars.offset(n as isize);
                }
            } else {
                if (*a.as_deref().unwrap()).alloced < (*a.as_deref().unwrap()).used {
                    {let crown_promoted_local_0 = (*a.as_deref().unwrap()).used;crate::src::allocator::libzahl_realloc(a.as_deref_mut(), crown_promoted_local_0)};
                }
                memcpy(
                    (*a.as_deref().unwrap()).chars as *mut libc::c_void,
                    (*c).chars as *const libc::c_void,
                    (*a.as_deref().unwrap()).used
                        .wrapping_mul(
                            ::std::mem::size_of::<zahl_char_t>() as libc::c_ulong,
                        ),
                );
                loop {
                    let fresh5 = n;
                    n= n.wrapping_sub(1);
                    if !(fresh5 != 0) {
                        break;
                    }
                    *(*a.as_deref().unwrap()).chars.offset(n as isize) &= *(*b).chars.offset(n as isize);
                }
            }
            (*a.as_deref_mut().unwrap()).sign= (zsignum(b) > 0 as libc::c_int || zsignum(c) > 0 as libc::c_int)
                as libc::c_int * 2 as libc::c_int - 1 as libc::c_int;
            return;
        }
    };
}
