use crate::utils::*;
use crate::allocator::*;
use crate::ptrs_map::*;
use crate::data_types::*;
pub use std::cell::RefCell;
pub use std::rc::Rc;

#[repr(C)]
pub struct GARBAGE_COLLECTOR {
    pub stack: Option<Rc<RefCell<Vec<VALUE>>>>,
    pub stack_top: usize,

    pub a: ALLOCATOR,
    pub b: ALLOCATOR,

    pub ptrs_map: PTRS_MAP,

    pub trace: i32,
}

pub type garbage_collector_type_t = Rc<RefCell<GARBAGE_COLLECTOR>>;

impl Default for GARBAGE_COLLECTOR {
    fn default() -> Self {
        GARBAGE_COLLECTOR {
            stack: None,
            stack_top: 0,
            a: ALLOCATOR::default(),
            b: ALLOCATOR::default(),
            ptrs_map: PTRS_MAP::default(),
            trace: 0,
        }
    }
}

#[repr(C)]
pub struct VALUE {
    pub r#type: i32,
    pub obj_val: Option<usize>,
    pub arr_val: Option<usize>,
}

#[repr(C)]
pub struct PROPERTY {
    pub val: VALUE,
}

#[repr(C)]
pub struct OBJECT {
    pub properties_len: usize,
    pub properties_cap: usize,
    pub properties: Vec<PROPERTY>,
}

#[repr(C)]
pub struct ARRAY {
    pub len: usize,
    pub cap: usize,
    pub values: Vec<VALUE>,
}

