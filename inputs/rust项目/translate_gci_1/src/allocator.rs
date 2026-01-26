use crate::utils::*;
use std::mem::size_of;
use std::process::exit;

pub const BLOCK_L_OVERHEAD: usize =
    size_of::<u8>() + size_of::<usize>() + 2 * size_of::<usize>();
pub const BLOCK_R_OVERHEAD: usize = size_of::<usize>() + size_of::<u8>();
pub const BLOCK_OVERHEAD: usize = BLOCK_L_OVERHEAD + BLOCK_R_OVERHEAD;
pub const MIN_BLOCK_LEN: usize = BLOCK_OVERHEAD + size_of::<u8>();
pub const FREE_BLOCK: u8 = 0;
pub const BUSY_BLOCK: u8 = 1;
pub const ANY_BLOCK: u8 = 2;

#[repr(C)]
pub struct ALLOCATOR_LIST {
    pub first: Option<usize>,
    pub last: Option<usize>,
    pub count: usize,
    pub sizemem: usize,
}

impl Default for ALLOCATOR_LIST {
    fn default() -> Self {
        Self {
            first: None,
            last: None,
            count: 0,
            sizemem: 0,
        }
    }
}

#[repr(C)]
pub struct ALLOCATOR {
    pub mem: Vec<u8>,
    pub sizemem: usize,

    /* free blocks. */
    pub free_list: ALLOCATOR_LIST,

    /* busy blocks. */
    pub busy_list: ALLOCATOR_LIST,
}

impl Default for ALLOCATOR {
    fn default() -> Self {
        Self {
            mem: Vec::new(),
            sizemem: 0,
            free_list: ALLOCATOR_LIST::default(),
            busy_list: ALLOCATOR_LIST::default(),
        }
    }
}

pub type allocator_type_t<'a> = &'a mut ALLOCATOR;

/* Helper safe accessors for the simulated memory buffer. */
/* All block identifiers in this implementation are byte offsets into a.mem Vec<u8>. */

fn read_u8(mem: &[u8], offset: usize) -> u8 {
    mem.get(offset)
        .copied()
        .expect("read_u8 out of bounds")
}

fn write_u8(mem: &mut [u8], offset: usize, val: u8) {
    let b = mem.get_mut(offset).expect("write_u8 out of bounds");
    *b = val;
}

fn read_usize(mem: &[u8], offset: usize) -> usize {
    let sz = size_of::<usize>();
    let slice = mem
        .get(offset..offset + sz)
        .expect("read_usize out of bounds");
    let mut arr = [0u8; size_of::<usize>()];
    arr.copy_from_slice(slice);
    usize::from_ne_bytes(arr)
}

fn write_usize(mem: &mut [u8], offset: usize, val: usize) {
    let sz = size_of::<usize>();
    let bytes = val.to_ne_bytes();
    let dst = mem
        .get_mut(offset..offset + sz)
        .expect("write_usize out of bounds");
    dst.copy_from_slice(&bytes);
}

/* Macro-like helpers translated to functions operating on mem and a block offset. */

fn block_l_flag(mem: &mut [u8], block: usize) -> u8 {
    read_u8(mem, block)
}

fn set_block_l_flag(mem: &mut [u8], block: usize, flag: u8) {
    write_u8(mem, block, flag);
}

fn block_l_data_len(mem: &mut [u8], block: usize) -> usize {
    read_usize(mem, block + size_of::<u8>())
}

fn set_block_l_data_len(mem: &mut [u8], block: usize, len: usize) {
    write_usize(mem, block + size_of::<u8>(), len);
}

fn block_list_prev(mem: &mut [u8], block: usize) -> Option<usize> {
    let off = block + size_of::<u8>() + size_of::<usize>();
    let v = read_usize(mem, off);
    if v == 0 {
        None
    } else {
        Some(v)
    }
}

fn set_block_list_prev(mem: &mut [u8], block: usize, p: Option<usize>) {
    let off = block + size_of::<u8>() + size_of::<usize>();
    write_usize(mem, off, p.unwrap_or(0));
}

fn block_list_next(mem: &mut [u8], block: usize) -> Option<usize> {
    let off = block + size_of::<u8>() + size_of::<usize>() + size_of::<usize>();
    let v = read_usize(mem, off);
    if v == 0 {
        None
    } else {
        Some(v)
    }
}

fn set_block_list_next(mem: &mut [u8], block: usize, n: Option<usize>) {
    let off = block + size_of::<u8>() + size_of::<usize>() + size_of::<usize>();
    write_usize(mem, off, n.unwrap_or(0));
}

