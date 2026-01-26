use translate_gci::utils::*;
use translate_gci::ast::*;

pub enum INTERPRETER_MODE {
    INTERPRETER_LEX,
    INTERPRETER_PARSE,
    INTERPRETER_BC,
    INTERPRETER_INTERPRET,
    INTERPRETER_TRACE,
}

const INTERPRETER_LEX_STR: &str = "lex";
const INTERPRETER_PARSE_STR: &str = "parse";
const INTERPRETER_BC_STR: &str = "bc";
const INTERPRETER_INTERPRET_STR: &str = "interpet";
const INTERPRETER_TRACE_STR: &str = "trace";
const STACKSIZE_STR: &str = "stacksize";
const HEAPSIZE_STR: &str = "heapsize";

#[derive(Default)]
pub struct INTERPRETER_PARAMS {
    pub in_file: String,
    pub out: String,
    pub mode: i32,
    pub stacksize: usize,
    pub heapsize: usize,
}

impl INTERPRETER_PARAMS {
    pub fn new() -> Self {
        let mut params = INTERPRETER_PARAMS::default();
        params.in_file = String::new();
        params.out = String::from("stdout");
        params.mode = 3;
        params.stacksize = 1024;
        params.heapsize = 1024 * 1024;
        params
    }
}

fn print_version(interpreter_name: &str) {
    eprintln!("{}", interpreter_name);
    eprintln!("GCI V0.1");
    eprintln!("GCI is interpreter of tiny JavaScript-like language with garbage collector and objects.");
    eprintln!("It supports JavaScript-like objects, numbers, if-else-stmts and while loops.");
    eprintln!("Functionality:");
    eprintln!("  - lexer;");
    eprintln!("  - parser;");
    eprintln!("  - bytecode generator;");
    eprintln!("  - vm;");
    std::process::exit(0);
}

fn print_help(interpreter_name: &str) {
    eprintln!("Usage: {} [options] input.js", interpreter_name);
    eprintln!("Options:");
    eprintln!("  --version (-v)");
    eprintln!("            Print version info.");
    eprintln!("  --help    (-h)");
    eprintln!("            Print this help info.");
    eprintln!("  --in      (-i)");
    eprintln!("            Path to input file (stdin).");
    eprintln!("  --out     (-o)");
    eprintln!("            Path to output file (stdout|stderr) (default: stdout).");
    eprintln!("  --mode    (-m)");
    eprintln!("            Mode (lex|parse|bc|trace|interpret) (default: interpret).");
    eprintln!("  --stacksize");
    eprintln!("            Size of stack in vars(default: 1024).");
    eprintln!("  --heapsize");
    eprintln!("            Size of heap in bytes (default: 1 MB).");
    std::process::exit(0);
}

pub fn parse_args(args: &[String], params: &mut INTERPRETER_PARAMS) -> i32 {
    let mut i = 1;
    
    while i < args.len() {
        let arg = &args[i];
        
        if arg == "--version" || arg == "-v" {
            print_version(&args[0]);
        } else if arg == "--help" || arg == "-h" {
            print_help(&args[0]);
        } else if arg == "--in" || arg == "-i" {
            if i + 1 < args.len() {
                i += 1;
                params.in_file = args[i].clone();
            }
        } else if arg == "--out" || arg == "-o" {
            if i + 1 < args.len() {
                i += 1;
                params.out = args[i].clone();
            }
        } else if arg == "--mode" || arg == "-m" {
            if i + 1 < args.len() {
                i += 1;
                let mode_str = &args[i];
                if mode_str == INTERPRETER_LEX_STR {
                    params.mode = 0;
                } else if mode_str == INTERPRETER_PARSE_STR {
                    params.mode = 1;
                } else if mode_str == INTERPRETER_BC_STR {
                    params.mode = 2;
                } else if mode_str == INTERPRETER_INTERPRET_STR {
                    params.mode = 3;
                } else if mode_str == INTERPRETER_TRACE_STR {
                    params.mode = 4;
                } else {
                    eprintln!("Invalid interpreter mode \"{}\"", mode_str);
                    std::process::exit(1);
                }
            }
        } else if arg == "--stacksize" {
            if i + 1 < args.len() {
                i += 1;
                if let Ok(size) = args[i].parse::<usize>() {
                    params.stacksize = size;
                }
            }
        } else if arg == "--heapsize" {
            if i + 1 < args.len() {
                i += 1;
                if let Ok(size) = args[i].parse::<usize>() {
                    params.heapsize = size;
                }
            }
        }
        
        i += 1;
    }
    
    0
}

fn print_lexer_result(fname: &str, lexer: &lexer_type_t) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        dump_lexer_to_xml_file(&mut f, lexer);
    }
}

fn print_parser_result(fname: &str, unit: &UNIT_AST) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        dump_unit_ast_to_xml_file(&mut f, unit);
    }
}

fn print_bytecode_result(fname: &str, bc: &bytecode_type_t) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        dump_bytecode_to_xml_file(&mut f, bc);
    }
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut params = INTERPRETER_PARAMS::new();
    parse_args(&args, &mut params);
    
    let mut lexer = create_lexer();
    lexer_conf_from_file(&mut lexer, &params.in_file);
    
    if params.mode == 0 {
        print_lexer_result(&params.out, &lexer);
        lexer_free(&mut lexer);
        return;
    }
    
    let mut parser = create_parser();
    parser_conf(&mut parser, &lexer);
    
    let mut unit: Option<Box<UNIT_AST>> = None;
    let r = parser_parse(&mut parser, &mut unit);
    
    if r != 0 {
        let err = parser_get_error(&parser);
        print_parser_error(&err);
        lexer_free(&mut lexer);
        parser_free(&mut parser);
        std::process::exit(1);
    }
    
    if params.mode == 1 {
        if let Some(ref u) = unit {
            print_parser_result(&params.out, u);
        }
        lexer_free(&mut lexer);
        parser_free(&mut parser);
        return;
    }
    
    lexer_free(&mut lexer);
    parser_free(&mut parser);
    
    let mut bc_gen = create_bytecode_generator();
    if let Some(ref u) = unit {
        bytecode_generator_conf(&mut bc_gen, u);
    }
    
    let mut bc: Option<Box<bytecode_type_t>> = None;
    let r = bytecode_generator_generate(&mut bc_gen, &mut bc);
    
    if r != 0 {
        let err = bytecode_generator_get_error(&bc_gen);
        print_bytecode_error(&err);
        bytecode_generator_free(&mut bc_gen);
        std::process::exit(2);
    }
    
    if params.mode == 2 {
        if let Some(ref b) = bc {
            print_bytecode_result(&params.out, b);
        }
        bytecode_generator_free(&mut bc_gen);
        return;
    }
    
    bytecode_generator_free(&mut bc_gen);
    
    let mut vm = create_virtual_machine();
    if let Some(ref b) = bc {
        virtual_machine_conf(&mut vm, b, params.stacksize, params.heapsize, params.mode == 4);
    }
    
    let r = virtual_machine_run(&mut vm);
    virtual_machine_free(&mut vm);
    
    println!("result: {}", r);
}