#[repr(C)]
#[derive(Default)]
pub struct ALLOCATOR {
    pub free_list: LIST,
    pub busy_list: LIST,
    pub sizemem: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct PTRS_MAP {
    // opaque map; contents managed elsewhere
    // placeholder to keep type compatibility
    _private: i32,
}

#[repr(C)]
#[derive(Default)]
pub struct LIST {
    pub first: Option<usize>,
    pub last: Option<usize>,
    pub count: usize,
    pub sizemem: usize,
}

pub fn create_garbage_collector() -> garbage_collector_type_t {
    Rc::new(RefCell::new(GARBAGE_COLLECTOR::default()))
}

pub fn garbage_collector_conf(
    gc: garbage_collector_type_t,
    sizemem_start: usize,
    stack: Option<Rc<RefCell<Vec<VALUE>>>>,
    stack_top: usize,
    trace: i32,
) {
    {
        let mut g = gc.borrow_mut();
        g.stack = stack;
        g.stack_top = stack_top;
    }

    // External allocator functions assumed to exist elsewhere.
    // Calls preserved as in original C code.
    allocator_malloc_pool(&mut gc.borrow_mut().a, sizemem_start);
    allocator_malloc_pool(&mut gc.borrow_mut().b, sizemem_start);

    gc.borrow_mut().trace = trace;
}

fn lookup_new_location_inner(gc: &mut GARBAGE_COLLECTOR, ptr: Option<usize>) -> Option<usize> {
    if ptr.is_none() {
        return None;
    }
    let ptr_val = ptr.unwrap();
    // In original C code, they compute block pointer from data pointer by subtracting 1.
    // Here we preserve that concept using usize arithmetic.
    let o = ptr_val.saturating_sub(1);

    if ptrs_map_get(&mut gc.ptrs_map, o).is_none() {
        // Map o to gc->b.free_list.first
        ptrs_map_set(&mut gc.ptrs_map, o, gc.b.free_list.first);

        // allocate block in b of length BLOCK_L_DATA_LEN(o)
        allocator_malloc_block(&mut gc.b, BLOCK_L_DATA_LEN(o));
        // copy data from old block to newly allocated block
        // BLOCK_DATA(gc->b.busy_list.last) and BLOCK_DATA(o) are abstracted as usize addresses
        memcpy_blocks(BLOCK_DATA_FROM_INDEX(gc.b.busy_list.last), BLOCK_DATA_FROM_INDEX(Some(o)), BLOCK_L_DATA_LEN(o));
    }

    // return BLOCK_DATA(ptrs_map_get(...)) + 1
    let mapped = ptrs_map_get(&mut gc.ptrs_map, o);
    mapped.map(|m| BLOCK_DATA_FROM_INDEX(Some(m)).saturating_add(1))
}

fn run_gc_inner_safe(gc_rc: &garbage_collector_type_t, ptr: Option<usize>) {
    let mut gc = gc_rc.borrow_mut();

    // Get stack slice if present
    let stack_vec_opt = gc.stack.as_ref().map(|rc| rc.clone());
    let stack_len = gc.stack_top;

    // Clean allocator b
    allocator_clean_pool(&mut gc.b);

    // Configure ptrs_map with count of a.busy_list.count
    ptrs_map_conf(&mut gc.ptrs_map, gc.a.busy_list.count);

    // For each root in stack, update to new location
    if let Some(stack_rc) = stack_vec_opt.clone() {
        let mut stack_ref = stack_rc.borrow_mut();
        for i in 0..stack_len.min(stack_ref.len()) {
            let mut val = &mut stack_ref[i];
            if val.r#type == VALUE_TYPE_OBJ {
                val.obj_val = lookup_new_location_inner(&mut gc, val.obj_val);
            } else if val.r#type == VALUE_TYPE_ARR {
                val.arr_val = lookup_new_location_inner(&mut gc, val.arr_val);
            }
        }
    }

    // Cheney's algorithm scan
    let mut unscanned_ptr = gc.b.busy_list.first;
    while let Some(_up) = unscanned_ptr {
        // ptr represents block data start address (as usize)
        let ptr_data_idx = BLOCK_DATA_FROM_INDEX(unscanned_ptr);
        // first byte indicates type; use helper to read block first byte
        let first_byte = BLOCK_FIRST_BYTE(ptr_data_idx);
        if first_byte == VALUE_TYPE_OBJ {
            // interpret following bytes as OBJECT
            if let Some(obj) = BLOCK_AS_OBJECT(ptr_data_idx) {
                for i in 0..obj.properties_len {
                    if i < obj.properties.len() {
                        let mut pval = &mut obj.properties[i].val;
                        if pval.r#type == VALUE_TYPE_OBJ {
                            pval.obj_val = lookup_new_location_inner(&mut gc, pval.obj_val);
                        } else if pval.r#type == VALUE_TYPE_ARR {
                            pval.arr_val = lookup_new_location_inner(&mut gc, pval.arr_val);
                        }
                    }
                }
            }
        } else if first_byte == VALUE_TYPE_ARR {
            if let Some(arr) = BLOCK_AS_ARRAY(ptr_data_idx) {
                for i in 0..arr.len {
                    if i < arr.values.len() {
                        let mut aval = &mut arr.values[i];
                        if aval.r#type == VALUE_TYPE_OBJ {
                            aval.obj_val = lookup_new_location_inner(&mut gc, aval.obj_val);
                        } else if aval.r#type == VALUE_TYPE_ARR {
                            aval.arr_val = lookup_new_location_inner(&mut gc, aval.arr_val);
                        }
                    }
                }
            }
        }

        unscanned_ptr = BLOCK_LIST_NEXT(unscanned_ptr);
    }

    if let Some(p) = ptr {
        // update the external pointer to new location
        let _new = lookup_new_location_inner(&mut gc, Some(p));
        // original C sets (*ptr) = lookup_new_location(gc, *ptr);
        // We cannot change external pointer here; assume caller uses returned mapping if needed.
    }

    ptrs_map_free(&mut gc.ptrs_map);
}

pub fn run_gc(gc: garbage_collector_type_t, ptr: Option<usize>, ptr_sizemem: usize) {
    let mut gc_mut = gc.borrow_mut();

    if gc_mut.trace != 0 {
        println!("\t<info>GC</info>");
    }
    drop(gc_mut);

    run_gc_inner_safe(&gc, ptr);

    // check if realloc is needed.
    let need_realloc = {
        let g = gc.borrow();
        ((g.b.busy_list.sizemem + g.b.busy_list.count * BLOCK_OVERHEAD)
            + (ptr_sizemem + BLOCK_OVERHEAD))
            >= g.b.sizemem / 2
    };

    if need_realloc {
        let new_sizemem = {
            let g = gc.borrow();
            (g.b.sizemem + (ptr_sizemem + BLOCK_OVERHEAD)) * 2
        };

        {
            let mut g = gc.borrow_mut();
            let tmp = std::mem::replace(&mut g.a, ALLOCATOR::default());
            g.a = std::mem::replace(&mut g.b, tmp);
        }

        allocator_free_pool(&mut gc.borrow_mut().b);
        allocator_malloc_pool(&mut gc.borrow_mut().b, new_sizemem);

        run_gc_inner_safe(&gc, ptr);

        allocator_free_pool(&mut gc.borrow_mut().a);
        allocator_malloc_pool(&mut gc.borrow_mut().a, new_sizemem);
    }

    {
        let mut g = gc.borrow_mut();
        let tmp = std::mem::replace(&mut g.a, ALLOCATOR::default());
        g.a = std::mem::replace(&mut g.b, tmp);
    }
}

