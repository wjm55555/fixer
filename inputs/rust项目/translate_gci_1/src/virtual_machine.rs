use crate::data_types::*;
use crate::utils::*;
use crate::garbage_collector::*;
use crate::bytecode_generator::*;
pub type size_t = usize;
pub type long_long = i64;

#[repr(C)]
pub struct VIRTUAL_MACHINE {
    pub bc: bytecode_type_t,
    pub ip: usize,
    
    pub stack: Vec<struct VALUE>,
    pub stack_top: usize,
    pub stack_cap: usize,

    pub gc: garbage_collector_type_t,

    pub trace: i32,
}

pub type virtual_machine_type_t = Box<VIRTUAL_MACHINE>;

impl Default for VIRTUAL_MACHINE {
    fn default() -> Self {
        VIRTUAL_MACHINE {
            bc: Default::default(),
            ip: 0,
            stack: Vec::new(),
            stack_top: 0,
            stack_cap: 0,
            gc: Default::default(),
            trace: 0,
        }
    }
}

pub fn READ_BYTE(vm: &mut VIRTUAL_MACHINE) -> usize {
    let b = vm.bc.op_codes[vm.ip];
    vm.ip += 1;
    b
}

pub fn create_value_from_int(int_val: long_long) -> struct VALUE {
    let mut val: struct VALUE = Default::default();
    val.type = VALUE_TYPE_INTEGER;
    val.int_val = int_val;
    val
}

pub fn create_value_from_obj(obj_val: *mut struct OBJECT) -> struct VALUE {
    let mut val: struct VALUE = Default::default();
    val.obj_val = obj_val;
    val.type = VALUE_TYPE_OBJ;
    val
}

pub fn create_value_from_arr(arr_val: *mut struct ARRAY) -> struct VALUE {
    let mut val: struct VALUE = Default::default();
    val.arr_val = arr_val;
    val.type = VALUE_TYPE_ARR;
    val
}

pub fn create_virtual_machine() -> virtual_machine_type_t {
    Box::new(VIRTUAL_MACHINE::default())
}

pub fn virtual_machine_conf(vm: &mut VIRTUAL_MACHINE, bc: bytecode_type_t, stack_size: size_t, start_heap_size_b: size_t, trace: i32) {
    vm.bc = bc;
    vm.ip = 0; // start at beginning of bc->op_codes

    vm.stack_cap = stack_size;
    vm.stack = vec![Default::default(); vm.stack_cap];
    vm.stack_top = 0;

    vm.gc = create_garbage_collector();
    garbage_collector_conf(&mut vm.gc, start_heap_size_b, &mut vm.stack, &mut vm.stack_top, trace);

    vm.trace = trace;
}

pub fn virtual_machine_stack_push(vm: &mut VIRTUAL_MACHINE, val: struct VALUE) {
    if vm.stack_top >= vm.stack.len() {
        vm.stack.push(val);
        vm.stack_top += 1;
    } else {
        vm.stack[vm.stack_top] = val;
        vm.stack_top += 1;
    }
}

pub fn virtual_machine_stack_pop(vm: &mut VIRTUAL_MACHINE) -> struct VALUE {
    vm.stack_top -= 1;
    vm.stack[vm.stack_top].clone()
}

pub fn create_obj(vm: &mut VIRTUAL_MACHINE) -> *mut struct OBJECT {
    let properties_num = READ_BYTE(vm);
    
    let obj = garbage_collector_malloc_obj(&mut vm.gc, properties_num);
    
    for i in 0..properties_num {
        let key = virtual_machine_stack_pop(vm);
        let val = virtual_machine_stack_pop(vm);
        
        unsafe {
            (*obj).properties[i].key = key.int_val;
            (*obj).properties[i].val = val;
        }
    }
    
    unsafe {
        (*obj).properties_len = properties_num;
        (*obj).properties_cap = properties_num * 2;
    }

    obj
}