fn block_data_offset(block: usize) -> usize {
    block + BLOCK_L_OVERHEAD
}

fn block_r_data_len(mem: &mut [u8], block: usize) -> usize {
    let l_len = block_l_data_len(mem, block);
    let off = block + BLOCK_L_OVERHEAD + l_len;
    read_usize(mem, off)
}

fn set_block_r_data_len(mem: &mut [u8], block: usize, len: usize) {
    let l_len = block_l_data_len(mem, block);
    let off = block + BLOCK_L_OVERHEAD + l_len;
    write_usize(mem, off, len);
}

fn block_r_flag(mem: &mut [u8], block: usize) -> u8 {
    let l_len = block_l_data_len(mem, block);
    let off = block + BLOCK_L_OVERHEAD + l_len + size_of::<usize>();
    read_u8(mem, off)
}

fn set_block_r_flag(mem: &mut [u8], block: usize, flag: u8) {
    let l_len = block_l_data_len(mem, block);
    let off = block + BLOCK_L_OVERHEAD + l_len + size_of::<usize>();
    write_u8(mem, off, flag);
}

fn block_arr_next(mem: &mut [u8], block: usize) -> Option<usize> {
    let l_len = block_l_data_len(mem, block);
    let off = block + BLOCK_OVERHEAD + l_len;
    // If off is beyond buffer, return None
    if off >= mem.len() {
        return None;
    }
    Some(off)
}

fn block_ptr_from_data(data_offset: usize) -> usize {
    data_offset - BLOCK_L_OVERHEAD
}

/* BLOCK_ARR_PREV_INNER and BLOCK_ARR_PREV as per macros, implemented carefully with bounds checks. */
fn block_arr_prev_inner(mem: &mut [u8], block: usize) -> Option<usize> {
    // inner = block - sizeof(char) - sizeof(size_t)
    let sub = size_of::<u8>() + size_of::<usize>();
    if block < sub {
        return None;
    }
    Some(block - sub)
}

fn block_arr_prev(mem: &mut [u8], block: usize) -> Option<usize> {
    // Implements:
    // ((void*) (BLOCK_ARR_PREV_INNER(block) - ((*((size_t*) BLOCK_ARR_PREV_INNER(block))) * sizeof(char)) - sizeof(void*) - sizeof(void*) - sizeof(size_t) - sizeof(char)))
    let inner_opt = block_arr_prev_inner(mem, block)?;
    // read size_t at inner
    if inner_opt + size_of::<usize>() > mem.len() {
        return None;
    }
    let size_chars = read_usize(mem, inner_opt);
    let mut sub = size_chars;
    // subtract pointer sizes and size_t and char
    sub = sub
        + size_of::<usize>() // sizeof(void*)
        + size_of::<usize>() // sizeof(void*)
        + size_of::<usize>() // sizeof(size_t)
        + size_of::<u8>(); // sizeof(char)
    if inner_opt < sub {
        return None;
    }
    Some(inner_opt - sub)
}

/* Public functions translated to operate safely on the ALLOCATOR structure. */

pub fn allocator_malloc_pool<'a>(a: allocator_type_t<'a>, sizemem: usize) {
    if sizemem < MIN_BLOCK_LEN {
        eprintln!("not enough memory for allocator:");
        eprintln!("got:         {} bytes", sizemem);
        eprintln!("minimum:     {} bytes;", MIN_BLOCK_LEN);
        eprintln!("recommended: {} bytes;", MIN_BLOCK_LEN * 1024 * 1024);
        exit(1);
    }

    a.sizemem = sizemem;
    a.mem = vec![0u8; a.sizemem];

    allocator_clean_pool(a);
}

pub fn allocator_free_pool<'a>(a: allocator_type_t<'a>) {
    // In Rust, dropping the Vec deallocates automatically.
    a.mem.clear();
    a.sizemem = 0;
    a.free_list = ALLOCATOR_LIST::default();
    a.busy_list = ALLOCATOR_LIST::default();
}

pub fn allocator_clean_pool<'a>(a: allocator_type_t<'a>) {
    let len = a.sizemem.checked_sub(BLOCK_OVERHEAD).expect("sizemem too small");

    // Initialize the single big free block at offset 0.
    set_block_l_flag(&mut a.mem, 0, FREE_BLOCK);
    set_block_l_data_len(&mut a.mem, 0, len);
    set_block_list_prev(&mut a.mem, 0, None);
    set_block_list_next(&mut a.mem, 0, None);
    set_block_r_data_len(&mut a.mem, 0, len);
    set_block_r_flag(&mut a.mem, 0, FREE_BLOCK);

    a.free_list.first = Some(0);
    a.free_list.last = Some(0);
    a.free_list.count = 1;
    a.free_list.sizemem = a.sizemem - BLOCK_OVERHEAD;

    a.busy_list.first = None;
    a.busy_list.last = None;
    a.busy_list.count = 0;
    a.busy_list.sizemem = 0;
}

