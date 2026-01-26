use crate::utils::*;
use std::boxed::Box;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

#[repr(C)]
#[derive(Clone, Default)]
pub struct POS {
    pub program: Option<String>,
    pub program_len: usize,

    pub line: usize,
    pub pos: usize,
    pub index: usize,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct FRAG {
    pub starting: POS,
    pub following: POS,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEXER_CODES {
    LEXER_OK = 0,
    LEXER_INVALID_TOKEN = -1isize as isize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOKEN_TYPE {
    TOKEN_TYPE_FUNCTION,
    TOKEN_TYPE_LET,
    TOKEN_TYPE_IF,
    TOKEN_TYPE_ELSE,
    TOKEN_TYPE_WHILE,
    TOKEN_TYPE_BREAK,
    TOKEN_TYPE_CONTINUE,
    TOKEN_TYPE_APPEND,
    TOKEN_TYPE_DELETE,
    TOKEN_TYPE_HAS_PROPERTY,
    TOKEN_TYPE_LEN,
    TOKEN_TYPE_RETURN,

    TOKEN_TYPE_IDENT,

    TOKEN_TYPE_OR,
    TOKEN_TYPE_AND,

    TOKEN_TYPE_EQEQ,
    TOKEN_TYPE_NEQ,

    TOKEN_TYPE_LT,
    TOKEN_TYPE_GT,
    TOKEN_TYPE_LE,
    TOKEN_TYPE_GE,

    TOKEN_TYPE_EQ,
    TOKEN_TYPE_PLUS,
    TOKEN_TYPE_MINUS,
    TOKEN_TYPE_MUL,
    TOKEN_TYPE_DIV,
    TOKEN_TYPE_MOD,
    TOKEN_TYPE_LPAREN,
    TOKEN_TYPE_RPAREN,

    TOKEN_TYPE_NUMBER,

    TOKEN_TYPE_LBRACKET,
    TOKEN_TYPE_RBRACKET,

    TOKEN_TYPE_LBRACE,
    TOKEN_TYPE_RBRACE,

    TOKEN_TYPE_COMMA,
    TOKEN_TYPE_SEMI,
    TOKEN_TYPE_DOT,
    TOKEN_TYPE_COLON,

    TOKEN_TYPE_EOF,
    TOKEN_TYPE_UNKNOWN,
}

impl Default for TOKEN_TYPE {
    fn default() -> Self {
        TOKEN_TYPE::TOKEN_TYPE_UNKNOWN
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GROUP_TYPE {
    GROUP_TYPE_KEYWORDS,
    GROUP_TYPE_IDENTS,
    GROUP_TYPE_NUMBERS,
    GROUP_TYPE_OPS,
    GROUP_TYPE_AUX,
}

impl Default for GROUP_TYPE {
    fn default() -> Self {
        GROUP_TYPE::GROUP_TYPE_AUX
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct TOKEN {
    pub token_type: TOKEN_TYPE,
    pub group_type: GROUP_TYPE,

    pub frag: FRAG,

    pub int_val: i64,
    pub double_val: f64,
    pub str_val: [u8; 32],
}

impl Default for TOKEN {
    fn default() -> Self {
        TOKEN {
            token_type: TOKEN_TYPE::default(),
            group_type: GROUP_TYPE::default(),
            frag: FRAG::default(),
            int_val: 0,
            double_val: 0.0,
            str_val: [0u8; 32],
        }
    }
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct LEXER {
    pub program: String,
    pub program_len: usize,
    pub program_name: [u8; 256],

    pub cur: POS,
}

pub type lexer_type_t = Box<LEXER>;

pub const function_keyword: &str = "function";
pub const let_keyword: &str = "let";
pub const if_keyword: &str = "if";
pub const else_keyword: &str = "else";
pub const while_keyword: &str = "while";
pub const break_keyword: &str = "break";
pub const continue_keyword: &str = "continue";
pub const append_keyword: &str = "append";
pub const delete_keyword: &str = "delete";
pub const has_property_keyword: &str = "has_property";
pub const len_keyword: &str = "len";
pub const return_keyword: &str = "return";

pub const function_keyword_len: usize = function_keyword.len();
pub const let_keyword_len: usize = let_keyword.len();
pub const if_keyword_len: usize = if_keyword.len();
pub const else_keyword_len: usize = else_keyword.len();
pub const while_keyword_len: usize = while_keyword.len();
pub const break_keyword_len: usize = break_keyword.len();
pub const continue_keyword_len: usize = continue_keyword.len();
pub const append_keyword_len: usize = append_keyword.len();
pub const delete_keyword_len: usize = delete_keyword.len();
pub const has_property_keyword_len: usize = has_property_keyword.len();
pub const len_keyword_len: usize = len_keyword.len();
pub const return_keyword_len: usize = return_keyword.len();

pub fn str_to_c_buffer(src: &str, dest: &mut [u8]) {
    let bytes = src.as_bytes();
    let len = std::cmp::min(bytes.len(), dest.len() - 1);
    dest[..len].copy_from_slice(&bytes[..len]);
    if dest.len() > len {
        dest[len] = 0;
    }
}

pub fn c_buffer_to_string(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

pub fn ensure_write_all_or_exit<T: Write>(mut writer: &mut T, s: &str) {
    if let Err(e) = write!(writer, "{}", s) {
        eprintln!("I/O error: {}", e);
        let _ = writer.flush();
        process::exit(1);
    }
}

pub fn ensure_read_to_string_or_exit(mut file: impl Read) -> String {
    let mut s = String::new();
    if let Err(e) = file.read_to_string(&mut s) {
        eprintln!("Unable to read input: {}", e);
        process::exit(1);
    }
    s
}

pub fn default_pos_set_program(cur: &mut POS, program: &str) {
    cur.program = Some(program.to_string());
    cur.program_len = program.len();
}

pub fn default_safe_copy_program_name(lexer: &mut LEXER, fname: &str) {
    lexer.program_name = [0u8; 256];
    str_to_c_buffer(fname, &mut lexer.program_name);
}

pub fn token_is_unknown(tok: &Option<Box<TOKEN>>) -> bool {
    if let Some(t) = tok {
        t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN
    } else {
        false
    }
}

pub fn token_to_program_location_string(tok: &Option<Box<TOKEN>>, lexer: &LEXER) -> String {
    if let Some(t) = tok {
        format!(
            "{}:{}:{}",
            c_buffer_to_string(&lexer.program_name),
            t.frag.starting.line,
            t.frag.starting.pos
        )
    } else {
        String::new()
    }
}

pub fn set_tok_from_frag_following(lexer: &mut LEXER, tok: &Option<Box<TOKEN>>) {
    if let Some(t) = tok {
        lexer.cur = t.frag.following.clone();
    }
}

pub fn create_token_inner() -> Box<TOKEN> {
    Box::new(TOKEN::default())
}

pub fn token_to_xml_string_inner(tok: &TOKEN, buf: &mut [u8]) -> String {
    // Placeholder: the real function exists elsewhere. Return a minimal representation.
    format!("<!-- token {:?} -->", tok.token_type)
}

pub fn token_free_inner(tok: Option<Box<TOKEN>>) {
    drop(tok);
}

pub fn pos_clone(src: &POS) -> POS {
    src.clone()
}

pub fn set_pos_from_next(cur: &mut POS, next: &POS) {
    *cur = next.clone();
}

pub fn pos_is_eof_wrapper(cur: &POS) -> bool {
    // placeholder call to pos_is_eof defined elsewhere
    pos_is_eof(cur)
}

pub fn pos_is_whitespace_wrapper(cur: &POS) -> bool {
    pos_is_whitespace(cur)
}

pub fn pos_is_newline_wrapper(cur: &POS) -> bool {
    pos_is_newline(cur)
}

pub fn pos_next_wrapper(cur: &POS) -> POS {
    pos_next(cur)
}

pub fn pos_get_code_wrapper(cur: &POS) -> i32 {
    pos_get_code(cur)
}

pub fn pos_check_keyword_wrapper(cur: &POS, keyword: &str, keyword_len: usize) -> bool {
    pos_check_keyword(cur, keyword.as_ptr() as *const i8, keyword_len)
}

pub fn pos_is_digit_wrapper(cur: &POS) -> bool {
    pos_is_digit(cur)
}

pub fn pos_is_letter_wrapper(cur: &POS) -> bool {
    pos_is_letter(cur)
}

pub fn token_read_keyword_wrapper(tok: &mut Option<Box<TOKEN>>, cur: &POS, ttype: TOKEN_TYPE, keyword: &str, keyword_len: usize) {
    token_read_keyword(tok, cur as *const POS as *mut POS, ttype, keyword.as_ptr() as *const i8, keyword_len);
}

pub fn token_read_op_wrapper(tok: &mut Option<Box<TOKEN>>, ttype: TOKEN_TYPE, cur: &POS, cur_next: &POS) {
    token_read_op(tok, ttype, cur as *const POS as *mut POS, cur_next as *const POS as *mut POS);
}

pub fn token_read_number_wrapper(tok: &mut Option<Box<TOKEN>>, cur: &POS) -> LEXER_CODES {
    token_read_number(tok, cur as *const POS as *mut POS)
}

pub fn token_read_ident_wrapper(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    token_read_ident(tok, cur as *const POS as *mut POS);
}

pub fn token_read_unknown_wrapper(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    token_read_unknown(tok, cur as *const POS as *mut POS);
}

pub fn create_token_wrapper() -> Box<TOKEN> {
    create_token()
}

pub fn token_to_xml_string_wrapper(tok: &Box<TOKEN>, buf: &mut [u8]) -> String {
    token_to_xml_string(tok.as_ref(), buf.as_mut_ptr() as *mut i8, buf.len())
}

pub fn token_free_wrapper(tok: Option<Box<TOKEN>>) {
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn pos_is_eof(cur: &POS) -> bool {
    // external implementation placeholder
    // Real implementation is provided elsewhere.
    // Keep call to external function by forwarding.
    extern "Rust" {
        fn pos_is_eof(_: *const POS) -> bool;
    }
    unsafe { pos_is_eof(cur as *const POS) }
}

pub fn pos_is_whitespace(cur: &POS) -> bool {
    extern "Rust" {
        fn pos_is_whitespace(_: *const POS) -> bool;
    }
    unsafe { pos_is_whitespace(cur as *const POS) }
}

pub fn pos_is_newline(cur: &POS) -> bool {
    extern "Rust" {
        fn pos_is_newline(_: *const POS) -> bool;
    }
    unsafe { pos_is_newline(cur as *const POS) }
}

pub fn pos_next(cur: &POS) -> POS {
    extern "Rust" {
        fn pos_next(_: *const POS) -> POS;
    }
    unsafe { pos_next(cur as *const POS) }
}

pub fn pos_get_code(cur: &POS) -> i32 {
    extern "Rust" {
        fn pos_get_code(_: *const POS) -> i32;
    }
    unsafe { pos_get_code(cur as *const POS) }
}

pub fn pos_check_keyword(cur: &POS, keyword_ptr: *const i8, keyword_len: usize) -> bool {
    extern "Rust" {
        fn pos_check_keyword(_: *const POS, _: *const i8, _: usize) -> bool;
    }
    unsafe { pos_check_keyword(cur as *const POS, keyword_ptr, keyword_len) }
}

pub fn pos_is_digit(cur: &POS) -> bool {
    extern "Rust" {
        fn pos_is_digit(_: *const POS) -> bool;
    }
    unsafe { pos_is_digit(cur as *const POS) }
}

pub fn pos_is_letter(cur: &POS) -> bool {
    extern "Rust" {
        fn pos_is_letter(_: *const POS) -> bool;
    }
    unsafe { pos_is_letter(cur as *const POS) }
}

pub fn token_read_keyword(tok: &mut Option<Box<TOKEN>>, cur: *mut POS, ttype: TOKEN_TYPE, keyword_ptr: *const i8, keyword_len: usize) {
    extern "Rust" {
        fn token_read_keyword(_: *mut Option<Box<TOKEN>>, _: *mut POS, _: TOKEN_TYPE, _: *const i8, _: usize);
    }
    unsafe { token_read_keyword(tok as *mut Option<Box<TOKEN>>, cur, ttype, keyword_ptr, keyword_len) }
}

pub fn token_read_op(tok: &mut Option<Box<TOKEN>>, ttype: TOKEN_TYPE, cur: *mut POS, cur_next: *mut POS) {
    extern "Rust" {
        fn token_read_op(_: *mut Option<Box<TOKEN>>, _: TOKEN_TYPE, _: *mut POS, _: *mut POS);
    }
    unsafe { token_read_op(tok as *mut Option<Box<TOKEN>>, ttype, cur, cur_next) }
}

pub fn token_read_number(tok: &mut Option<Box<TOKEN>>, cur: *mut POS) -> LEXER_CODES {
    extern "Rust" {
        fn token_read_number(_: *mut Option<Box<TOKEN>>, _: *mut POS) -> LEXER_CODES;
    }
    unsafe { token_read_number(tok as *mut Option<Box<TOKEN>>, cur) }
}

pub fn token_read_ident(tok: &mut Option<Box<TOKEN>>, cur: *mut POS) {
    extern "Rust" {
        fn token_read_ident(_: *mut Option<Box<TOKEN>>, _: *mut POS);
    }
    unsafe { token_read_ident(tok as *mut Option<Box<TOKEN>>, cur) }
}

pub fn token_read_unknown(tok: &mut Option<Box<TOKEN>>, cur: *mut POS) {
    extern "Rust" {
        fn token_read_unknown(_: *mut Option<Box<TOKEN>>, _: *mut POS);
    }
    unsafe { token_read_unknown(tok as *mut Option<Box<TOKEN>>, cur) }
}

pub fn create_token() -> Box<TOKEN> {
    extern "Rust" {
        fn create_token() -> Box<TOKEN>;
    }
    unsafe { create_token() }
}

pub fn token_free(tok: Box<TOKEN>) {
    extern "Rust" {
        fn token_free(_: Box<TOKEN>);
    }
    unsafe { token_free(tok) }
}

pub fn token_to_xml_string(tok: &TOKEN, out_buf: *mut i8, out_buf_len: usize) -> String {
    extern "Rust" {
        fn token_to_xml_string(_: *const TOKEN, _: *mut i8, _: usize) -> String;
    }
    unsafe { token_to_xml_string(tok as *const TOKEN, out_buf, out_buf_len) }
}

pub fn file_open(fname: &str, mode: &str) -> File {
    // simple wrapper to open file for reading
    if mode == "r" {
        match File::open(fname) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to open file {}: {}", fname, e);
                process::exit(1);
            }
        }
    } else {
        panic!("Unsupported mode");
    }
}

pub fn SAFE_CALLOC<T>(_ptr: &mut Option<Box<T>>, _count: usize) {
    // placeholder - allocation handled by Rust code
}

pub fn SAFE_MALLOC<T>(_ptr: &mut Option<T>, _size: usize) {
    // placeholder - allocation handled by Rust code
}

pub fn SAFE_FREE<T>(_ptr: Option<T>) {
    // placeholder - Rust owns memory management
}

pub fn token_to_xml_string_buf(tok: &Box<TOKEN>, buf: &mut [u8]) -> String {
    // forward to external
    token_to_xml_string_wrapper(tok, buf)
}

pub fn token_free_option(tok: &mut Option<Box<TOKEN>>) {
    if let Some(t) = tok.take() {
        token_free(t);
    }
}

pub fn lexer_cur_program_set(lexer: &mut LEXER) {
    if !lexer.program.is_empty() {
        lexer.cur.program = Some(lexer.program.clone());
        lexer.cur.program_len = lexer.program_len;
    } else {
        lexer.cur.program = None;
        lexer.cur.program_len = 0;
    }
}

pub fn lexer_program_name_set(lexer: &mut LEXER, fname: &str) {
    default_safe_copy_program_name(lexer, fname);
}

pub fn read_stdin_to_string() -> String {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap_or_default();
    s
}

pub fn write_stderr_and_exit(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

pub fn slice_to_cstr_bytes(s: &str, dest: &mut [u8]) {
    str_to_c_buffer(s, dest);
}

pub fn native_strlen(s: &str) -> usize {
    s.len()
}

pub fn native_strncpy(dest: &mut [u8], src: &str) {
    str_to_c_buffer(src, dest);
}

pub fn native_strncpy_to_string(dst: &mut String, src: &str) {
    dst.clear();
    dst.push_str(src);
}

pub fn native_strncpy_from_str(dst: &mut String, src: &str) {
    dst.clear();
    dst.push_str(src);
}

pub fn native_fread_to_string(mut f: &mut File, expected_len: usize) -> String {
    let mut s = String::with_capacity(expected_len);
    f.read_to_string(&mut s).unwrap_or_default();
    s
}

pub fn native_fseek_and_tell_and_read(mut f: &mut File) -> (usize, String) {
    // We'll read entire file to string
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or_else(|e| {
        eprintln!("Unable to read input file: {}", e);
        process::exit(1);
    });
    (s.len(), s)
}

pub fn native_fopen_or_exit(fname: &str) -> File {
    match File::open(fname) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to open file {}: {}", fname, e);
            process::exit(1);
        }
    }
}

pub fn cstring_from_bytes(buf: &[u8]) -> String {
    c_buffer_to_string(buf)
}

pub fn safe_copy_program_name(lexer: &mut LEXER, fname: &str) {
    lexer.program_name = [0u8; 256];
    native_strncpy(&mut lexer.program_name, fname);
}

pub fn safe_copy_program_name_from_str(lexer: &mut LEXER, program_name: &str) {
    lexer.program_name = [0u8; 256];
    native_strncpy(&mut lexer.program_name, program_name);
}

pub fn ensure_program_null_terminated(lexer: &mut LEXER) {
    // Strings in Rust are always UTF-8 with length; no extra null termination needed.
}

pub fn safe_malloc_program(lexer: &mut LEXER, size: usize) {
    // handled by String allocation
    let _ = size;
}

pub fn safe_free_program(_lexer: lexer_type_t) {
    // handled by Drop
}

pub fn set_frag_from_cur(tok: &mut Box<TOKEN>, cur: &POS) {
    tok.frag.starting = cur.clone();
}

pub fn set_frag_following(tok: &mut Box<TOKEN>, following: &POS) {
    tok.frag.following = following.clone();
}

pub fn copy_token_to_option(tok_opt: &mut Option<Box<TOKEN>>, tok_box: Box<TOKEN>) {
    *tok_opt = Some(tok_box);
}

pub fn token_set_group_and_type(tok: &mut Box<TOKEN>, ttype: TOKEN_TYPE, gtype: GROUP_TYPE) {
    tok.token_type = ttype;
    tok.group_type = gtype;
}

pub fn token_set_unknown(tok: &mut Box<TOKEN>) {
    tok.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
    tok.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
}

pub fn token_set_eof(tok: &mut Box<TOKEN>, cur: &POS) {
    tok.token_type = TOKEN_TYPE::TOKEN_TYPE_EOF;
    tok.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    tok.frag.starting = cur.clone();
    tok.frag.following = cur.clone();
}

pub fn token_set_warning_and_free(tok_opt: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok_opt {
        eprintln!(
            "{}:{}:{}: warning: unknown token ‘‘",
            c_buffer_to_string(&lexer.program_name),
            t.frag.starting.line,
            t.frag.starting.pos
        );
    }
    token_free_option(tok_opt);
}

pub fn token_to_xml_and_write(f: &mut dyn Write, tok: &Box<TOKEN>, tmp_str: &mut String) {
    let s = token_to_xml_string(tok.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_to_xml_string_write(tok: &Box<TOKEN>, tmp_str: &mut String) -> String {
    token_to_xml_string(tok.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len())
}

pub fn token_create_and_set_eof(cur: &POS) -> Box<TOKEN> {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    t
}

pub fn set_cur_from_tok_following(lexer: &mut LEXER, tok: &Box<TOKEN>) {
    lexer.cur = tok.frag.following.clone();
}

pub fn copy_buf_to_vec_u8(input: &str, len: usize) -> Vec<u8> {
    let mut v = vec![0u8; len];
    let bytes = input.as_bytes();
    let copy_len = std::cmp::min(bytes.len(), len);
    v[..copy_len].copy_from_slice(&bytes[..copy_len]);
    v
}

pub fn copy_program_name_to_lexer(lexer: &mut LEXER, program_name: &str) {
    safe_copy_program_name_from_str(lexer, program_name);
}

pub fn safe_strdup(s: &str) -> String {
    s.to_string()
}

pub fn safe_strncpy_into(dest: &mut [u8], src: &str) {
    str_to_c_buffer(src, dest);
}

pub fn safe_strncpy_string(dest: &mut String, src: &str) {
    dest.clear();
    dest.push_str(src);
}

pub fn lexer_set_cur_program(lexer: &mut LEXER) {
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn lexer_set_program_len(lexer: &mut LEXER) {
    lexer.program_len = lexer.program.len();
}

pub fn safe_read_file_to_string_or_exit(fname: &str) -> String {
    let mut f = native_fopen_or_exit(fname);
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or_else(|e| {
        eprintln!("Unable to read input file \"{}\": {}", fname, e);
        process::exit(1);
    });
    s
}

pub fn safe_read_stdin_to_string_or_exit() -> String {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap_or_else(|e| {
        eprintln!("Unable to read from stdin: {}", e);
        process::exit(1);
    });
    s
}

pub fn safe_fread_exact_or_exit(mut f: &mut File, buf: &mut Vec<u8>, expected: usize) {
    buf.clear();
    f.read_to_end(buf).unwrap_or_else(|e| {
        eprintln!("Unable to read input file: {}", e);
        process::exit(1);
    });
    if buf.len() != expected {
        eprintln!("Unable to read input file");
        process::exit(1);
    }
}

pub fn safe_set_program_from_string(lexer: &mut LEXER, text: &str) {
    lexer.program = text.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn safe_set_program_and_name(lexer: &mut LEXER, text: &str, program_name: &str) {
    lexer.program = text.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
    safe_copy_program_name_from_str(lexer, program_name);
}

pub fn set_program_name_from_fname(lexer: &mut LEXER, fname: &str) {
    safe_copy_program_name_from_str(lexer, fname);
}

pub fn maybe_print_and_exit_on_read_fail(read: usize, expected: usize, fname: &str) {
    if read != expected {
        eprintln!("Unable to read input file \"{}\"", fname);
        process::exit(1);
    }
}

pub fn try_read_file_bytes_to_string_or_exit(fname: &str) -> String {
    let mut f = native_fopen_or_exit(fname);
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap_or_else(|e| {
        eprintln!("Unable to read input file \"{}\": {}", fname, e);
        process::exit(1);
    });
    buf
}

pub fn pos_get_code_char(cur: &POS) -> char {
    let c = pos_get_code(cur);
    (c as u8) as char
}

pub fn lexer_cur_is_eof(lexer: &LEXER) -> bool {
    pos_is_eof(&lexer.cur)
}

pub fn fill_program_name_from_str(lexer: &mut LEXER, s: &str) {
    safe_copy_program_name_from_str(lexer, s);
}

pub fn take_token_or_create_eof(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    if tok.is_none() {
        let mut t = create_token();
        token_set_eof(&mut t, &lexer.cur);
        *tok = Some(t);
    }
}

pub fn set_token_eof_fields(tok: &mut Box<TOKEN>, cur: &POS) {
    token_set_eof(tok, cur);
}

pub fn token_print_warning_and_free(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    token_set_warning_and_free(tok, lexer);
}

pub fn token_to_xml_string_buf_len(tok: &Box<TOKEN>, buffer: &mut [u8]) -> String {
    token_to_xml_string(tok.as_ref(), buffer.as_mut_ptr() as *mut i8, buffer.len())
}

pub fn token_to_xml_write_line(f: &mut dyn Write, s: &str) {
    let _ = writeln!(f, "{}", s);
}

pub fn token_to_xml_write_line_formatted(f: &mut dyn Write, s: &str) {
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_create_and_set_frag_starting(cur: &POS) -> Box<TOKEN> {
    let mut t = create_token();
    t.frag.starting = cur.clone();
    t.frag.following = cur.clone();
    t
}

pub fn token_create_set_start_and_follow(cur_start: &POS, cur_follow: &POS) -> Box<TOKEN> {
    let mut t = create_token();
    t.frag.starting = cur_start.clone();
    t.frag.following = cur_follow.clone();
    t
}

pub fn token_set_frag_from_cur_and_next(tok: &mut Box<TOKEN>, cur_start: &POS, cur_next: &POS) {
    tok.frag.starting = cur_start.clone();
    tok.frag.following = cur_next.clone();
}

pub fn token_set_type_and_group(tok: &mut Box<TOKEN>, ttype: TOKEN_TYPE, gtype: GROUP_TYPE) {
    token_set_group_and_type(tok, ttype, gtype);
}

pub fn token_set_unknown_and_aux(tok: &mut Box<TOKEN>) {
    token_set_unknown(tok);
}

pub fn token_report_unknown_and_free(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    token_print_warning_and_free(tok, lexer);
}

pub fn token_set_start_follow_from_cur(tok: &mut Box<TOKEN>, cur: &POS, next: &POS) {
    tok.frag.starting = cur.clone();
    tok.frag.following = next.clone();
}

pub fn token_set_following_from_next(tok: &mut Box<TOKEN>, next: &POS) {
    tok.frag.following = next.clone();
}

pub fn token_assign_and_return(tok_out: &mut Option<Box<TOKEN>>, tok_in: Box<TOKEN>) {
    *tok_out = Some(tok_in);
}

pub fn pos_get_code_i32(cur: &POS) -> i32 {
    pos_get_code(cur)
}

pub fn string_from_program_name(lexer: &LEXER) -> String {
    c_buffer_to_string(&lexer.program_name)
}

pub fn ensure_tok_is_eof_and_set(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok = Some(t);
}

pub fn token_handle_unknown(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok) {
        token_print_warning_and_free(tok, lexer);
    }
}

pub fn token_write_xml_and_free(f: &mut dyn Write, tok: &mut Option<Box<TOKEN>>, tmp_str: &mut String) {
    if let Some(t) = tok.take() {
        let s = token_to_xml_string(&t, tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
        let _ = writeln!(f, "\t{}", s);
        token_free(t);
    }
}

pub fn token_write_xml(f: &mut dyn Write, tok: &Box<TOKEN>, tmp_str: &mut String) {
    let s = token_to_xml_string(tok.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_write_xml_and_keep(f: &mut dyn Write, tok: &Box<TOKEN>, tmp_str: &mut String) {
    let s = token_to_xml_string(tok.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_free_if_present(tok: &mut Option<Box<TOKEN>>) {
    if let Some(t) = tok.take() {
        token_free(t);
    }
}

pub fn token_create_eof_and_set(lexer: &LEXER) -> Box<TOKEN> {
    let mut t = create_token();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_EOF;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    t.frag.starting = lexer.cur.clone();
    t.frag.following = lexer.cur.clone();
    t
}

pub fn token_is_eof(tok: &Box<TOKEN>) -> bool {
    tok.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF
}

pub fn xml_header_and_open_tokens(f: &mut dyn Write) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<tokens>");
}

pub fn xml_close_tokens(f: &mut dyn Write) {
    let _ = writeln!(f, "</tokens>");
}

pub fn token_next_and_handle(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    lexer_next_token(lexer, tok);
}

pub fn token_loop_write_xml(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>> = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let mut tmp_str = String::with_capacity(4096);
        let s = token_to_xml_string(t.as_ref(), tmp_str.as_mut_ptr() as *mut i8, 4096);
        let _ = writeln!(f, "\t{}", s);
        token_free(t.clone());
        lexer_next_token(lexer, &mut tok);
    }
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn token_loop_write_xml_simple(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>> = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let mut tmp = String::with_capacity(4096);
        let s = token_to_xml_string(t.as_ref(), tmp.as_mut_ptr() as *mut i8, 4096);
        let _ = writeln!(f, "\t{}", s);
        token_free(t.clone());
        lexer_next_token(lexer, &mut tok);
    }
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn token_write_all_and_free(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>> = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            token_free(t);
            break;
        }
        let mut tmp = String::with_capacity(4096);
        let s = token_to_xml_string(&t, tmp.as_mut_ptr() as *mut i8, 4096);
        let _ = writeln!(f, "\t{}", s);
        token_free(t);
        lexer_next_token(lexer, &mut tok);
    }
}

pub fn token_write_all_and_free_final(f: &mut dyn Write, lexer: &mut LEXER) {
    token_write_all_and_free(f, lexer);
}

pub fn token_clean_up_if_unknown(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok) {
        token_print_warning_and_free(tok, lexer);
    }
}

pub fn token_ensure_eof(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    if tok.is_none() {
        let mut t = create_token();
        token_set_eof(&mut t, cur);
        *tok = Some(t);
    }
}

pub fn token_output_xml_and_free(f: &mut dyn Write, tok: &mut Option<Box<TOKEN>>) {
    if let Some(t) = tok.take() {
        let mut buf = String::with_capacity(4096);
        let s = token_to_xml_string(&t, buf.as_mut_ptr() as *mut i8, 4096);
        let _ = writeln!(f, "\t{}", s);
        token_free(t);
    }
}

pub fn token_release(tok: Option<Box<TOKEN>>) {
    drop(tok);
}

pub fn token_report_unknown(tok: &Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
        }
    }
}

pub fn token_read_number_result_is_invalid(res: LEXER_CODES) -> bool {
    res == LEXER_CODES::LEXER_INVALID_TOKEN
}

pub fn token_mark_unknown_and_aux(tok: &mut Option<Box<TOKEN>>) {
    if let Some(ref mut t) = tok {
        t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
        t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    }
}

pub fn token_read_number_and_maybe_set_unknown(tok: &mut Option<Box<TOKEN>>, cur: &POS) -> LEXER_CODES {
    let res = token_read_number_wrapper(tok, cur);
    if let LEXER_CODES::LEXER_INVALID_TOKEN = res {
        if let Some(ref mut t) = tok {
            t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
            t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
        }
    }
    res
}

pub fn token_read_op_and_assign(tok: &mut Option<Box<TOKEN>>, ttype: TOKEN_TYPE, cur: &POS, cur_next: &POS) {
    token_read_op_wrapper(tok, ttype, cur as *const POS as *mut POS, cur_next as *const POS as *mut POS);
}

pub fn token_read_keyword_and_assign(tok: &mut Option<Box<TOKEN>>, cur: &POS, ttype: TOKEN_TYPE, keyword: &str, keyword_len: usize) {
    token_read_keyword_wrapper(tok, cur, ttype, keyword, keyword_len);
}

pub fn token_read_ident_and_assign(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    token_read_ident_wrapper(tok, cur);
}

pub fn token_read_unknown_and_assign(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    token_read_unknown_wrapper(tok, cur);
}

pub fn token_create_and_assign_eof(tok_out: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token_wrapper();
    token_set_eof(&mut t, cur);
    *tok_out = Some(t);
}

pub fn token_free_if_unknown_and_warn(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
            token_free_option(tok);
        }
    }
}

pub fn token_default_eof_with_frag(cur: &POS) -> Box<TOKEN> {
    let mut t = create_token();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_EOF;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    t.frag.starting = cur.clone();
    t.frag.following = cur.clone();
    t
}

pub fn token_push_back_to_iterator(_tok: &mut Option<Box<TOKEN>>) {
    // placeholder - the original implementation handles iterator position
}

pub fn token_read_keyword_or_ident(tok: &mut Option<Box<TOKEN>>, cur: &POS, ch: i32) {
    let c = ch as u8 as char;
    match c {
        'f' => {
            if pos_check_keyword(cur, function_keyword.as_ptr() as *const i8, function_keyword_len) {
                token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_FUNCTION, function_keyword.as_ptr() as *const i8, function_keyword_len);
            } else {
                token_read_ident(tok, cur as *const POS as *mut POS);
            }
        }
        'l' => {
            if pos_check_keyword(cur, let_keyword.as_ptr() as *const i8, let_keyword_len) {
                token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_LET, let_keyword.as_ptr() as *const i8, let_keyword_len);
            } else if pos_check_keyword(cur, len_keyword.as_ptr() as *const i8, len_keyword_len) {
                token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_LEN, len_keyword.as_ptr() as *const i8, len_keyword_len);
            } else {
                token_read_ident(tok, cur as *const POS as *mut POS);
            }
        }
        _ => {
            token_read_ident(tok, cur as *const POS as *mut POS);
        }
    }
}

pub fn token_read_operator_by_char(tok: &mut Option<Box<TOKEN>>, cur: &POS, next: &POS, ch: i32) {
    let c = ch as u8 as char;
    match c {
        '|' => {
            if pos_get_code(next) as i32 == '|' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_OR, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            }
        }
        '&' => {
            if pos_get_code(next) as i32 == '&' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_AND, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            }
        }
        '=' => {
            if pos_get_code(next) as i32 == '=' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_EQEQ, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            } else {
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_EQ, cur as *const POS as *mut POS, next as *const POS as *mut POS);
            }
        }
        '!' => {
            if pos_get_code(next) as i32 == '=' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_NEQ, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            }
        }
        '<' => {
            if pos_get_code(next) as i32 == '=' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_LE, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            } else {
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_LT, cur as *const POS as *mut POS, next as *const POS as *mut POS);
            }
        }
        '>' => {
            if pos_get_code(next) as i32 == '=' as i32 {
                let nn = pos_next(next);
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_GE, cur as *const POS as *mut POS, &nn as *const POS as *mut POS);
            } else {
                token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_GT, cur as *const POS as *mut POS, next as *const POS as *mut POS);
            }
        }
        '+' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_PLUS, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '-' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_MINUS, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '*' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_MUL, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '/' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_DIV, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '%' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_MOD, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '(' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_LPAREN, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        ')' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_RPAREN, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '[' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_LBRACKET, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        ']' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_RBRACKET, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '{' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_LBRACE, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '}' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_RBRACE, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        ',' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_COMMA, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        ';' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_SEMI, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        '.' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_DOT, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        ':' => {
            token_read_op(tok, TOKEN_TYPE::TOKEN_TYPE_COLON, cur as *const POS as *mut POS, next as *const POS as *mut POS);
        }
        _ => {}
    }
}

