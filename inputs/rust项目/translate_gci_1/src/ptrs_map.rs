use crate::utils::*;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::process;

#[repr(C)]
pub struct PTRS_MAP_ELEM {
    pub key: *mut c_void,
    pub val: *mut c_void,
}

impl Default for PTRS_MAP_ELEM {
    fn default() -> Self {
        PTRS_MAP_ELEM {
            key: null_mut(),
            val: null_mut(),
        }
    }
}

#[repr(C)]
pub struct PTRS_MAP {
    pub elems: Vec<PTRS_MAP_ELEM>,
    pub len: usize,
    pub cap: usize,
}

impl Default for PTRS_MAP {
    fn default() -> Self {
        PTRS_MAP {
            elems: Vec::new(),
            len: 0,
            cap: 0,
        }
    }
}

pub type ptrs_map_type_t<'a> = &'a mut PTRS_MAP;

pub fn ptrs_map_conf<'a>(pm: ptrs_map_type_t<'a>, cap: usize) {
    pm.len = 0;
    pm.cap = cap;
    pm.elems = Vec::with_capacity(cap);
}

pub fn ptrs_map_set<'a>(pm: ptrs_map_type_t<'a>, key: *mut c_void, val: *mut c_void) {
    if key.is_null() {
        eprintln!("insertion null-key in map is prohibited");
        process::exit(1);
    }

    for i in 0..pm.len {
        if pm.elems[i].key == key {
            pm.elems[i].val = val;
            return;
        }
    }

    if pm.len == pm.cap {
        eprintln!("non enough memory in map to handle new value");
        process::exit(1);
    }

    pm.elems.push(PTRS_MAP_ELEM { key, val });
    pm.len += 1;
}

pub fn ptrs_map_get<'a>(pm: ptrs_map_type_t<'a>, key: *mut c_void) -> *mut c_void {
    if key.is_null() {
        eprintln!("selection by null-key from map is prohibited");
        process::exit(1);
    }

    for i in 0..pm.len {
        if pm.elems[i].key == key {
            return pm.elems[i].val;
        }
    }

    null_mut()
}

pub fn ptrs_map_free<'a>(pm: ptrs_map_type_t<'a>) {
    pm.elems.clear();
    pm.elems.shrink_to_fit();
    pm.len = 0;
    pm.cap = 0;
}