use std::process::exit;

const BLOCK_L_OVERHEAD: usize =
    std::mem::size_of::<char>() + std::mem::size_of::<usize>() + 2 * std::mem::size_of::<*mut u8>();
const BLOCK_R_OVERHEAD: usize = std::mem::size_of::<usize>() + std::mem::size_of::<char>();
const BLOCK_OVERHEAD: usize = BLOCK_L_OVERHEAD + BLOCK_R_OVERHEAD;
const MIN_BLOCK_LEN: usize = BLOCK_OVERHEAD + std::mem::size_of::<char>();
const FREE_BLOCK: u8 = 0;
const BUSY_BLOCK: u8 = 1;
const ANY_BLOCK: u8 = 2;

fn BLOCK_L_FLAG(block: *mut u8) -> &'static mut u8 {
    unsafe { &mut *(block as *mut u8) }
}

fn BLOCK_L_DATA_LEN(block: *mut u8) -> &'static mut usize {
    unsafe { &mut *((block as usize + std::mem::size_of::<char>()) as *mut usize) }
}

fn BLOCK_LIST_PREV(block: *mut u8) -> &'static mut *mut u8 {
    unsafe {
        &mut *((block as usize + std::mem::size_of::<char>() + std::mem::size_of::<usize>())
            as *mut *mut u8)
    }
}

fn BLOCK_LIST_NEXT(block: *mut u8) -> &'static mut *mut u8 {
    unsafe {
        &mut *((block as usize
            + std::mem::size_of::<char>()
            + std::mem::size_of::<usize>()
            + std::mem::size_of::<*mut u8>()) as *mut *mut u8)
    }
}

fn BLOCK_DATA(block: *mut u8) -> *mut u8 {
    (block as usize
        + std::mem::size_of::<char>()
        + std::mem::size_of::<usize>()
        + 2 * std::mem::size_of::<*mut u8>()) as *mut u8
}

fn BLOCK_R_DATA_LEN(block: *mut u8) -> &'static mut usize {
    unsafe {
        let offset = std::mem::size_of::<char>()
            + std::mem::size_of::<usize>()
            + 2 * std::mem::size_of::<*mut u8>()
            + (*BLOCK_L_DATA_LEN(block) as usize);
        &mut *((block as usize + offset) as *mut usize)
    }
}

fn BLOCK_R_FLAG(block: *mut u8) -> &'static mut u8 {
    unsafe {
        let offset = std::mem::size_of::<char>()
            + std::mem::size_of::<usize>()
            + 2 * std::mem::size_of::<*mut u8>()
            + (*BLOCK_L_DATA_LEN(block) as usize)
            + std::mem::size_of::<usize>();
        &mut *((block as usize + offset) as *mut u8)
    }
}

fn BLOCK_ARR_NEXT(block: *mut u8) -> *mut u8 {
    unsafe {
        let offset = std::mem::size_of::<char>()
            + std::mem::size_of::<usize>()
            + 2 * std::mem::size_of::<*mut u8>()
            + (*BLOCK_L_DATA_LEN(block) as usize)
            + std::mem::size_of::<usize>()
            + std::mem::size_of::<char>();
        (block as usize + offset) as *mut u8
    }
}

fn BLOCK_PTR_FROM_DATA(block: *mut u8) -> *mut u8 {
    (block as usize
        - 2 * std::mem::size_of::<*mut u8>()
        - std::mem::size_of::<usize>()
        - std::mem::size_of::<char>()) as *mut u8
}

fn BLOCK_ARR_PREV_INNER(block: *mut u8) -> *mut u8 {
    (block as usize - std::mem::size_of::<char>() - std::mem::size_of::<usize>()) as *mut u8
}

fn BLOCK_ARR_PREV(block: *mut u8) -> *mut u8 {
    unsafe {
        let inner = BLOCK_ARR_PREV_INNER(block);
        let data_len = *(inner as *mut usize);
        (inner as usize
            - data_len
            - 2 * std::mem::size_of::<*mut u8>()
            - std::mem::size_of::<usize>()
            - std::mem::size_of::<char>()) as *mut u8
    }
}

#[derive(Clone)]
pub struct ALLOCATOR_LIST {
    pub first: *mut u8,
    pub last: *mut u8,
    pub count: usize,
    pub sizemem: usize,
}

impl Default for ALLOCATOR_LIST {
    fn default() -> Self {
        Self {
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
            count: 0,
            sizemem: 0,
        }
    }
}

#[derive(Clone)]
pub struct ALLOCATOR {
    pub mem: *mut u8,
    pub sizemem: usize,
    pub free_list: ALLOCATOR_LIST,
    pub busy_list: ALLOCATOR_LIST,
}