pub fn create_arr(vm: &mut VIRTUAL_MACHINE) -> *mut struct ARRAY {
    let arr_len = READ_BYTE(vm);

    let arr = garbage_collector_malloc_arr(&mut vm.gc, arr_len);

    for i in 0..arr_len {
        let val = virtual_machine_stack_pop(vm);
        unsafe {
            (*arr).values[i] = val;
        }
    }
    unsafe {
        (*arr).len = arr_len;
    }

    arr
}

pub fn virtual_machine_run(vm: &mut VIRTUAL_MACHINE) -> long_long {
    if vm.trace != 0 {
        println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        println!("<trace>");
    }
    
    loop {
        let instruction = READ_BYTE(vm);

        if vm.trace != 0 {
            println!("\t<step>OP: {}; SS: {} b; HS: {} b; FB: {}; BB: {}</step>",
                   instruction, (vm.stack_top as isize - vm.stack.len() as isize + vm.stack.len() as isize), vm.gc.a.sizemem,
                   vm.gc.a.free_list.count, vm.gc.a.busy_list.count);
        }
        
        match instruction {
        x if x == BC_OP_POP => {
            virtual_machine_stack_pop(vm);
        }

        x if x == BC_OP_CONSTANT => {
            let cnst = vm.bc.constant_pool[READ_BYTE(vm)];
            let val = create_value_from_int(cnst.int_cnst);
            virtual_machine_stack_push(vm, val);
        }

        x if x == BC_OP_SET_LOCAL => {
            let idx = READ_BYTE(vm);
            let top_idx = vm.stack_top - 1;
            vm.stack[idx] = vm.stack[top_idx].clone();
            if idx != (vm.stack_top - vm.stack.len() + vm.stack.len() - 1) {
                virtual_machine_stack_pop(vm);
            }
        }
        x if x == BC_OP_GET_LOCAL => {
            let idx = READ_BYTE(vm);
            virtual_machine_stack_push(vm, vm.stack[idx].clone());
        }

        x if x == BC_OP_CREATE_OBJ => {
            let obj = create_obj(vm);
            let val = create_value_from_obj(obj);
            virtual_machine_stack_push(vm, val);
        }
        x if x == BC_OP_INIT_OBJ_PROP => {
            let val = create_value_from_int(READ_BYTE(vm) as long_long);
            virtual_machine_stack_push(vm, val);
        }

        x if x == BC_OP_CREATE_ARR => {
            let arr = create_arr(vm);
            let val = create_value_from_arr(arr);
            virtual_machine_stack_push(vm, val);
        }
            
        x if x == BC_OP_SET_HEAP => {
            let idx = READ_BYTE(vm);
            let len = READ_BYTE(vm);
            let mut pops: usize = 0;
            let mut val = vm.stack[idx].clone();
            for i in 0..len {
                let hop = READ_BYTE(vm);
                if hop == BC_ARRAY_INDEX {
                    let offset = READ_BYTE(vm);
                    let index = vm.stack[vm.stack_top - offset - 1].clone();
                    if val.type != VALUE_TYPE_ARR {
                        println!("attempt to index non-array value");
                        std::process::exit(1);
                    }
                    if index.type != VALUE_TYPE_INTEGER {
                        println!("attempt to use non-integer value as array index");
                        std::process::exit(1);
                    }
                    if index.int_val < 0 {
                        println!("invalid array index: {}", index.int_val);
                        std::process::exit(1);
                    }
                    if (index.int_val as usize) > unsafe { (*val.arr_val).len } {
                        println!("array index to unitialized data: {}", index.int_val);
                        std::process::exit(1);
                    }
                    pops += 1;
                    if i < len - 1 {
                        unsafe {
                            val = (*val.arr_val).values[index.int_val as usize].clone();
                        }
                    } else if i == len - 1 {
                        for _k in 0..pops {
                            virtual_machine_stack_pop(vm);
                        }

                        unsafe {
                            (*val.arr_val).values[index.int_val as usize] = virtual_machine_stack_pop(vm);
                        }
                    }
                } else if hop == BC_OBJECT_FIELD {
                    let key = READ_BYTE(vm);
                    let mut found = 0;
                    if val.type != VALUE_TYPE_OBJ {
                        println!("attempt to query field of non-object value");
                        std::process::exit(1);
                    }
                    let mut j = 0usize;
                    let props_len = unsafe { (*val.obj_val).properties_len };
                    let mut done = false;
                    while j < props_len {
                        unsafe {
                            if (*val.obj_val).properties[j].key == key {
                                if i < len - 1 {
                                    val = (*val.obj_val).properties[j].val.clone();
                                    found = 1;
                                    done = true;
                                    break;
                                } else if i == len - 1 {
                                    for _k in 0..pops {
                                        virtual_machine_stack_pop(vm);
                                    }
                                    (*val.obj_val).properties[j].val = virtual_machine_stack_pop(vm);
                                    found = 1;
                                    done = true;
                                    break;
                                }
                            }
                        }
                        j += 1;
                    }
                    if !done && found == 0 {
                        for _k in 0..pops {
                            virtual_machine_stack_pop(vm);
                        }
                        
                        if i < len - 1 {
                            println!("need to create to many fields");
                            std::process::exit(1);
                        } else {
                            unsafe {
                                if (*val.obj_val).properties_len == (*val.obj_val).properties_cap {
                                    val.obj_val = garbage_collector_realloc_obj(&mut vm.gc, val.obj_val, (*val.obj_val).properties_len + 1);
                                    (*val.obj_val).properties_len += 1;
                                    let jpos = (*val.obj_val).properties_len - 1;
                                    (*val.obj_val).properties[jpos].key = key;
                                    (*val.obj_val).properties[jpos].val = virtual_machine_stack_pop(vm);
                                } else {
                                    let jpos = (*val.obj_val).properties_len;
                                    (*val.obj_val).properties_len += 1;
                                    (*val.obj_val).properties[jpos].key = key;
                                    (*val.obj_val).properties[jpos].val = virtual_machine_stack_pop(vm);
                                }
                            }
                        }
                    }
                }
            }
        }
        x if x == BC_OP_GET_HEAP => {
            let idx = READ_BYTE(vm);
            let len = READ_BYTE(vm);
            let mut pops: usize = 0;
            let mut val = vm.stack[idx].clone();
            for _i in 0..len {
                let hop = READ_BYTE(vm);
                if hop == BC_ARRAY_INDEX {
                    let offset = READ_BYTE(vm);
                    let index = vm.stack[vm.stack_top - offset - 1].clone();
                    if val.type != VALUE_TYPE_ARR {
                        println!("attempt to index non-array value");
                        std::process::exit(1);
                    }
                    if index.type != VALUE_TYPE_INTEGER {
                        println!("attempt to use non-integer value as array index");
                        std::process::exit(1);
                    }
                    if index.int_val < 0 {
                        println!("invalid array index: {}", index.int_val);
                        std::process::exit(1);
                    }
                    if (index.int_val as usize) > unsafe { (*val.arr_val).len } {
                        println!("array index to unitialized data: {}", index.int_val);
                        std::process::exit(1);
                    }
                    unsafe {
                        val = (*val.arr_val).values[index.int_val as usize].clone();
                    }
                    pops += 1;
                } else if hop == BC_OBJECT_FIELD {
                    let key = READ_BYTE(vm);
                    let mut found = 0;
                    if val.type != VALUE_TYPE_OBJ {
                        println!("attempt to query field of non-object value");
                        std::process::exit(1);
                    }
                    let mut j = 0usize;
                    let props_len = unsafe { (*val.obj_val).properties_len };
                    let mut done = false;
                    while j < props_len {
                        unsafe {
                            if (*val.obj_val).properties[j].key == key {
                                val = (*val.obj_val).properties[j].val.clone();
                                found = 1;
                                done = true;
                                break;
                            }
                        }
                        j += 1;
                    }
                    if !done && found == 0 {
                        println!("unknown fieldref: {}", key);
                        std::process::exit(1);
                    }
                }
            }

            for _ in 0..pops {
                virtual_machine_stack_pop(vm);
            }
            
            virtual_machine_stack_push(vm, val);
        }

        x if x == BC_OP_LOGICAL_OR => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for OR!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val != 0 || val1.int_val != 0) as long_long);
            virtual_machine_stack_push(vm, res);                        
        }
        x if x == BC_OP_LOGICAL_AND => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for AND!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val != 0 && val1.int_val != 0) as long_long);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_EQ_EQEQ => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for EQEQ!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val == val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_EQ_NEQ => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for NEQ!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val != val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_REL_LT => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for LT!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val < val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_REL_GT => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for GT!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val > val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_REL_LE => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for LE!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val <= val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_REL_GE => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for GE!");
                std::process::exit(1);
            }
            let res = create_value_from_int((val2.int_val >= val1.int_val) as long_long);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_ADDITIVE_PLUS => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for PLUS!");
                std::process::exit(1);
            }
            let res = create_value_from_int(val2.int_val + val1.int_val);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_ADDITIVE_MINUS => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for MINUS!");
                std::process::exit(1);
            }
            let res = create_value_from_int(val2.int_val - val1.int_val);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_MULTIPLICATIVE_MUL => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for MUL!");
                std::process::exit(1);
            }
            let res = create_value_from_int(val2.int_val * val1.int_val);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_MULTIPLICATIVE_DIV => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for DIV!");
                std::process::exit(1);
            }
            let res = create_value_from_int(val2.int_val / val1.int_val);
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_MULTIPLICATIVE_MOD => {
            let val1 = virtual_machine_stack_pop(vm);
            let val2 = virtual_machine_stack_pop(vm);
            if (val1.type != VALUE_TYPE_INTEGER) || (val2.type != VALUE_TYPE_INTEGER) {
                println!("invalid value for MOD!");
                std::process::exit(1);
            }
            let res = create_value_from_int(val2.int_val % val1.int_val);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_NEGATE => {
            let val = virtual_machine_stack_pop(vm);
            if val.type != VALUE_TYPE_INTEGER {
                println!("invalid value for NEGATE!");
                std::process::exit(1);
            }
            let res = create_value_from_int(-val.int_val);
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_HAS_PROPERTY => {
            let mut val = virtual_machine_stack_pop(vm);
            let key = READ_BYTE(vm);
            let res;
            if val.type == VALUE_TYPE_OBJ {
                let mut found = 0;
                let mut j = 0usize;
                let props_len = unsafe { (*val.obj_val).properties_len };
                while j < props_len {
                    unsafe {
                        if (*val.obj_val).properties[j].key == key {
                            val = (*val.obj_val).properties[j].val.clone();
                            found = 1;
                            break;
                        }
                    }
                    j += 1;
                }
                res = create_value_from_int(found as long_long);
            } else {
                res = create_value_from_int(-1);
            }
            virtual_machine_stack_push(vm, res);
        }
        x if x == BC_OP_LEN => {
            let val = virtual_machine_stack_pop(vm);
            let res;
            if val.type == VALUE_TYPE_ARR {
                res = create_value_from_int(unsafe { (*val.arr_val).len } as long_long);
            } else {
                res = create_value_from_int(-1);
            }
            virtual_machine_stack_push(vm, res);
        }

        x if x == BC_OP_JUMP_IF_FALSE => {
            let offset = READ_BYTE(vm) as isize;
            let val = vm.stack[vm.stack_top - 1].clone();
            if val.int_val == 0 {
                vm.ip = ((vm.ip as isize) + offset) as usize;
            }
        }
        x if x == BC_OP_JUMP => {
            let offset = READ_BYTE(vm) as isize;
            vm.ip = ((vm.ip as isize) + offset) as usize;
        }

        x if x == BC_OP_RETURN => {
            let val = virtual_machine_stack_pop(vm);
            if vm.trace != 0 {
                println!("</trace>");
            }
            return val.int_val;
        }

        _ => {
            // unknown instruction - break or continue
        }
        }
    }
}

pub fn virtual_machine_free(vm: virtual_machine_type_t) {
    drop(vm);
}