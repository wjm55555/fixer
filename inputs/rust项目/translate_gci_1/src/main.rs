use translate_gci::lexer::*;
use translate_gci::bytecode_generator::*;
use translate_gci::parser::*;
use translate_gci::virtual_machine::*;
pub const INTERPRETER_LEX_STR: &str = "lex";
pub const INTERPRETER_PARSE_STR: &str = "parse";
pub const INTERPRETER_BC_STR: &str = "bc";
pub const INTERPRETER_INTERPRET_STR: &str = "interpet";
pub const INTERPRETER_TRACE_STR: &str = "trace";
pub const STACKSIZE_STR: &str = "stacksize";
pub const HEAPSIZE_STR: &str = "heapsize";

pub const STR_BUF_SIZE: usize = 1024;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum INTERPRETER_MODE {
    INTERPRETER_LEX,
    INTERPRETER_PARSE,
    INTERPRETER_BC,
    INTERPRETER_INTERPRET,
    INTERPRETER_TRACE,
}

impl Default for INTERPRETER_MODE {
    fn default() -> Self {
        INTERPRETER_MODE::INTERPRETER_INTERPRET
    }
}

#[repr(C)]
pub struct INTERPRETER_PARAMS {
    pub r#in: String,
    pub out: String,
    pub mode: INTERPRETER_MODE,
    pub stacksize: usize,
    pub heapsize: usize,
}

impl Default for INTERPRETER_PARAMS {
    fn default() -> Self {
        INTERPRETER_PARAMS {
            r#in: String::new(),
            out: String::from("stdout"),
            mode: INTERPRETER_MODE::INTERPRETER_INTERPRET,
            stacksize: 1024,
            heapsize: 1024 * 1024,
        }
    }
}