impl Default for ALLOCATOR {
    fn default() -> Self {
        Self {
            mem: std::ptr::null_mut(),
            sizemem: 0,
            free_list: ALLOCATOR_LIST::default(),
            busy_list: ALLOCATOR_LIST::default(),
        }
    }
}

pub type allocator_type_t = *mut ALLOCATOR;

pub fn allocator_malloc_pool(a: allocator_type_t, sizemem: usize) {
    if sizemem < MIN_BLOCK_LEN {
        eprintln!("not enough memory for allocator:");
        eprintln!("got:         {} bytes", sizemem);
        eprintln!("minimum:     {} bytes;", MIN_BLOCK_LEN);
        eprintln!("recommended: {} bytes;", MIN_BLOCK_LEN * 1024 * 1024);
        exit(1);
    }

    unsafe {
        (*a).sizemem = sizemem;
        (*a).mem = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(sizemem, 1));
        allocator_clean_pool(a);
    }
}

pub fn allocator_free_pool(a: allocator_type_t) {
    unsafe {
        if !(*a).mem.is_null() {
            std::alloc::dealloc(
                (*a).mem,
                std::alloc::Layout::from_size_align_unchecked((*a).sizemem, 1),
            );
            (*a).mem = std::ptr::null_mut();
        }
    }
}

pub fn allocator_clean_pool(a: allocator_type_t) {
    unsafe {
        let len = (*a).sizemem - BLOCK_OVERHEAD;
        let mem = (*a).mem;

        *BLOCK_L_FLAG(mem) = FREE_BLOCK;
        *BLOCK_L_DATA_LEN(mem) = len;
        *BLOCK_LIST_PREV(mem) = std::ptr::null_mut();
        *BLOCK_LIST_NEXT(mem) = std::ptr::null_mut();
        *BLOCK_R_DATA_LEN(mem) = len;
        *BLOCK_R_FLAG(mem) = FREE_BLOCK;

        (*a).free_list.first = mem;
        (*a).free_list.last = mem;
        (*a).free_list.count = 1;
        (*a).free_list.sizemem = (*a).sizemem - BLOCK_OVERHEAD;
        (*a).busy_list.first = std::ptr::null_mut();
        (*a).busy_list.last = std::ptr::null_mut();
        (*a).busy_list.count = 0;
        (*a).busy_list.sizemem = 0;
    }
}

fn allocator_list_remove_elem(list: &mut ALLOCATOR_LIST, elem: *mut u8) {
    unsafe {
        let list_prev = *BLOCK_LIST_PREV(elem);
        let list_next = *BLOCK_LIST_NEXT(elem);

        if !list_prev.is_null() {
            *BLOCK_LIST_NEXT(list_prev) = list_next;
        }

        if !list_next.is_null() {
            *BLOCK_LIST_PREV(list_next) = list_prev;
        }

        if list_prev.is_null() {
            list.first = list_next;
        }

        if list_next.is_null() {
            list.last = list_prev;
        }

        list.count -= 1;
        list.sizemem -= *BLOCK_L_DATA_LEN(elem);
    }
}

fn allocator_list_search_by_sizemem(list: &mut ALLOCATOR_LIST, sizemem: usize) -> *mut u8 {
    unsafe {
        let mut cur = list.first;
        while !cur.is_null() {
            if *BLOCK_L_DATA_LEN(cur) >= sizemem {
                return cur;
            }
            cur = *BLOCK_LIST_NEXT(cur);
        }
        std::ptr::null_mut()
    }
}

fn allocator_list_push_front(list: &mut ALLOCATOR_LIST, elem: *mut u8) {
    unsafe {
        *BLOCK_LIST_NEXT(elem) = list.first;
        *BLOCK_LIST_PREV(elem) = std::ptr::null_mut();

        if !list.first.is_null() {
            *BLOCK_LIST_PREV(list.first) = elem;
        }

        list.first = elem;

        if list.last.is_null() {
            list.last = elem;
        }

        list.count += 1;
        list.sizemem += *BLOCK_L_DATA_LEN(elem);
    }
}

fn allocator_list_push_back(list: &mut ALLOCATOR_LIST, elem: *mut u8) {
    unsafe {
        *BLOCK_LIST_NEXT(elem) = std::ptr::null_mut();
        *BLOCK_LIST_PREV(elem) = list.last;

        if !list.last.is_null() {
            *BLOCK_LIST_NEXT(list.last) = elem;
        }

        list.last = elem;

        if list.first.is_null() {
            list.first = elem;
        }

        list.count += 1;
        list.sizemem += *BLOCK_L_DATA_LEN(elem);
    }
}

