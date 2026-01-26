use translate_gci::ptrs_map::*;
use translate_gci::allocator::*;
use std::mem;
use std::ptr;

#[repr(C)]
pub struct GARBAGE_COLLECTOR {
    pub stack: *mut *mut VALUE,
    pub stack_top: *mut *mut VALUE,
    pub a: ALLOCATOR,
    pub b: ALLOCATOR,
    pub ptrs_map: PTRS_MAP,
    pub trace: i32,
}

pub type garbage_collector_type_t = *mut GARBAGE_COLLECTOR;

pub fn create_garbage_collector() -> garbage_collector_type_t {
    let gc = Box::new(GARBAGE_COLLECTOR {
        stack: ptr::null_mut(),
        stack_top: ptr::null_mut(),
        a: ALLOCATOR::default(),
        b: ALLOCATOR::default(),
        ptrs_map: PTRS_MAP::default(),
        trace: 0,
    });
    Box::into_raw(gc)
}

pub fn garbage_collector_conf(
    gc: garbage_collector_type_t,
    sizemem_start: usize,
    stack: *mut *mut VALUE,
    stack_top: *mut *mut VALUE,
    trace: i32,
) {
    unsafe {
        (*gc).stack = stack;
        (*gc).stack_top = stack_top;
        allocator_malloc_pool(&mut (*gc).a, sizemem_start);
        allocator_malloc_pool(&mut (*gc).b, sizemem_start);
        (*gc).trace = trace;
    }
}

unsafe fn lookup_new_location(gc: garbage_collector_type_t, ptr_val: *mut u8) -> *mut u8 {
    let o = BLOCK_PTR_FROM_DATA((ptr_val as *mut u8).offset(-1));
    if ptrs_map_get(&(*gc).ptrs_map, o).is_null() {
        ptrs_map_set(
            &mut (*gc).ptrs_map,
            o,
            (*(*gc).b.free_list.first) as *mut u8,
        );
        allocator_malloc_block(&mut (*gc).b, BLOCK_L_DATA_LEN(o));
        let src = BLOCK_DATA(o);
        let dst = BLOCK_DATA((*(*gc).b.busy_list.last) as *mut u8);
        std::ptr::copy_nonoverlapping(src, dst, BLOCK_L_DATA_LEN(o));
    }
    ((ptrs_map_get(&(*gc).ptrs_map, o) as *mut u8).offset(1)) as *mut u8
}

