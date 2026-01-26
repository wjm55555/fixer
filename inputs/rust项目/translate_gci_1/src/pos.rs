use crate::lexer::*;
#[repr(C)]
#[derive(Clone)]
pub struct POS<'a> {
    pub program: &'a [u8],
    pub program_len: usize,
    pub line: usize,
    pub pos: usize,
    pub index: usize,
}

impl<'a> Default for POS<'a> {
    fn default() -> Self {
        POS {
            program: &[],
            program_len: 0,
            line: 0,
            pos: 0,
            index: 0,
        }
    }
}

pub fn pos_is_eof<'a>(pos: &POS<'a>) -> i32 {
    if pos.program_len == 0 || pos.index == pos.program_len.saturating_sub(1) {
        1
    } else {
        0
    }
}

pub fn pos_get_code<'a>(pos: &POS<'a>) -> i32 {
    match pos.program.get(pos.index) {
        Some(&b) => b as i32,
        None => 0,
    }
}

pub fn pos_is_whitespace<'a>(pos: &POS<'a>) -> i32 {
    let code = pos_get_code(pos);
    if code == b' ' as i32 || code == b'\t' as i32 {
        1
    } else {
        0
    }
}

pub fn pos_is_newline<'a>(pos: &POS<'a>) -> i32 {
    if let Some(&b'\r') = pos.program.get(pos.index) {
        if pos.index + 1 < pos.program_len {
            if let Some(&b'\n') = pos.program.get(pos.index + 1) {
                return 1;
            }
        }
    }

    if let Some(&b'\n') = pos.program.get(pos.index) {
        1
    } else {
        0
    }
}

pub fn pos_is_digit<'a>(pos: &POS<'a>) -> i32 {
    let code = pos_get_code(pos);
    if code >= b'0' as i32 && code <= b'9' as i32 {
        1
    } else {
        0
    }
}

pub fn pos_is_letter<'a>(pos: &POS<'a>) -> i32 {
    let code = pos_get_code(pos);
    if (code >= b'a' as i32 && code <= b'z' as i32) || (code >= b'A' as i32 && code <= b'Z' as i32) {
        1
    } else {
        0
    }
}

pub fn pos_is_unknown<'a>(pos: &POS<'a>) -> i32 {
    const SYMBOLS: [u8; 18] = [b'|', b'&', b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/', b'%', b'(', b')', b'{', b'}', b',', b';', b'.']; // ':' omitted intentionally? original had ':', add it
    // Adjust to include ':' as in original symbols list
    // Reconstruct with ':' included
    let symbols: [u8; 19] = [b'|', b'&', b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/', b'%', b'(', b')', b'{', b'}', b',', b';', b'.', b':'];

    if pos_is_whitespace(pos) != 0 || pos_is_newline(pos) != 0 || pos_is_digit(pos) != 0 || pos_is_letter(pos) != 0 {
        return 0;
    }

    let code = pos_get_code(pos) as u8;
    for &s in symbols.iter() {
        if s == code {
            return 0;
        }
    }

    1
}

pub fn pos_next<'a>(pos: &POS<'a>) -> POS<'a> {
    let mut new_line = pos.line;
    let mut new_pos = pos.pos;
    let mut new_index = pos.index;

    if pos_is_eof(pos) == 0 {
        if pos_is_newline(pos) != 0 {
            new_line = new_line.wrapping_add(1);
            new_pos = 1;
        } else {
            new_pos = new_pos.wrapping_add(1);
        }
        new_index = new_index.wrapping_add(1);
    }

    POS {
        program: pos.program,
        program_len: pos.program_len,
        line: new_line,
        pos: new_pos,
        index: new_index,
    }
}

pub fn pos_check_keyword<'a>(pos: &POS<'a>, keyword: &str, keyword_len: usize) -> i32 {
    let mut i: usize = 0;
    let mut p = pos.clone();
    let bytes = keyword.as_bytes();

    if keyword_len > bytes.len() {
        return 0;
    }

    while i != keyword_len {
        let kb = bytes[i];
        if kb as i32 != pos_get_code(&p) {
            return 0;
        }
        p = pos_next(&p);
        i += 1;
    }

    if pos_is_letter(&p) == 0 && pos_is_digit(&p) == 0 {
        1
    } else {
        0
    }
}