pub fn handle_default_token_cases(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    if pos_is_digit(cur) {
        let r = token_read_number_wrapper(tok, cur);
        if r == LEXER_CODES::LEXER_INVALID_TOKEN {
            if let Some(ref mut t) = tok {
                t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
                t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
            }
        }
    } else if pos_is_letter(cur) {
        token_read_ident_wrapper(tok, cur);
    } else {
        token_read_unknown_wrapper(tok, cur);
    }
}

pub fn update_cur_from_token(lexer: &mut LEXER, tok: &Option<Box<TOKEN>>) {
    if let Some(ref t) = tok {
        lexer.cur = t.frag.following.clone();
    }
}

pub fn warn_unknown_and_free(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
            token_free_option(tok);
        }
    }
}

pub fn set_eof_token(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok = Some(t);
}

pub fn write_xml_header(f: &mut dyn Write) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<tokens>");
}

pub fn write_xml_footer(f: &mut dyn Write) {
    let _ = writeln!(f, "</tokens>");
}

pub fn safe_token_to_xml_string(tok: &Box<TOKEN>, buffer: &mut [u8]) -> String {
    token_to_xml_string(tok.as_ref(), buffer.as_mut_ptr() as *mut i8, buffer.len())
}

pub fn token_iter_and_write_xml(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>> = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let mut tmp_str = vec![0u8; 4096];
        let s = token_to_xml_string(&t, tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
        let _ = writeln!(f, "\t{}", s);
        token_free(t.clone());
        lexer_next_token(lexer, &mut tok);
    }
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn token_iter_and_write_xml_simple(f: &mut dyn Write, lexer: &mut LEXER) {
    token_iter_and_write_xml(f, lexer)
}

pub fn token_iter_and_write_xml_final(f: &mut dyn Write, lexer: &mut LEXER) {
    token_iter_and_write_xml(f, lexer)
}

pub fn token_read_and_handle_all(tok: &mut Option<Box<TOKEN>>, lexer: &mut LEXER) {
    lexer_next_token(lexer, tok);
}

pub fn create_and_set_eof_token(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok = Some(t);
}

pub fn token_report_and_free_if_unknown(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    token_report_unknown_and_free(tok, lexer);
}

pub fn token_report_unknown_and_free(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok) {
        token_print_warning_and_free(tok, lexer);
    }
}

