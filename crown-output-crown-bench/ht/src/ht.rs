use libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct ht {
    pub entries: *mut ht_entry,
    pub capacity: size_t,
    pub length: size_t,
}
impl Default for ht {
    fn default() -> Self {
        Self {
            entries: std::ptr::null_mut(),
            capacity: Default::default(),
            length: Default::default(),
        }
    }
}
impl ht {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *const libc::c_char,
    pub value: *mut libc::c_void,
}
impl Default for ht_entry {
    fn default() -> Self {
        Self {
            key: std::ptr::null(),
            value: std::ptr::null_mut(),
        }
    }
}

pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hti {
    pub key: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub _table: *mut ht,
    pub _index: size_t,
}
impl Default for hti {
    fn default() -> Self {
        Self {
            key: std::ptr::null(),
            value: std::ptr::null_mut(),
            _table: std::ptr::null_mut(),
            _index: Default::default(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ht_create() -> Option<Box<ht>> {
    let mut table = Some(Box::new(<crate::src::ht::ht as Default>::default()));
    if table.as_deref().is_none() {
        ();
        return None;
    }
    (*table.as_deref_mut().unwrap()).length = 0 as libc::c_int as size_t;
    (*table.as_deref_mut().unwrap()).capacity = 16 as libc::c_int as size_t;
    (*table.as_deref_mut().unwrap()).entries = calloc(
        (*table.as_deref().unwrap()).capacity,
        ::std::mem::size_of::<ht_entry>() as libc::c_ulong,
    ) as *mut ht_entry;
    if (*table.as_deref().unwrap()).entries.is_null() {
        ();
        ();
        return None;
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn ht_destroy(mut table: Option<Box<ht>>) {
    let mut i = 0 as libc::c_int as size_t;
    while i < (*table.as_deref().unwrap()).capacity {
        free((*(*table.as_deref().unwrap()).entries.offset(i as isize)).key as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*table.as_deref().unwrap()).entries as *mut libc::c_void);
    ();
}
unsafe extern "C" fn hash_key(mut key: *const libc::c_char) -> uint64_t {
    let mut hash = 14695981039346656037 as libc::c_ulong;
    let mut p = key;
    while (*p) != 0 {
        hash ^= (*p) as libc::c_uchar as uint64_t;
        hash = (hash as libc::c_ulong).wrapping_mul(1099511628211 as libc::c_ulong) as uint64_t
            as uint64_t;
        p = p.offset(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn ht_get(
    mut table: *mut ht,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut hash = hash_key(key);
    let mut index = hash
        & (*table)
            .capacity
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while !((*(*table).entries.offset(index as isize)).key).is_null() {
        if strcmp(key, (*(*table).entries.offset(index as isize)).key) == 0 as libc::c_int {
            return (*(*table).entries.offset(index as isize)).value;
        }
        index = index.wrapping_add(1);
        if index >= (*table).capacity {
            index = 0 as libc::c_int as size_t;
        }
    }
    ();
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn ht_set_entry(
    mut entries: *mut ht_entry,
    mut capacity: size_t,
    mut key: *const libc::c_char,
    mut value: *mut libc::c_void,
    mut plength: Option<&mut size_t>,
) -> *const libc::c_char {
    let mut hash = hash_key(key);
    let mut index = hash & capacity.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while !((*entries.offset(index as isize)).key).is_null() {
        if strcmp(key, (*entries.offset(index as isize)).key) == 0 as libc::c_int {
            (*entries.offset(index as isize)).value = value;
            return (*entries.offset(index as isize)).key;
        }
        index = index.wrapping_add(1);
        if index >= capacity {
            index = 0 as libc::c_int as size_t;
        }
    }
    ();
    if !plength.as_deref().is_none() {
        key = strdup(key);
        if key.is_null() {
            ();
            return 0 as *const libc::c_char;
        }
        *plength.as_deref_mut().unwrap() = (*plength.as_deref().unwrap()).wrapping_add(1);
    } else {
        ();
    }
    (*entries.offset(index as isize)).key = key as *mut libc::c_char;
    (*entries.offset(index as isize)).value = value;
    return key;
}
unsafe extern "C" fn ht_expand(mut table: Option<&mut ht>) -> bool {
    let mut new_capacity = (*table.as_deref().unwrap())
        .capacity
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if new_capacity < (*table.as_deref().unwrap()).capacity {
        return 0 as libc::c_int != 0;
    }
    let mut new_entries = calloc(
        new_capacity,
        ::std::mem::size_of::<ht_entry>() as libc::c_ulong,
    ) as *mut ht_entry;
    if new_entries.is_null() {
        ();
        return 0 as libc::c_int != 0;
    }
    let mut i = 0 as libc::c_int as size_t;
    while i < (*table.as_deref().unwrap()).capacity {
        let mut entry = *(*table.as_deref().unwrap()).entries.offset(i as isize);
        if !entry.key.is_null() {
            ht_set_entry(new_entries, new_capacity, entry.key, entry.value, None);
        } else {
            ();
        }
        i = i.wrapping_add(1);
    }
    free((*table.as_deref().unwrap()).entries as *mut libc::c_void);
    (*table.as_deref_mut().unwrap()).entries = new_entries;
    (*table.as_deref_mut().unwrap()).capacity = new_capacity;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ht_set(
    mut table: Option<&mut ht>,
    mut key: *const libc::c_char,
    mut value: *mut libc::c_void,
) -> *const libc::c_char {
    if !value.is_null() {
    } else {
        ();
        __assert_fail(
            b"value != NULL\0" as *const u8 as *const libc::c_char,
            b"ht.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            b"const char *ht_set(ht *, const char *, void *)\0" as *const u8 as *const libc::c_char,
        );
    }
    if value.is_null() {
        ();
        return 0 as *const libc::c_char;
    }
    if (*table.as_deref().unwrap()).length
        >= (*table.as_deref().unwrap())
            .capacity
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        if !ht_expand(table.as_deref_mut()) {
            return 0 as *const libc::c_char;
        }
    }
    return ht_set_entry(
        (*table.as_deref().unwrap()).entries,
        (*table.as_deref().unwrap()).capacity,
        key,
        value,
        Some(&mut (*table.as_deref_mut().unwrap()).length),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ht_length(mut table: *mut ht) -> size_t {
    return (*table).length;
}
#[no_mangle]
pub unsafe extern "C" fn ht_iterator(mut table: *mut ht) -> hti {
    let mut it = hti {
        key: 0 as *const libc::c_char,
        value: 0 as *mut libc::c_void,
        _table: 0 as *mut ht,
        _index: 0,
    };
    it._table = table;
    it._index = 0 as libc::c_int as size_t;
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn ht_next(mut it: Option<&mut hti>) -> bool {
    let mut table = (*it.as_deref().unwrap())._table;
    while (*it.as_deref().unwrap())._index < (*table).capacity {
        let mut i = (*it.as_deref().unwrap())._index;
        (*it.as_deref_mut().unwrap())._index = (*it.as_deref().unwrap())._index.wrapping_add(1);
        if !((*(*table).entries.offset(i as isize)).key).is_null() {
            let mut entry = *(*table).entries.offset(i as isize);
            (*it.as_deref_mut().unwrap()).key = entry.key;
            (*it.as_deref_mut().unwrap()).value = entry.value;
            return 1 as libc::c_int != 0;
        } else {
            ();
        }
    }
    return 0 as libc::c_int != 0;
}
