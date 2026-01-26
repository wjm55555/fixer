use crate::garbage_collector;

#[derive(Clone, Copy)]
pub struct VALUE {
    pub type_: u8,
    pub int_val: i64,
    pub obj_val: Option<usize>,
    pub arr_val: Option<usize>,
}

pub const VALUE_TYPE_INTEGER: u8 = 0;
pub const VALUE_TYPE_OBJ: u8 = 1;
pub const VALUE_TYPE_ARR: u8 = 2;

fn create_value_from_int(int_val: i64) -> VALUE {
    VALUE {
        type_: VALUE_TYPE_INTEGER,
        int_val,
        obj_val: None,
        arr_val: None,
    }
}

fn create_value_from_obj(obj_val: Box<OBJECT>) -> VALUE {
    VALUE {
        type_: VALUE_TYPE_OBJ,
        int_val: 0,
        obj_val: Some(Box::into_raw(obj_val) as usize),
        arr_val: None,
    }
}

fn create_value_from_arr(arr_val: Box<ARRAY>) -> VALUE {
    VALUE {
        type_: VALUE_TYPE_ARR,
        int_val: 0,
        obj_val: None,
        arr_val: Some(Box::into_raw(arr_val) as usize),
    }
}

#[derive(Clone)]
pub struct PROPERTY {
    pub key: usize,
    pub val: VALUE,
}

pub struct OBJECT {
    pub properties: Vec<PROPERTY>,
    pub properties_len: usize,
    pub properties_cap: usize,
}

pub struct ARRAY {
    pub values: Vec<VALUE>,
    pub len: usize,
}

#[derive(Clone)]
pub struct CONSTANT {
    pub int_cnst: i64,
}

pub struct BYTECODE {
    pub op_codes: Vec<u8>,
    pub constant_pool: Vec<CONSTANT>,
}

pub const BC_OP_POP: u8 = 0;
pub const BC_OP_CONSTANT: u8 = 1;
pub const BC_OP_SET_LOCAL: u8 = 2;
pub const BC_OP_GET_LOCAL: u8 = 3;
pub const BC_OP_CREATE_OBJ: u8 = 4;
pub const BC_OP_INIT_OBJ_PROP: u8 = 5;
pub const BC_OP_CREATE_ARR: u8 = 6;
pub const BC_OP_SET_HEAP: u8 = 7;
pub const BC_OP_GET_HEAP: u8 = 8;
pub const BC_OP_LOGICAL_OR: u8 = 9;
pub const BC_OP_LOGICAL_AND: u8 = 10;
pub const BC_OP_EQ_EQEQ: u8 = 11;
pub const BC_OP_EQ_NEQ: u8 = 12;
pub const BC_OP_REL_LT: u8 = 13;
pub const BC_OP_REL_GT: u8 = 14;
pub const BC_OP_REL_LE: u8 = 15;
pub const BC_OP_REL_GE: u8 = 16;
pub const BC_OP_ADDITIVE_PLUS: u8 = 17;
pub const BC_OP_ADDITIVE_MINUS: u8 = 18;
pub const BC_OP_MULTIPLICATIVE_MUL: u8 = 19;
pub const BC_OP_MULTIPLICATIVE_DIV: u8 = 20;
pub const BC_OP_MULTIPLICATIVE_MOD: u8 = 21;
pub const BC_OP_NEGATE: u8 = 22;
pub const BC_OP_HAS_PROPERTY: u8 = 23;
pub const BC_OP_LEN: u8 = 24;
pub const BC_OP_JUMP_IF_FALSE: u8 = 25;
pub const BC_OP_JUMP: u8 = 26;
pub const BC_OP_RETURN: u8 = 27;

pub const BC_ARRAY_INDEX: u8 = 0;
pub const BC_OBJECT_FIELD: u8 = 1;

pub struct VIRTUAL_MACHINE {
    pub bc: Box<BYTECODE>,
    pub ip: usize,
    pub stack: Vec<VALUE>,
    pub stack_top: usize,
    pub stack_cap: usize,
    pub gc: Box<garbage_collector::GARBAGE_COLLECTOR>,
    pub trace: i32,
}

pub type virtual_machine_type_t = *mut VIRTUAL_MACHINE;

pub fn create_virtual_machine() -> Box<VIRTUAL_MACHINE> {
    Box::new(VIRTUAL_MACHINE {
        bc: Box::new(BYTECODE {
            op_codes: Vec::new(),
            constant_pool: Vec::new(),
        }),
        ip: 0,
        stack: Vec::new(),
        stack_top: 0,
        stack_cap: 0,
        gc: Box::new(garbage_collector::GARBAGE_COLLECTOR::default()),
        trace: 0,
    })
}