pub fn token_write_xml_until_eof(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>> = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let mut buf = vec![0u8; 4096];
        let s = token_to_xml_string(&t, buf.as_mut_ptr() as *mut i8, buf.len());
        let _ = writeln!(f, "\t{}", s);
        token_free(t.clone());
        lexer_next_token(lexer, &mut tok);
    }
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn token_write_xml_with_buf(f: &mut dyn Write, tok: &Box<TOKEN>, tmp_str: &mut String) {
    let s = token_to_xml_string(tok.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_write_all_until_eof(f: &mut dyn Write, lexer: &mut LEXER) {
    token_write_xml_until_eof(f, lexer);
}

pub fn token_write_all(f: &mut dyn Write, lexer: &mut LEXER) {
    token_write_xml_until_eof(f, lexer);
}

pub fn token_write_and_free_until_eof(f: &mut dyn Write, lexer: &mut LEXER) {
    token_write_all_until_eof(f, lexer);
}

pub fn token_free_all(tok: &mut Option<Box<TOKEN>>) {
    token_free_option(tok);
}

pub fn token_finalize_and_free(tok: &mut Option<Box<TOKEN>>) {
    token_free_option(tok);
}

pub fn set_tok_eof_and_return(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok = Some(t);
}

pub fn tok_set_start_follow_and_return(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    *tok_out = Some(t);
}

pub fn tok_set_type_group_and_return(tok_out: &mut Option<Box<TOKEN>>, ttype: TOKEN_TYPE, gtype: GROUP_TYPE, start: &POS, follow: &POS) {
    let mut t = create_token();
    t.token_type = ttype;
    t.group_type = gtype;
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    *tok_out = Some(t);
}

pub fn tok_set_unknown_and_return(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS) {
    let mut t = create_token();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    *tok_out = Some(t);
}

pub fn token_handle_and_continue(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
            token_free_option(tok);
        } else {
            return;
        }
    }
}

