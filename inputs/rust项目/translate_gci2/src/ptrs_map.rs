pub struct PTRS_MAP_ELEM {
    pub key: *mut std::ffi::c_void,
    pub val: *mut std::ffi::c_void,
}

pub struct PTRS_MAP {
    pub elems: Option<Box<[PTRS_MAP_ELEM]>>,
    pub len: usize,
    pub cap: usize,
}

impl Default for PTRS_MAP {
    fn default() -> Self {
        Self {
            elems: None,
            len: 0,
            cap: 0,
        }
    }
}

pub type ptrs_map_type_t = *mut PTRS_MAP;

pub fn ptrs_map_conf(pm: &mut PTRS_MAP, cap: usize) {
    pm.len = 0;
    pm.cap = cap;
    let mut elems_vec: Vec<PTRS_MAP_ELEM> = Vec::with_capacity(cap);
    for _ in 0..cap {
        elems_vec.push(PTRS_MAP_ELEM {
            key: std::ptr::null_mut(),
            val: std::ptr::null_mut(),
        });
    }
    pm.elems = Some(elems_vec.into_boxed_slice());
}

pub fn ptrs_map_set(pm: &mut PTRS_MAP, key: *mut std::ffi::c_void, val: *mut std::ffi::c_void) {
    if key.is_null() {
        eprintln!("insertion null-key in map is prohibited");
        std::process::exit(1);
    }

    if let Some(ref mut elems) = pm.elems {
        for i in 0..pm.len {
            if elems[i].key == key {
                elems[i].val = val;
                return;
            }
        }

        if pm.len == pm.cap {
            eprintln!("non enough memory in map to handle new value");
            std::process::exit(1);
        }

        elems[pm.len].key = key;
        elems[pm.len].val = val;
        pm.len += 1;
    }
}

pub fn ptrs_map_get(pm: &PTRS_MAP, key: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    if key.is_null() {
        eprintln!("selection by null-key from map is prohibited");
        std::process::exit(1);
    }

    if let Some(ref elems) = pm.elems {
        for i in 0..pm.len {
            if elems[i].key == key {
                return elems[i].val;
            }
        }
    }

    std::ptr::null_mut()
}

pub fn ptrs_map_free(pm: &mut PTRS_MAP) {
    pm.elems = None;
}