pub fn virtual_machine_conf(
    vm: &mut VIRTUAL_MACHINE,
    bc: Box<BYTECODE>,
    stack_size: usize,
    start_heap_size_b: usize,
    trace: i32,
) {
    vm.bc = bc;
    vm.ip = 0;
    vm.stack_cap = stack_size;
    vm.stack = vec![
        VALUE {
            type_: VALUE_TYPE_INTEGER,
            int_val: 0,
            obj_val: None,
            arr_val: None,
        };
        stack_size
    ];
    vm.stack_top = 0;
    vm.trace = trace;
}

fn virtual_machine_stack_push(vm: &mut VIRTUAL_MACHINE, val: VALUE) {
    if vm.stack_top < vm.stack_cap {
        vm.stack[vm.stack_top] = val;
        vm.stack_top += 1;
    }
}

fn virtual_machine_stack_pop(vm: &mut VIRTUAL_MACHINE) -> VALUE {
    if vm.stack_top > 0 {
        vm.stack_top -= 1;
        vm.stack[vm.stack_top]
    } else {
        VALUE {
            type_: VALUE_TYPE_INTEGER,
            int_val: 0,
            obj_val: None,
            arr_val: None,
        }
    }
}

fn READ_BYTE(vm: &mut VIRTUAL_MACHINE) -> u8 {
    if vm.ip < vm.bc.op_codes.len() {
        let byte = vm.bc.op_codes[vm.ip];
        vm.ip += 1;
        byte
    } else {
        0
    }
}

fn create_obj(vm: &mut VIRTUAL_MACHINE) -> Box<OBJECT> {
    let properties_num = READ_BYTE(vm) as usize;
    let mut obj = Box::new(OBJECT {
        properties: Vec::with_capacity(properties_num * 2),
        properties_len: 0,
        properties_cap: properties_num * 2,
    });

    for _ in 0..properties_num {
        let val = virtual_machine_stack_pop(vm);
        let key_val = virtual_machine_stack_pop(vm);
        obj.properties.push(PROPERTY {
            key: key_val.int_val as usize,
            val,
        });
    }

    obj.properties_len = properties_num;
    obj
}

fn create_arr(vm: &mut VIRTUAL_MACHINE) -> Box<ARRAY> {
    let arr_len = READ_BYTE(vm) as usize;
    let mut arr = Box::new(ARRAY {
        values: Vec::with_capacity(arr_len),
        len: 0,
    });

    for _ in 0..arr_len {
        let val = virtual_machine_stack_pop(vm);
        arr.values.push(val);
    }

    arr.len = arr_len;
    arr
}