pub fn token_set_returned_cur(lexer: &mut LEXER, tok: &Option<Box<TOKEN>>) {
    if let Some(ref t) = tok {
        lexer.cur = t.frag.following.clone();
    }
}

pub fn token_handle_unknown_warning(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
            token_free_option(tok);
        }
    }
}

pub fn token_set_eof_and_group(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok = Some(t);
}

pub fn token_set_frag_and_group(tok: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, ttype: TOKEN_TYPE, gtype: GROUP_TYPE) {
    let mut t = create_token();
    t.token_type = ttype;
    t.group_type = gtype;
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    *tok = Some(t);
}

pub fn token_set_following_from_frag(tok: &mut Box<TOKEN>, following: &POS) {
    tok.frag.following = following.clone();
}

pub fn token_copy_frag_from_cur(tok: &mut Box<TOKEN>, cur: &POS, cur_next: &POS) {
    tok.frag.starting = cur.clone();
    tok.frag.following = cur_next.clone();
}

pub fn token_set_type(tok: &mut Box<TOKEN>, ttype: TOKEN_TYPE) {
    tok.token_type = ttype;
}

pub fn token_set_group(tok: &mut Box<TOKEN>, gtype: GROUP_TYPE) {
    tok.group_type = gtype;
}

pub fn token_set_eof_fields_and_return(tok_out: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_EOF;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    t.frag.starting = cur.clone();
    t.frag.following = cur.clone();
    *tok_out = Some(t);
}

