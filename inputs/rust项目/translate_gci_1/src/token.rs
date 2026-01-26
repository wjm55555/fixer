use crate::lexer::*;
use std::cmp::min;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct POS {
    pub program: String,
    pub index: usize,
}

impl Default for POS {
    fn default() -> Self {
        POS {
            program: String::new(),
            index: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FRAG {
    pub starting: POS,
    pub following: POS,
}

impl Default for FRAG {
    fn default() -> Self {
        FRAG {
            starting: POS::default(),
            following: POS::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOKEN_TYPE {
    TOKEN_TYPE_NUMBER,
    TOKEN_TYPE_IDENT,
    TOKEN_TYPE_UNKNOWN,
    TOKEN_TYPE_OTHER,
}

impl Default for TOKEN_TYPE {
    fn default() -> Self {
        TOKEN_TYPE::TOKEN_TYPE_OTHER
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
    GROUP_TYPE_OTHER,
}

impl Default for GROUP_TYPE {
    fn default() -> Self {
        GROUP_TYPE::GROUP_TYPE_OTHER
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEXER_CODES {
    LEXER_OK = 0,
    LEXER_INVALID_TOKEN = 1,
}

impl Default for LEXER_CODES {
    fn default() -> Self {
        LEXER_CODES::LEXER_OK
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct TOKEN {
    pub token_type: TOKEN_TYPE,
    pub group_type: GROUP_TYPE,
    pub frag: FRAG,
    pub int_val: i64,
    pub str_val: String,
}

impl Default for TOKEN {
    fn default() -> Self {
        TOKEN {
            token_type: TOKEN_TYPE::default(),
            group_type: GROUP_TYPE::default(),
            frag: FRAG::default(),
            int_val: 0,
            str_val: String::new(),
        }
    }
}

pub fn pos_next(p: &POS) -> POS {
    let mut np = p.clone();
    if np.index < np.program.len() {
        // advance by one byte (approximate as in C char step)
        np.index = np.index.saturating_add(1);
        if np.index > np.program.len() {
            np.index = np.program.len();
        }
    }
    np
}

pub fn pos_is_digit(p: &POS) -> bool {
    if p.index < p.program.len() {
        p.program.as_bytes()[p.index].is_ascii_digit()
    } else {
        false
    }
}

pub fn pos_get_code(p: &POS) -> i32 {
    if p.index < p.program.len() {
        p.program.as_bytes()[p.index] as i32
    } else {
        0
    }
}

pub fn pos_is_letter(p: &POS) -> bool {
    if p.index < p.program.len() {
        let b = p.program.as_bytes()[p.index];
        (b as char).is_alphabetic()
    } else {
        false
    }
}

pub fn pos_is_unknown(p: &POS) -> bool {
    if p.index < p.program.len() {
        let b = p.program.as_bytes()[p.index];
        let c = b as char;
        !c.is_ascii_alphanumeric()
    } else {
        false
    }
}

pub fn create_token() -> Option<Box<TOKEN>> {
    Some(Box::new(TOKEN::default()))
}

pub fn token_read_keyword(
    tok: &mut Option<Box<TOKEN>>,
    pos: &POS,
    tok_type: TOKEN_TYPE,
    keyword: &str,
    keyword_len: usize,
) {
    let mut new_tok = match create_token() {
        Some(t) => t,
        None => return,
    };
    let mut p = pos.clone();
    let mut i: usize = 0;

    let _ = keyword;

    new_tok.token_type = tok_type;
    new_tok.group_type = GROUP_TYPE::GROUP_TYPE_KEYWORDS;

    i = 0;
    while i != keyword_len {
        p = pos_next(&p);
        i += 1;
    }

    new_tok.frag.starting = pos.clone();
    new_tok.frag.following = p;

    *tok = Some(new_tok);
}

pub fn token_read_number(tok: &mut Option<Box<TOKEN>>, pos: &POS) -> i32 {
    let mut r = LEXER_CODES::LEXER_OK;

    let mut new_tok = match create_token() {
        Some(t) => t,
        None => return LEXER_CODES::LEXER_INVALID_TOKEN as i32,
    };
    let mut p = pos.clone();

    new_tok.token_type = TOKEN_TYPE::TOKEN_TYPE_NUMBER;
    new_tok.group_type = GROUP_TYPE::GROUP_TYPE_NUMBERS;

    while pos_is_digit(&p) {
        p = pos_next(&p);
    }
    if pos_get_code(&p) == ('.' as i32) {
        r = LEXER_CODES::LEXER_INVALID_TOKEN;
        p = pos_next(&p);
        while pos_is_digit(&p) {
            r = LEXER_CODES::LEXER_OK;
            p = pos_next(&p);
        }
    }

    new_tok.frag.starting = pos.clone();
    new_tok.frag.following = p.clone();

    let start = new_tok.frag.starting.index;
    let end = new_tok.frag.following.index;
    let tmp_str = if start <= end && end <= new_tok.frag.starting.program.len() {
        &new_tok.frag.starting.program[start..end]
    } else {
        ""
    };

    new_tok.int_val = tmp_str.parse::<i64>().unwrap_or(0);

    *tok = Some(new_tok);

    r as i32
}

pub fn token_read_ident(tok: &mut Option<Box<TOKEN>>, pos: &POS) {
    let mut new_tok = match create_token() {
        Some(t) => t,
        None => return,
    };
    let mut p = pos.clone();

    new_tok.token_type = TOKEN_TYPE::TOKEN_TYPE_IDENT;
    new_tok.group_type = GROUP_TYPE::GROUP_TYPE_IDENTS;

    while pos_is_digit(&p) || pos_is_letter(&p) {
        p = pos_next(&p);
    }

    new_tok.frag.starting = pos.clone();
    new_tok.frag.following = p.clone();

    let start = new_tok.frag.starting.index;
    let end = new_tok.frag.following.index;
    let s = if start <= end && end <= new_tok.frag.starting.program.len() {
        &new_tok.frag.starting.program[start..end]
    } else {
        ""
    };

    new_tok.str_val.clear();
    new_tok.str_val.push_str(s);

    *tok = Some(new_tok);
}

pub fn token_read_op(
    tok: &mut Option<Box<TOKEN>>,
    tok_type: TOKEN_TYPE,
    starting: &POS,
    following: &POS,
) {
    let mut new_tok = match create_token() {
        Some(t) => t,
        None => {
            *tok = None;
            return;
        }
    };

    new_tok.token_type = tok_type;
    new_tok.group_type = GROUP_TYPE::GROUP_TYPE_OPS;
    new_tok.frag.starting = starting.clone();
    new_tok.frag.following = following.clone();

    *tok = Some(new_tok);
}

pub fn token_read_unknown(tok: &mut Option<Box<TOKEN>>, pos: &POS) {
    let mut new_tok = match create_token() {
        Some(t) => t,
        None => return,
    };
    let mut p = pos.clone();

    while pos_is_unknown(&p) {
        p = pos_next(&p);
    }

    new_tok.token_type = TOKEN_TYPE::TOKEN_TYPE_UNKNOWN;
    new_tok.group_type = GROUP_TYPE::GROUP_TYPE_AUX;
    new_tok.frag.starting = pos.clone();
    new_tok.frag.following = p.clone();

    *tok = Some(new_tok);
}

pub fn token_to_xml_string(tok: &TOKEN, str: &mut [u8], len: usize) {
    let tag = match tok.group_type {
        GROUP_TYPE::GROUP_TYPE_KEYWORDS => "keyword",
        GROUP_TYPE::GROUP_TYPE_IDENTS => "ident",
        GROUP_TYPE::GROUP_TYPE_NUMBERS => "number",
        GROUP_TYPE::GROUP_TYPE_OPS => "operator",
        GROUP_TYPE::GROUP_TYPE_AUX => "aux",
        _ => "token",
    };

    let start = tok.frag.starting.index;
    let end = tok.frag.following.index;
    let content = if start <= end && end <= tok.frag.starting.program.len() {
        &tok.frag.starting.program[start..end]
    } else {
        ""
    };

    let xml = format!("<{0}>{1}</{0}>", tag, content);

    let bytes = xml.as_bytes();
    let copy_len = min(len, bytes.len());
    if copy_len > 0 && !str.is_empty() {
        let to_copy = min(copy_len, str.len());
        str[..to_copy].copy_from_slice(&bytes[..to_copy]);
        if to_copy < str.len() {
            // Ensure null-termination style emulation if space available
            str[to_copy] = 0;
        }
    } else if !str.is_empty() {
        // write empty string terminator
        str[0] = 0;
    }
}

pub fn token_free(tok: Option<Box<TOKEN>>) {
    drop(tok);
}