pub fn virtual_machine_run(vm: &mut VIRTUAL_MACHINE) -> i64 {
    if vm.trace != 0 {
        println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        println!("<trace>");
    }

    loop {
        let instruction = READ_BYTE(vm);

        if vm.trace != 0 {
            println!(
                "\t<step>OP: {}; SS: {} b; HS: {} b; FB: 0; BB: 0</step>",
                instruction, vm.stack_top, 0
            );
        }

        match instruction {
            BC_OP_POP => {
                virtual_machine_stack_pop(vm);
            }
            BC_OP_CONSTANT => {
                let idx = READ_BYTE(vm) as usize;
                if idx < vm.bc.constant_pool.len() {
                    let cnst = vm.bc.constant_pool[idx].clone();
                    let val = create_value_from_int(cnst.int_cnst);
                    virtual_machine_stack_push(vm, val);
                }
            }
            BC_OP_SET_LOCAL => {
                let idx = READ_BYTE(vm) as usize;
                if vm.stack_top > 0 && idx < vm.stack.len() {
                    vm.stack[idx] = vm.stack[vm.stack_top - 1];
                    if idx != vm.stack_top - 1 {
                        virtual_machine_stack_pop(vm);
                    }
                }
            }
            BC_OP_GET_LOCAL => {
                let idx = READ_BYTE(vm) as usize;
                if idx < vm.stack.len() {
                    let val = vm.stack[idx];
                    virtual_machine_stack_push(vm, val);
                }
            }
            BC_OP_CREATE_OBJ => {
                let obj = create_obj(vm);
                let val = create_value_from_obj(obj);
                virtual_machine_stack_push(vm, val);
            }
            BC_OP_INIT_OBJ_PROP => {
                let byte_val = READ_BYTE(vm);
                let val = create_value_from_int(byte_val as i64);
                virtual_machine_stack_push(vm, val);
            }
            BC_OP_CREATE_ARR => {
                let arr = create_arr(vm);
                let val = create_value_from_arr(arr);
                virtual_machine_stack_push(vm, val);
            }
            BC_OP_SET_HEAP => {
                let idx = READ_BYTE(vm) as usize;
                let len = READ_BYTE(vm) as usize;
                let mut pops = 0;

                if idx < vm.stack.len() {
                    let mut val = vm.stack[idx];

                    for i in 0..len {
                        let hop = READ_BYTE(vm);

                        if hop == BC_ARRAY_INDEX {
                            let offset = READ_BYTE(vm) as usize;
                            let index_pos = if vm.stack_top > offset {
                                vm.stack_top - offset - 1
                            } else {
                                0
                            };
                            let index = vm.stack[index_pos];

                            if val.type_ != VALUE_TYPE_ARR {
                                eprintln!("attempt to index non-array value");
                                std::process::exit(1);
                            }
                            if index.type_ != VALUE_TYPE_INTEGER {
                                eprintln!("attempt to use non-integer value as array index");
                                std::process::exit(1);
                            }
                            if index.int_val < 0 {
                                eprintln!("invalid array index: {}", index.int_val);
                                std::process::exit(1);
                            }

                            pops += 1;

                            if i < len - 1 {
                                if let Some(arr_ptr) = val.arr_val {
                                    let arr = unsafe { &*(arr_ptr as *const ARRAY) };
                                    if (index.int_val as usize) < arr.values.len() {
                                        val = arr.values[index.int_val as usize];
                                    }
                                }
                            } else if i == len - 1 {
                                for _ in 0..pops {
                                    virtual_machine_stack_pop(vm);
                                }
                                let new_val = virtual_machine_stack_pop(vm);
                                if let Some(arr_ptr) = val.arr_val {
                                    let arr = unsafe { &mut *(arr_ptr as *mut ARRAY) };
                                    if (index.int_val as usize) < arr.values.len() {
                                        arr.values[index.int_val as usize] = new_val;
                                    }
                                }
                            }
                        } else if hop == BC_OBJECT_FIELD {
                            let key = READ_BYTE(vm) as usize;
                            let mut found = false;

                            if val.type_ != VALUE_TYPE_OBJ {
                                eprintln!("attempt to query field of non-object value");
                                std::process::exit(1);
                            }

                            if let Some(obj_ptr) = val.obj_val {
                                let obj = unsafe { &mut *(obj_ptr as *mut OBJECT) };

                                for j in 0..obj.properties_len {
                                    if obj.properties[j].key == key {
                                        if i < len - 1 {
                                            val = obj.properties[j].val;
                                            found = true;
                                            break;
                                        } else if i == len - 1 {
                                            for _ in 0..pops {
                                                virtual_machine_stack_pop(vm);
                                            }
                                            let new_val = virtual_machine_stack_pop(vm);
                                            obj.properties[j].val = new_val;
                                            found = true;
                                            break;
                                        }
                                    }
                                }

                                if !found {
                                    for _ in 0..pops {
                                        virtual_machine_stack_pop(vm);
                                    }
                                    if i < len - 1 {
                                        eprintln!("need to create to many fields");
                                        std::process::exit(1);
                                    } else {
                                        if obj.properties_len == obj.properties_cap {
                                            obj.properties_cap = obj.properties_len + 1;
                                        }
                                        obj.properties_len += 1;
                                        if obj.properties.len() > key {
                                            obj.properties[key].key = key;
                                            let new_val = virtual_machine_stack_pop(vm);
                                            obj.properties[key].val = new_val;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            BC_OP_GET_HEAP => {
                let idx = READ_BYTE(vm) as usize;
                let len = READ_BYTE(vm) as usize;
                let mut pops = 0;

                if idx < vm.stack.len() {
                    let mut val = vm.stack[idx];

                    for i in 0..len {
                        let hop = READ_BYTE(vm);

                        if hop == BC_ARRAY_INDEX {
                            let offset = READ_BYTE(vm) as usize;
                            let index_pos = if vm.stack_top > offset {
                                vm.stack_top - offset - 1
                            } else {
                                0
                            };
                            let index = vm.stack[index_pos];

                            if val.type_ != VALUE_TYPE_ARR {
                                eprintln!("attempt to index non-array value");
                                std::process::exit(1);
                            }
                            if index.type_ != VALUE_TYPE_INTEGER {
                                eprintln!("attempt to use non-integer value as array index");
                                std::process::exit(1);
                            }
                            if index.int_val < 0 {
                                eprintln!("invalid array index: {}", index.int_val);
                                std::process::exit(1);
                            }

                            if let Some(arr_ptr) = val.arr_val {
                                let arr = unsafe { &*(arr_ptr as *const ARRAY) };
                                if (index.int_val as usize) < arr.len {
                                    val = arr.values[index.int_val as usize];
                                    pops += 1;
                                } else {
                                    eprintln!("array index to unitialized data: {}", index.int_val);
                                    std::process::exit(1);
                                }
                            }
                        } else if hop == BC_OBJECT_FIELD {
                            let key = READ_BYTE(vm) as usize;
                            let mut found = false;

                            if val.type_ != VALUE_TYPE_OBJ {
                                eprintln!("attempt to query field of non-object value");
                                std::process::exit(1);
                            }

                            if let Some(obj_ptr) = val.obj_val {
                                let obj = unsafe { &*(obj_ptr as *const OBJECT) };

                                for j in 0..obj.properties_len {
                                    if obj.properties[j].key == key {
                                        val = obj.properties[j].val;
                                        found = true;
                                        break;
                                    }
                                }

                                if !found {
                                    eprintln!("unknown fieldref: {}", key);
                                    std::process::exit(1);
                                }
                            }
                        }
                    }

                    for _ in 0..pops {
                        virtual_machine_stack_pop(vm);
                    }
                    virtual_machine_stack_push(vm, val);
                }
            }
            BC_OP_LOGICAL_OR => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for OR!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val != 0 || val1.int_val != 0 {
                    1
                } else {
                    0
                });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_LOGICAL_AND => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for AND!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val != 0 && val1.int_val != 0 {
                    1
                } else {
                    0
                });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_EQ_EQEQ => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for EQEQ!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val == val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_EQ_NEQ => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for NEQ!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val != val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_REL_LT => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for LT!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val < val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_REL_GT => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for GT!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val > val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_REL_LE => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for LE!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val <= val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_REL_GE => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for GE!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(if val2.int_val >= val1.int_val { 1 } else { 0 });
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_ADDITIVE_PLUS => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for PLUS!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(val2.int_val + val1.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_ADDITIVE_MINUS => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for MINUS!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(val2.int_val - val1.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_MULTIPLICATIVE_MUL => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for MUL!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(val2.int_val * val1.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_MULTIPLICATIVE_DIV => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for DIV!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(val2.int_val / val1.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_MULTIPLICATIVE_MOD => {
                let val1 = virtual_machine_stack_pop(vm);
                let val2 = virtual_machine_stack_pop(vm);

                if val1.type_ != VALUE_TYPE_INTEGER || val2.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for MOD!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(val2.int_val % val1.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_NEGATE => {
                let val = virtual_machine_stack_pop(vm);

                if val.type_ != VALUE_TYPE_INTEGER {
                    eprintln!("invalid value for NEGATE!");
                    std::process::exit(1);
                }

                let res = create_value_from_int(-val.int_val);
                virtual_machine_stack_push(vm, res);
            }
            BC_OP_HAS_PROPERTY => {
                let val = virtual_machine_stack_pop(vm);
                let key = READ_BYTE(vm) as usize;
                let res;

                if val.type_ == VALUE_TYPE_OBJ {
                    let mut found = 0;
                    if let Some(obj_ptr) = val.obj_val {
                        let obj = unsafe { &*(obj_ptr as *const OBJECT) };

                        for j in 0..obj.properties_len {
                            if obj.properties[j].key == key {
                                found = 1;
                                break;
                            }
                        }
                    }
                    res = create_value_from_int(found);
                } else {
                    res = create_value_from_int(-1);
                }

                virtual_machine_stack_push(vm, res);
            }
            BC_OP_LEN => {
                let val = virtual_machine_stack_pop(vm);
                let res;

                if val.type_ == VALUE_TYPE_ARR {
                    if let Some(arr_ptr) = val.arr_val {
                        let arr = unsafe { &*(arr_ptr as *const ARRAY) };
                        res = create_value_from_int(arr.len as i64);
                    } else {
                        res = create_value_from_int(-1);
                    }
                } else {
                    res = create_value_from_int(-1);
                }

                virtual_machine_stack_push(vm, res);
            }
            BC_OP_JUMP_IF_FALSE => {
                let offset = READ_BYTE(vm) as i32;
                if vm.stack_top > 0 {
                    let val = vm.stack[vm.stack_top - 1];
                    if val.int_val == 0 {
                        vm.ip = ((vm.ip as i32) + offset) as usize;
                    }
                }
            }
            BC_OP_JUMP => {
                let offset = READ_BYTE(vm) as i32;
                vm.ip = ((vm.ip as i32) + offset) as usize;
            }
            BC_OP_RETURN => {
                let val = virtual_machine_stack_pop(vm);
                if vm.trace != 0 {
                    println!("</trace>");
                }
                return val.int_val;
            }
            _ => {
                break;
            }
        }
    }

    0
}

pub fn virtual_machine_free(_vm: Box<VIRTUAL_MACHINE>) {}