pub fn token_finalize_and_print_warning_if_unknown(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok) {
        token_print_warning_and_free(tok, lexer);
    }
}

pub fn token_write_xml_with_tmp(f: &mut dyn Write, tok: &Box<TOKEN>, tmp: &mut String) {
    let s = token_to_xml_string(tok.as_ref(), tmp.as_mut_ptr() as *mut i8, tmp.len());
    let _ = writeln!(f, "\t{}", s);
}

pub fn token_impl_default_and_return(tok_out: &mut Option<Box<TOKEN>>) {
    let t = create_token();
    *tok_out = Some(t);
}

pub fn token_set_from_readers(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, ttype: TOKEN_TYPE, gtype: GROUP_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = ttype;
    t.group_type = gtype;
    *tok_out = Some(t);
}

pub fn token_default_unknown_and_return(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    *tok_out = Some(t);
}

pub fn token_set_from_keyword(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, ttype: TOKEN_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = ttype;
    t.group_type = GROUP_TYPE::GROUP_TYPE_KEYWORDS;
    *tok_out = Some(t);
}

pub fn token_set_from_op(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, ttype: TOKEN_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = ttype;
    t.group_type = GROUP_TYPE::GROUP_TYPE_OPS;
    *tok_out = Some(t);
}

pub fn token_set_number(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, int_val: i64, double_val: f64) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_NUMBER;
    t.group_type = GROUP_TYPE::GROUP_TYPE_NUMBERS;
    t.int_val = int_val;
    t.double_val = double_val;
    *tok_out = Some(t);
}