pub fn allocator_list_remove_elem<'a>(list: &mut ALLOCATOR_LIST, elem: usize, a: allocator_type_t<'a>) {
    let mem = &mut a.mem;
    let list_prev = block_list_prev(mem, elem);
    let list_next = block_list_next(mem, elem);

    if let Some(lp) = list_prev {
        set_block_list_next(mem, lp, list_next);
    }
    if let Some(ln) = list_next {
        set_block_list_prev(mem, ln, list_prev);
    }

    if list_prev.is_none() {
        list.first = list_next;
    }
    if list_next.is_none() {
        list.last = list_prev;
    }

    list.count = list.count.saturating_sub(1);
    let data_len = block_l_data_len(mem, elem);
    list.sizemem = list.sizemem.saturating_sub(data_len);
}

pub fn allocator_list_search_by_sizemem<'a>(list: &ALLOCATOR_LIST, sizemem: usize, a: allocator_type_t<'a>) -> Option<usize> {
    let mem = &mut a.mem;
    let mut cur = list.first;

    while let Some(c) = cur {
        if block_l_data_len(mem, c) >= sizemem {
            return Some(c);
        }
        cur = block_list_next(mem, c);
    }

    None
}

pub fn allocator_list_push_front<'a>(list: &mut ALLOCATOR_LIST, elem: usize, a: allocator_type_t<'a>) {
    let mem = &mut a.mem;
    set_block_list_next(mem, elem, list.first);
    set_block_list_prev(mem, elem, None);

    if let Some(first) = list.first {
        set_block_list_prev(mem, first, Some(elem));
    }

    list.first = Some(elem);

    if list.last.is_none() {
        list.last = Some(elem);
    }

    list.count += 1;
    list.sizemem += block_l_data_len(mem, elem);
}

pub fn allocator_list_push_back<'a>(list: &mut ALLOCATOR_LIST, elem: usize, a: allocator_type_t<'a>) {
    let mem = &mut a.mem;
    set_block_list_next(mem, elem, None);
    set_block_list_prev(mem, elem, list.last);

    if let Some(last) = list.last {
        set_block_list_next(mem, last, Some(elem));
    }

    list.last = Some(elem);

    if list.first.is_none() {
        list.first = Some(elem);
    }

    list.count += 1;
    list.sizemem += block_l_data_len(mem, elem);
}

pub fn allocator_list_insert_elem<'a>(list: &mut ALLOCATOR_LIST, elem: usize, a: allocator_type_t<'a>) {
    let mem = &mut a.mem;
    let len = block_l_data_len(mem, elem);
    let list_next = allocator_list_search_by_sizemem(list, len, a);

    if list_next.is_none() {
        allocator_list_push_back(list, elem, a);
    } else {
        let list_next = list_next.unwrap();
        if block_list_prev(mem, list_next).is_none() {
            allocator_list_push_front(list, elem, a);
            return;
        }

        let prev_elem = block_list_prev(mem, list_next).unwrap();
        set_block_list_prev(mem, elem, Some(prev_elem));
        set_block_list_next(mem, prev_elem, Some(elem));
        set_block_list_next(mem, elem, Some(list_next));
        set_block_list_prev(mem, list_next, Some(elem));

        list.count += 1;
        list.sizemem += block_l_data_len(mem, elem);
    }
}

