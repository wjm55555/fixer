use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::pos::*;
use crate::token::*;
use crate::utils::*;

pub const LEXER_OK: i32 = 0;
pub const LEXER_INVALID_TOKEN: i32 = -1;

#[repr(u32)]
pub enum GROUP_TYPE {
    GROUP_TYPE_KEYWORDS = 0,
    GROUP_TYPE_IDENTS = 1,
    GROUP_TYPE_NUMBERS = 2,
    GROUP_TYPE_OPS = 3,
    GROUP_TYPE_AUX = 4,
}

#[repr(u32)]
pub enum TOKEN_TYPE {
    TOKEN_TYPE_FUNCTION = 0,
    TOKEN_TYPE_LET = 1,
    TOKEN_TYPE_IF = 2,
    TOKEN_TYPE_ELSE = 3,
    TOKEN_TYPE_WHILE = 4,
    TOKEN_TYPE_BREAK = 5,
    TOKEN_TYPE_CONTINUE = 6,
    TOKEN_TYPE_APPEND = 7,
    TOKEN_TYPE_DELETE = 8,
    TOKEN_TYPE_HAS_PROPERTY = 9,
    TOKEN_TYPE_LEN = 10,
    TOKEN_TYPE_RETURN = 11,
    TOKEN_TYPE_IDENT = 12,
    TOKEN_TYPE_OR = 13,
    TOKEN_TYPE_AND = 14,
    TOKEN_TYPE_EQEQ = 15,
    TOKEN_TYPE_NEQ = 16,
    TOKEN_TYPE_LT = 17,
    TOKEN_TYPE_GT = 18,
    TOKEN_TYPE_LE = 19,
    TOKEN_TYPE_GE = 20,
    TOKEN_TYPE_EQ = 21,
    TOKEN_TYPE_PLUS = 22,
    TOKEN_TYPE_MINUS = 23,
    TOKEN_TYPE_MUL = 24,
    TOKEN_TYPE_DIV = 25,
    TOKEN_TYPE_MOD = 26,
    TOKEN_TYPE_LPAREN = 27,
    TOKEN_TYPE_RPAREN = 28,
    TOKEN_TYPE_NUMBER = 29,
    TOKEN_TYPE_LBRACKET = 30,
    TOKEN_TYPE_RBRACKET = 31,
    TOKEN_TYPE_LBRACE = 32,
    TOKEN_TYPE_RBRACE = 33,
    TOKEN_TYPE_COMMA = 34,
    TOKEN_TYPE_SEMI = 35,
    TOKEN_TYPE_DOT = 36,
    TOKEN_TYPE_COLON = 37,
    TOKEN_TYPE_EOF = 38,
    TOKEN_TYPE_UNKNOWN = 39,
}

// Allow using `TOKEN_TYPE_FOO` without `TOKEN_TYPE::` qualification.
use self::TOKEN_TYPE::*;
// Same for group types.
use self::GROUP_TYPE::*;

#[derive(Copy, Clone)]
pub struct POS {
    pub program: *const [u8],
    pub program_len: usize,
    pub line: usize,
    pub pos: usize,
    pub index: usize,
}

impl Default for POS {
    fn default() -> Self {
        POS {
            program: std::ptr::null(),
            program_len: 0,
            line: 1,
            pos: 1,
            index: 0,
        }
    }
}

pub struct FRAG {
    pub starting: POS,
    pub following: POS,
}

impl Default for FRAG {
    fn default() -> Self {
        Self {
            starting: POS::default(),
            following: POS::default(),
        }
    }
}

#[derive(Clone)]
pub struct TOKEN {
    pub token_type: TOKEN_TYPE,
    pub group_type: GROUP_TYPE,
    pub frag: FRAG,
    pub int_val: i64,
    pub str_val: String,
}

impl Default for TOKEN {
    fn default() -> Self {
        Self {
            token_type: TOKEN_TYPE_EOF,
            group_type: GROUP_TYPE_AUX,
            frag: FRAG::default(),
            int_val: 0,
            str_val: String::new(),
        }
    }
}