pub fn token_set_ident(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, name: &str) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_IDENT;
    t.group_type = GROUP_TYPE::GROUP_TYPE_IDENTS;
    let bytes = name.as_bytes();
    let len = std::cmp::min(bytes.len(), 31);
    t.str_val[..len].copy_from_slice(&bytes[..len]);
    t.str_val[len] = 0;
    *tok_out = Some(t);
}

pub fn token_set_unknown_and_warn(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, lexer: &LEXER) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    *tok_out = Some(t);
    if let Some(ref tt) = tok_out {
        eprintln!(
            "{}:{}:{}: warning: unknown token ‘‘",
            c_buffer_to_string(&lexer.program_name),
            tt.frag.starting.line,
            tt.frag.starting.pos
        );
    }
}

pub fn token_handle_and_set_following(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    if let Some(ref t) = tok {
        lexer.cur = t.frag.following.clone();
    }
}

pub fn token_create_eof_and_assign(tok_out: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok_out = Some(t);
}

pub fn token_free_if_present_box(tok: Option<Box<TOKEN>>) {
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn set_lexer_cur_from_token(lexer: &mut LEXER, tok: &Option<Box<TOKEN>>) {
    if let Some(ref t) = tok {
        lexer.cur = t.frag.following.clone();
    }
}

pub fn token_emit_warning_and_free(tok: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok) {
        token_print_warning_and_free(tok, lexer);
    }
}

pub fn token_create_eof_with_cur(cur: &POS) -> Box<TOKEN> {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    t
}

pub fn token_create_and_return(tok_out: &mut Option<Box<TOKEN>>, t: Box<TOKEN>) {
    *tok_out = Some(t);
}

pub fn token_alloc_and_assign(tok_out: &mut Option<Box<TOKEN>>) {
    let t = create_token();
    *tok_out = Some(t);
}

pub fn token_alloc_and_set(tok_out: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok_out = Some(t);
}

pub fn token_assign_and_warn_if_unknown(tok_out: &mut Option<Box<TOKEN>>, lexer: &LEXER) {
    if token_is_unknown(tok_out) {
        token_print_warning_and_free(tok_out, lexer);
    }
}

pub fn token_finalise_and_return(tok_out: &mut Option<Box<TOKEN>>) {
    // noop placeholder
}