pub fn print_version(interpreter_name: &str) {
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

pub fn print_help(interpreter_name: &str) {
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

pub fn parse_args(argc: i32, argv: &[String], params: &mut INTERPRETER_PARAMS) -> i32 {
    // Initialize defaults
    params.r#in.clear();
    params.out = String::from("stdout");
    params.mode = INTERPRETER_MODE::INTERPRETER_INTERPRET;
    params.stacksize = 1024;
    params.heapsize = 1024 * 1024;

    let mut i = 1usize;
    while (i as i32) < (argc) as i32 {
        let arg = &argv[i];
        if arg == "--version" || arg == "-v" {
            let prog = if !argv.is_empty() { &argv[0] } else { "interpreter" };
            print_version(prog);
        } else if arg == "--help" || arg == "-h" {
            let prog = if !argv.is_empty() { &argv[0] } else { "interpreter" };
            print_help(prog);
        } else if arg == "--in" || arg == "-i" {
            if i + 1 < argv.len() {
                params.r#in = argv[i + 1].clone();
                i += 1;
            }
        } else if arg == "--out" || arg == "-o" {
            if i + 1 < argv.len() {
                params.out = argv[i + 1].clone();
                i += 1;
            }
        } else if arg == "--mode" || arg == "-m" {
            if i + 1 < argv.len() {
                let optarg = &argv[i + 1];
                if optarg == INTERPRETER_LEX_STR {
                    params.mode = INTERPRETER_MODE::INTERPRETER_LEX;
                } else if optarg == INTERPRETER_PARSE_STR {
                    params.mode = INTERPRETER_MODE::INTERPRETER_PARSE;
                } else if optarg == INTERPRETER_BC_STR {
                    params.mode = INTERPRETER_MODE::INTERPRETER_BC;
                } else if optarg == INTERPRETER_INTERPRET_STR {
                    params.mode = INTERPRETER_MODE::INTERPRETER_INTERPRET;
                } else if optarg == INTERPRETER_TRACE_STR {
                    params.mode = INTERPRETER_MODE::INTERPRETER_TRACE;
                } else {
                    eprintln!("Invalid interpreter mode \"{}\"", optarg);
                    std::process::exit(1);
                }
                i += 1;
            }
        } else if arg.starts_with("--") {
            // long options with '=' or separate value
            if let Some(eq_idx) = arg.find('=') {
                let name = &arg[2..eq_idx];
                let val = &arg[eq_idx + 1..];
                if name == STACKSIZE_STR {
                    if let Ok(v) = val.parse::<usize>() {
                        params.stacksize = v;
                    }
                } else if name == HEAPSIZE_STR {
                    if let Ok(v) = val.parse::<usize>() {
                        params.heapsize = v;
                    }
                }
            } else {
                let name = &arg[2..];
                if name == STACKSIZE_STR {
                    if i + 1 < argv.len() {
                        if let Ok(v) = argv[i + 1].parse::<usize>() {
                            params.stacksize = v;
                        }
                        i += 1;
                    }
                } else if name == HEAPSIZE_STR {
                    if i + 1 < argv.len() {
                        if let Ok(v) = argv[i + 1].parse::<usize>() {
                            params.heapsize = v;
                        }
                        i += 1;
                    }
                }
            }
        } else if arg.starts_with('-') {
            // short options group - handle only those defined
            let chars: Vec<char> = arg.chars().collect();
            for k in 1..chars.len() {
                match chars[k] {
                    'v' => {
                        let prog = if !argv.is_empty() { &argv[0] } else { "interpreter" };
                        print_version(prog);
                    }
                    'h' => {
                        let prog = if !argv.is_empty() { &argv[0] } else { "interpreter" };
                        print_help(prog);
                    }
                    'i' => {
                        if i + 1 < argv.len() {
                            params.r#in = argv[i + 1].clone();
                            i += 1;
                        }
                    }
                    'o' => {
                        if i + 1 < argv.len() {
                            params.out = argv[i + 1].clone();
                            i += 1;
                        }
                    }
                    'm' => {
                        if i + 1 < argv.len() {
                            let optarg = &argv[i + 1];
                            if optarg == INTERPRETER_LEX_STR {
                                params.mode = INTERPRETER_MODE::INTERPRETER_LEX;
                            } else if optarg == INTERPRETER_PARSE_STR {
                                params.mode = INTERPRETER_MODE::INTERPRETER_PARSE;
                            } else if optarg == INTERPRETER_BC_STR {
                                params.mode = INTERPRETER_MODE::INTERPRETER_BC;
                            } else if optarg == INTERPRETER_INTERPRET_STR {
                                params.mode = INTERPRETER_MODE::INTERPRETER_INTERPRET;
                            } else if optarg == INTERPRETER_TRACE_STR {
                                params.mode = INTERPRETER_MODE::INTERPRETER_TRACE;
                            } else {
                                eprintln!("Invalid interpreter mode \"{}\"", optarg);
                                std::process::exit(1);
                            }
                            i += 1;
                        }
                    }
                    _ => {
                        // unknown short option - ignore
                    }
                }
            }
        } else {
            // positional argument (e.g., input file)
            if params.r#in.is_empty() {
                params.r#in = arg.clone();
            }
        }
        i += 1;
    }

    0
}

// Opaque types used by the rest of the code. These are placeholders to preserve identifiers.
pub type lexer_type_t = usize;
pub type parser_type_t = usize;
pub type bytecode_generator_type_t = usize;
pub type virtual_machine_type_t = usize;
pub type bytecode_type_t = usize;

pub struct UNIT_AST {}
pub struct PARSER_ERROR {}
pub struct BYTECODE_ERROR {}

// Stubs and helpers for dumping results to files:
pub fn print_lexer_result(fname: &str, lexer: lexer_type_t) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        let _ = dump_lexer_to_xml_file(&mut f, lexer);
        // file will be closed when dropped
    }
}

pub fn print_parser_result(fname: &str, unit: &UNIT_AST) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        let _ = dump_unit_ast_to_xml_file(&mut f, unit);
    }
}

pub fn print_bytecode_result(fname: &str, bc: &bytecode_type_t) {
    if let Ok(mut f) = std::fs::File::create(fname) {
        let _ = dump_bytecode_to_xml_file(&mut f, bc);
    }
}

// The rest of functions called in main are provided as safe stubs to preserve identifier usage.
// They simulate the signatures and minimal behaviors. In the original project these are defined elsewhere.

pub fn create_lexer() -> lexer_type_t {
    0usize
}
pub fn lexer_conf_from_file(_lexer: lexer_type_t, _path: &str) {}
pub fn lexer_free(_lexer: lexer_type_t) {}

pub fn create_parser() -> parser_type_t {
    0usize
}
pub fn parser_conf(_parser: parser_type_t, _lexer: lexer_type_t) {}
pub const PARSER_OK: i32 = 0;
pub fn parser_parse(_parser: parser_type_t, _unit_out: &mut *mut UNIT_AST) -> i32 {
    PARSER_OK
}
pub fn parser_get_error(_parser: parser_type_t) -> PARSER_ERROR {
    PARSER_ERROR {}
}
pub fn print_parser_error(_err: &PARSER_ERROR) {}
pub fn parser_free(_parser: parser_type_t) {}

