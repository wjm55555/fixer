use std::process;

pub const MAX_ADT_SIZE: usize = 100;

pub type ubyte_2 = u16;
pub type byte_2 = i16;
pub type ubyte_8 = u64;
pub type byte_8 = i64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum token_type_e {
    TOKEN_OPERATOR = 1,
    TOKEN_NUMBER = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct token_t {
    pub r#type: token_type_e,
    pub val: byte_8,
}

impl Default for token_t {
    fn default() -> Self {
        token_t {
            r#type: token_type_e::TOKEN_OPERATOR,
            val: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct simple_stack_t {
    pub pos: byte_2,
    pub data: [byte_8; MAX_ADT_SIZE],
}

impl Default for simple_stack_t {
    fn default() -> Self {
        simple_stack_t {
            pos: -1,
            data: [0; MAX_ADT_SIZE],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct simple_queue_t {
    pub front: byte_2,
    pub back: byte_2,
    pub data: [token_t; MAX_ADT_SIZE],
}

impl Default for simple_queue_t {
    fn default() -> Self {
        simple_queue_t {
            front: -1,
            back: -1,
            data: [token_t::default(); MAX_ADT_SIZE],
        }
    }
}

pub fn stack_init(s: &mut simple_stack_t) {
    s.pos = -1;
    s.data = [0; MAX_ADT_SIZE];
}

pub fn stack_push(data: byte_8, s: &mut simple_stack_t) {
    let next = (s.pos as isize) + 1;
    if next as usize >= MAX_ADT_SIZE {
        println!("Stack overflow");
        process::exit(1);
    }
    s.pos += 1;
    let idx = s.pos as usize;
    s.data[idx] = data;
}

pub fn stack_pop(s: &mut simple_stack_t) -> byte_8 {
    if stack_is_empty(s) != 0 {
        println!("Empty stack");
        process::exit(1);
    }
    let val = s.data[s.pos as usize];
    s.pos -= 1;
    val
}

pub fn stack_is_empty(s: &simple_stack_t) -> i32 {
    if s.pos == -1 {
        1
    } else {
        0
    }
}

pub fn stack_peek(s: &simple_stack_t) -> byte_8 {
    if stack_is_empty(s) != 0 {
        println!("Empty stack");
        process::exit(1);
    }
    s.data[s.pos as usize]
}

pub fn queue_init(q: &mut simple_queue_t) {
    q.front = -1;
    q.back = -1;
    q.data = [token_t::default(); MAX_ADT_SIZE];
}

pub fn queue_enqueue(data: token_t, q: &mut simple_queue_t) {
    let next = (q.back as isize) + 1;
    if next as usize >= MAX_ADT_SIZE {
        println!("Queue overflow");
        process::exit(1);
    }
    if q.front == -1 {
        q.front = 0;
    }
    q.back += 1;
    q.data[q.back as usize] = data;
}

pub fn queue_dequeue(q: &mut simple_queue_t) -> token_t {
    if queue_is_empty(q) != 0 {
        println!("Empty queue");
        process::exit(1);
    }
    let t = q.data[q.front as usize];
    q.front += 1;
    t
}

pub fn queue_is_empty(q: &simple_queue_t) -> i32 {
    if q.front == -1 || q.front > q.back {
        1
    } else {
        0
    }
}

pub fn get_op_precedence(op: char) -> i32 {
    let mut res = 0;
    if op == '+' || op == '-' {
        res = 9;
    } else if op == '*' || op == '/' || op == '%' {
        res = 10;
    } else if op == '^' {
        res = 11;
    }
    res
}

pub fn is_higher_or_equal_precedence(op1: char, op2: char) -> i32 {
    if get_op_precedence(op1) >= get_op_precedence(op2) {
        1
    } else {
        0
    }
}

pub fn is_higher_precedence(op1: char, op2: char) -> i32 {
    if get_op_precedence(op1) > get_op_precedence(op2) {
        1
    } else {
        0
    }
}

pub fn is_left_assoc_operator(op: char) -> i32 {
    if op == '+' || op == '-' || op == '*' || op == '/' || op == '%' {
        1
    } else {
        0
    }
}

pub fn is_right_assoc_operator(op: char) -> i32 {
    if op == '^' {
        1
    } else {
        0
    }
}

pub fn shunting_yard(input: &str, ops: &mut simple_stack_t, output: &mut simple_queue_t) {
    let mut i: usize = 0;
    let mut parenthesis = simple_stack_t::default();
    stack_init(&mut parenthesis);

    let len = input.len();
    while i < len {
        let c = input.as_bytes()[i] as char;
        let mut new_token = token_t::default();

        match c {
            ' ' | '\t' => {
                i += 1;
            }
            '+' | '-' | '/' | '*' | '%' | '^' => {
                while stack_is_empty(ops) == 0
                    && (
                        (is_left_assoc_operator(c) != 0
                            && is_higher_or_equal_precedence((stack_peek(ops) as u8) as char, c) != 0)
                            || (is_right_assoc_operator(c) != 0
                                && is_higher_precedence((stack_peek(ops) as u8) as char, c) != 0)
                    )
                {
                    let stack_top = stack_pop(ops);
                    new_token.r#type = token_type_e::TOKEN_OPERATOR;
                    new_token.val = stack_top;
                    queue_enqueue(new_token, output);
                }
                stack_push(c as byte_8, ops);
                i += 1;
            }
            '(' => {
                stack_push(c as byte_8, ops);
                stack_push(c as byte_8, &mut parenthesis);
                i += 1;
            }
            ')' => {
                while stack_is_empty(ops) == 0 && ((stack_peek(ops) as u8) as char) != '(' {
                    let stack_top = stack_pop(ops);
                    new_token.r#type = token_type_e::TOKEN_OPERATOR;
                    new_token.val = stack_top;
                    queue_enqueue(new_token, output);
                }
                if stack_is_empty(&parenthesis) != 0 {
                    println!("Unbalanced parenthesis");
                    process::exit(1);
                }
                stack_pop(&mut parenthesis);
                i += 1;
            }
            _ => {
                let mut number: byte_8 = 0;
                if !c.is_ascii_digit() {
                    println!("Invalid character [{}]", c);
                    process::exit(1);
                }
                while i < len && (input.as_bytes()[i] as char).is_ascii_digit() {
                    number *= 10;
                    number += (input.as_bytes()[i] - b'0') as byte_8;
                    i += 1;
                }
                new_token.r#type = token_type_e::TOKEN_NUMBER;
                new_token.val = number;
                queue_enqueue(new_token, output);
            }
        }
    }

    if stack_is_empty(&parenthesis) == 0 {
        println!("Unbalanced parenthesis");
        process::exit(1);
    }

    while stack_is_empty(ops) == 0 {
        let stack_top = stack_pop(ops);
        let new_token = token_t {
            r#type: token_type_e::TOKEN_OPERATOR,
            val: stack_top,
        };
        queue_enqueue(new_token, output);
    }
}

pub fn compute_rpn(q: &mut simple_queue_t) -> byte_8 {
    let mut res: byte_8 = 0;
    let mut work_area = simple_stack_t::default();
    stack_init(&mut work_area);

    while queue_is_empty(q) == 0 {
        let queue_front = queue_dequeue(q);

        if queue_front.r#type == token_type_e::TOKEN_OPERATOR {
            match (queue_front.val as u8) as char {
                '+' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    res = op1 + op2;
                    stack_push(res, &mut work_area);
                }
                '-' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    res = op1 - op2;
                    stack_push(res, &mut work_area);
                }
                '*' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    res = op1 * op2;
                    stack_push(res, &mut work_area);
                }
                '/' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    if op2 == 0 {
                        println!("Division by zero");
                        process::exit(1);
                    }
                    res = op1 / op2;
                    stack_push(res, &mut work_area);
                }
                '%' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    if op2 == 0 {
                        println!("Modulo by zero");
                        process::exit(1);
                    }
                    res = op1 % op2;
                    stack_push(res, &mut work_area);
                }
                '^' => {
                    let op2 = stack_pop(&mut work_area);
                    let op1 = stack_pop(&mut work_area);
                    if op2 < 0 {
                        println!("Invalid exponent");
                        process::exit(1);
                    }
                    let pow = (op2 as u32) as u32;
                    res = op1.pow(pow);
                    stack_push(res, &mut work_area);
                }
                _ => {}
            }
        } else {
            stack_push(queue_front.val, &mut work_area);
        }
    }

    if work_area.pos != 0 {
        println!("Invalid expression entered");
        process::exit(1);
    }
    stack_peek(&work_area)
}