pub type lexer_type_t = *mut LEXER;

// Keep older naming used by the translated parser.
pub type LexerType = lexer_type_t;

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

pub const function_keyword_len: usize = 8;
pub const let_keyword_len: usize = 3;
pub const if_keyword_len: usize = 2;
pub const else_keyword_len: usize = 4;
pub const while_keyword_len: usize = 5;
pub const break_keyword_len: usize = 5;
pub const continue_keyword_len: usize = 8;
pub const append_keyword_len: usize = 6;
pub const delete_keyword_len: usize = 6;
pub const has_property_keyword_len: usize = 12;
pub const len_keyword_len: usize = 3;
pub const return_keyword_len: usize = 6;

pub struct LEXER {
    pub program: Box<[u8]>,
    pub program_len: usize,
    pub program_name: [u8; 256],
    pub cur: POS,
}

pub fn create_lexer() -> Box<LEXER> {
    Box::new(LEXER {
        program: Box::new([]),
        program_len: 0,
        program_name: [0; 256],
        cur: POS::default(),
    })
}

// Compatibility wrapper (older code uses `lexer_free(&mut lexer)` style).
pub fn lexer_free(_lexer: &mut LEXER) {}

pub fn lexer_conf_from_file(lexer: &mut LEXER, fname: &str) {
    let program_name_bytes = fname.as_bytes();
    let copy_len = std::cmp::min(program_name_bytes.len(), 255);
    lexer.program_name[..copy_len].copy_from_slice(&program_name_bytes[..copy_len]);
    lexer.program_name[copy_len] = 0;

    if fname == "stdin" {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            lexer.program_len = buffer.len();
            let mut program_data = vec![0u8; buffer.len() + 1];
            program_data[..buffer.len()].copy_from_slice(buffer.as_bytes());
            program_data[buffer.len()] = 0;
            lexer.program = program_data.into_boxed_slice();
            lexer.cur.program = lexer.program.as_ref();
            lexer.cur.program_len = lexer.program_len;
        } else {
            eprintln!("Unable to read from stdin");
            std::process::exit(1);
        }
    } else {
        let mut file = match File::open(fname) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Unable to read input file \"{}\"", fname);
                std::process::exit(1);
            }
        };

        let file_size = match file.seek(SeekFrom::End(0)) {
            Ok(size) => size as usize,
            Err(_) => {
                eprintln!("Unable to read input file \"{}\"", fname);
                std::process::exit(1);
            }
        };
        let _ = file.seek(SeekFrom::Start(0));
        lexer.program_len = file_size;

        let mut program_data = vec![0u8; file_size + 1];
        match file.read_exact(&mut program_data[..file_size]) {
            Ok(_) => {
                program_data[file_size] = 0;
                lexer.program = program_data.into_boxed_slice();
            }
            Err(_) => {
                eprintln!("Unable to read input file \"{}\"", fname);
                std::process::exit(1);
            }
        }

        lexer.cur.program = lexer.program.as_ref();
        lexer.cur.program_len = lexer.program_len;
    }
}

pub fn lexer_conf_from_buf(lexer: &mut LEXER, text: &str, program_name: &str) {
    let program_name_bytes = program_name.as_bytes();
    let copy_len = std::cmp::min(program_name_bytes.len(), 255);
    lexer.program_name[..copy_len].copy_from_slice(&program_name_bytes[..copy_len]);
    lexer.program_name[copy_len] = 0;

    lexer.program_len = text.len();
    let mut program_data = vec![0u8; text.len() + 1];
    program_data[..text.len()].copy_from_slice(text.as_bytes());
    program_data[text.len()] = 0;
    lexer.program = program_data.into_boxed_slice();

    lexer.cur.program = lexer.program.as_ref();
    lexer.cur.program_len = lexer.program_len;
}