fn allocator_list_insert_elem(list: &mut ALLOCATOR_LIST, elem: *mut u8) {
    unsafe {
        let len = *BLOCK_L_DATA_LEN(elem);
        let list_next = allocator_list_search_by_sizemem(list, len);

        if list_next.is_null() {
            allocator_list_push_back(list, elem);
        } else {
            if BLOCK_LIST_PREV(list_next).is_null() {
                allocator_list_push_front(list, elem);
                return;
            }

            let prev_elem = *BLOCK_LIST_PREV(list_next);
            *BLOCK_LIST_PREV(elem) = prev_elem;
            *BLOCK_LIST_NEXT(prev_elem) = elem;
            *BLOCK_LIST_NEXT(elem) = list_next;
            *BLOCK_LIST_PREV(list_next) = elem;

            list.count += 1;
            list.sizemem += *BLOCK_L_DATA_LEN(elem);
        }
    }
}

pub fn allocator_malloc_block(a: allocator_type_t, sizemem: usize) -> *mut u8 {
    unsafe {
        if (*BLOCK_L_DATA_LEN((*a).free_list.last) + BLOCK_OVERHEAD) < sizemem {
            return std::ptr::null_mut();
        }

        let cur = allocator_list_search_by_sizemem(&mut (*a).free_list, sizemem);
        allocator_list_remove_elem(&mut (*a).free_list, cur);

        if *BLOCK_L_DATA_LEN(cur) < (sizemem + MIN_BLOCK_LEN) {
            *BLOCK_L_FLAG(cur) = BUSY_BLOCK;
            *BLOCK_R_FLAG(cur) = BUSY_BLOCK;
            allocator_list_push_back(&mut (*a).busy_list, cur);
        }

        let len = *BLOCK_L_DATA_LEN(cur) - sizemem - BLOCK_OVERHEAD;

        *BLOCK_L_FLAG(cur) = BUSY_BLOCK;
        *BLOCK_L_DATA_LEN(cur) = sizemem;
        *BLOCK_LIST_PREV(cur) = std::ptr::null_mut();
        *BLOCK_LIST_NEXT(cur) = std::ptr::null_mut();
        *BLOCK_R_DATA_LEN(cur) = sizemem;
        *BLOCK_R_FLAG(cur) = BUSY_BLOCK;

        allocator_list_push_back(&mut (*a).busy_list, cur);

        let tmp_block = BLOCK_ARR_NEXT(cur);

        *BLOCK_L_FLAG(tmp_block) = FREE_BLOCK;
        *BLOCK_L_DATA_LEN(tmp_block) = len;
        *BLOCK_LIST_PREV(tmp_block) = std::ptr::null_mut();
        *BLOCK_LIST_NEXT(tmp_block) = std::ptr::null_mut();
        *BLOCK_R_DATA_LEN(tmp_block) = len;
        *BLOCK_R_FLAG(tmp_block) = FREE_BLOCK;

        allocator_list_insert_elem(&mut (*a).free_list, tmp_block);

        BLOCK_DATA(cur)
    }
}

pub fn allocator_realloc_block(a: allocator_type_t, ptrmem: *mut u8, sizemem: usize) -> *mut u8 {
    unsafe {
        if (*BLOCK_L_DATA_LEN((*a).free_list.last) + BLOCK_OVERHEAD) < sizemem {
            return std::ptr::null_mut();
        }

        let orig = BLOCK_PTR_FROM_DATA(ptrmem);
        let cur = allocator_list_search_by_sizemem(&mut (*a).free_list, sizemem);
        allocator_list_remove_elem(&mut (*a).free_list, cur);

        if *BLOCK_L_DATA_LEN(cur) < (sizemem + MIN_BLOCK_LEN) {
            *BLOCK_L_FLAG(cur) = BUSY_BLOCK;
            *BLOCK_R_FLAG(cur) = BUSY_BLOCK;
            allocator_list_push_back(&mut (*a).busy_list, cur);
            std::ptr::copy_nonoverlapping(
                BLOCK_DATA(orig),
                BLOCK_DATA(cur),
                *BLOCK_L_DATA_LEN(orig),
            );
            allocator_free_block(a, ptrmem);
        }

        let len = *BLOCK_L_DATA_LEN(cur) - sizemem - BLOCK_OVERHEAD;

        *BLOCK_L_FLAG(cur) = BUSY_BLOCK;
        *BLOCK_L_DATA_LEN(cur) = sizemem;
        *BLOCK_LIST_PREV(cur) = std::ptr::null_mut();
        *BLOCK_LIST_NEXT(cur) = std::ptr::null_mut();
        *BLOCK_R_DATA_LEN(cur) = sizemem;
        *BLOCK_R_FLAG(cur) = BUSY_BLOCK;

        allocator_list_push_back(&mut (*a).busy_list, cur);

        std::ptr::copy_nonoverlapping(BLOCK_DATA(orig), BLOCK_DATA(cur), *BLOCK_L_DATA_LEN(orig));

        let tmp_block = BLOCK_ARR_NEXT(cur);

        *BLOCK_L_FLAG(tmp_block) = FREE_BLOCK;
        *BLOCK_L_DATA_LEN(tmp_block) = len;
        *BLOCK_LIST_PREV(tmp_block) = std::ptr::null_mut();
        *BLOCK_LIST_NEXT(tmp_block) = std::ptr::null_mut();
        *BLOCK_R_DATA_LEN(tmp_block) = len;
        *BLOCK_R_FLAG(tmp_block) = FREE_BLOCK;

        allocator_list_insert_elem(&mut (*a).free_list, tmp_block);

        allocator_free_block(a, ptrmem);

        BLOCK_DATA(cur)
    }
}