pub fn allocator_malloc_block<'a>(a: allocator_type_t<'a>, sizemem: usize) -> Option<usize> {
    // Return Option<usize> which represents the data offset pointer (BLOCK_DATA).
    // If allocation fails, return None.

    // Quick check if largest free block can satisfy request
    if a.free_list.last.is_none() {
        return None;
    }
    let last_block = a.free_list.last.unwrap();
    if block_l_data_len(&mut a.mem, last_block)
        .checked_add(BLOCK_OVERHEAD)
        .unwrap_or(0)
        < sizemem
    {
        return None;
    }

    let cur_opt = allocator_list_search_by_sizemem(&a.free_list, sizemem, a);
    let cur = match cur_opt {
        Some(c) => c,
        None => return None,
    };

    // remove block from free list
    allocator_list_remove_elem(&mut a.free_list, cur, a);

    if block_l_data_len(&mut a.mem, cur) < (sizemem + MIN_BLOCK_LEN) {
        // don't need to divide block.
        set_block_l_flag(&mut a.mem, cur, BUSY_BLOCK);
        set_block_r_flag(&mut a.mem, cur, BUSY_BLOCK);
        allocator_list_push_back(&mut a.busy_list, cur, a);

        return Some(block_data_offset(cur));
    }

    let len = block_l_data_len(&mut a.mem, cur) - sizemem - BLOCK_OVERHEAD;
    // split block into two: cur (busy) and tmp_block (free)
    set_block_l_flag(&mut a.mem, cur, BUSY_BLOCK);
    set_block_l_data_len(&mut a.mem, cur, sizemem);
    set_block_list_prev(&mut a.mem, cur, None);
    set_block_list_next(&mut a.mem, cur, None);
    set_block_r_data_len(&mut a.mem, cur, sizemem);
    set_block_r_flag(&mut a.mem, cur, BUSY_BLOCK);
    allocator_list_push_back(&mut a.busy_list, cur, a);

    // put free block to free list
    let tmp_block = block_arr_next(&mut a.mem, cur).expect("tmp_block out of bounds");
    set_block_l_flag(&mut a.mem, tmp_block, FREE_BLOCK);
    set_block_l_data_len(&mut a.mem, tmp_block, len);
    set_block_list_prev(&mut a.mem, tmp_block, None);
    set_block_list_next(&mut a.mem, tmp_block, None);
    set_block_r_data_len(&mut a.mem, tmp_block, len);
    set_block_r_flag(&mut a.mem, tmp_block, FREE_BLOCK);
    allocator_list_insert_elem(&mut a.free_list, tmp_block, a);

    Some(block_data_offset(cur))
}

pub fn allocator_realloc_block<'a>(a: allocator_type_t<'a>, ptrmem: usize, sizemem: usize) -> Option<usize> {
    // ptrmem is data offset
    if a.free_list.last.is_none() {
        return None;
    }
    let last_block = a.free_list.last.unwrap();
    if block_l_data_len(&mut a.mem, last_block)
        .checked_add(BLOCK_OVERHEAD)
        .unwrap_or(0)
        < sizemem
    {
        return None;
    }

    let orig = block_ptr_from_data(ptrmem);

    let cur_opt = allocator_list_search_by_sizemem(&a.free_list, sizemem, a);
    let cur = match cur_opt {
        Some(c) => c,
        None => return None,
    };

    allocator_list_remove_elem(&mut a.free_list, cur, a);

    if block_l_data_len(&mut a.mem, cur) < (sizemem + MIN_BLOCK_LEN) {
        // don't need to divide block.
        set_block_l_flag(&mut a.mem, cur, BUSY_BLOCK);
        set_block_r_flag(&mut a.mem, cur, BUSY_BLOCK);
        allocator_list_push_back(&mut a.busy_list, cur, a);
        // copy data
        let orig_len = block_l_data_len(&mut a.mem, orig);
        let src_off = block_data_offset(orig);
        let dst_off = block_data_offset(cur);
        let copy_len = orig_len.min(block_l_data_len(&mut a.mem, cur));
        let src_slice = a.mem[src_off..src_off + copy_len].to_vec();
        a.mem[dst_off..dst_off + copy_len].copy_from_slice(&src_slice);
        allocator_free_block(a, ptrmem);
        return Some(block_data_offset(cur));
    }

    let len = block_l_data_len(&mut a.mem, cur) - sizemem - BLOCK_OVERHEAD;
    // split block into two
    set_block_l_flag(&mut a.mem, cur, BUSY_BLOCK);
    set_block_l_data_len(&mut a.mem, cur, sizemem);
    set_block_list_prev(&mut a.mem, cur, None);
    set_block_list_next(&mut a.mem, cur, None);
    set_block_r_data_len(&mut a.mem, cur, sizemem);
    set_block_r_flag(&mut a.mem, cur, BUSY_BLOCK);
    allocator_list_push_back(&mut a.busy_list, cur, a);

    // copy data
    let orig_len = block_l_data_len(&mut a.mem, orig);
    let src_off = block_data_offset(orig);
    let dst_off = block_data_offset(cur);
    let copy_len = orig_len.min(block_l_data_len(&mut a.mem, cur));
    let src_slice = a.mem[src_off..src_off + copy_len].to_vec();
    a.mem[dst_off..dst_off + copy_len].copy_from_slice(&src_slice);

    // put free remainder to free list
    let tmp_block = block_arr_next(&mut a.mem, cur).expect("tmp_block out of bounds");
    set_block_l_flag(&mut a.mem, tmp_block, FREE_BLOCK);
    set_block_l_data_len(&mut a.mem, tmp_block, len);
    set_block_list_prev(&mut a.mem, tmp_block, None);
    set_block_list_next(&mut a.mem, tmp_block, None);
    set_block_r_data_len(&mut a.mem, tmp_block, len);
    set_block_r_flag(&mut a.mem, tmp_block, FREE_BLOCK);
    allocator_list_insert_elem(&mut a.free_list, tmp_block, a);

    allocator_free_block(a, ptrmem);

    Some(block_data_offset(cur))
}

