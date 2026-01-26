use crate::lexer::*;
use crate::bytecode_generator::*;
use crate::parser::*;
use crate::virtual_machine::*;
pub const MAX_FNAME_SIZE: usize = 1024;
pub const STACKSIZE: i32 = 1024;
pub const HEAPSIZE: i32 = 35;
pub const SYNTAX_TESTS_NUM: usize = 10;
pub const GC_TESTS_NUM: usize = 10;

pub static syntax_tests_fnames: [&str; SYNTAX_TESTS_NUM] = [
    "data/tests/syntax/01.js",
    "data/tests/syntax/02.js",
    "data/tests/syntax/03.js",
    "data/tests/syntax/04.js",
    "data/tests/syntax/05.js",
    "data/tests/syntax/06.js",
    "data/tests/syntax/07.js",
    "data/tests/syntax/08.js",
    "data/tests/syntax/09.js",
    "data/tests/syntax/10.js",
];

pub static syntax_tests_results: [i32; SYNTAX_TESTS_NUM] = [
    55,
    72,
    0,
    3628800,
    45,
    1,
    2,
    25,
    15,
    -100,
];

pub static gc_tests_fnames: [&str; GC_TESTS_NUM] = [
    "data/tests/gc/01.js",
    "data/tests/gc/02.js",
    "data/tests/gc/03.js",
    "data/tests/gc/04.js",
    "data/tests/gc/05.js",
    "data/tests/gc/06.js",
    "data/tests/gc/07.js",
    "data/tests/gc/08.js",
    "data/tests/gc/09.js",
    "data/tests/gc/10.js",
];

pub static gc_tests_results: [i32; GC_TESTS_NUM] = [
    8,
    45,
    5,
    220,
    3,
    12345,
    3,
    30,
    2,
    66,
];

pub fn PREFIX_UNUSED<T>(_v: T) {}

pub fn run_single_test(num: u32, fname: &str, exp: i32) {
    let r;
    
    let lexer = create_lexer();
    lexer_conf_from_file(lexer, fname);

    let parser = create_parser();
    parser_conf(parser, lexer);
    let mut unit = std::ptr::null_mut();
    r = parser_parse(parser, &mut unit);
    if r != PARSER_OK {
        println!("{}) PARSER ERROR", num);
        std::process::exit(0);
    }

    lexer_free(lexer);
    parser_free(parser);

    let bc_gen = create_bytecode_generator();
    bytecode_generator_conf(bc_gen, unit);

    let mut bc = std::ptr::null_mut();
    r = bytecode_generator_generate(bc_gen, &mut bc);
    if r != BYTECODE_GENERATOR_OK {
        println!("{}) BYTECODE GENERATOR ERROR", num);
        std::process::exit(0);
    }

    let vm = create_virtual_machine();
    virtual_machine_conf(vm, bc, STACKSIZE, HEAPSIZE, 0);
    let got = virtual_machine_run(vm);
    bytecode_free(bc);
    virtual_machine_free(vm);
    
    println!("{}) EXP = {}; GOT = {}; {}", num, exp, got, if exp == got { "PASSED" } else { "FAILED" });
    if exp != got {
        std::process::exit(0);
    }
}

pub fn run_syntax_tests() {
    println!("RUNNING SYNTAX TESTS:");
    for i in 0..SYNTAX_TESTS_NUM {
        run_single_test((i as u32) + 1, syntax_tests_fnames[i], syntax_tests_results[i]);
    }
    println!("ALL SYNTAX TESTS PASSED!");
}

pub fn convention() {
    let f = file_open("conv", "w");
    fclose(f);
}

pub fn run_gc_tests() {
    println!("RUNNING GC TESTS:");
    for i in 0..GC_TESTS_NUM {
        run_single_test((i as u32) + 1, gc_tests_fnames[i], gc_tests_results[i]);
    }
    println!("ALL GC TESTS PASSED!");
}

pub fn main() {
    let argc: i32 = 0;
    let argv: Vec<*const u8> = Vec::new();
    PREFIX_UNUSED(argc);
    PREFIX_UNUSED(argv);

    println!("RUNNING TESTS:\n");
    run_syntax_tests();
    run_gc_tests();
    println!("ALL TESTS PASSED:\n");
}