unsafe fn run_gc_inner(gc: garbage_collector_type_t, ptr_ref: *mut *mut u8) {
    let stack = *(*gc).stack;
    let stack_top = *(*gc).stack_top;

    allocator_clean_pool(&mut (*gc).b);
    ptrs_map_conf(&mut (*gc).ptrs_map, (*(*gc).a.busy_list.count));

    let mut val = stack;
    while val != stack_top {
        match (*val).value_type {
            VALUE_TYPE_OBJ => {
                (*val).obj_val = lookup_new_location(gc, (*val).obj_val as *mut u8) as *mut OBJECT;
            }
            VALUE_TYPE_ARR => {
                (*val).arr_val = lookup_new_location(gc, (*val).arr_val as *mut u8) as *mut ARRAY;
            }
            _ => {}
        }
        val = val.offset(1);
    }

    let mut unscanned_ptr = (*(*gc).b.busy_list.first) as *mut u8;
    while !unscanned_ptr.is_null() {
        let ptr = BLOCK_DATA(unscanned_ptr);
        if *ptr == VALUE_TYPE_OBJ {
            let obj = (ptr.offset(1)) as *mut OBJECT;
            let mut i = 0;
            while i < (*obj).properties_len {
                let val = &mut (*obj).properties.offset(i as isize).as_mut().unwrap().val;
                match (*val).value_type {
                    VALUE_TYPE_OBJ => {
                        (*val).obj_val =
                            lookup_new_location(gc, (*val).obj_val as *mut u8) as *mut OBJECT;
                    }
                    VALUE_TYPE_ARR => {
                        (*val).arr_val =
                            lookup_new_location(gc, (*val).arr_val as *mut u8) as *mut ARRAY;
                    }
                    _ => {}
                }
                i += 1;
            }
        } else if *ptr == VALUE_TYPE_ARR {
            let arr = (ptr.offset(1)) as *mut ARRAY;
            let mut i = 0;
            while i < (*arr).len {
                let val = &mut (*arr).values.offset(i as isize).as_mut().unwrap();
                match (*val).value_type {
                    VALUE_TYPE_OBJ => {
                        (*val).obj_val =
                            lookup_new_location(gc, (*val).obj_val as *mut u8) as *mut OBJECT;
                    }
                    VALUE_TYPE_ARR => {
                        (*val).arr_val =
                            lookup_new_location(gc, (*val).arr_val as *mut u8) as *mut ARRAY;
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        unscanned_ptr = BLOCK_LIST_NEXT(unscanned_ptr);
    }

    if !ptr_ref.is_null() && !(*ptr_ref).is_null() {
        *ptr_ref = lookup_new_location(gc, *ptr_ref);
    }

    ptrs_map_free(&mut (*gc).ptrs_map);
}

unsafe fn run_gc(
    gc: garbage_collector_type_t,
    ptr_ref: *mut *mut u8,
    ptr_sizemem: usize,
) {
    let mut tmp: ALLOCATOR;
    let need_realloc: i32;

    if (*gc).trace != 0 {
        println!("\t<info>GC</info>");
    }

    run_gc_inner(gc, ptr_ref);

    need_realloc = if (((*(*gc).b.busy_list.sizemem)
        + ((*(*gc).b.busy_list.count) * BLOCK_OVERHEAD))
        + (ptr_sizemem + BLOCK_OVERHEAD))
        >= (*(*gc).b.sizemem) / 2
    {
        1
    } else {
        0
    };

    if need_realloc != 0 {
        let new_sizemem = ((*(*gc).b.sizemem) + (ptr_sizemem + BLOCK_OVERHEAD)) * 2;
        tmp = (*gc).a;
        (*gc).a = (*gc).b;
        (*gc).b = tmp;
        allocator_free_pool(&mut (*gc).b);
        allocator_malloc_pool(&mut (*gc).b, new_sizemem);
        run_gc_inner(gc, ptr_ref);
        allocator_free_pool(&mut (*gc).a);
        allocator_malloc_pool(&mut (*gc).a, new_sizemem);
    }

    tmp = (*gc).a;
    (*gc).a = (*gc).b;
    (*gc).b = tmp;
}

unsafe fn malloc_obj_force(
    gc: garbage_collector_type_t,
    start_properties_cap: usize,
    sizemem: usize,
) -> *mut OBJECT {
    let ptr = allocator_malloc_block(&mut (*gc).a, sizemem);
    *ptr = VALUE_TYPE_OBJ;
    let obj = (ptr.offset(1)) as *mut OBJECT;
    (*obj).properties_len = 0;
    (*obj).properties_cap = start_properties_cap;
    obj
}

pub fn garbage_collector_malloc_obj(
    gc: garbage_collector_type_t,
    start_properties_num: usize,
) -> *mut OBJECT {
    unsafe {
        let start_properties_cap = start_properties_num * 2;
        let sizemem = mem::size_of::<u8>()
            + mem::size_of::<OBJECT>()
            + mem::size_of::<PROPERTY>() * start_properties_cap;

        if !(*(*gc).a.free_list.last).is_null()
            && BLOCK_L_DATA_LEN((*(*gc).a.free_list.last) as *mut u8) >= (sizemem + BLOCK_OVERHEAD)
        {
        }

        run_gc(gc, ptr::null_mut(), sizemem);
        malloc_obj_force(gc, start_properties_cap, sizemem)
    }
}

unsafe fn change_vals_ptr(val: *mut VALUE, prev_ptr: *mut u8, new_ptr: *mut u8) {
    if (*val).value_type == VALUE_TYPE_OBJ
        && (*val).obj_val == ((prev_ptr.offset(1)) as *mut OBJECT)
    {
        (*val).obj_val = (new_ptr.offset(1)) as *mut OBJECT;
    } else if (*val).value_type == VALUE_TYPE_ARR
        && (*val).arr_val == ((prev_ptr.offset(1)) as *mut ARRAY)
    {
        (*val).arr_val = (new_ptr.offset(1)) as *mut ARRAY;
    }
}

unsafe fn change_one_ptr(
    gc: garbage_collector_type_t,
    prev_ptr: *mut u8,
    new_ptr: *mut u8,
) {
    let stack = *(*gc).stack;
    let stack_top = *(*gc).stack_top;

    let mut val = stack;
    while val != stack_top {
        change_vals_ptr(val, prev_ptr, new_ptr);
        val = val.offset(1);
    }

    let mut cur = (*(*gc).a.busy_list.first) as *mut u8;
    while !cur.is_null() {
        let ptr = BLOCK_DATA(cur);
        if *ptr == VALUE_TYPE_OBJ {
            let obj = (ptr.offset(1)) as *mut OBJECT;
            let mut i = 0;
            while i < (*obj).properties_len {
                let val = &mut (*obj).properties.offset(i as isize).as_mut().unwrap().val;
                change_vals_ptr(val, prev_ptr, new_ptr);
                i += 1;
            }
        } else if *ptr == VALUE_TYPE_ARR {
            let arr = (ptr.offset(1)) as *mut ARRAY;
            let mut i = 0;
            while i < (*arr).len {
                let val = &mut (*arr).values.offset(i as isize).as_mut().unwrap();
                change_vals_ptr(val, prev_ptr, new_ptr);
                i += 1;
            }
        }
        cur = BLOCK_LIST_NEXT(cur);
    }
}

unsafe fn realloc_obj_force(
    gc: garbage_collector_type_t,
    obj: *mut OBJECT,
    new_properties_cap: usize,
    sizemem: usize,
) -> *mut OBJECT {
    let prev_ptr = (obj as *mut u8).offset(-1);
    let new_ptr = allocator_realloc_block(&mut (*gc).a, prev_ptr, sizemem);
    change_one_ptr(gc, prev_ptr, new_ptr);
    let obj = (new_ptr.offset(1)) as *mut OBJECT;
    (*obj).properties_cap = new_properties_cap;
    obj
}

pub fn garbage_collector_realloc_obj(
    gc: garbage_collector_type_t,
    obj: *mut OBJECT,
    new_properties_num: usize,
) -> *mut OBJECT {
    unsafe {
        let new_properties_cap = new_properties_num * 2;
        let sizemem = mem::size_of::<u8>()
            + mem::size_of::<OBJECT>()
            + mem::size_of::<PROPERTY>() * new_properties_cap;

        if !(*(*gc).a.free_list.last).is_null()
            && BLOCK_L_DATA_LEN((*(*gc).a.free_list.last) as *mut u8) >= (sizemem + BLOCK_OVERHEAD)
        {
        }

        let mut obj_ptr = obj;
        run_gc(gc, &mut (obj_ptr as *mut u8), sizemem);
        realloc_obj_force(gc, obj_ptr, new_properties_cap, sizemem)
    }
}

pub fn garbage_collector_free(gc: garbage_collector_type_t) {
    unsafe {
        allocator_free_pool(&mut (*gc).a);
        allocator_free_pool(&mut (*gc).b);
        let _ = Box::from_raw(gc);
    }
}

unsafe fn malloc_arr_force(
    gc: garbage_collector_type_t,
    start_cap: usize,
    sizemem: usize,
) -> *mut ARRAY {
    let ptr = allocator_malloc_block(&mut (*gc).a, sizemem);
    *ptr = VALUE_TYPE_ARR;
    let arr = (ptr.offset(1)) as *mut ARRAY;
    (*arr).len = 0;
    (*arr).cap = start_cap;
    arr
}

pub fn garbage_collector_malloc_arr(
    gc: garbage_collector_type_t,
    arr_len: usize,
) -> *mut ARRAY {
    unsafe {
        let start_arr_cap = arr_len * 2;
        let sizemem =
            mem::size_of::<u8>() + mem::size_of::<ARRAY>() + mem::size_of::<VALUE>() * start_arr_cap;

        if !(*(*gc).a.free_list.last).is_null()
            && BLOCK_L_DATA_LEN((*(*gc).a.free_list.last) as *mut u8) >= (sizemem + BLOCK_OVERHEAD)
        {
        }

        run_gc(gc, ptr::null_mut(), sizemem);
        malloc_arr_force(gc, start_arr_cap, sizemem)
    }
}

unsafe fn realloc_arr_force(
    gc: garbage_collector_type_t,
    arr: *mut ARRAY,
    new_arr_cap: usize,
    sizemem: usize,
) -> *mut ARRAY {
    let prev_ptr = (arr as *mut u8).offset(-1);
    let new_ptr = allocator_realloc_block(&mut (*gc).a, prev_ptr, sizemem);
    change_one_ptr(gc, prev_ptr, new_ptr);
    let arr = (new_ptr.offset(1)) as *mut ARRAY;
    (*arr).cap = new_arr_cap;
    arr
}

pub fn garbage_collector_realloc_arr(
    gc: garbage_collector_type_t,
    arr: *mut ARRAY,
    new_arr_len: usize,
) -> *mut ARRAY {
    unsafe {
        let new_arr_cap = new_arr_len * 2;
        let sizemem = mem::size_of::<u8>() + mem::size_of::<ARRAY>() + mem::size_of::<VALUE>() * new_arr_cap;

        if !(*(*gc).a.free_list.last).is_null()
            && BLOCK_L_DATA_LEN((*(*gc).a.free_list.last) as *mut u8) >= (sizemem + BLOCK_OVERHEAD)
        {
        }

        let mut arr_ptr = arr;
        run_gc(gc, &mut (arr_ptr as *mut u8), sizemem);
        realloc_arr_force(gc, arr_ptr, new_arr_cap, sizemem)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VALUE {
    pub value_type: u8,
    pub obj_val: *mut OBJECT,
    pub arr_val: *mut ARRAY,
}

impl Default for VALUE {
    fn default() -> Self {
        VALUE {
            value_type: 0,
            obj_val: ptr::null_mut(),
            arr_val: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct OBJECT {
    pub properties: *mut PROPERTY,
    pub properties_len: usize,
    pub properties_cap: usize,
}

impl Default for OBJECT {
    fn default() -> Self {
        OBJECT {
            properties: ptr::null_mut(),
            properties_len: 0,
            properties_cap: 0,
        }
    }
}

#[repr(C)]
pub struct ARRAY {
    pub values: *mut VALUE,
    pub len: usize,
    pub cap: usize,
}

impl Default for ARRAY {
    fn default() -> Self {
        ARRAY {
            values: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}

#[repr(C)]
pub struct PROPERTY {
    pub val: VALUE,
}

impl Default for PROPERTY {
    fn default() -> Self {
        PROPERTY {
            val: VALUE::default(),
        }
    }
}

pub const VALUE_TYPE_OBJ: u8 = 1;
pub const VALUE_TYPE_ARR: u8 = 2;

#[link(name = "c")]
extern "C" {
    pub fn BLOCK_PTR_FROM_DATA(ptr: *mut u8) -> *mut u8;
    pub fn BLOCK_DATA(ptr: *mut u8) -> *mut u8;
    pub fn BLOCK_L_DATA_LEN(ptr: *mut u8) -> usize;
    pub fn BLOCK_LIST_NEXT(ptr: *mut u8) -> *mut u8;
}

pub const BLOCK_OVERHEAD: usize = 64;