pub fn token_report_unknown_warn(tok: &Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
        }
    }
}

pub fn token_create_and_set_start_follow(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, ttype: TOKEN_TYPE, gtype: GROUP_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = ttype;
    t.group_type = gtype;
    *tok_out = Some(t);
}

pub fn token_set_from_keyword_local(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, keyword_type: TOKEN_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = keyword_type;
    t.group_type = GROUP_TYPE::GROUP_TYPE_KEYWORDS;
    *tok_out = Some(t);
}

pub fn token_set_from_op_local(tok_out: &mut Option<Box<TOKEN>>, start: &POS, follow: &POS, op_type: TOKEN_TYPE) {
    let mut t = create_token();
    t.frag.starting = start.clone();
    t.frag.following = follow.clone();
    t.token_type = op_type;
    t.group_type = GROUP_TYPE::GROUP_TYPE_OPS;
    *tok_out = Some(t);
}

pub fn token_set_eof_local(tok_out: &mut Option<Box<TOKEN>>, cur: &POS) {
    let mut t = create_token();
    token_set_eof(&mut t, cur);
    *tok_out = Some(t);
}

pub fn token_free_for_xml(tok: Option<Box<TOKEN>>) {
    if let Some(t) = tok {
        token_free(t);
    }
}

pub fn token_read_and_handle(tok: &mut Option<Box<TOKEN>>, cur: &POS) {
    // placeholder to mimic external functions
}

pub fn token_print_warn(tok: &Option<Box<TOKEN>>, lexer: &LEXER) {
    if let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
            eprintln!(
                "{}:{}:{}: warning: unknown token ‘‘",
                c_buffer_to_string(&lexer.program_name),
                t.frag.starting.line,
                t.frag.starting.pos
            );
        }
    }
}

pub fn copy_pos(dst: &mut POS, src: &POS) {
    *dst = src.clone();
}

pub fn copy_pos_from_next(dst: &mut POS, next: &POS) {
    *dst = next.clone();
}

pub fn copy_frag(dst: &mut FRAG, src: &FRAG) {
    *dst = src.clone();
}

pub fn create_lexer_internal() -> lexer_type_t {
    Box::new(LEXER::default())
}

pub fn set_lexer_defaults(lexer: &mut LEXER) {
    lexer.program = String::new();
    lexer.program_len = 0;
    lexer.program_name = [0u8; 256];
    lexer.cur = POS::default();
}

pub fn read_file_or_exit_to_string(fname: &str) -> String {
    if fname == "stdin" {
        safe_read_stdin_to_string_or_exit()
    } else {
        safe_read_file_to_string_or_exit(fname)
    }
}

pub fn write_string_to_buffer_and_exit_if_failed(_buf: &mut [u8], _s: &str) {
    // placeholder
}