fn malloc_obj_force_inner(gc: &mut GARBAGE_COLLECTOR, start_properties_cap: usize, sizemem: usize) -> Option<Rc<RefCell<OBJECT>>> {
    let ptr_idx = allocator_malloc_block(&mut gc.a, sizemem);
    // write first byte type
    BLOCK_SET_FIRST_BYTE(ptr_idx, VALUE_TYPE_OBJ);
    // create OBJECT at block data + 1
    let obj = OBJECT {
        properties_len: 0,
        properties_cap: start_properties_cap,
        properties: Vec::with_capacity(start_properties_cap),
    };
    BLOCK_WRITE_OBJECT(ptr_idx, obj);
    BLOCK_AS_OBJECT(BLOCK_DATA_FROM_INDEX(Some(ptr_idx)))
}

pub fn garbage_collector_malloc_obj(gc: garbage_collector_type_t, start_properties_num: usize) -> Option<Rc<RefCell<OBJECT>>> {
    let start_properties_cap = start_properties_num * 2;
    let sizemem = std::mem::size_of::<u8>() + std::mem::size_of::<OBJECT>() + std::mem::size_of::<PROPERTY>() * start_properties_cap;

    if gc.borrow().a.free_list.last.is_some() && (BLOCK_L_DATA_LEN(gc.borrow().a.free_list.last) >= (sizemem + BLOCK_OVERHEAD)) {
        return malloc_obj_force_inner(&mut gc.borrow_mut(), start_properties_cap, sizemem);
    }

    // need garbage collection
    run_gc(gc.clone(), None, sizemem);

    malloc_obj_force_inner(&mut gc.borrow_mut(), start_properties_cap, sizemem)
}

fn change_vals_ptr(val: &mut VALUE, prev_ptr: usize, new_ptr: usize) {
    if (val.r#type == VALUE_TYPE_OBJ) && (val.obj_val == Some(prev_ptr.saturating_add(1))) {
        val.obj_val = Some(new_ptr.saturating_add(1));
    } else if (val.r#type == VALUE_TYPE_ARR) && (val.arr_val == Some(prev_ptr.saturating_add(1))) {
        val.arr_val = Some(new_ptr.saturating_add(1));
    }
}

fn change_one_ptr_inner(gc: &mut GARBAGE_COLLECTOR, prev_ptr: usize, new_ptr: usize) {
    if let Some(stack_rc) = gc.stack.as_ref() {
        let mut stack_ref = stack_rc.borrow_mut();
        for val in stack_ref.iter_mut() {
            change_vals_ptr(val, prev_ptr, new_ptr);
        }
    }

    let mut cur = gc.a.busy_list.first;
    while let Some(_c) = cur {
        let ptr_data_idx = BLOCK_DATA_FROM_INDEX(cur);
        let first_byte = BLOCK_FIRST_BYTE(ptr_data_idx);
        if first_byte == VALUE_TYPE_OBJ {
            if let Some(mut obj) = BLOCK_AS_OBJECT(ptr_data_idx) {
                for i in 0..obj.properties_len {
                    if i < obj.properties.len() {
                        let val = &mut obj.properties[i].val;
                        change_vals_ptr(val, prev_ptr, new_ptr);
                    }
                }
            }
        } else if first_byte == VALUE_TYPE_ARR {
            if let Some(mut arr) = BLOCK_AS_ARRAY(ptr_data_idx) {
                for i in 0..arr.len {
                    if i < arr.values.len() {
                        let val = &mut arr.values[i];
                        change_vals_ptr(val, prev_ptr, new_ptr);
                    }
                }
            }
        }
        cur = BLOCK_LIST_NEXT(cur);
    }
}

