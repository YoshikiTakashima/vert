use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_s {
    pub payload: *mut libc::c_void,
    pub type_0: size_t,
}
impl Default for json_value_s {fn default() -> Self {Self {
payload: std::ptr::null_mut(),
type_0: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_result_s {
    pub error: size_t,
    pub error_offset: size_t,
    pub error_line_no: size_t,
    pub error_row_no: size_t,
}
impl Default for json_parse_result_s {fn default() -> Self {Self {
error: Default::default(),
error_offset: Default::default(),
error_line_no: Default::default(),
error_row_no: Default::default(),
}}}

pub type json_parse_flags_e = libc::c_uint;
pub const json_parse_flags_allow_json5: json_parse_flags_e = 16163;
pub const json_parse_flags_allow_simplified_json: json_parse_flags_e = 31;
pub const json_parse_flags_allow_multi_line_strings: json_parse_flags_e = 8192;
pub const json_parse_flags_allow_inf_and_nan: json_parse_flags_e = 4096;
pub const json_parse_flags_allow_leading_or_trailing_decimal_point: json_parse_flags_e = 2048;
pub const json_parse_flags_allow_leading_plus_sign: json_parse_flags_e = 1024;
pub const json_parse_flags_allow_hexadecimal_numbers: json_parse_flags_e = 512;
pub const json_parse_flags_allow_single_quoted_strings: json_parse_flags_e = 256;
pub const json_parse_flags_allow_location_information: json_parse_flags_e = 128;
pub const json_parse_flags_deprecated: json_parse_flags_e = 64;
pub const json_parse_flags_allow_c_style_comments: json_parse_flags_e = 32;
pub const json_parse_flags_allow_no_commas: json_parse_flags_e = 16;
pub const json_parse_flags_allow_equals_in_object: json_parse_flags_e = 8;
pub const json_parse_flags_allow_global_object: json_parse_flags_e = 4;
pub const json_parse_flags_allow_unquoted_keys: json_parse_flags_e = 2;
pub const json_parse_flags_allow_trailing_comma: json_parse_flags_e = 1;
pub const json_parse_flags_default: json_parse_flags_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_state_s {
    pub src: *const libc::c_char,
    pub size: size_t,
    pub offset: size_t,
    pub flags_bitset: size_t,
    pub data: *mut libc::c_char,
    pub dom: *mut libc::c_char,
    pub dom_size: size_t,
    pub data_size: size_t,
    pub line_no: size_t,
    pub line_offset: size_t,
    pub error: size_t,
}
impl Default for json_parse_state_s {fn default() -> Self {Self {
src: std::ptr::null(),
size: Default::default(),
offset: Default::default(),
flags_bitset: Default::default(),
data: std::ptr::null_mut(),
dom: std::ptr::null_mut(),
dom_size: Default::default(),
data_size: Default::default(),
line_no: Default::default(),
line_offset: Default::default(),
error: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_number_s {
    pub number: *const libc::c_char,
    pub number_size: size_t,
}
impl Default for json_number_s {fn default() -> Self {Self {
number: std::ptr::null(),
number_size: Default::default(),
}}}

pub const json_type_number: json_type_e = 1;
pub const json_type_null: json_type_e = 6;
pub const json_type_false: json_type_e = 5;
pub const json_type_true: json_type_e = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_s {
    pub start: *mut json_array_element_s,
    pub length: size_t,
}
impl Default for json_array_s {fn default() -> Self {Self {
start: std::ptr::null_mut(),
length: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_element_s {
    pub value: *mut json_value_s,
    pub next: *mut json_array_element_s,
}
impl Default for json_array_element_s {fn default() -> Self {Self {
value: std::ptr::null_mut(),
next: std::ptr::null_mut(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_ex_s {
    pub value: json_value_s,
    pub offset: size_t,
    pub line_no: size_t,
    pub row_no: size_t,
}
impl Default for json_value_ex_s {fn default() -> Self {Self {
value: Default::default(),
offset: Default::default(),
line_no: Default::default(),
row_no: Default::default(),
}}}

pub const json_parse_error_premature_end_of_buffer: json_parse_error_e = 7;
pub const json_type_array: json_type_e = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_s {
    pub start: *mut json_object_element_s,
    pub length: size_t,
}
impl Default for json_object_s {fn default() -> Self {Self {
start: std::ptr::null_mut(),
length: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_element_s {
    pub name: *mut json_string_s,
    pub value: *mut json_value_s,
    pub next: *mut json_object_element_s,
}
impl Default for json_object_element_s {fn default() -> Self {Self {
name: std::ptr::null_mut(),
value: std::ptr::null_mut(),
next: std::ptr::null_mut(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_s {
    pub string: *const libc::c_char,
    pub string_size: size_t,
}
impl Default for json_string_s {fn default() -> Self {Self {
string: std::ptr::null(),
string_size: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_ex_s {
    pub string: json_string_s,
    pub offset: size_t,
    pub line_no: size_t,
    pub row_no: size_t,
}
impl Default for json_string_ex_s {fn default() -> Self {Self {
string: Default::default(),
offset: Default::default(),
line_no: Default::default(),
row_no: Default::default(),
}}}

pub const json_type_object: json_type_e = 2;
pub const json_type_string: json_type_e = 0;
pub const json_parse_error_allocator_failed: json_parse_error_e = 9;
pub const json_parse_error_unexpected_trailing_characters: json_parse_error_e = 10;
pub const json_parse_error_invalid_value: json_parse_error_e = 6;
pub const json_parse_error_invalid_number_format: json_parse_error_e = 5;
pub const json_parse_error_expected_comma_or_closing_bracket: json_parse_error_e = 1;
pub const json_parse_error_unknown: json_parse_error_e = 11;
pub const json_parse_error_expected_colon: json_parse_error_e = 2;
pub const json_parse_error_invalid_string: json_parse_error_e = 8;
pub const json_parse_error_invalid_string_escape_sequence: json_parse_error_e = 4;
pub const json_parse_error_expected_opening_quote: json_parse_error_e = 3;
pub const json_parse_error_none: json_parse_error_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_state_s {
    pub dom: *mut libc::c_char,
    pub data: *mut libc::c_char,
}
impl Default for json_extract_state_s {fn default() -> Self {Self {
dom: std::ptr::null_mut(),
data: std::ptr::null_mut(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_result_s {
    pub dom_size: size_t,
    pub data_size: size_t,
}
impl Default for json_extract_result_s {fn default() -> Self {Self {
dom_size: Default::default(),
data_size: Default::default(),
}}}

pub type uintmax_t = __uintmax_t;
pub type __uintmax_t = libc::c_ulong;
pub type json_type_e = libc::c_uint;
pub type json_parse_error_e = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn json_hexadecimal_digit(mut c: libc::c_char) -> libc::c_int {
    if '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32;
    }
    if 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
    }
    if 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_hexadecimal_value(
    mut c: *const libc::c_char,
    mut size: libc::c_ulong,
    mut result: Option<&mut libc::c_ulong>,
) -> libc::c_int {
    let mut p = 0 as *const libc::c_char;
    let mut digit: libc::c_int = 0;
    if size
        > (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    *result.as_deref_mut().unwrap()= 0 as libc::c_int as libc::c_ulong;
    p= c;
    while (p.offset_from(c) as libc::c_long as libc::c_ulong) < size {
        *result.as_deref_mut().unwrap()<<= 4 as libc::c_int;
        digit= json_hexadecimal_digit((*p));
        if digit < 0 as libc::c_int || digit > 15 as libc::c_int {
            return 0 as libc::c_int;
        }
        *result.as_deref_mut().unwrap()|= digit as libc::c_uchar as libc::c_ulong;
        p= p.offset(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_whitespace(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    let mut offset = (*state.as_deref().unwrap()).offset;
    let size = (*state.as_deref().unwrap()).size;
    let src = (*state.as_deref().unwrap()).src;
    if offset >= (*state.as_deref().unwrap()).size {
        return 0 as libc::c_int;
    }
    match  *src.offset(offset as isize) as libc::c_int {
        32 | 13 | 9 | 10 => {}
        _ => return 0 as libc::c_int,
    }
    loop {
        match  *src.offset(offset as isize) as libc::c_int {
            32 | 13 | 9 => {}
            10 => {
                (*state.as_deref_mut().unwrap()).line_no= (*state.as_deref().unwrap()).line_no.wrapping_add(1);
                (*state.as_deref_mut().unwrap()).line_offset= offset;
            }
            _ => {
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
        }
        offset= offset.wrapping_add(1);
        if !(offset < size) {
            break;
        }
    }
    (*state.as_deref_mut().unwrap()).offset= offset;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_c_style_comments(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    if (*state.as_deref().unwrap()).offset.wrapping_add(2 as libc::c_int as libc::c_ulong) > (*state.as_deref().unwrap()).size
    {
        return 0 as libc::c_int;
    }
    if '/' as i32 == *(*state.as_deref().unwrap()).src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
        if '/' as i32
            == *(*state.as_deref().unwrap()).src
                .offset(
                    (*state.as_deref().unwrap()).offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int
        {
            (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
            (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
            while (*state.as_deref().unwrap()).offset < (*state.as_deref().unwrap()).size {
                match  *(*state.as_deref().unwrap()).src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                    10 => {
                        (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                        (*state.as_deref_mut().unwrap()).line_no= (*state.as_deref().unwrap()).line_no.wrapping_add(1);
                        (*state.as_deref_mut().unwrap()).line_offset= (*state.as_deref().unwrap()).offset;
                        return 1 as libc::c_int;
                    }
                    _ => {
                        (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                    }
                }
            }
            return 1 as libc::c_int;
        } else {
            if '*' as i32
                == *(*state.as_deref().unwrap()).src
                    .offset(
                        (*state.as_deref().unwrap()).offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int
            {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                while (*state.as_deref().unwrap()).offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    < (*state.as_deref().unwrap()).size
                {
                    if '*' as i32
                        == *(*state.as_deref().unwrap()).src.offset((*state.as_deref().unwrap()).offset as isize)
                            as libc::c_int
                        && '/' as i32
                            == *(*state.as_deref().unwrap()).src
                                .offset(
                                    (*state.as_deref().unwrap()).offset
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int
                    {
                        (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        return 1 as libc::c_int;
                    } else {
                        if '\n' as i32
                            == *(*state.as_deref().unwrap()).src.offset((*state.as_deref().unwrap()).offset as isize)
                                as libc::c_int
                        {
                            (*state.as_deref_mut().unwrap()).line_no= (*state.as_deref().unwrap()).line_no.wrapping_add(1);
                            (*state.as_deref_mut().unwrap()).line_offset= (*state.as_deref().unwrap()).offset;
                        }
                    }
                    (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                }
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_all_skippables(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    let mut did_consume = 0 as libc::c_int;
    let size = (*state.as_deref().unwrap()).size;
    if json_parse_flags_allow_c_style_comments as libc::c_int as libc::c_ulong
        & (*state.as_deref().unwrap()).flags_bitset != 0
    {
        loop {
            if (*state.as_deref().unwrap()).offset == size {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                return 1 as libc::c_int;
            }
            did_consume= json_skip_whitespace(state.as_deref_mut());
            if (*state.as_deref().unwrap()).offset >= size {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                return 1 as libc::c_int;
            }
            did_consume|= json_skip_c_style_comments(state.as_deref_mut());
            if !(0 as libc::c_int != did_consume) {
                break;
            }
        }
    } else {
        loop {
            if (*state.as_deref().unwrap()).offset == size {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                return 1 as libc::c_int;
            }
            did_consume= json_skip_whitespace(state.as_deref_mut());
            if !(0 as libc::c_int != did_consume) {
                break;
            }
        }
    }
    if (*state.as_deref().unwrap()).offset == size {
        (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_string_size(
    mut state: Option<&mut json_parse_state_s>,
    mut is_key: size_t,
) -> libc::c_int {
    let mut offset = (*state.as_deref().unwrap()).offset;
    let size = (*state.as_deref().unwrap()).size;
    let mut data_size = 0 as libc::c_int as size_t;
    let src = (*state.as_deref().unwrap()).src;
    let is_single_quote = ('\'' as i32 == *src.offset(offset as isize) as libc::c_int)
        as libc::c_int;
    let quote_to_use = (if is_single_quote != 0 { '\'' as i32 } else { '"' as i32 })
        as libc::c_char;
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let mut codepoint: libc::c_ulong = 0;
    let mut high_surrogate = 0 as libc::c_int as libc::c_ulong;
    if json_parse_flags_allow_location_information as libc::c_int as libc::c_ulong
        & flags_bitset != 0 as libc::c_int as libc::c_ulong
        && is_key != 0 as libc::c_int as libc::c_ulong
    {
        (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_string_ex_s>() as libc::c_ulong)
            as size_t as size_t;
    } else {
        (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_string_s>() as libc::c_ulong)
            as size_t as size_t;
    }
    if '"' as i32 != *src.offset(offset as isize) as libc::c_int {
        if !(json_parse_flags_allow_single_quoted_strings as libc::c_int as libc::c_ulong
            & flags_bitset != 0 && is_single_quote != 0)
        {
            (*state.as_deref_mut().unwrap()).error= json_parse_error_expected_opening_quote as libc::c_int
                as size_t;
            (*state.as_deref_mut().unwrap()).offset= offset;
            return 1 as libc::c_int;
        }
    }
    offset= offset.wrapping_add(1);
    while offset < size
        && quote_to_use as libc::c_int != *src.offset(offset as isize) as libc::c_int
    {
        data_size= data_size.wrapping_add(1);
        match  *src.offset(offset as isize) as libc::c_int {
            0 | 9 => {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string as libc::c_int as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
            _ => {}
        }
        if '\\' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset= offset.wrapping_add(1);
            if offset == size {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
            match  *src.offset(offset as isize) as libc::c_int {
                34 | 92 | 47 | 98 | 102 | 110 | 114 | 116 => {
                    offset= offset.wrapping_add(1);
                }
                117 => {
                    if !(offset.wrapping_add(5 as libc::c_int as libc::c_ulong) < size) {
                        (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                            as libc::c_int as size_t;
                        (*state.as_deref_mut().unwrap()).offset= offset;
                        return 1 as libc::c_int;
                    }
                    codepoint= 0 as libc::c_int as libc::c_ulong;
                    if json_hexadecimal_value(
                        &*src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ),
                        4 as libc::c_int as libc::c_ulong,
                        Some(&mut codepoint),
                    ) == 0
                    {
                        (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                            as libc::c_int as size_t;
                        (*state.as_deref_mut().unwrap()).offset= offset;
                        return 1 as libc::c_int;
                    }
                    if high_surrogate != 0 as libc::c_int as libc::c_ulong {
                        if codepoint >= 0xdc00 as libc::c_int as libc::c_ulong
                            && codepoint <= 0xdfff as libc::c_int as libc::c_ulong
                        {
                            data_size= (data_size as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            high_surrogate= 0 as libc::c_int as libc::c_ulong;
                        } else {
                            (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                                as libc::c_int as size_t;
                            (*state.as_deref_mut().unwrap()).offset= offset;
                            return 1 as libc::c_int;
                        }
                    } else if codepoint <= 0x7f as libc::c_int as libc::c_ulong {
                        data_size= (data_size as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    } else if codepoint <= 0x7ff as libc::c_int as libc::c_ulong {
                        data_size= (data_size as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    } else if codepoint >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdbff as libc::c_int as libc::c_ulong
                    {
                        if offset.wrapping_add(11 as libc::c_int as libc::c_ulong) > size
                            || '\\' as i32
                                != *src
                                    .offset(
                                        offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                            || 'u' as i32
                                != *src
                                    .offset(
                                        offset.wrapping_add(6 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                        {
                            (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                                as libc::c_int as size_t;
                            (*state.as_deref_mut().unwrap()).offset= offset;
                            return 1 as libc::c_int;
                        }
                        high_surrogate= codepoint;
                    } else if codepoint >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdfff as libc::c_int as libc::c_ulong
                    {
                        (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                            as libc::c_int as size_t;
                        (*state.as_deref_mut().unwrap()).offset= offset;
                        return 1 as libc::c_int;
                    } else {
                        data_size= (data_size as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                    offset= (offset as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                _ => {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                        as libc::c_int as size_t;
                    (*state.as_deref_mut().unwrap()).offset= offset;
                    return 1 as libc::c_int;
                }
            }
        } else if '\r' as i32 == *src.offset(offset as isize) as libc::c_int
            || '\n' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            if json_parse_flags_allow_multi_line_strings as libc::c_int as libc::c_ulong
                & flags_bitset == 0
            {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string_escape_sequence
                    as libc::c_int as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
            offset= offset.wrapping_add(1);
        } else {
            offset= offset.wrapping_add(1);
        }
    }
    if offset == size {
        (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        (*state.as_deref_mut().unwrap()).offset= offset.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return 1 as libc::c_int;
    }
    offset= offset.wrapping_add(1);
    (*state.as_deref_mut().unwrap()).data_size= ((*state.as_deref().unwrap()).data_size as libc::c_ulong).wrapping_add(data_size) as size_t as size_t;
    (*state.as_deref_mut().unwrap()).data_size= (*state.as_deref().unwrap()).data_size.wrapping_add(1);
    (*state.as_deref_mut().unwrap()).offset= offset;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_unquoted_key_char(mut c: libc::c_char) -> libc::c_int {
    return ('0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32
        || 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'z' as i32
        || 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'Z' as i32
        || '_' as i32 == c as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_key_size(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    if json_parse_flags_allow_unquoted_keys as libc::c_int as libc::c_ulong
        & flags_bitset != 0
    {
        let mut offset = (*state.as_deref().unwrap()).offset;
        let size = (*state.as_deref().unwrap()).size;
        let src = (*state.as_deref().unwrap()).src;
        let mut data_size = (*state.as_deref().unwrap()).data_size;
        if '"' as i32 == *src.offset(offset as isize) as libc::c_int {
            return json_get_string_size(state.as_deref_mut(), 1 as libc::c_int as size_t)
        } else if json_parse_flags_allow_single_quoted_strings as libc::c_int
            as libc::c_ulong & flags_bitset != 0
            && '\'' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            return json_get_string_size(state.as_deref_mut(), 1 as libc::c_int as size_t)
        } else {
            while offset < size
                && is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0
            {
                offset= offset.wrapping_add(1);
                data_size= data_size.wrapping_add(1);
            }
            data_size= data_size.wrapping_add(1);
            if json_parse_flags_allow_location_information as libc::c_int
                as libc::c_ulong & flags_bitset != 0
            {
                (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<json_string_ex_s>() as libc::c_ulong,
                    ) as size_t as size_t;
            } else {
                (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<json_string_s>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            (*state.as_deref_mut().unwrap()).offset= offset;
            (*state.as_deref_mut().unwrap()).data_size= data_size;
            return 0 as libc::c_int;
        }
    } else {
        return json_get_string_size(state.as_deref_mut(), 1 as libc::c_int as size_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_get_object_size(
    mut state: Option<&mut json_parse_state_s>,
    mut is_global_object: libc::c_int,
) -> libc::c_int {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let src = (*state.as_deref().unwrap()).src;
    let size = (*state.as_deref().unwrap()).size;
    let mut elements = 0 as libc::c_int as size_t;
    let mut allow_comma = 0 as libc::c_int;
    let mut found_closing_brace = 0 as libc::c_int;
    if is_global_object != 0 {
        if json_skip_all_skippables(state.as_deref_mut()) == 0
            && '{' as i32
                == *(*state.as_deref().unwrap()).src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int
        {
            is_global_object= 0 as libc::c_int;
        }
    }
    if is_global_object == 0 {
        if '{' as i32 != *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
            (*state.as_deref_mut().unwrap()).error= json_parse_error_unknown as libc::c_int as size_t;
            return 1 as libc::c_int;
        }
        (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
    }
    (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_object_s>() as libc::c_ulong) as size_t
        as size_t;
    if (*state.as_deref().unwrap()).offset == size && is_global_object == 0 {
        (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    let mut current_block_66: u64;
    loop {
        if is_global_object == 0 {
            if json_skip_all_skippables(state.as_deref_mut()) != 0 {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                return 1 as libc::c_int;
            }
            if '}' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                found_closing_brace= 1 as libc::c_int;
                break;
            }
        } else if json_skip_all_skippables(state.as_deref_mut()) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                allow_comma= 0 as libc::c_int;
            } else if json_parse_flags_allow_no_commas as libc::c_int as libc::c_ulong
                & flags_bitset != 0
            {
                allow_comma= 0 as libc::c_int;
            } else {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_expected_comma_or_closing_bracket
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            if json_parse_flags_allow_trailing_comma as libc::c_int as libc::c_ulong
                & flags_bitset != 0
            {
                current_block_66= 6057473163062296781;
            } else {
                if json_skip_all_skippables(state.as_deref_mut()) != 0 {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                current_block_66= 2122094917359643297;
            }
        } else {
            current_block_66= 2122094917359643297;
        }
        match current_block_66 {
            2122094917359643297 => {
                if json_get_key_size(state.as_deref_mut()) != 0 {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_string as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_skip_all_skippables(state.as_deref_mut()) != 0 {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_parse_flags_allow_equals_in_object as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                {
                    let current = *src.offset((*state.as_deref().unwrap()).offset as isize);
                    if ':' as i32 != current as libc::c_int
                        && '=' as i32 != current as libc::c_int
                    {
                        (*state.as_deref_mut().unwrap()).error= json_parse_error_expected_colon as libc::c_int
                            as size_t;
                        return 1 as libc::c_int;
                    }
                } else if ':' as i32
                    != *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int
                {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_expected_colon as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                if json_skip_all_skippables(state.as_deref_mut()) != 0 {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_get_value_size(state.as_deref_mut(), 0 as libc::c_int) != 0 {
                    return 1 as libc::c_int;
                }
                elements= elements.wrapping_add(1);
                allow_comma= 1 as libc::c_int;
            }
            _ => {}
        }
        if !((*state.as_deref().unwrap()).offset < size) {
            break;
        }
    }
    if (*state.as_deref().unwrap()).offset == size && is_global_object == 0 && found_closing_brace == 0 {
        (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as libc::c_ulong)
                .wrapping_mul(elements),
        ) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_array_size(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let mut elements = 0 as libc::c_int as size_t;
    let mut allow_comma = 0 as libc::c_int;
    let src = (*state.as_deref().unwrap()).src;
    let size = (*state.as_deref().unwrap()).size;
    if '[' as i32 != *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
        (*state.as_deref_mut().unwrap()).error= json_parse_error_unknown as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
    (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_array_s>() as libc::c_ulong) as size_t
        as size_t;
    while (*state.as_deref().unwrap()).offset < size {
        if json_skip_all_skippables(state.as_deref_mut()) != 0 {
            (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                as size_t;
            return 1 as libc::c_int;
        }
        if ']' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
            (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
            (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<json_array_element_s>() as libc::c_ulong)
                        .wrapping_mul(elements),
                ) as size_t as size_t;
            return 0 as libc::c_int;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                allow_comma= 0 as libc::c_int;
            } else if json_parse_flags_allow_no_commas as libc::c_int as libc::c_ulong
                & flags_bitset == 0
            {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_expected_comma_or_closing_bracket
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            if json_parse_flags_allow_trailing_comma as libc::c_int as libc::c_ulong
                & flags_bitset != 0
            {
                allow_comma= 0 as libc::c_int;
                continue;
            } else if json_skip_all_skippables(state.as_deref_mut()) != 0 {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                    as size_t;
                return 1 as libc::c_int;
            }
        }
        if json_get_value_size(state.as_deref_mut(), 0 as libc::c_int) != 0 {
            return 1 as libc::c_int;
        }
        elements= elements.wrapping_add(1);
        allow_comma= 1 as libc::c_int;
    }
    (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_number_size(
    mut state: Option<&mut json_parse_state_s>,
) -> libc::c_int {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let mut offset = (*state.as_deref().unwrap()).offset;
    let size = (*state.as_deref().unwrap()).size;
    let mut had_leading_digits = 0 as libc::c_int;
    let src = (*state.as_deref().unwrap()).src;
    (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_number_s>() as libc::c_ulong) as size_t
        as size_t;
    if json_parse_flags_allow_hexadecimal_numbers as libc::c_int as libc::c_ulong
        & flags_bitset != 0
        && offset.wrapping_add(1 as libc::c_int as libc::c_ulong) < size
        && '0' as i32 == *src.offset(offset as isize) as libc::c_int
        && ('x' as i32
            == *src
                .offset(offset.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
            || 'X' as i32
                == *src
                    .offset(
                        offset.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int)
    {
        offset= (offset as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32
                || 'a' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= 'f' as i32
                || 'A' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= 'F' as i32)
        {
            offset= offset.wrapping_add(1);
        }
    } else {
        let mut found_sign = 0 as libc::c_int;
        let mut inf_or_nan = 0 as libc::c_int;
        if offset < size
            && ('-' as i32 == *src.offset(offset as isize) as libc::c_int
                || json_parse_flags_allow_leading_plus_sign as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                    && '+' as i32 == *src.offset(offset as isize) as libc::c_int)
        {
            offset= offset.wrapping_add(1);
            found_sign= 1 as libc::c_int;
        }
        if json_parse_flags_allow_inf_and_nan as libc::c_int as libc::c_ulong
            & flags_bitset != 0
        {
            let inf: [libc::c_char; 9] = *::std::mem::transmute::<
                &[u8; 9],
                &[libc::c_char; 9],
            >(b"Infinity\0");
            let inf_strlen = (::std::mem::size_of::<[libc::c_char; 9]>()
                as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let nan: [libc::c_char; 4] = *::std::mem::transmute::<
                &[u8; 4],
                &[libc::c_char; 4],
            >(b"NaN\0");
            let nan_strlen = (::std::mem::size_of::<[libc::c_char; 4]>()
                as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if offset.wrapping_add(inf_strlen) < size {
                let mut found = 1 as libc::c_int;
                let mut i: size_t = 0;
                i= 0 as libc::c_int as size_t;
                while i < inf_strlen {
                    if inf[i as usize] as libc::c_int
                        != *src.offset(offset.wrapping_add(i) as isize) as libc::c_int
                    {
                        found= 0 as libc::c_int;
                        break;
                    } else {
                        i= i.wrapping_add(1);
                    }
                }
                if found != 0 {
                    offset= (offset as libc::c_ulong).wrapping_add(inf_strlen) as size_t
                        as size_t;
                    inf_or_nan= 1 as libc::c_int;
                }
            }
            if offset.wrapping_add(nan_strlen) < size {
                let mut found_0 = 1 as libc::c_int;
                let mut i_0: size_t = 0;
                i_0= 0 as libc::c_int as size_t;
                while i_0 < nan_strlen {
                    if nan[i_0 as usize] as libc::c_int
                        != *src.offset(offset.wrapping_add(i_0) as isize) as libc::c_int
                    {
                        found_0= 0 as libc::c_int;
                        break;
                    } else {
                        i_0= i_0.wrapping_add(1);
                    }
                }
                if found_0 != 0 {
                    offset= (offset as libc::c_ulong).wrapping_add(nan_strlen) as size_t
                        as size_t;
                    inf_or_nan= 1 as libc::c_int;
                }
            }
            if inf_or_nan != 0 {
                if offset < size {
                    match  *src.offset(offset as isize) as libc::c_int {
                        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 101 | 69 => {
                            (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format
                                as libc::c_int as size_t;
                            (*state.as_deref_mut().unwrap()).offset= offset;
                            return 1 as libc::c_int;
                        }
                        _ => {}
                    }
                }
            }
        }
        if found_sign != 0 && inf_or_nan == 0 && offset < size
            && !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
        {
            if json_parse_flags_allow_leading_or_trailing_decimal_point as libc::c_int
                as libc::c_ulong & flags_bitset == 0
                || '.' as i32 != *src.offset(offset as isize) as libc::c_int
            {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                    as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
        }
        if offset < size && '0' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset= offset.wrapping_add(1);
            had_leading_digits= 1 as libc::c_int;
            if offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
            {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                    as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
        }
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
        {
            offset= offset.wrapping_add(1);
            had_leading_digits= 1 as libc::c_int;
        }
        if offset < size && '.' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset= offset.wrapping_add(1);
            if offset >= size
                || !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
            {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as libc::c_int as libc::c_ulong & flags_bitset == 0
                    || had_leading_digits == 0
                {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                        as size_t;
                    (*state.as_deref_mut().unwrap()).offset= offset;
                    return 1 as libc::c_int;
                }
            }
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
            {
                offset= offset.wrapping_add(1);
            }
        }
        if offset < size
            && ('e' as i32 == *src.offset(offset as isize) as libc::c_int
                || 'E' as i32 == *src.offset(offset as isize) as libc::c_int)
        {
            offset= offset.wrapping_add(1);
            if offset < size
                && ('-' as i32 == *src.offset(offset as isize) as libc::c_int
                    || '+' as i32 == *src.offset(offset as isize) as libc::c_int)
            {
                offset= offset.wrapping_add(1);
            }
            if offset < size
                && !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
            {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                    as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
            loop {
                offset= offset.wrapping_add(1);
                if !(offset < size
                    && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int <= '9' as i32))
                {
                    break;
                }
            }
        }
    }
    if offset < size {
        match  *src.offset(offset as isize) as libc::c_int {
            32 | 9 | 13 | 10 | 125 | 44 | 93 => {}
            61 => {
                if !(json_parse_flags_allow_equals_in_object as libc::c_int
                    as libc::c_ulong & flags_bitset != 0)
                {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                        as size_t;
                    (*state.as_deref_mut().unwrap()).offset= offset;
                    return 1 as libc::c_int;
                }
            }
            _ => {
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                    as size_t;
                (*state.as_deref_mut().unwrap()).offset= offset;
                return 1 as libc::c_int;
            }
        }
    }
    (*state.as_deref_mut().unwrap()).data_size= ((*state.as_deref().unwrap()).data_size as libc::c_ulong)
        .wrapping_add(offset.wrapping_sub((*state.as_deref().unwrap()).offset)) as size_t as size_t;
    (*state.as_deref_mut().unwrap()).data_size= (*state.as_deref().unwrap()).data_size.wrapping_add(1);
    (*state.as_deref_mut().unwrap()).offset= offset;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_value_size(
    mut state: Option<&mut json_parse_state_s>,
    mut is_global_object: libc::c_int,
) -> libc::c_int {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let src = (*state.as_deref().unwrap()).src;
    let mut offset: size_t = 0;
    let size = (*state.as_deref().unwrap()).size;
    if json_parse_flags_allow_location_information as libc::c_int as libc::c_ulong
        & flags_bitset != 0
    {
        (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_value_ex_s>() as libc::c_ulong)
            as size_t as size_t;
    } else {
        (*state.as_deref_mut().unwrap()).dom_size= ((*state.as_deref().unwrap()).dom_size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_value_s>() as libc::c_ulong)
            as size_t as size_t;
    }
    if is_global_object != 0 {
        return json_get_object_size(state.as_deref_mut(), 1 as libc::c_int)
    } else {
        if json_skip_all_skippables(state.as_deref_mut()) != 0 {
            (*state.as_deref_mut().unwrap()).error= json_parse_error_premature_end_of_buffer as libc::c_int
                as size_t;
            return 1 as libc::c_int;
        }
        offset= (*state.as_deref().unwrap()).offset;
        match  *src.offset(offset as isize) as libc::c_int {
            34 => return json_get_string_size(state.as_deref_mut(), 0 as libc::c_int as size_t),
            39 => {
                if json_parse_flags_allow_single_quoted_strings as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                {
                    return json_get_string_size(state.as_deref_mut(), 0 as libc::c_int as size_t)
                } else {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_value as libc::c_int as size_t;
                    return 1 as libc::c_int;
                }
            }
            123 => return json_get_object_size(state.as_deref_mut(), 0 as libc::c_int),
            91 => return json_get_array_size(state.as_deref_mut()),
            45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return json_get_number_size(state.as_deref_mut());
            }
            43 => {
                if json_parse_flags_allow_leading_plus_sign as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                {
                    return json_get_number_size(state.as_deref_mut())
                } else {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
            }
            46 => {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as libc::c_int as libc::c_ulong & flags_bitset != 0
                {
                    return json_get_number_size(state.as_deref_mut())
                } else {
                    (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_number_format as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
            }
            _ => {
                if offset.wrapping_add(4 as libc::c_int as libc::c_ulong) <= size
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'r' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    return 0 as libc::c_int;
                } else {
                    if offset.wrapping_add(5 as libc::c_int as libc::c_ulong) <= size
                        && 'f' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                        && 'a' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                        && 'l' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                        && 's' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                        && 'e' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                    {
                        (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        return 0 as libc::c_int;
                    } else {
                        if offset.wrapping_add(4 as libc::c_int as libc::c_ulong) <= size
                            && 'n' as i32
                                == *(*state.as_deref().unwrap()).src
                                    .offset(
                                        offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                            && 'u' as i32
                                == *(*state.as_deref().unwrap()).src
                                    .offset(
                                        offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                            && 'l' as i32
                                == *(*state.as_deref().unwrap()).src
                                    .offset(
                                        offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                            && 'l' as i32
                                == *(*state.as_deref().unwrap()).src
                                    .offset(
                                        offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                        {
                            (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            return 0 as libc::c_int;
                        } else {
                            if json_parse_flags_allow_inf_and_nan as libc::c_int
                                as libc::c_ulong & flags_bitset != 0
                                && offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    <= size
                                && 'N' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ) as libc::c_int
                                && 'a' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ) as libc::c_int
                                && 'N' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ) as libc::c_int
                            {
                                return json_get_number_size(state.as_deref_mut())
                            } else {
                                if json_parse_flags_allow_inf_and_nan as libc::c_int
                                    as libc::c_ulong & flags_bitset != 0
                                    && offset.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        <= size
                                    && 'I' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'n' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'f' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'i' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'n' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'i' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 't' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                    && 'y' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(7 as libc::c_int as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                {
                                    return json_get_number_size(state.as_deref_mut());
                                }
                            }
                        }
                    }
                }
                (*state.as_deref_mut().unwrap()).error= json_parse_error_invalid_value as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_string(
    mut state: Option<&mut json_parse_state_s>,
    mut string: *mut json_string_s,
) {
    let mut offset = (*state.as_deref().unwrap()).offset;
    let mut bytes_written = 0 as libc::c_int as size_t;
    let src = (*state.as_deref().unwrap()).src;
    let quote_to_use = (if '\'' as i32 == *src.offset(offset as isize) as libc::c_int {
        '\'' as i32
    } else {
        '"' as i32
    }) as libc::c_char;
    let mut data = (*state.as_deref().unwrap()).data;
    let mut high_surrogate = 0 as libc::c_int as libc::c_ulong;
    let mut codepoint: libc::c_ulong = 0;
    (*string).string= data;
    offset= offset.wrapping_add(1);
    while quote_to_use as libc::c_int != *src.offset(offset as isize) as libc::c_int {
        if '\\' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset= offset.wrapping_add(1);
            let fresh37 = offset;
            offset= offset.wrapping_add(1);
            match  *src.offset(fresh37 as isize) as libc::c_int {
                117 => {
                    codepoint= 0 as libc::c_int as libc::c_ulong;
                    if json_hexadecimal_value(
                        &*src.offset(offset as isize),
                        4 as libc::c_int as libc::c_ulong,
                        Some(&mut codepoint),
                    ) == 0
                    {
                        return;
                    }
                    offset= (offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    if codepoint <= 0x7f as libc::c_uint as libc::c_ulong {
                        let fresh38 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data.offset(fresh38 as isize) = codepoint as libc::c_char;
                    } else if codepoint <= 0x7ff as libc::c_uint as libc::c_ulong {
                        let fresh39 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh39 as isize,
                            ) = (0xc0 as libc::c_uint as libc::c_ulong
                            | codepoint >> 6 as libc::c_int) as libc::c_char;
                        let fresh40 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh40 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    } else if codepoint >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdbff as libc::c_int as libc::c_ulong
                    {
                        high_surrogate= codepoint;
                    } else if codepoint >= 0xdc00 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdfff as libc::c_int as libc::c_ulong
                    {
                        let surrogate_offset = (0x10000 as libc::c_uint)
                            .wrapping_sub((0xd800 as libc::c_uint) << 10 as libc::c_int)
                            .wrapping_sub(0xdc00 as libc::c_uint) as libc::c_ulong;
                        codepoint= (high_surrogate << 10 as libc::c_int)
                            .wrapping_add(codepoint)
                            .wrapping_add(surrogate_offset);
                        high_surrogate= 0 as libc::c_int as libc::c_ulong;
                        let fresh41 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh41 as isize,
                            ) = (0xf0 as libc::c_uint as libc::c_ulong
                            | codepoint >> 18 as libc::c_int) as libc::c_char;
                        let fresh42 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh42 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint >> 12 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong) as libc::c_char;
                        let fresh43 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh43 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint >> 6 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong) as libc::c_char;
                        let fresh44 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh44 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    } else {
                        let fresh45 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh45 as isize,
                            ) = (0xe0 as libc::c_uint as libc::c_ulong
                            | codepoint >> 12 as libc::c_int) as libc::c_char;
                        let fresh46 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh46 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint >> 6 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong) as libc::c_char;
                        let fresh47 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh47 as isize,
                            ) = (0x80 as libc::c_uint as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    }
                }
                34 => {
                    let fresh48 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh48 as isize) = '"' as i32 as libc::c_char;
                }
                92 => {
                    let fresh49 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh49 as isize) = '\\' as i32 as libc::c_char;
                }
                47 => {
                    let fresh50 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh50 as isize) = '/' as i32 as libc::c_char;
                }
                98 => {
                    let fresh51 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh51 as isize) = '\u{8}' as i32 as libc::c_char;
                }
                102 => {
                    let fresh52 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh52 as isize) = '\u{c}' as i32 as libc::c_char;
                }
                110 => {
                    let fresh53 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh53 as isize) = '\n' as i32 as libc::c_char;
                }
                114 => {
                    let fresh54 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh54 as isize) = '\r' as i32 as libc::c_char;
                }
                116 => {
                    let fresh55 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh55 as isize) = '\t' as i32 as libc::c_char;
                }
                13 => {
                    let fresh56 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh56 as isize) = '\r' as i32 as libc::c_char;
                    if '\n' as i32 == *src.offset(offset as isize) as libc::c_int {
                        let fresh57 = bytes_written;
                        bytes_written= bytes_written.wrapping_add(1);
                        *data.offset(fresh57 as isize) = '\n' as i32 as libc::c_char;
                        offset= offset.wrapping_add(1);
                    }
                }
                10 => {
                    let fresh58 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh58 as isize) = '\n' as i32 as libc::c_char;
                }
                _ => return,
            }
        } else {
            let fresh59 = offset;
            offset= offset.wrapping_add(1);
            let fresh60 = bytes_written;
            bytes_written= bytes_written.wrapping_add(1);
            *data.offset(fresh60 as isize) = *src.offset(fresh59 as isize);
        }
    }
    offset= offset.wrapping_add(1);
    (*string).string_size= bytes_written;
    let fresh61 = bytes_written;
    bytes_written= bytes_written.wrapping_add(1);
    *data.offset(fresh61 as isize) = '\0' as i32 as libc::c_char;
    (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data.offset(bytes_written as isize);
    (*state.as_deref_mut().unwrap()).offset= offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_key(
    mut state: Option<&mut json_parse_state_s>,
    mut string: *mut json_string_s,
) {
    if json_parse_flags_allow_unquoted_keys as libc::c_int as libc::c_ulong
        & (*state.as_deref().unwrap()).flags_bitset != 0
    {
        let src = (*state.as_deref().unwrap()).src;
        let data = (*state.as_deref().unwrap()).data;
        let mut offset = (*state.as_deref().unwrap()).offset;
        if '"' as i32 == *src.offset(offset as isize) as libc::c_int
            || '\'' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            json_parse_string(state.as_deref_mut(), string);
        } else {
            let mut size = 0 as libc::c_int as size_t;
            (*string).string= (*state.as_deref().unwrap()).data;
            while is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0 {
                let fresh64 = offset;
                offset= offset.wrapping_add(1);
                let fresh65 = size;
                size= size.wrapping_add(1);
                *data.offset(fresh65 as isize) = *src.offset(fresh64 as isize);
            }
            *data.offset(size as isize) = '\0' as i32 as libc::c_char;
            let fresh66 = size;
            size= size.wrapping_add(1);
            (*string).string_size= fresh66;
            (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data.offset(size as isize);
            (*state.as_deref_mut().unwrap()).offset= offset;
        }
    } else {
        json_parse_string(state.as_deref_mut(), string);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_object(
    mut state: Option<&mut json_parse_state_s>,
    mut is_global_object: libc::c_int,
    mut object: *mut json_object_s,
) {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let size = (*state.as_deref().unwrap()).size;
    let src = (*state.as_deref().unwrap()).src;
    let mut elements = 0 as libc::c_int as size_t;
    let mut allow_comma = 0 as libc::c_int;
    let mut previous = 0 as *mut json_object_element_s;
    if is_global_object != 0 {
        if '{' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
            is_global_object= 0 as libc::c_int;
        }
    }
    if is_global_object == 0 {
        (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
    }
    json_skip_all_skippables(state.as_deref_mut());
    elements= 0 as libc::c_int as size_t;
    while (*state.as_deref().unwrap()).offset < size {
        let mut element = 0 as *mut json_object_element_s;
        let mut string = 0 as *mut json_string_s;
        let mut value = 0 as *mut json_value_s;
        if is_global_object == 0 {
            json_skip_all_skippables(state.as_deref_mut());
            if '}' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                break;
            }
        } else if json_skip_all_skippables(state.as_deref_mut()) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                allow_comma= 0 as libc::c_int;
                continue;
            }
        }
        element= (*state.as_deref().unwrap()).dom as *mut json_object_element_s;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(
                ::std::mem::size_of::<json_object_element_s>() as libc::c_ulong as isize,
            );
        if previous.is_null() {();
            (*object).start= element;
        } else {
            (*previous).next= element;
        }
        previous= element;
        if json_parse_flags_allow_location_information as libc::c_int as libc::c_ulong
            & flags_bitset != 0
        {
            let mut string_ex = (*state.as_deref().unwrap()).dom as *mut json_string_ex_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_string_ex_s>() as libc::c_ulong as isize,
                );
            (*string_ex).offset= (*state.as_deref().unwrap()).offset;
            (*string_ex).line_no= (*state.as_deref().unwrap()).line_no;
            (*string_ex).row_no= (*state.as_deref().unwrap()).offset.wrapping_sub((*state.as_deref().unwrap()).line_offset);
            string= core::ptr::addr_of_mut!((*string_ex).string);
        } else {
            string= (*state.as_deref().unwrap()).dom as *mut json_string_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_string_s>() as libc::c_ulong as isize,
                );
        }
        (*element).name= string;
        json_parse_key(state.as_deref_mut(), string);
        json_skip_all_skippables(state.as_deref_mut());
        (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
        json_skip_all_skippables(state.as_deref_mut());
        if json_parse_flags_allow_location_information as libc::c_int as libc::c_ulong
            & flags_bitset != 0
        {
            let mut value_ex = (*state.as_deref().unwrap()).dom as *mut json_value_ex_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_value_ex_s>() as libc::c_ulong as isize,
                );
            (*value_ex).offset= (*state.as_deref().unwrap()).offset;
            (*value_ex).line_no= (*state.as_deref().unwrap()).line_no;
            (*value_ex).row_no= (*state.as_deref().unwrap()).offset.wrapping_sub((*state.as_deref().unwrap()).line_offset);
            value= core::ptr::addr_of_mut!((*value_ex).value);
        } else {
            value= (*state.as_deref().unwrap()).dom as *mut json_value_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(::std::mem::size_of::<json_value_s>() as libc::c_ulong as isize);
        }
        (*element).value= value;
        json_parse_value(state.as_deref_mut(), 0 as libc::c_int, value);
        elements= elements.wrapping_add(1);
        allow_comma= 1 as libc::c_int;
    }
    if !previous.is_null() {
        (*previous).next= 0 as *mut json_object_element_s;
    }else { (); }
    if 0 as libc::c_int as libc::c_ulong == elements {
        (*object).start= 0 as *mut json_object_element_s;
    }
    (*object).length= elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_array(
    mut state: Option<&mut json_parse_state_s>,
    mut array: *mut json_array_s,
) {
    let src = (*state.as_deref().unwrap()).src;
    let size = (*state.as_deref().unwrap()).size;
    let mut elements = 0 as libc::c_int as size_t;
    let mut allow_comma = 0 as libc::c_int;
    let mut previous = 0 as *mut json_array_element_s;
    (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
    json_skip_all_skippables(state.as_deref_mut());
    elements= 0 as libc::c_int as size_t;
    let mut current_block_28: u64;
    loop {
        let mut element = 0 as *mut json_array_element_s;
        let mut value = 0 as *mut json_value_s;
        json_skip_all_skippables(state.as_deref_mut());
        if ']' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
            (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
            break;
        } else {
            if allow_comma != 0 {
                if ',' as i32 == *src.offset((*state.as_deref().unwrap()).offset as isize) as libc::c_int {
                    (*state.as_deref_mut().unwrap()).offset= (*state.as_deref().unwrap()).offset.wrapping_add(1);
                    allow_comma= 0 as libc::c_int;
                    current_block_28= 6873731126896040597;
                } else {
                    current_block_28= 13056961889198038528;
                }
            } else {
                current_block_28= 13056961889198038528;
            }
            match current_block_28 {
                13056961889198038528 => {
                    element= (*state.as_deref().unwrap()).dom as *mut json_array_element_s;
                    (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                        .offset(
                            ::std::mem::size_of::<json_array_element_s>()
                                as libc::c_ulong as isize,
                        );
                    if previous.is_null() {();
                        (*array).start= element;
                    } else {
                        (*previous).next= element;
                    }
                    previous= element;
                    if json_parse_flags_allow_location_information as libc::c_int
                        as libc::c_ulong & (*state.as_deref().unwrap()).flags_bitset != 0
                    {
                        let mut value_ex = (*state.as_deref().unwrap()).dom as *mut json_value_ex_s;
                        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                            .offset(
                                ::std::mem::size_of::<json_value_ex_s>() as libc::c_ulong
                                    as isize,
                            );
                        (*value_ex).offset= (*state.as_deref().unwrap()).offset;
                        (*value_ex).line_no= (*state.as_deref().unwrap()).line_no;
                        (*value_ex).row_no= (*state.as_deref().unwrap()).offset
                            .wrapping_sub((*state.as_deref().unwrap()).line_offset);
                        value= core::ptr::addr_of_mut!((*value_ex).value);
                    } else {
                        value= (*state.as_deref().unwrap()).dom as *mut json_value_s;
                        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                            .offset(
                                ::std::mem::size_of::<json_value_s>() as libc::c_ulong
                                    as isize,
                            );
                    }
                    (*element).value= value;
                    json_parse_value(state.as_deref_mut(), 0 as libc::c_int, value);
                    elements= elements.wrapping_add(1);
                    allow_comma= 1 as libc::c_int;
                }
                _ => {}
            }
            if !((*state.as_deref().unwrap()).offset < size) {
                break;
            }
        }
    }
    if !previous.is_null() {
        (*previous).next= 0 as *mut json_array_element_s;
    }else { (); }
    if 0 as libc::c_int as libc::c_ulong == elements {
        (*array).start= 0 as *mut json_array_element_s;
    }
    (*array).length= elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_number(
    mut state: Option<&mut json_parse_state_s>,
    mut number: *mut json_number_s,
) {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let mut offset = (*state.as_deref().unwrap()).offset;
    let size = (*state.as_deref().unwrap()).size;
    let mut bytes_written = 0 as libc::c_int as size_t;
    let src = (*state.as_deref().unwrap()).src;
    let mut data = (*state.as_deref().unwrap()).data;
    (*number).number= data;
    if json_parse_flags_allow_hexadecimal_numbers as libc::c_int as libc::c_ulong
        & flags_bitset != 0
    {
        if '0' as i32 == *src.offset(offset as isize) as libc::c_int
            && ('x' as i32
                == *src
                    .offset(
                        offset.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int
                || 'X' as i32
                    == *src
                        .offset(
                            offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int)
        {
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int <= '9' as i32
                    || 'a' as i32 <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int <= 'f' as i32
                    || 'A' as i32 <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int <= 'F' as i32
                    || 'x' as i32 == *src.offset(offset as isize) as libc::c_int
                    || 'X' as i32 == *src.offset(offset as isize) as libc::c_int)
            {
                let fresh95 = offset;
                offset= offset.wrapping_add(1);
                let fresh96 = bytes_written;
                bytes_written= bytes_written.wrapping_add(1);
                *data.offset(fresh96 as isize) = *src.offset(fresh95 as isize);
            }
        }
    }
    while offset < size {
        let mut end = 0 as libc::c_int;
        match  *src.offset(offset as isize) as libc::c_int {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 | 101 | 69 | 43
            | 45 => {
                let fresh97 = offset;
                offset= offset.wrapping_add(1);
                let fresh98 = bytes_written;
                bytes_written= bytes_written.wrapping_add(1);
                *data.offset(fresh98 as isize) = *src.offset(fresh97 as isize);
            }
            _ => {
                end= 1 as libc::c_int;
            }
        }
        if 0 as libc::c_int != end {
            break;
        }
    }
    if json_parse_flags_allow_inf_and_nan as libc::c_int as libc::c_ulong & flags_bitset
        != 0
    {
        let inf_strlen = 8 as libc::c_int as size_t;
        let nan_strlen = 3 as libc::c_int as size_t;
        if offset.wrapping_add(inf_strlen) < size {
            if 'I' as i32 == *src.offset(offset as isize) as libc::c_int {
                let mut i: size_t = 0;
                i= 0 as libc::c_int as size_t;
                while i < inf_strlen {
                    let fresh99 = offset;
                    offset= offset.wrapping_add(1);
                    let fresh100 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh100 as isize) = *src.offset(fresh99 as isize);
                    i= i.wrapping_add(1);
                }
            }
        }
        if offset.wrapping_add(nan_strlen) < size {
            if 'N' as i32 == *src.offset(offset as isize) as libc::c_int {
                let mut i_0: size_t = 0;
                i_0= 0 as libc::c_int as size_t;
                while i_0 < nan_strlen {
                    let fresh101 = offset;
                    offset= offset.wrapping_add(1);
                    let fresh102 = bytes_written;
                    bytes_written= bytes_written.wrapping_add(1);
                    *data.offset(fresh102 as isize) = *src.offset(fresh101 as isize);
                    i_0= i_0.wrapping_add(1);
                }
            }
        }
    }
    (*number).number_size= bytes_written;
    let fresh103 = bytes_written;
    bytes_written= bytes_written.wrapping_add(1);
    *data.offset(fresh103 as isize) = '\0' as i32 as libc::c_char;
    (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data.offset(bytes_written as isize);
    (*state.as_deref_mut().unwrap()).offset= offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_value(
    mut state: Option<&mut json_parse_state_s>,
    mut is_global_object: libc::c_int,
    mut value: *mut json_value_s,
) {
    let flags_bitset = (*state.as_deref().unwrap()).flags_bitset;
    let src = (*state.as_deref().unwrap()).src;
    let size = (*state.as_deref().unwrap()).size;
    let mut offset: size_t = 0;
    json_skip_all_skippables(state.as_deref_mut());
    offset= (*state.as_deref().unwrap()).offset;
    if is_global_object != 0 {
        (*value).type_0= json_type_object as libc::c_int as size_t;
        (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(::std::mem::size_of::<json_object_s>() as libc::c_ulong as isize);
        json_parse_object(
            state.as_deref_mut(),
            1 as libc::c_int,
            (*value).payload as *mut json_object_s,
        );
    } else {
        match  *src.offset(offset as isize) as libc::c_int {
            34 | 39 => {
                (*value).type_0= json_type_string as libc::c_int as size_t;
                (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                    .offset(
                        ::std::mem::size_of::<json_string_s>() as libc::c_ulong as isize,
                    );
                json_parse_string(state.as_deref_mut(), (*value).payload as *mut json_string_s);
            }
            123 => {
                (*value).type_0= json_type_object as libc::c_int as size_t;
                (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                    .offset(
                        ::std::mem::size_of::<json_object_s>() as libc::c_ulong as isize,
                    );
                json_parse_object(
                    state.as_deref_mut(),
                    0 as libc::c_int,
                    (*value).payload as *mut json_object_s,
                );
            }
            91 => {
                (*value).type_0= json_type_array as libc::c_int as size_t;
                (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                    .offset(
                        ::std::mem::size_of::<json_array_s>() as libc::c_ulong as isize,
                    );
                json_parse_array(state.as_deref_mut(), (*value).payload as *mut json_array_s);
            }
            45 | 43 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 => {
                (*value).type_0= json_type_number as libc::c_int as size_t;
                (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                    .offset(
                        ::std::mem::size_of::<json_number_s>() as libc::c_ulong as isize,
                    );
                json_parse_number(state.as_deref_mut(), (*value).payload as *mut json_number_s);
            }
            _ => {
                if offset.wrapping_add(4 as libc::c_int as libc::c_ulong) <= size
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'r' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*value).type_0= json_type_true as libc::c_int as size_t;
                    (*value).payload= 0 as *mut libc::c_void;
                    (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else if offset.wrapping_add(5 as libc::c_int as libc::c_ulong) <= size
                    && 'f' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'a' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 's' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*value).type_0= json_type_false as libc::c_int as size_t;
                    (*value).payload= 0 as *mut libc::c_void;
                    (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else if offset.wrapping_add(4 as libc::c_int as libc::c_ulong) <= size
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*value).type_0= json_type_null as libc::c_int as size_t;
                    (*value).payload= 0 as *mut libc::c_void;
                    (*state.as_deref_mut().unwrap()).offset= ((*state.as_deref().unwrap()).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                    && offset.wrapping_add(3 as libc::c_int as libc::c_ulong) <= size
                    && 'N' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'a' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'N' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*value).type_0= json_type_number as libc::c_int as size_t;
                    (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                    (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                        .offset(
                            ::std::mem::size_of::<json_number_s>() as libc::c_ulong
                                as isize,
                        );
                    json_parse_number(state.as_deref_mut(), (*value).payload as *mut json_number_s);
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong & flags_bitset != 0
                    && offset.wrapping_add(8 as libc::c_int as libc::c_ulong) <= size
                    && 'I' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'f' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'i' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'i' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(6 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                    && 'y' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int
                {
                    (*value).type_0= json_type_number as libc::c_int as size_t;
                    (*value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
                    (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                        .offset(
                            ::std::mem::size_of::<json_number_s>() as libc::c_ulong
                                as isize,
                        );
                    json_parse_number(state.as_deref_mut(), (*value).payload as *mut json_number_s);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_ex(
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut flags_bitset: size_t,
    mut alloc_func_ptr: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut user_data: *mut libc::c_void,
    mut result: Option<&mut json_parse_result_s>,
) -> *mut /* owning */ json_value_s {
    let mut state = json_parse_state_s {
        src: 0 as *const libc::c_char,
        size: 0,
        offset: 0,
        flags_bitset: 0,
        data: 0 as *mut libc::c_char,
        dom: 0 as *mut libc::c_char,
        dom_size: 0,
        data_size: 0,
        line_no: 0,
        line_offset: 0,
        error: 0,
    };
    let mut allocation = 0 as *mut libc::c_void;
    let mut value = 0 as *mut json_value_s;
    let mut total_size: size_t = 0;
    let mut input_error: libc::c_int = 0;
    if !result.as_deref().is_none() {
        (*result.as_deref_mut().unwrap()).error= json_parse_error_none as libc::c_int as size_t;
        (*result.as_deref_mut().unwrap()).error_offset= 0 as libc::c_int as size_t;
        (*result.as_deref_mut().unwrap()).error_line_no= 0 as libc::c_int as size_t;
        (*result.as_deref_mut().unwrap()).error_row_no= 0 as libc::c_int as size_t;
    }else { (); }
    if src.is_null() {();
        return 0 as *mut json_value_s;
    }
    state.src= src as *const libc::c_char;
    state.size= src_size;
    state.offset= 0 as libc::c_int as size_t;
    state.line_no= 1 as libc::c_int as size_t;
    state.line_offset= 0 as libc::c_int as size_t;
    state.error= json_parse_error_none as libc::c_int as size_t;
    state.dom_size= 0 as libc::c_int as size_t;
    state.data_size= 0 as libc::c_int as size_t;
    state.flags_bitset= flags_bitset;
    input_error= {let crown_promoted_local_0 = (json_parse_flags_allow_global_object as libc::c_int as libc::c_ulong
            & state.flags_bitset) as libc::c_int;json_get_value_size(
        Some(&mut state),
        crown_promoted_local_0,
    )};
    if 0 as libc::c_int == input_error {
        json_skip_all_skippables(Some(&mut state));
        if state.offset != state.size {
            state.error= json_parse_error_unexpected_trailing_characters as libc::c_int
                as size_t;
            input_error= 1 as libc::c_int;
        }
    }
    if input_error != 0 {
        if !result.as_deref().is_none() {
            (*result.as_deref_mut().unwrap()).error= state.error;
            (*result.as_deref_mut().unwrap()).error_offset= state.offset;
            (*result.as_deref_mut().unwrap()).error_line_no= state.line_no;
            (*result.as_deref_mut().unwrap()).error_row_no= state.offset.wrapping_sub(state.line_offset);
        }else { (); }
        return 0 as *mut json_value_s;
    }
    total_size= state.dom_size.wrapping_add(state.data_size);
    if alloc_func_ptr.is_none() {
        allocation= malloc(total_size);
    } else {
        allocation= alloc_func_ptr
            .expect("non-null function pointer")(user_data, total_size);
    }
    if allocation.is_null() {();
        if !result.as_deref().is_none() {
            (*result.as_deref_mut().unwrap()).error= json_parse_error_allocator_failed as libc::c_int as size_t;
            (*result.as_deref_mut().unwrap()).error_offset= 0 as libc::c_int as size_t;
            (*result.as_deref_mut().unwrap()).error_line_no= 0 as libc::c_int as size_t;
            (*result.as_deref_mut().unwrap()).error_row_no= 0 as libc::c_int as size_t;
        }else { (); }
        return 0 as *mut json_value_s;
    }
    state.offset= 0 as libc::c_int as size_t;
    state.line_no= 1 as libc::c_int as size_t;
    state.line_offset= 0 as libc::c_int as size_t;
    state.dom= allocation as *mut libc::c_char;
    state.data= state.dom.offset(state.dom_size as isize);
    if json_parse_flags_allow_location_information as libc::c_int as libc::c_ulong
        & state.flags_bitset != 0
    {
        let mut value_ex = state.dom as *mut json_value_ex_s;
        state.dom= state.dom
            .offset(::std::mem::size_of::<json_value_ex_s>() as libc::c_ulong as isize);
        (*value_ex).offset= state.offset;
        (*value_ex).line_no= state.line_no;
        (*value_ex).row_no= state.offset.wrapping_sub(state.line_offset);
        value= core::ptr::addr_of_mut!((*value_ex).value);
    } else {
        value= state.dom as *mut json_value_s;
        state.dom= state.dom
            .offset(::std::mem::size_of::<json_value_s>() as libc::c_ulong as isize);
    }
    {let crown_promoted_local_1 = (json_parse_flags_allow_global_object as libc::c_int as libc::c_ulong
            & state.flags_bitset) as libc::c_int;json_parse_value(
        Some(&mut state),
        crown_promoted_local_1,
        value,
    )};
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse(
    mut src: *const libc::c_void,
    mut src_size: size_t,
) -> Option<Box<json_value_s>> {
    return Some(Box::from_raw(json_parse_ex(
        src,
        src_size,
        json_parse_flags_default as libc::c_int as size_t,
        None,
        0 as *mut libc::c_void,
        None,
    )));
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value(
    mut value: *const json_value_s,
) -> Option<Box<json_value_s>> {
    return Some(Box::from_raw(json_extract_value_ex(value, None, 0 as *mut libc::c_void)));
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_number_size(
    mut number: *const json_number_s,
) -> json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    result.dom_size= ::std::mem::size_of::<json_number_s>() as libc::c_ulong;
    result.data_size= (*number).number_size;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_string_size(
    mut string: *const json_string_s,
) -> json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    result.dom_size= ::std::mem::size_of::<json_string_s>() as libc::c_ulong;
    result.data_size= (*string).string_size
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_object_size(
    mut object: *const json_object_s,
) -> json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut i: size_t = 0;
    let mut element: *const json_object_element_s = (*object).start;
    result.dom_size= (::std::mem::size_of::<json_object_s>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as libc::c_ulong)
                .wrapping_mul((*object).length),
        );
    result.data_size= 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < (*object).length {
        let string_result = json_extract_get_string_size((*element).name);
        let value_result = json_extract_get_value_size((*element).value);
        result.dom_size= (result.dom_size as libc::c_ulong)
            .wrapping_add(string_result.dom_size) as size_t as size_t;
        result.data_size= (result.data_size as libc::c_ulong)
            .wrapping_add(string_result.data_size) as size_t as size_t;
        result.dom_size= (result.dom_size as libc::c_ulong)
            .wrapping_add(value_result.dom_size) as size_t as size_t;
        result.data_size= (result.data_size as libc::c_ulong)
            .wrapping_add(value_result.data_size) as size_t as size_t;
        element= (*element).next;
        i= i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_array_size(
    mut array: *const json_array_s,
) -> json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut i: size_t = 0;
    let mut element: *const json_array_element_s = (*array).start;
    result.dom_size= (::std::mem::size_of::<json_array_s>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_array_element_s>() as libc::c_ulong)
                .wrapping_mul((*array).length),
        );
    result.data_size= 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < (*array).length {
        let value_result = json_extract_get_value_size((*element).value);
        result.dom_size= (result.dom_size as libc::c_ulong)
            .wrapping_add(value_result.dom_size) as size_t as size_t;
        result.data_size= (result.data_size as libc::c_ulong)
            .wrapping_add(value_result.data_size) as size_t as size_t;
        element= (*element).next;
        i= i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_value_size(
    mut value: *const json_value_s,
) -> json_extract_result_s {
    let mut result = {
        let mut init = json_extract_result_s {
            dom_size: 0 as libc::c_int as size_t,
            data_size: 0 as libc::c_int as size_t,
        };
        init
    };
    match (*value).type_0 {
        2 => {
            result= json_extract_get_object_size(
                (*value).payload as *const json_object_s,
            );
        }
        3 => {
            result= json_extract_get_array_size(
                (*value).payload as *const json_array_s,
            );
        }
        1 => {
            result= json_extract_get_number_size(
                (*value).payload as *const json_number_s,
            );
        }
        0 => {
            result= json_extract_get_string_size(
                (*value).payload as *const json_string_s,
            );
        }
        _ => {}
    }
    result.dom_size= (result.dom_size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_value_s>() as libc::c_ulong) as size_t
        as size_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_copy_value(
    mut state: Option<&mut json_extract_state_s>,
    mut value: *const json_value_s,
) {
    let mut string = 0 as *mut json_string_s;
    let mut number = 0 as *mut json_number_s;
    let mut object = 0 as *mut json_object_s;
    let mut array = 0 as *mut json_array_s;
    let mut new_value = 0 as *mut json_value_s;
    memcpy(
        (*state.as_deref().unwrap()).dom as *mut libc::c_void,
        value as *const libc::c_void,
        ::std::mem::size_of::<json_value_s>() as libc::c_ulong,
    );
    new_value= (*state.as_deref().unwrap()).dom as *mut json_value_s;
    (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
        .offset(::std::mem::size_of::<json_value_s>() as libc::c_ulong as isize);
    (*new_value).payload= (*state.as_deref().unwrap()).dom as *mut libc::c_void;
    if json_type_string as libc::c_int as libc::c_ulong == (*value).type_0 {
        memcpy(
            (*state.as_deref().unwrap()).dom as *mut libc::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_string_s>() as libc::c_ulong,
        );
        string= (*state.as_deref().unwrap()).dom as *mut json_string_s;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(::std::mem::size_of::<json_string_s>() as libc::c_ulong as isize);
        memcpy(
            (*state.as_deref().unwrap()).data as *mut libc::c_void,
            (*string).string as *const libc::c_void,
            (*string).string_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        (*string).string= (*state.as_deref().unwrap()).data;
        (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data
            .offset(
                (*string).string_size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
    } else if json_type_number as libc::c_int as libc::c_ulong == (*value).type_0 {
        memcpy(
            (*state.as_deref().unwrap()).dom as *mut libc::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_number_s>() as libc::c_ulong,
        );
        number= (*state.as_deref().unwrap()).dom as *mut json_number_s;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(::std::mem::size_of::<json_number_s>() as libc::c_ulong as isize);
        memcpy(
            (*state.as_deref().unwrap()).data as *mut libc::c_void,
            (*number).number as *const libc::c_void,
            (*number).number_size,
        );
        (*number).number= (*state.as_deref().unwrap()).data;
        (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data.offset((*number).number_size as isize);
    } else if json_type_object as libc::c_int as libc::c_ulong == (*value).type_0 {
        let mut element = 0 as *mut json_object_element_s;
        let mut i: size_t = 0;
        memcpy(
            (*state.as_deref().unwrap()).dom as *mut libc::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_object_s>() as libc::c_ulong,
        );
        object= (*state.as_deref().unwrap()).dom as *mut json_object_s;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(::std::mem::size_of::<json_object_s>() as libc::c_ulong as isize);
        element= (*object).start;
        (*object).start= (*state.as_deref().unwrap()).dom as *mut json_object_element_s;
        i= 0 as libc::c_int as size_t;
        while i < (*object).length {
            let mut previous_value = 0 as *mut json_value_s;
            let mut previous_element = 0 as *mut json_object_element_s;
            memcpy(
                (*state.as_deref().unwrap()).dom as *mut libc::c_void,
                element as *const libc::c_void,
                ::std::mem::size_of::<json_object_element_s>() as libc::c_ulong,
            );
            element= (*state.as_deref().unwrap()).dom as *mut json_object_element_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_object_element_s>() as libc::c_ulong
                        as isize,
                );
            string= (*element).name;
            memcpy(
                (*state.as_deref().unwrap()).dom as *mut libc::c_void,
                string as *const libc::c_void,
                ::std::mem::size_of::<json_string_s>() as libc::c_ulong,
            );
            string= (*state.as_deref().unwrap()).dom as *mut json_string_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_string_s>() as libc::c_ulong as isize,
                );
            (*element).name= string;
            memcpy(
                (*state.as_deref().unwrap()).data as *mut libc::c_void,
                (*string).string as *const libc::c_void,
                (*string).string_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            (*string).string= (*state.as_deref().unwrap()).data;
            (*state.as_deref_mut().unwrap()).data= (*state.as_deref().unwrap()).data
                .offset(
                    (*string).string_size
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            previous_value= (*element).value;
            (*element).value= (*state.as_deref().unwrap()).dom as *mut json_value_s;
            json_extract_copy_value(state.as_deref_mut(), previous_value);
            previous_element= element;
            element= (*element).next;
            if !element.is_null() {
                (*previous_element).next= (*state.as_deref().unwrap()).dom as *mut json_object_element_s;
            }else { (); }
            i= i.wrapping_add(1);
        }
    } else if json_type_array as libc::c_int as libc::c_ulong == (*value).type_0 {
        let mut element_0 = 0 as *mut json_array_element_s;
        let mut i_0: size_t = 0;
        memcpy(
            (*state.as_deref().unwrap()).dom as *mut libc::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_array_s>() as libc::c_ulong,
        );
        array= (*state.as_deref().unwrap()).dom as *mut json_array_s;
        (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
            .offset(::std::mem::size_of::<json_array_s>() as libc::c_ulong as isize);
        element_0= (*array).start;
        (*array).start= (*state.as_deref().unwrap()).dom as *mut json_array_element_s;
        i_0= 0 as libc::c_int as size_t;
        while i_0 < (*array).length {
            let mut previous_value_0 = 0 as *mut json_value_s;
            let mut previous_element_0 = 0 as *mut json_array_element_s;
            memcpy(
                (*state.as_deref().unwrap()).dom as *mut libc::c_void,
                element_0 as *const libc::c_void,
                ::std::mem::size_of::<json_array_element_s>() as libc::c_ulong,
            );
            element_0= (*state.as_deref().unwrap()).dom as *mut json_array_element_s;
            (*state.as_deref_mut().unwrap()).dom= (*state.as_deref().unwrap()).dom
                .offset(
                    ::std::mem::size_of::<json_array_element_s>() as libc::c_ulong
                        as isize,
                );
            previous_value_0= (*element_0).value;
            (*element_0).value= (*state.as_deref().unwrap()).dom as *mut json_value_s;
            json_extract_copy_value(state.as_deref_mut(), previous_value_0);
            previous_element_0= element_0;
            element_0= (*element_0).next;
            if !element_0.is_null() {
                (*previous_element_0).next= (*state.as_deref().unwrap()).dom as *mut json_array_element_s;
            }else { (); }
            i_0= i_0.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value_ex(
    mut value: *const json_value_s,
    mut alloc_func_ptr: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut user_data: *mut libc::c_void,
) -> *mut /* owning */ json_value_s {
    let mut allocation = 0 as *mut libc::c_void;
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut state = json_extract_state_s {
        dom: 0 as *mut libc::c_char,
        data: 0 as *mut libc::c_char,
    };
    let mut total_size: size_t = 0;
    if value.is_null() {();
        return 0 as *mut json_value_s;
    }
    result= json_extract_get_value_size(value);
    total_size= result.dom_size.wrapping_add(result.data_size);
    if alloc_func_ptr.is_none() {
        allocation= malloc(total_size);
    } else {
        allocation= alloc_func_ptr
            .expect("non-null function pointer")(user_data, total_size);
    }
    state.dom= allocation as *mut libc::c_char;
    state.data= state.dom.offset(result.dom_size as isize);
    json_extract_copy_value(Some(&mut state), value);
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_string(
    mut value: *mut json_value_s,
) -> *mut json_string_s {
    if (*value).type_0 != json_type_string as libc::c_int as libc::c_ulong {
        return 0 as *mut json_string_s;
    }
    return (*value).payload as *mut json_string_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_number(
    mut value: *mut json_value_s,
) -> *mut json_number_s {
    if (*value).type_0 != json_type_number as libc::c_int as libc::c_ulong {
        return 0 as *mut json_number_s;
    }
    return (*value).payload as *mut json_number_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_object(
    mut value: *mut json_value_s,
) -> *mut json_object_s {
    if (*value).type_0 != json_type_object as libc::c_int as libc::c_ulong {
        return 0 as *mut json_object_s;
    }
    return (*value).payload as *mut json_object_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_array(
    mut value: *mut json_value_s,
) -> *mut json_array_s {
    if (*value).type_0 != json_type_array as libc::c_int as libc::c_ulong {
        return 0 as *mut json_array_s;
    }
    return (*value).payload as *mut json_array_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_true(mut value: *const json_value_s) -> libc::c_int {
    return ((*value).type_0 == json_type_true as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_false(mut value: *const json_value_s) -> libc::c_int {
    return ((*value).type_0 == json_type_false as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_null(mut value: *const json_value_s) -> libc::c_int {
    return ((*value).type_0 == json_type_null as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_number_size(
    mut number: *const json_number_s,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut parsed_number: uintmax_t = 0;
    let mut i: size_t = 0;
    if (*number).number_size >= 2 as libc::c_int as libc::c_ulong {
        match  *(*number).number.offset(1 as libc::c_int as isize) as libc::c_int {
            120 | 88 => {
                parsed_number= strtoumax(
                    (*number).number,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                i= 0 as libc::c_int as size_t;
                while 0 as libc::c_int as libc::c_ulong != parsed_number {
                    parsed_number= (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                        as uintmax_t;
                    i= i.wrapping_add(1);
                }
                *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(i) as size_t as size_t;
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    i= 0 as libc::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *(*number).number.offset(i as isize) as libc::c_int
            || '-' as i32 == *(*number).number.offset(i as isize) as libc::c_int)
    {
        i= i.wrapping_add(1);
    }
    if i < (*number).number_size
        && 'I' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        let mut inf = b"Infinity\0" as *const u8 as *const libc::c_char;
        let mut k: size_t = 0;
        k= i;
        while k < (*number).number_size {
            let fresh147 = inf;
            inf= inf.offset(1);
            let c = (*fresh147);
            if '\0' as i32 == c as libc::c_int {
                break;
            }
            if c as libc::c_int != *(*number).number.offset(k as isize) as libc::c_int
            {
                break;
            }
            k= k.wrapping_add(1);
        }
        if '\0' as i32 == (*inf) as libc::c_int {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(22 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if '-' as i32
                == *(*number).number.offset(0 as libc::c_int as isize) as libc::c_int
            {
                *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        return 0 as libc::c_int;
    }
    if i < (*number).number_size
        && 'N' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        let mut nan = b"NaN\0" as *const u8 as *const libc::c_char;
        let mut k_0: size_t = 0;
        k_0= i;
        while k_0 < (*number).number_size {
            let fresh148 = nan;
            nan= nan.offset(1);
            let c_0 = (*fresh148);
            if '\0' as i32 == c_0 as libc::c_int {
                break;
            }
            if c_0 as libc::c_int
                != *(*number).number.offset(k_0 as isize) as libc::c_int
            {
                break;
            }
            k_0= k_0.wrapping_add(1);
        }
        if '\0' as i32 == (*nan) as libc::c_int {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else {
        while i < (*number).number_size {
            let c_1 = *(*number).number.offset(i as isize);
            if !('0' as i32 <= c_1 as libc::c_int && c_1 as libc::c_int <= '9' as i32) {
                break;
            }
            i= i.wrapping_add(1);
        }
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == (*number).number_size
            && '.' as i32 == *(*number).number.offset(i as isize) as libc::c_int
        {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    }
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add((*number).number_size) as size_t
        as size_t;
    if '+' as i32 == *(*number).number.offset(0 as libc::c_int as isize) as libc::c_int
    {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_string_size(
    mut string: *const json_string_s,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < (*string).string_size {
        match  *(*string).string.offset(i as isize) as libc::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            _ => {
                *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        i= i.wrapping_add(1);
    }
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_array_size(
    mut array: *const json_array_s,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut element = 0 as *mut json_array_element_s;
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if (1 as libc::c_int as libc::c_ulong) < (*array).length {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
            .wrapping_add(
                (*array).length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    element= (*array).start;
    while !element.is_null() {
        if json_write_minified_get_value_size((*element).value, size.as_deref_mut()) != 0 {
            return 1 as libc::c_int;
        }
        element= (*element).next;
    }();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_object_size(
    mut object: *const json_object_s,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut element = 0 as *mut json_object_element_s;
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add((*object).length) as size_t as size_t;
    if (1 as libc::c_int as libc::c_ulong) < (*object).length {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
            .wrapping_add(
                (*object).length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    element= (*object).start;
    while !element.is_null() {
        if json_write_get_string_size((*element).name, size.as_deref_mut()) != 0 {
            return 1 as libc::c_int;
        }
        if json_write_minified_get_value_size((*element).value, size.as_deref_mut()) != 0 {
            return 1 as libc::c_int;
        }
        element= (*element).next;
    }();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_value_size(
    mut value: *const json_value_s,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size.as_deref_mut(),
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size.as_deref_mut(),
            );
        }
        3 => {
            return json_write_minified_get_array_size(
                (*value).payload as *mut json_array_s,
                size.as_deref_mut(),
            );
        }
        2 => {
            return json_write_minified_get_object_size(
                (*value).payload as *mut json_object_s,
                size.as_deref_mut(),
            );
        }
        4 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        5 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        6 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_number(
    mut number: *const json_number_s,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut parsed_number: uintmax_t = 0;
    let mut backup: uintmax_t = 0;
    let mut i: size_t = 0;
    if (*number).number_size >= 2 as libc::c_int as libc::c_ulong {
        match  *(*number).number.offset(1 as libc::c_int as isize) as libc::c_int {
            120 | 88 => {
                parsed_number= strtoumax(
                    (*number).number,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                backup= parsed_number;
                i= 0 as libc::c_int as size_t;
                while 0 as libc::c_int as libc::c_ulong != parsed_number {
                    parsed_number= (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                        as uintmax_t;
                    i= i.wrapping_add(1);
                }
                parsed_number= backup;
                backup= i;
                loop {
                    *data
                        .offset(i as isize)
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = ('0' as i32
                        + parsed_number.wrapping_rem(10 as libc::c_int as libc::c_ulong)
                            as libc::c_char as libc::c_int) as libc::c_char;
                    parsed_number= (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                        as uintmax_t;
                    i= i.wrapping_sub(1);
                    if !(0 as libc::c_int as libc::c_ulong != parsed_number) {
                        break;
                    }
                }
                data= data.offset(backup as isize);
                return data;
            }
            _ => {}
        }
    }
    i= 0 as libc::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *(*number).number.offset(i as isize) as libc::c_int
            || '-' as i32 == *(*number).number.offset(i as isize) as libc::c_int)
    {
        i= i.wrapping_add(1);
    }
    if i < (*number).number_size
        && 'I' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        let mut inf = b"Infinity\0" as *const u8 as *const libc::c_char;
        let mut k: size_t = 0;
        k= i;
        while k < (*number).number_size {
            let fresh149 = inf;
            inf= inf.offset(1);
            let c = (*fresh149);
            if '\0' as i32 == c as libc::c_int {
                break;
            }
            if c as libc::c_int != *(*number).number.offset(k as isize) as libc::c_int
            {
                break;
            }
            k= k.wrapping_add(1);
        }
        let fresh150 = inf;
        inf= inf.offset(1);
        if '\0' as i32 == (*fresh150) as libc::c_int {
            let mut dbl_max = 0 as *const libc::c_char;
            if '-' as i32
                == *(*number).number.offset(0 as libc::c_int as isize) as libc::c_int
            {
                let fresh151 = data;
                data= data.offset(1);
                *fresh151= '-' as i32 as libc::c_char;
            }
            dbl_max= b"1.7976931348623158e308\0" as *const u8 as *const libc::c_char;
            while '\0' as i32 != (*dbl_max) as libc::c_int {
                let fresh152 = data;
                data= data.offset(1);
                *fresh152= (*dbl_max);
                dbl_max= dbl_max.offset(1);
            }
            return data;
        }
    }
    if i < (*number).number_size
        && 'N' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        let mut nan = b"NaN\0" as *const u8 as *const libc::c_char;
        let mut k_0: size_t = 0;
        k_0= i;
        while k_0 < (*number).number_size {
            let fresh153 = nan;
            nan= nan.offset(1);
            let c_0 = (*fresh153);
            if '\0' as i32 == c_0 as libc::c_int {
                break;
            }
            if c_0 as libc::c_int
                != *(*number).number.offset(k_0 as isize) as libc::c_int
            {
                break;
            }
            k_0= k_0.wrapping_add(1);
        }
        let fresh154 = nan;
        nan= nan.offset(1);
        if '\0' as i32 == (*fresh154) as libc::c_int {
            let fresh155 = data;
            data= data.offset(1);
            *fresh155= '0' as i32 as libc::c_char;
            return data;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        i= 0 as libc::c_int as size_t;
        if '+' as i32 == *(*number).number.offset(i as isize) as libc::c_int {
            i= i.wrapping_add(1);
        }
        if '-' as i32 == *(*number).number.offset(i as isize) as libc::c_int {
            let fresh156 = data;
            data= data.offset(1);
            *fresh156= '-' as i32 as libc::c_char;
            i= i.wrapping_add(1);
        }
        let fresh157 = data;
        data= data.offset(1);
        *fresh157= '0' as i32 as libc::c_char;
        while i < (*number).number_size {
            let fresh158 = data;
            data= data.offset(1);
            *fresh158= *(*number).number.offset(i as isize);
            i= i.wrapping_add(1);
        }
        return data;
    }
    while i < (*number).number_size {
        let c_1 = *(*number).number.offset(i as isize);
        if !('0' as i32 <= c_1 as libc::c_int && c_1 as libc::c_int <= '9' as i32) {
            break;
        }
        i= i.wrapping_add(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == (*number).number_size
        && '.' as i32 == *(*number).number.offset(i as isize) as libc::c_int
    {
        i= 0 as libc::c_int as size_t;
        if '+' as i32 == *(*number).number.offset(i as isize) as libc::c_int {
            i= i.wrapping_add(1);
        }
        if '-' as i32 == *(*number).number.offset(i as isize) as libc::c_int {
            let fresh159 = data;
            data= data.offset(1);
            *fresh159= '-' as i32 as libc::c_char;
            i= i.wrapping_add(1);
        }
        while i < (*number).number_size {
            let fresh160 = data;
            data= data.offset(1);
            *fresh160= *(*number).number.offset(i as isize);
            i= i.wrapping_add(1);
        }
        let fresh161 = data;
        data= data.offset(1);
        *fresh161= '0' as i32 as libc::c_char;
        return data;
    }
    i= 0 as libc::c_int as size_t;
    if '+' as i32 == *(*number).number.offset(i as isize) as libc::c_int {
        i= i.wrapping_add(1);
    }
    while i < (*number).number_size {
        let fresh162 = data;
        data= data.offset(1);
        *fresh162= *(*number).number.offset(i as isize);
        i= i.wrapping_add(1);
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_string(
    mut string: *const json_string_s,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let fresh163 = data;
    data= data.offset(1);
    *fresh163= '"' as i32 as libc::c_char;
    i= 0 as libc::c_int as size_t;
    while i < (*string).string_size {
        match  *(*string).string.offset(i as isize) as libc::c_int {
            34 => {
                let fresh164 = data;
                data= data.offset(1);
                *fresh164= '\\' as i32 as libc::c_char;
                let fresh165 = data;
                data= data.offset(1);
                *fresh165= '"' as i32 as libc::c_char;
            }
            92 => {
                let fresh166 = data;
                data= data.offset(1);
                *fresh166= '\\' as i32 as libc::c_char;
                let fresh167 = data;
                data= data.offset(1);
                *fresh167= '\\' as i32 as libc::c_char;
            }
            8 => {
                let fresh168 = data;
                data= data.offset(1);
                *fresh168= '\\' as i32 as libc::c_char;
                let fresh169 = data;
                data= data.offset(1);
                *fresh169= 'b' as i32 as libc::c_char;
            }
            12 => {
                let fresh170 = data;
                data= data.offset(1);
                *fresh170= '\\' as i32 as libc::c_char;
                let fresh171 = data;
                data= data.offset(1);
                *fresh171= 'f' as i32 as libc::c_char;
            }
            10 => {
                let fresh172 = data;
                data= data.offset(1);
                *fresh172= '\\' as i32 as libc::c_char;
                let fresh173 = data;
                data= data.offset(1);
                *fresh173= 'n' as i32 as libc::c_char;
            }
            13 => {
                let fresh174 = data;
                data= data.offset(1);
                *fresh174= '\\' as i32 as libc::c_char;
                let fresh175 = data;
                data= data.offset(1);
                *fresh175= 'r' as i32 as libc::c_char;
            }
            9 => {
                let fresh176 = data;
                data= data.offset(1);
                *fresh176= '\\' as i32 as libc::c_char;
                let fresh177 = data;
                data= data.offset(1);
                *fresh177= 't' as i32 as libc::c_char;
            }
            _ => {
                let fresh178 = data;
                data= data.offset(1);
                *fresh178= *(*string).string.offset(i as isize);
            }
        }
        i= i.wrapping_add(1);
    }
    let fresh179 = data;
    data= data.offset(1);
    *fresh179= '"' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_array(
    mut array: *const json_array_s,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut element = 0 as *mut json_array_element_s;
    let fresh180 = data;
    data= data.offset(1);
    *fresh180= '[' as i32 as libc::c_char;
    element= (*array).start;
    while !element.is_null() {
        if element != (*array).start {
            let fresh181 = data;
            data= data.offset(1);
            *fresh181= ',' as i32 as libc::c_char;
        }
        data= json_write_minified_value((*element).value, data);
        if data.is_null() {();
            return 0 as *mut libc::c_char;
        }
        element= (*element).next;
    }();
    let fresh182 = data;
    data= data.offset(1);
    *fresh182= ']' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_object(
    mut object: *const json_object_s,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut element = 0 as *mut json_object_element_s;
    let fresh183 = data;
    data= data.offset(1);
    *fresh183= '{' as i32 as libc::c_char;
    element= (*object).start;
    while !element.is_null() {
        if element != (*object).start {
            let fresh184 = data;
            data= data.offset(1);
            *fresh184= ',' as i32 as libc::c_char;
        }
        data= json_write_string((*element).name, data);
        if data.is_null() {();
            return 0 as *mut libc::c_char;
        }
        let fresh185 = data;
        data= data.offset(1);
        *fresh185= ':' as i32 as libc::c_char;
        data= json_write_minified_value((*element).value, data);
        if data.is_null() {();
            return 0 as *mut libc::c_char;
        }
        element= (*element).next;
    }();
    let fresh186 = data;
    data= data.offset(1);
    *fresh186= '}' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_value(
    mut value: *const json_value_s,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    match (*value).type_0 {
        1 => return json_write_number((*value).payload as *mut json_number_s, data),
        0 => return json_write_string((*value).payload as *mut json_string_s, data),
        3 => {
            return json_write_minified_array((*value).payload as *mut json_array_s, data);
        }
        2 => {
            return json_write_minified_object(
                (*value).payload as *mut json_object_s,
                data,
            );
        }
        4 => {
            *data.offset(0 as libc::c_int as isize) = 't' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'r' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'u' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 'e' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        5 => {
            *data.offset(0 as libc::c_int as isize) = 'f' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'a' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 's' as i32 as libc::c_char;
            *data.offset(4 as libc::c_int as isize) = 'e' as i32 as libc::c_char;
            return data.offset(5 as libc::c_int as isize);
        }
        6 => {
            *data.offset(0 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'u' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        _ => return 0 as *mut libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified(
    mut value: Option<&mut json_value_s>,
    mut out_size: Option<&mut size_t>,
) -> *mut /* owning */ libc::c_void {
    let mut size = 0 as libc::c_int as size_t;
    let mut data = 0 as *mut libc::c_char;
    let mut data_end = 0 as *mut libc::c_char;
    if value.as_deref().is_none() {();
        return 0 as *mut libc::c_void;
    }
    if json_write_minified_get_value_size(value.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()), Some(&mut size)) != 0 {
        return 0 as *mut libc::c_void;
    }
    size= (size as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    data= malloc(size) as *mut libc::c_char;
    if data.is_null() {();
        return 0 as *mut libc::c_void;
    }
    data_end= json_write_minified_value(value.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()), data);
    if data_end.is_null() {();
        free(data as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    *data_end= '\0' as i32 as libc::c_char;
    if !out_size.as_deref().is_none() {
        *out_size.as_deref_mut().unwrap()= size;
    }else { (); }
    return data as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_array_size(
    mut array: *const json_array_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut element = 0 as *mut json_array_element_s;
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if (0 as libc::c_int as libc::c_ulong) < (*array).length {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(newline_size) as size_t as size_t;
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
            .wrapping_add(
                (*array).length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        element= (*array).start;
        while !element.is_null() {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(
                    depth
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(indent_size),
                ) as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent_size,
                newline_size,
                size.as_deref_mut(),
            ) != 0
            {
                return 1 as libc::c_int;
            }
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(newline_size) as size_t
                as size_t;
            element= (*element).next;
        }();
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_object_size(
    mut object: *const json_object_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    let mut element = 0 as *mut json_object_element_s;
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if (0 as libc::c_int as libc::c_ulong) < (*object).length {
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(newline_size) as size_t as size_t;
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
            .wrapping_add(
                (*object).length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        element= (*object).start;
        while !element.is_null() {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(
                    depth
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(indent_size),
                ) as size_t as size_t;
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(newline_size) as size_t
                as size_t;
            if json_write_get_string_size((*element).name, size.as_deref_mut()) != 0 {
                return 1 as libc::c_int;
            }
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent_size,
                newline_size,
                size.as_deref_mut(),
            ) != 0
            {
                return 1 as libc::c_int;
            }
            element= (*element).next;
        }();
        *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_value_size(
    mut value: *const json_value_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: Option<&mut size_t>,
) -> libc::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size.as_deref_mut(),
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size.as_deref_mut(),
            );
        }
        3 => {
            return json_write_pretty_get_array_size(
                (*value).payload as *mut json_array_s,
                depth,
                indent_size,
                newline_size,
                size.as_deref_mut(),
            );
        }
        2 => {
            return json_write_pretty_get_object_size(
                (*value).payload as *mut json_object_s,
                depth,
                indent_size,
                newline_size,
                size.as_deref_mut(),
            );
        }
        4 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        5 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        6 => {
            *size.as_deref_mut().unwrap()= ((*size.as_deref().unwrap()) as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 0 as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_array(
    mut array: *const json_array_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    let mut element = 0 as *mut json_array_element_s;
    let fresh187 = data;
    data= data.offset(1);
    *fresh187= '[' as i32 as libc::c_char;
    if (0 as libc::c_int as libc::c_ulong) < (*array).length {
        k= 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh188 = data;
            data= data.offset(1);
            *fresh188= *newline.offset(k as isize);
            k= k.wrapping_add(1);
        }
        element= (*array).start;
        while !element.is_null() {
            if element != (*array).start {
                let fresh189 = data;
                data= data.offset(1);
                *fresh189= ',' as i32 as libc::c_char;
                k= 0 as libc::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
                    let fresh190 = data;
                    data= data.offset(1);
                    *fresh190= *newline.offset(k as isize);
                    k= k.wrapping_add(1);
                }
            }
            k= 0 as libc::c_int as size_t;
            while k < depth.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                m= 0 as libc::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                    let fresh191 = data;
                    data= data.offset(1);
                    *fresh191= *indent.offset(m as isize);
                    m= m.wrapping_add(1);
                }
                k= k.wrapping_add(1);
            }
            data= json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent,
                newline,
                data,
            );
            if data.is_null() {();
                return 0 as *mut libc::c_char;
            }
            element= (*element).next;
        }();
        k= 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh192 = data;
            data= data.offset(1);
            *fresh192= *newline.offset(k as isize);
            k= k.wrapping_add(1);
        }
        k= 0 as libc::c_int as size_t;
        while k < depth {
            m= 0 as libc::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                let fresh193 = data;
                data= data.offset(1);
                *fresh193= *indent.offset(m as isize);
                m= m.wrapping_add(1);
            }
            k= k.wrapping_add(1);
        }
    }
    let fresh194 = data;
    data= data.offset(1);
    *fresh194= ']' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_object(
    mut object: *const json_object_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    let mut element = 0 as *mut json_object_element_s;
    let fresh195 = data;
    data= data.offset(1);
    *fresh195= '{' as i32 as libc::c_char;
    if (0 as libc::c_int as libc::c_ulong) < (*object).length {
        k= 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh196 = data;
            data= data.offset(1);
            *fresh196= *newline.offset(k as isize);
            k= k.wrapping_add(1);
        }
        element= (*object).start;
        while !element.is_null() {
            if element != (*object).start {
                let fresh197 = data;
                data= data.offset(1);
                *fresh197= ',' as i32 as libc::c_char;
                k= 0 as libc::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
                    let fresh198 = data;
                    data= data.offset(1);
                    *fresh198= *newline.offset(k as isize);
                    k= k.wrapping_add(1);
                }
            }
            k= 0 as libc::c_int as size_t;
            while k < depth.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                m= 0 as libc::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                    let fresh199 = data;
                    data= data.offset(1);
                    *fresh199= *indent.offset(m as isize);
                    m= m.wrapping_add(1);
                }
                k= k.wrapping_add(1);
            }
            data= json_write_string((*element).name, data);
            if data.is_null() {();
                return 0 as *mut libc::c_char;
            }
            let fresh200 = data;
            data= data.offset(1);
            *fresh200= ' ' as i32 as libc::c_char;
            let fresh201 = data;
            data= data.offset(1);
            *fresh201= ':' as i32 as libc::c_char;
            let fresh202 = data;
            data= data.offset(1);
            *fresh202= ' ' as i32 as libc::c_char;
            data= json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent,
                newline,
                data,
            );
            if data.is_null() {();
                return 0 as *mut libc::c_char;
            }
            element= (*element).next;
        }();
        k= 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh203 = data;
            data= data.offset(1);
            *fresh203= *newline.offset(k as isize);
            k= k.wrapping_add(1);
        }
        k= 0 as libc::c_int as size_t;
        while k < depth {
            m= 0 as libc::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                let fresh204 = data;
                data= data.offset(1);
                *fresh204= *indent.offset(m as isize);
                m= m.wrapping_add(1);
            }
            k= k.wrapping_add(1);
        }
    }
    let fresh205 = data;
    data= data.offset(1);
    *fresh205= '}' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_value(
    mut value: *const json_value_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char,
) -> *mut libc::c_char {
    match (*value).type_0 {
        1 => return json_write_number((*value).payload as *mut json_number_s, data),
        0 => return json_write_string((*value).payload as *mut json_string_s, data),
        3 => {
            return json_write_pretty_array(
                (*value).payload as *mut json_array_s,
                depth,
                indent,
                newline,
                data,
            );
        }
        2 => {
            return json_write_pretty_object(
                (*value).payload as *mut json_object_s,
                depth,
                indent,
                newline,
                data,
            );
        }
        4 => {
            *data.offset(0 as libc::c_int as isize) = 't' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'r' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'u' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 'e' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        5 => {
            *data.offset(0 as libc::c_int as isize) = 'f' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'a' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 's' as i32 as libc::c_char;
            *data.offset(4 as libc::c_int as isize) = 'e' as i32 as libc::c_char;
            return data.offset(5 as libc::c_int as isize);
        }
        6 => {
            *data.offset(0 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) = 'u' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) = 'l' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        _ => return 0 as *mut libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty(
    mut value: Option<&mut json_value_s>,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut out_size: Option<&mut size_t>,
) -> *mut /* owning */ libc::c_void {
    let mut size = 0 as libc::c_int as size_t;
    let mut indent_size = 0 as libc::c_int as size_t;
    let mut newline_size = 0 as libc::c_int as size_t;
    let mut data = 0 as *mut libc::c_char;
    let mut data_end = 0 as *mut libc::c_char;
    if value.as_deref().is_none() {();
        return 0 as *mut libc::c_void;
    }
    if indent.is_null() {();
        indent= b"  \0" as *const u8 as *const libc::c_char;
    }
    if newline.is_null() {();
        newline= b"\n\0" as *const u8 as *const libc::c_char;
    }
    while '\0' as i32 != *indent.offset(indent_size as isize) as libc::c_int {
        indent_size= indent_size.wrapping_add(1);
    }
    while '\0' as i32 != *newline.offset(newline_size as isize) as libc::c_int {
        newline_size= newline_size.wrapping_add(1);
    }
    if json_write_pretty_get_value_size(
        value.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()),
        0 as libc::c_int as size_t,
        indent_size,
        newline_size,
        Some(&mut size),
    ) != 0
    {
        return 0 as *mut libc::c_void;
    }
    size= (size as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    data= malloc(size) as *mut libc::c_char;
    if data.is_null() {();
        return 0 as *mut libc::c_void;
    }
    data_end= json_write_pretty_value(
        value.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()),
        0 as libc::c_int as size_t,
        indent,
        newline,
        data,
    );
    if data_end.is_null() {();
        free(data as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    *data_end= '\0' as i32 as libc::c_char;
    if !out_size.as_deref().is_none() {
        *out_size.as_deref_mut().unwrap()= size;
    }else { (); }
    return data as *mut libc::c_void;
}