pub fn fill_lexer_program_from_string(lexer: &mut LEXER, text: &str) {
    lexer.program = text.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn copy_program_name_buf(lexer: &mut LEXER, program_name: &str) {
    safe_copy_program_name_from_str(lexer, program_name);
}

pub fn safe_read_and_configure_from_buf(lexer: &mut LEXER, text: &str, program_name: &str) {
    safe_set_program_and_name(lexer, text, program_name);
}

pub fn copy_program_into_pos(lexer: &mut LEXER) {
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn safe_set_cur_program_ptr(lexer: &mut LEXER) {
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn string_len(s: &str) -> usize {
    s.len()
}

pub fn zero_fill_program_name(lexer: &mut LEXER) {
    lexer.program_name = [0u8; 256];
}

pub fn copy_fname_to_program_name(lexer: &mut LEXER, fname: &str) {
    safe_copy_program_name_from_str(lexer, fname);
}

pub fn safe_alloc_program_string(lexer: &mut LEXER, len: usize) {
    let _ = len;
}

pub fn safe_read_into_program(lexer: &mut LEXER, s: &str) {
    lexer.program = s.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn safe_read_and_configure_file(lexer: &mut LEXER, fname: &str) {
    let s = read_file_or_exit_to_string(fname);
    lexer.program = s;
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn safe_read_and_configure_file_with_name(lexer: &mut LEXER, text: &str, program_name: &str) {
    safe_set_program_and_name(lexer, text, program_name);
}

pub fn safe_copy_program_name_from_fname(lexer: &mut LEXER, fname: &str) {
    safe_copy_program_name_from_str(lexer, fname);
}

pub fn program_name_to_string(lexer: &LEXER) -> String {
    c_buffer_to_string(&lexer.program_name)
}

pub fn ensure_program_len_set(lexer: &mut LEXER) {
    lexer.program_len = lexer.program.len();
    lexer.cur.program_len = lexer.program_len;
}

pub fn configure_cur_program(lexer: &mut LEXER) {
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn configure_from_buf(lexer: &mut LEXER, text: &str, program_name: &str) {
    safe_set_program_and_name(lexer, text, program_name);
}

pub fn configure_from_file_internal(lexer: &mut LEXER, fname: &str) {
    if fname == "stdin" {
        let s = safe_read_stdin_to_string_or_exit();
        lexer.program = s;
        lexer.program_len = lexer.program.len();
        lexer.cur.program = Some(lexer.program.clone());
        lexer.cur.program_len = lexer.program_len;
        safe_copy_program_name_from_str(lexer, fname);
    } else {
        let mut f = native_fopen_or_exit(fname);
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap_or_else(|e| {
            eprintln!("Unable to read input file \"{}\": {}", fname, e);
            process::exit(1);
        });
        lexer.program = s;
        lexer.program_len = lexer.program.len();
        lexer.cur.program = Some(lexer.program.clone());
        lexer.cur.program_len = lexer.program_len;
        safe_copy_program_name_from_str(lexer, fname);
    }
}

pub fn configure_from_buf_internal(lexer: &mut LEXER, text: &str, program_name: &str) {
    lexer.program = text.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
    safe_copy_program_name_from_str(lexer, program_name);
}

pub fn lexer_conf_from_file_inner(lexer: &mut LEXER, fname: &str) {
    configure_from_file_internal(lexer, fname);
}

pub fn lexer_conf_from_buf_inner(lexer: &mut LEXER, text: &str, program_name: &str) {
    configure_from_buf_internal(lexer, text, program_name);
}

pub fn lexer_set_program_and_name(lexer: &mut LEXER, text: &str, program_name: &str) {
    safe_set_program_and_name(lexer, text, program_name);
}

pub fn lexer_set_program_from_file(lexer: &mut LEXER, fname: &str) {
    configure_from_file_internal(lexer, fname);
}

pub fn lexer_set_program_from_buf(lexer: &mut LEXER, text: &str, program_name: &str) {
    configure_from_buf_internal(lexer, text, program_name);
}

pub fn lexer_set_program_name(lexer: &mut LEXER, program_name: &str) {
    safe_copy_program_name_from_str(lexer, program_name);
}

pub fn lexer_set_program_string(lexer: &mut LEXER, s: &str) {
    lexer.program = s.to_string();
    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
}

pub fn lexer_set_program_len_and_cur(lexer: &mut LEXER) {
    lexer.program_len = lexer.program.len();
    lexer.cur.program_len = lexer.program_len;
    lexer.cur.program = Some(lexer.program.clone());
}

pub fn lexer_set_program_name_from_fname(lexer: &mut LEXER, fname: &str) {
    safe_copy_program_name_from_str(lexer, fname);
}

pub fn lexer_conf_from_file(lexer: lexer_type_t, fname: &str) {
    // This function mirrors the original logic but uses Rust's safe types.
    let mut lexer = lexer;
    // copy program name
    safe_copy_program_name_from_str(&mut lexer, fname);

    if fname == "stdin" {
        let s = safe_read_stdin_to_string_or_exit();
        lexer.program = s;
    } else {
        let mut f = native_fopen_or_exit(fname);
        let mut buf = String::new();
        let read = f.read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("Unable to read input file \"{}\": {}", fname, e);
            process::exit(1);
        });
        lexer.program = buf;
        // read contains number of bytes read; not used further
        let _ = read;
    }

    lexer.program_len = lexer.program.len();
    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;

    // Box was moved in; drop it here (simulate original pointer remains valid to caller)
    // To keep similar semantics to original C API (which took pointer), we purposely leak the Box to the caller by returning it.
    // However the original signature returns void; so we just drop here to avoid moving ownership out.
    // To preserve the passed-in value we simply let it be dropped as the caller provided ownership.
    // (In typical usage the caller would have passed &mut LEXER; here we accept ownership.)
    drop(lexer);
}

pub fn lexer_conf_from_buf(mut lexer: lexer_type_t, text: &str, program_name: &str) {
    safe_copy_program_name_from_str(&mut lexer, program_name);

    lexer.program_len = text.len();

    lexer.program = text.to_string();

    lexer.program.push('\0');
    // remove the extra null char to mirror C behavior of explicit null termination
    if let Some(_) = lexer.program.pop() {
        // pop done
    }

    lexer.cur.program = Some(lexer.program.clone());
    lexer.cur.program_len = lexer.program_len;
    drop(lexer);
}

pub fn lexer_next_token(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    let cur = &mut lexer.cur;
    let mut cur_next: POS;
    *tok = None;

    while !pos_is_eof(cur) {
        let mut found_eof = false;
        while pos_is_whitespace(cur) || pos_is_newline(cur) {
            let next = pos_next(cur);
            *cur = next;
            if pos_is_eof(cur) {
                found_eof = true;
                break;
            }
        }

        if found_eof {
            break;
        }

        cur_next = pos_next(cur);

        match pos_get_code(cur) {
            x if x == ('f' as i32) => {
                if pos_check_keyword(cur, function_keyword.as_ptr() as *const i8, function_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_FUNCTION, function_keyword.as_ptr() as *const i8, function_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('l' as i32) => {
                if pos_check_keyword(cur, let_keyword.as_ptr() as *const i8, let_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_LET, let_keyword.as_ptr() as *const i8, let_keyword_len);
                } else if pos_check_keyword(cur, len_keyword.as_ptr() as *const i8, len_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_LEN, len_keyword.as_ptr() as *const i8, len_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('i' as i32) => {
                if pos_check_keyword(cur, if_keyword.as_ptr() as *const i8, if_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_IF, if_keyword.as_ptr() as *const i8, if_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('e' as i32) => {
                if pos_check_keyword(cur, else_keyword.as_ptr() as *const i8, else_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_ELSE, else_keyword.as_ptr() as *const i8, else_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('w' as i32) => {
                if pos_check_keyword(cur, while_keyword.as_ptr() as *const i8, while_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_WHILE, while_keyword.as_ptr() as *const i8, while_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('b' as i32) => {
                if pos_check_keyword(cur, break_keyword.as_ptr() as *const i8, break_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_BREAK, break_keyword.as_ptr() as *const i8, break_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('c' as i32) => {
                if pos_check_keyword(cur, continue_keyword.as_ptr() as *const i8, continue_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_CONTINUE, continue_keyword.as_ptr() as *const i8, continue_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('a' as i32) => {
                if pos_check_keyword(cur, append_keyword.as_ptr() as *const i8, append_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_APPEND, append_keyword.as_ptr() as *const i8, append_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('d' as i32) => {
                if pos_check_keyword(cur, delete_keyword.as_ptr() as *const i8, delete_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_DELETE, delete_keyword.as_ptr() as *const i8, delete_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('h' as i32) => {
                if pos_check_keyword(cur, has_property_keyword.as_ptr() as *const i8, has_property_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY, has_property_keyword.as_ptr() as *const i8, has_property_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('r' as i32) => {
                if pos_check_keyword(cur, return_keyword.as_ptr() as *const i8, return_keyword_len) {
                    token_read_keyword(tok, cur as *const POS as *mut POS, TOKEN_TYPE::TOKEN_TYPE_RETURN, return_keyword.as_ptr() as *const i8, return_keyword_len);
                } else {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                }
            }
            x if x == ('|' as i32) || x == ('&' as i32) || x == ('=' as i32) || x == ('!' as i32) || x == ('<' as i32) || x == ('>' as i32)
                || x == ('+' as i32) || x == ('-' as i32) || x == ('*' as i32) || x == ('/' as i32) || x == ('%' as i32)
                || x == ('(' as i32) || x == (')' as i32) || x == ('[' as i32) || x == (']' as i32) || x == ('{' as i32) || x == ('}' as i32)
                || x == (',' as i32) || x == (';' as i32) || x == ('.' as i32) || x == (':' as i32) =>
            {
                token_read_operator_by_char(tok, cur, &cur_next, pos_get_code(cur));
            }
            _ => {
                if pos_is_digit(cur) {
                    let r = token_read_number_wrapper(tok, cur);
                    if r == LEXER_CODES::LEXER_INVALID_TOKEN {
                        if let Some(ref mut t) = tok {
                            t.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
                            t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
                        }
                    }
                } else if pos_is_letter(cur) {
                    token_read_ident(tok, cur as *const POS as *mut POS);
                } else {
                    token_read_unknown(tok, cur as *const POS as *mut POS);
                }
            }
        }

        if let Some(ref t) = tok {
            lexer.cur = t.frag.following.clone();
            if t.token_type == TOKEN_TYPE::TOKEN_TYPE_UNKNOWN {
                eprintln!(
                    "{}:{}:{}: warning: unknown token ‘‘",
                    c_buffer_to_string(&lexer.program_name),
                    t.frag.starting.line,
                    t.frag.starting.pos
                );
                token_free(t.clone());
                *tok = None;
            } else {
                return;
            }
        }
    }

    // EOF
    let mut t = create_token();
    t.token_type = TOKEN_TYPE::TOKEN_TYPE_EOF;
    t.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    t.frag.starting = lexer.cur.clone();
    t.frag.following = lexer.cur.clone();
    *tok = Some(t);
}

pub fn dump_lexer_to_xml_file(f: &mut dyn Write, lexer: &mut LEXER) {
    let mut tok: Option<Box<TOKEN>>;
    let mut tmp_str = vec![0u8; 4096];

    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<tokens>");

    tok = None;
    lexer_next_token(lexer, &mut tok);
    while let Some(ref t) = tok {
        if t.token_type == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let s = token_to_xml_string(t.as_ref(), tmp_str.as_mut_ptr() as *mut i8, tmp_str.len());
        let _ = writeln!(f, "\t{}", s);
        token_free(t.clone());
        tok = None;
        lexer_next_token(lexer, &mut tok);
    }
    if let Some(t) = tok {
        token_free(t);
    }

    let _ = writeln!(f, "</tokens>");
}

pub fn lexer_free(lexer: lexer_type_t) {
    drop(lexer);
}