pub fn allocator_free_block<'a>(a: allocator_type_t<'a>, ptrmem: usize) {
    let cur = block_ptr_from_data(ptrmem);
    let mut arr_prev: Option<usize> = None;
    let mut arr_next: Option<usize> = None;

    // arr_prev determination
    if cur == 0 {
        arr_prev = None;
    } else {
        if let Some(prev_block) = block_arr_prev(&mut a.mem, cur) {
            if block_l_flag(&mut a.mem, prev_block) == BUSY_BLOCK {
                arr_prev = None;
            } else {
                arr_prev = Some(prev_block);
            }
        } else {
            arr_prev = None;
        }
    }

    // arr_next determination
    let potential_next = block_arr_next(&mut a.mem, cur);
    if potential_next.is_none() {
        arr_next = None;
    } else {
        let next_block = potential_next.unwrap();
        // check bounds: if (cur + BLOCK_L_DATA_LEN(cur) + BLOCK_OVERHEAD) >= (a.mem + a.sizemem)
        let end_of_cur = cur
            .checked_add(block_l_data_len(&mut a.mem, cur))
            .and_then(|v| v.checked_add(BLOCK_OVERHEAD));
        if end_of_cur.is_none() || end_of_cur.unwrap() >= a.sizemem {
            arr_next = None;
        } else {
            if block_l_flag(&mut a.mem, next_block) == BUSY_BLOCK {
                arr_next = None;
            } else {
                arr_next = Some(next_block);
            }
        }
    }

    // remove from busy list
    allocator_list_remove_elem(&mut a.busy_list, cur, a);

    if arr_prev.is_some() && arr_next.is_none() {
        // concatenate previous free array block with current block.
        let prev = arr_prev.unwrap();
        let len = block_l_data_len(&mut a.mem, prev)
            + BLOCK_R_OVERHEAD
            + BLOCK_L_OVERHEAD
            + block_l_data_len(&mut a.mem, cur);
        allocator_list_remove_elem(&mut a.free_list, prev, a);
        set_block_l_data_len(&mut a.mem, prev, len);
        set_block_r_data_len(&mut a.mem, cur, len);
        allocator_list_insert_elem(&mut a.free_list, prev, a);
    } else if arr_prev.is_none() && arr_next.is_some() {
        // concatenate current block with next free array block.
        let next = arr_next.unwrap();
        let len = block_l_data_len(&mut a.mem, cur)
            + BLOCK_R_OVERHEAD
            + BLOCK_L_OVERHEAD
            + block_l_data_len(&mut a.mem, next);
        allocator_list_remove_elem(&mut a.free_list, next, a);
        set_block_l_data_len(&mut a.mem, cur, len);
        set_block_r_data_len(&mut a.mem, next, len);
        allocator_list_insert_elem(&mut a.free_list, cur, a);
    } else if arr_prev.is_some() && arr_next.is_some() {
        // concatenate previous free array block with current block with next free array block.
        let prev = arr_prev.unwrap();
        let next = arr_next.unwrap();
        let len = block_l_data_len(&mut a.mem, prev)
            + BLOCK_R_OVERHEAD
            + BLOCK_L_OVERHEAD
            + block_l_data_len(&mut a.mem, cur)
            + BLOCK_R_OVERHEAD
            + BLOCK_L_OVERHEAD
            + block_l_data_len(&mut a.mem, next);
        allocator_list_remove_elem(&mut a.free_list, prev, a);
        allocator_list_remove_elem(&mut a.free_list, next, a);
        set_block_l_data_len(&mut a.mem, prev, len);
        set_block_r_data_len(&mut a.mem, next, len);
        allocator_list_insert_elem(&mut a.free_list, prev, a);
    } else {
        set_block_l_flag(&mut a.mem, cur, FREE_BLOCK);
        set_block_r_flag(&mut a.mem, cur, FREE_BLOCK);
        allocator_list_insert_elem(&mut a.free_list, cur, a);
    }
}