fn realloc_obj_force_inner(gc: &mut GARBAGE_COLLECTOR, obj: Rc<RefCell<OBJECT>>, new_properties_cap: usize, sizemem: usize) -> Option<Rc<RefCell<OBJECT>>> {
    // derive prev_ptr from object location
    let prev_ptr = object_to_block_index(&obj) ;
    let new_ptr = allocator_realloc_block(&mut gc.a, prev_ptr, sizemem);

    change_one_ptr_inner(gc, prev_ptr, new_ptr);
    let mut new_obj = obj.borrow_mut();
    new_obj.properties_cap = new_properties_cap;
    Some(Rc::new(RefCell::new(new_obj.clone())))
}

pub fn garbage_collector_realloc_obj(gc: garbage_collector_type_t, obj: Rc<RefCell<OBJECT>>, new_properties_num: usize) -> Option<Rc<RefCell<OBJECT>>> {
    let new_properties_cap = new_properties_num * 2;
    let sizemem = std::mem::size_of::<u8>() + std::mem::size_of::<OBJECT>() + std::mem::size_of::<PROPERTY>() * new_properties_cap;

    if gc.borrow().a.free_list.last.is_some() && (BLOCK_L_DATA_LEN(gc.borrow().a.free_list.last) >= (sizemem + BLOCK_OVERHEAD)) {
        return realloc_obj_force_inner(&mut gc.borrow_mut(), obj, new_properties_cap, sizemem);
    }

    // need garbage collection
    // pass pointer as usize derived from obj
    let obj_ptr = object_to_block_index(&obj);
    run_gc(gc.clone(), Some(obj_ptr), sizemem);

    realloc_obj_force_inner(&mut gc.borrow_mut(), obj, new_properties_cap, sizemem)
}

pub fn garbage_collector_free(gc: garbage_collector_type_t) {
    allocator_free_pool(&mut gc.borrow_mut().a);
    allocator_free_pool(&mut gc.borrow_mut().b);
    // Rc will be dropped automatically when out of scope
}

fn malloc_arr_force_inner(gc: &mut GARBAGE_COLLECTOR, start_cap: usize, sizemem: usize) -> Option<Rc<RefCell<ARRAY>>> {
    let ptr_idx = allocator_malloc_block(&mut gc.a, sizemem);
    BLOCK_SET_FIRST_BYTE(ptr_idx, VALUE_TYPE_ARR);
    let arr = ARRAY {
        len: 0,
        cap: start_cap,
        values: Vec::with_capacity(start_cap),
    };
    BLOCK_WRITE_ARRAY(ptr_idx, arr);
    BLOCK_AS_ARRAY(BLOCK_DATA_FROM_INDEX(Some(ptr_idx)))
}

pub fn garbage_collector_malloc_arr(gc: garbage_collector_type_t, arr_len: usize) -> Option<Rc<RefCell<ARRAY>>> {
    let start_arr_cap = arr_len * 2;
    let sizemem = std::mem::size_of::<u8>() + std::mem::size_of::<ARRAY>() + std::mem::size_of::<VALUE>() * start_arr_cap;

    if gc.borrow().a.free_list.last.is_some() && (BLOCK_L_DATA_LEN(gc.borrow().a.free_list.last) >= (sizemem + BLOCK_OVERHEAD)) {
        return malloc_arr_force_inner(&mut gc.borrow_mut(), start_arr_cap, sizemem);
    }

    run_gc(gc.clone(), None, sizemem);

    malloc_arr_force_inner(&mut gc.borrow_mut(), start_arr_cap, sizemem)
}

fn realloc_arr_force_inner(gc: &mut GARBAGE_COLLECTOR, arr: Rc<RefCell<ARRAY>>, new_arr_cap: usize, sizemem: usize) -> Option<Rc<RefCell<ARRAY>>> {
    let prev_ptr = array_to_block_index(&arr);
    let new_ptr = allocator_realloc_block(&mut gc.a, prev_ptr, sizemem);

    change_one_ptr_inner(gc, prev_ptr, new_ptr);
    let mut new_arr = arr.borrow_mut();
    new_arr.cap = new_arr_cap;
    Some(Rc::new(RefCell::new(new_arr.clone())))
}