pub fn lexer_next_token(lexer: &mut LEXER, tok: &mut Option<Box<TOKEN>>) {
    let cur = &mut lexer.cur;
    *tok = None;

    while !pos_is_eof(cur) {
        let mut found_eof = 0;
        while pos_is_whitespace(cur) || pos_is_newline(cur) {
            *cur = pos_next(cur);
            if pos_is_eof(cur) {
                found_eof = 1;
                break;
            }
        }
        if found_eof != 0 {
            break;
        }

        let cur_next = pos_next(cur);
        let code = pos_get_code(cur);

        match code {
            b'f' => {
                if pos_check_keyword(cur, function_keyword.as_bytes(), function_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_FUNCTION,
                        function_keyword.as_bytes(),
                        function_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'l' => {
                if pos_check_keyword(cur, let_keyword.as_bytes(), let_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_LET,
                        let_keyword.as_bytes(),
                        let_keyword_len,
                    );
                } else if pos_check_keyword(cur, len_keyword.as_bytes(), len_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_LEN,
                        len_keyword.as_bytes(),
                        len_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'i' => {
                if pos_check_keyword(cur, if_keyword.as_bytes(), if_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_IF,
                        if_keyword.as_bytes(),
                        if_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'e' => {
                if pos_check_keyword(cur, else_keyword.as_bytes(), else_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_ELSE,
                        else_keyword.as_bytes(),
                        else_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'w' => {
                if pos_check_keyword(cur, while_keyword.as_bytes(), while_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_WHILE,
                        while_keyword.as_bytes(),
                        while_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'b' => {
                if pos_check_keyword(cur, break_keyword.as_bytes(), break_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_BREAK,
                        break_keyword.as_bytes(),
                        break_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'c' => {
                if pos_check_keyword(cur, continue_keyword.as_bytes(), continue_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_CONTINUE,
                        continue_keyword.as_bytes(),
                        continue_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'a' => {
                if pos_check_keyword(cur, append_keyword.as_bytes(), append_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_APPEND,
                        append_keyword.as_bytes(),
                        append_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'd' => {
                if pos_check_keyword(cur, delete_keyword.as_bytes(), delete_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_DELETE,
                        delete_keyword.as_bytes(),
                        delete_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'h' => {
                if pos_check_keyword(
                    cur,
                    has_property_keyword.as_bytes(),
                    has_property_keyword_len,
                ) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_HAS_PROPERTY,
                        has_property_keyword.as_bytes(),
                        has_property_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'r' => {
                if pos_check_keyword(cur, return_keyword.as_bytes(), return_keyword_len) {
                    token_read_keyword(
                        tok,
                        cur,
                        TOKEN_TYPE_RETURN,
                        return_keyword.as_bytes(),
                        return_keyword_len,
                    );
                } else {
                    token_read_ident(tok, cur);
                }
            }
            b'|' => {
                if pos_get_code(&cur_next) == b'|' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_OR, cur, &cur_next_next);
                }
            }
            b'&' => {
                if pos_get_code(&cur_next) == b'&' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_AND, cur, &cur_next_next);
                }
            }
            b'=' => {
                if pos_get_code(&cur_next) == b'=' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_EQEQ, cur, &cur_next_next);
                } else {
                    token_read_op(tok, TOKEN_TYPE_EQ, cur, &cur_next);
                }
            }
            b'!' => {
                if pos_get_code(&cur_next) == b'=' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_NEQ, cur, &cur_next_next);
                }
            }
            b'<' => {
                if pos_get_code(&cur_next) == b'=' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_LE, cur, &cur_next_next);
                } else {
                    token_read_op(tok, TOKEN_TYPE_LT, cur, &cur_next);
                }
            }
            b'>' => {
                if pos_get_code(&cur_next) == b'=' {
                    let cur_next_next = pos_next(&cur_next);
                    token_read_op(tok, TOKEN_TYPE_GE, cur, &cur_next_next);
                } else {
                    token_read_op(tok, TOKEN_TYPE_GT, cur, &cur_next);
                }
            }
            b'+' => {
                token_read_op(tok, TOKEN_TYPE_PLUS, cur, &cur_next);
            }
            b'-' => {
                token_read_op(tok, TOKEN_TYPE_MINUS, cur, &cur_next);
            }
            b'*' => {
                token_read_op(tok, TOKEN_TYPE_MUL, cur, &cur_next);
            }
            b'/' => {
                token_read_op(tok, TOKEN_TYPE_DIV, cur, &cur_next);
            }
            b'%' => {
                token_read_op(tok, TOKEN_TYPE_MOD, cur, &cur_next);
            }
            b'(' => {
                token_read_op(tok, TOKEN_TYPE_LPAREN, cur, &cur_next);
            }
            b')' => {
                token_read_op(tok, TOKEN_TYPE_RPAREN, cur, &cur_next);
            }
            b'[' => {
                token_read_op(tok, TOKEN_TYPE_LBRACKET, cur, &cur_next);
            }
            b']' => {
                token_read_op(tok, TOKEN_TYPE_RBRACKET, cur, &cur_next);
            }
            b'{' => {
                token_read_op(tok, TOKEN_TYPE_LBRACE, cur, &cur_next);
            }
            b'}' => {
                token_read_op(tok, TOKEN_TYPE_RBRACE, cur, &cur_next);
            }
            b',' => {
                token_read_op(tok, TOKEN_TYPE_COMMA, cur, &cur_next);
            }
            b';' => {
                token_read_op(tok, TOKEN_TYPE_SEMI, cur, &cur_next);
            }
            b'.' => {
                token_read_op(tok, TOKEN_TYPE_DOT, cur, &cur_next);
            }
            b':' => {
                token_read_op(tok, TOKEN_TYPE_COLON, cur, &cur_next);
            }
            _ => {
                if pos_is_digit(cur) {
                    let r = token_read_number(tok, cur);
                    if r == LEXER_INVALID_TOKEN {
                        if let Some(ref mut t) = tok {
                            t.token_type = TOKEN_TYPE_UNKNOWN;
                            t.group_type = GROUP_TYPE_AUX;
                        }
                    }
                } else if pos_is_letter(cur) {
                    token_read_ident(tok, cur);
                } else {
                    token_read_unknown(tok, cur);
                }
            }
        }

        if let Some(ref t) = tok {
            *cur = t.frag.following;
        }

        if let Some(ref t) = tok {
            if t.token_type == TOKEN_TYPE_UNKNOWN {
                let program_name_str = String::from_utf8_lossy(
                    &lexer.program_name[..lexer
                        .program_name
                        .iter()
                        .position(|&b| b == 0)
                        .unwrap_or(256)],
                );
                eprintln!(
                    "{}:{}:{}: warning: unknown token ''\n",
                    program_name_str, t.frag.starting.line, t.frag.starting.pos
                );
                *tok = None;
            } else {
                return;
            }
        }
    }

    *tok = Some(create_token());
    if let Some(ref mut t) = tok {
        t.token_type = TOKEN_TYPE_EOF;
        t.group_type = GROUP_TYPE_AUX;
        t.frag.starting = *cur;
        t.frag.following = *cur;
    }
}

pub fn dump_lexer_to_xml_file(f: &mut dyn std::io::Write, lexer: &mut LEXER) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<tokens>");

    loop {
        let mut tok: Option<Box<TOKEN>> = None;
        lexer_next_token(lexer, &mut tok);

        if let Some(t) = tok {
            if t.token_type != TOKEN_TYPE_EOF {
                let mut tmp_str = [0u8; 4096];
                token_to_xml_string_c(&t, &mut tmp_str, 4096);
                let s = String::from_utf8_lossy(
                    &tmp_str[..tmp_str.iter().position(|&b| b == 0).unwrap_or(4096)],
                );
                let _ = writeln!(f, "\t{}", s);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let _ = writeln!(f, "</tokens>");
}

pub fn lexer_free_box(_lexer: Box<LEXER>) {}
