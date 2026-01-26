use translate_calculator::calculator::*;
use std::env;

pub fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: i32 = argv.len() as i32;

    let mut result: byte_8;
    let mut output: simple_queue_t = simple_queue_t::default();
    let mut ops: simple_stack_t = simple_stack_t::default();

    if argc != 2 {
        eprintln!("Use {} 'expression'", argv[0]);
        std::process::exit(1);
    }

    queue_init(&mut output);
    stack_init(&mut ops);

    shunting_yard(argv[1].as_str(), &mut ops, &mut output);

    result = compute_rpn(&output);
    println!("result: {}", result);
}