pub fn garbage_collector_realloc_arr(gc: garbage_collector_type_t, arr: Rc<RefCell<ARRAY>>, new_arr_len: usize) -> Option<Rc<RefCell<ARRAY>>> {
    let new_arr_cap = new_arr_len * 2;
    let sizemem = std::mem::size_of::<ARRAY>() + std::mem::size_of::<VALUE>() * new_arr_cap;

    if gc.borrow().a.free_list.last.is_some() && (BLOCK_L_DATA_LEN(gc.borrow().a.free_list.last) >= (sizemem + BLOCK_OVERHEAD)) {
        return realloc_arr_force_inner(&mut gc.borrow_mut(), arr, new_arr_cap, sizemem);
    }

    let arr_ptr = array_to_block_index(&arr);
    run_gc(gc.clone(), Some(arr_ptr), sizemem);

    realloc_arr_force_inner(&mut gc.borrow_mut(), arr, new_arr_cap, sizemem)
}

/* -----------------------
   Placeholder helpers
   ----------------------- */

/* The following helper functions and constants are placeholders to emulate
   the behavior of macros and external functions referenced in the original
   C code. They are intentionally minimal and act as adapters so that the
   transpiled functions can express the original algorithm in safe Rust.
   Implementations of allocator_*, ptrs_map_* and block helpers are assumed
   to exist elsewhere in the project. */

pub const VALUE_TYPE_OBJ: i32 = 1;
pub const VALUE_TYPE_ARR: i32 = 2;
pub const BLOCK_OVERHEAD: usize = 0;

fn allocator_malloc_pool(_a: &mut ALLOCATOR, _sizemem_start: usize) {
    // external
}

fn allocator_free_pool(_a: &mut ALLOCATOR) {
    // external
}

fn allocator_clean_pool(_a: &mut ALLOCATOR) {
    // external
}

fn allocator_malloc_block(_a: &mut ALLOCATOR, _sizemem: usize) -> usize {
    0usize
}

fn allocator_realloc_block(_a: &mut ALLOCATOR, _prev_ptr: usize, _sizemem: usize) -> usize {
    0usize
}

fn ptrs_map_get(_m: &mut PTRS_MAP, _o: usize) -> Option<usize> {
    None
}

fn ptrs_map_set(_m: &mut PTRS_MAP, _o: usize, _val: Option<usize>) {
    // external
}

fn ptrs_map_conf(_m: &mut PTRS_MAP, _count: usize) {
    // external
}

fn ptrs_map_free(_m: &mut PTRS_MAP) {
    // external
}

fn BLOCK_L_DATA_LEN(_idx: Option<usize>) -> usize {
    0usize
}

fn BLOCK_DATA_FROM_INDEX(_idx: Option<usize>) -> usize {
    0usize
}

fn BLOCK_DATA(_idx: usize) -> usize {
    _idx
}

fn BLOCK_LIST_NEXT(_idx: Option<usize>) -> Option<usize> {
    None
}

fn BLOCK_SET_FIRST_BYTE(_idx: usize, _b: i32) {
    // external
}

fn BLOCK_FIRST_BYTE(_idx: usize) -> i32 {
    0
}

fn BLOCK_AS_OBJECT(_idx: usize) -> Option<Rc<RefCell<OBJECT>>> {
    None
}

fn BLOCK_AS_ARRAY(_idx: usize) -> Option<Rc<RefCell<ARRAY>>> {
    None
}

fn BLOCK_WRITE_OBJECT(_idx: usize, _obj: OBJECT) {
    // external
}

fn BLOCK_WRITE_ARRAY(_idx: usize, _arr: ARRAY) {
    // external
}

fn memcpy_blocks(_dst: usize, _src: usize, _len: usize) {
    // external
}

fn BLOCK_DATA_FROM_RAW(_raw: usize) -> usize {
    _raw
}

// Helpers to derive block indices from high-level Rc objects.
// In a real integration these would consult allocator metadata.

fn object_to_block_index(_obj: &Rc<RefCell<OBJECT>>) -> usize {
    0usize
}

fn array_to_block_index(_arr: &Rc<RefCell<ARRAY>>) -> usize {
    0usize
}