pub fn unit_ast_free(_unit: *mut UNIT_AST) {}

pub fn create_bytecode_generator() -> bytecode_generator_type_t {
    0usize
}
pub fn bytecode_generator_conf(_bcg: bytecode_generator_type_t, _unit: *mut UNIT_AST) {}
pub const BYTECODE_GENERATOR_OK: i32 = 0;
pub fn bytecode_generator_generate(_bcg: bytecode_generator_type_t, _bc_out: &mut bytecode_type_t) -> i32 {
    BYTECODE_GENERATOR_OK
}
pub fn bytecode_generator_get_error(_bcg: bytecode_generator_type_t) -> BYTECODE_ERROR {
    BYTECODE_ERROR {}
}
pub fn bytecode_generator_free(_bcg: bytecode_generator_type_t) {}
pub fn bytecode_free(_bc: bytecode_type_t) {}
pub fn dump_lexer_to_xml_file(_f: &mut std::fs::File, _lexer: lexer_type_t) {}
pub fn dump_unit_ast_to_xml_file(_f: &mut std::fs::File, _unit: &UNIT_AST) {}
pub fn dump_bytecode_to_xml_file(_f: &mut std::fs::File, _bc: &bytecode_type_t) {}

pub fn create_virtual_machine() -> virtual_machine_type_t {
    0usize
}
pub fn virtual_machine_conf(_vm: virtual_machine_type_t, _bc: bytecode_type_t, _stacksize: usize, _heapsize: usize, _trace: bool) {}
pub fn virtual_machine_run(_vm: virtual_machine_type_t) -> i32 {
    0
}
pub fn virtual_machine_free(_vm: virtual_machine_type_t) {}

pub fn bytecode_free(_bc: bytecode_type_t) {}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len() as i32;

    let mut r = 0i32;

    let mut params = INTERPRETER_PARAMS::default();
    let mut unit_ptr: *mut UNIT_AST = std::ptr::null_mut();
    let mut bc: bytecode_type_t = 0usize;

    let _ = parse_args(argc, &args, &mut params);

    let lexer = create_lexer();
    lexer_conf_from_file(lexer, params.r#in.as_str());

    if params.mode == INTERPRETER_MODE::INTERPRETER_LEX {
        print_lexer_result(&params.out, lexer);
        lexer_free(lexer);
        return;
    }

    let parser = create_parser();
    parser_conf(parser, lexer);
    r = parser_parse(parser, &mut unit_ptr);
    if r != PARSER_OK {
        let err = parser_get_error(parser);
        print_parser_error(&err);
        lexer_free(lexer);
        parser_free(parser);
        return;
    }

    if params.mode == INTERPRETER_MODE::INTERPRETER_PARSE {
        if !unit_ptr.is_null() {
            // Safe to create a reference for printing because we just parsed it (simulated).
            unsafe {
                print_parser_result(&params.out, &*unit_ptr);
            }
        } else {
            // If unit is null, still create empty UNIT_AST for printing stub
            let dummy = UNIT_AST {};
            print_parser_result(&params.out, &dummy);
        }
        lexer_free(lexer);
        parser_free(parser);
        unit_ast_free(unit_ptr);
        return;
    }

    lexer_free(lexer);
    parser_free(parser);

    let bc_gen = create_bytecode_generator();
    bytecode_generator_conf(bc_gen, unit_ptr);

    r = bytecode_generator_generate(bc_gen, &mut bc);
    if r != BYTECODE_GENERATOR_OK {
        let err = bytecode_generator_get_error(bc_gen);
        // print_bytecode_error(&err); // not defined originally in provided code's prototypes but called in C
        let _ = err;
        unit_ast_free(unit_ptr);
        bytecode_generator_free(bc_gen);
        return;
    }

    unit_ast_free(unit_ptr);

    if params.mode == INTERPRETER_MODE::INTERPRETER_BC {
        print_bytecode_result(&params.out, &bc);
        bytecode_generator_free(bc_gen);
        bytecode_free(bc);
        return;
    }

    bytecode_generator_free(bc_gen);

    let vm = create_virtual_machine();
    virtual_machine_conf(vm, bc, params.stacksize, params.heapsize, params.mode == INTERPRETER_MODE::INTERPRETER_TRACE);
    r = virtual_machine_run(vm);
    bytecode_free(bc);
    virtual_machine_free(vm);

    println!("result: {}", r);
}