pub fn allocator_free_block(a: allocator_type_t, ptrmem: *mut u8) {
    unsafe {
        let cur = BLOCK_PTR_FROM_DATA(ptrmem);
        let mut arr_prev: *mut u8 = std::ptr::null_mut();
        let mut arr_next: *mut u8 = std::ptr::null_mut();

        if (cur == (*a).mem) || (*BLOCK_L_FLAG(BLOCK_ARR_PREV(cur)) == BUSY_BLOCK) {
            arr_prev = std::ptr::null_mut();
        } else {
            arr_prev = BLOCK_ARR_PREV(cur);
        }

        if ((cur as usize + *BLOCK_L_DATA_LEN(cur) + BLOCK_OVERHEAD)
            >= ((*a).mem as usize + (*a).sizemem))
            || (*BLOCK_L_FLAG(BLOCK_ARR_NEXT(cur)) == BUSY_BLOCK)
        {
            arr_next = std::ptr::null_mut();
        } else {
            arr_next = BLOCK_ARR_NEXT(cur);
        }

        allocator_list_remove_elem(&mut (*a).busy_list, cur);

        if !arr_prev.is_null() && arr_next.is_null() {
            let len = *BLOCK_L_DATA_LEN(arr_prev)
                + BLOCK_R_OVERHEAD
                + BLOCK_L_OVERHEAD
                + *BLOCK_L_DATA_LEN(cur);
            allocator_list_remove_elem(&mut (*a).free_list, arr_prev);
            *BLOCK_L_DATA_LEN(arr_prev) = len;
            *BLOCK_R_DATA_LEN(cur) = len;
            allocator_list_insert_elem(&mut (*a).free_list, arr_prev);
        } else if arr_prev.is_null() && !arr_next.is_null() {
            let len = *BLOCK_L_DATA_LEN(cur)
                + BLOCK_R_OVERHEAD
                + BLOCK_L_OVERHEAD
                + *BLOCK_L_DATA_LEN(arr_next);
            allocator_list_remove_elem(&mut (*a).free_list, arr_next);
            *BLOCK_L_DATA_LEN(cur) = len;
            *BLOCK_R_DATA_LEN(arr_next) = len;
            allocator_list_insert_elem(&mut (*a).free_list, cur);
        } else if !arr_prev.is_null() && !arr_next.is_null() {
            let len = *BLOCK_L_DATA_LEN(arr_prev)
                + BLOCK_R_OVERHEAD
                + BLOCK_L_OVERHEAD
                + *BLOCK_L_DATA_LEN(cur)
                + BLOCK_R_OVERHEAD
                + BLOCK_L_OVERHEAD
                + *BLOCK_L_DATA_LEN(arr_next);
            allocator_list_remove_elem(&mut (*a).free_list, arr_prev);
            allocator_list_remove_elem(&mut (*a).free_list, arr_next);
            *BLOCK_L_DATA_LEN(arr_prev) = len;
            *BLOCK_R_DATA_LEN(arr_next) = len;
            allocator_list_insert_elem(&mut (*a).free_list, arr_prev);
        } else {
            *BLOCK_L_FLAG(cur) = FREE_BLOCK;
            *BLOCK_R_FLAG(cur) = FREE_BLOCK;
            allocator_list_insert_elem(&mut (*a).free_list, cur);
        }
    }
}
