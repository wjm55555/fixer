use translate_gci::lexer_priv::*;
use translate_gci::lexer::*;

#[inline]
pub fn pos_is_eof(pos: &POS) -> i32 {
    if (pos.program_len == 0) || (pos.index == (pos.program_len - 1)) {
        1
    } else {
        0
    }
}

#[inline]
pub fn pos_is_whitespace(pos: &POS) -> i32 {
    if (pos_get_code(pos) == b' ' as i32) || (pos_get_code(pos) == b'\t' as i32) {
        1
    } else {
        0
    }
}

#[inline]
pub fn pos_is_newline(pos: &POS) -> i32 {
    if (pos.program[pos.index] == b'\r') && (pos.index + 1 < pos.program_len) {
        if pos.program[pos.index + 1] == b'\n' {
            1
        } else {
            0
        }
    } else if pos.program[pos.index] == b'\n' {
        1
    } else {
        0
    }
}

#[inline]
pub fn pos_is_digit(pos: &POS) -> i32 {
    if (pos_get_code(pos) >= b'0' as i32) && (pos_get_code(pos) <= b'9' as i32) {
        1
    } else {
        0
    }
}

#[inline]
pub fn pos_is_letter(pos: &POS) -> i32 {
    if ((pos_get_code(pos) >= b'a' as i32) && (pos_get_code(pos) <= b'z' as i32))
        || ((pos_get_code(pos) >= b'A' as i32) && (pos_get_code(pos) <= b'Z' as i32))
    {
        1
    } else {
        0
    }
}

pub fn pos_is_unknown(pos: &POS) -> i32 {
    let symbols: [u8; 19] = [
        b'|', b'&', b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/', b'%', b'(', b')', b'{',
        b'}', b',', b';', b'.', b':',
    ];

    if (pos_is_whitespace(pos) != 0)
        || (pos_is_newline(pos) != 0)
        || (pos_is_digit(pos) != 0)
        || (pos_is_letter(pos) != 0)
    {
        return 0;
    }

    for i in 0..symbols.len() {
        if symbols[i] as i32 == pos_get_code(pos) {
            return 0;
        }
    }
    1
}

#[inline]
pub fn pos_get_code(pos: &POS) -> i32 {
    if pos.index < pos.program_len {
        pos.program[pos.index] as i32
    } else {
        0
    }
}

pub fn pos_next(pos: &POS) -> POS {
    let mut new_line = pos.line;
    let mut new_pos = pos.pos;
    let mut new_index = pos.index;

    if pos_is_eof(pos) == 0 {
        if pos_is_newline(pos) != 0 {
            new_line += 1;
            new_pos = 1;
        } else {
            new_pos += 1;
        }
        new_index += 1;
    }

    POS {
        program: pos.program.clone(),
        program_len: pos.program_len,
        line: new_line,
        pos: new_pos,
        index: new_index,
    }
}

pub fn pos_check_keyword<'a>(pos: &'a POS, keyword: &'a [u8], keyword_len: usize) -> i32 {
    let mut i = 0;
    let mut p = pos.clone();

    while i != keyword_len {
        if i >= pos.program_len || keyword[i] as i32 != pos_get_code(&p) {
            return 0;
        }
        p = pos_next(&p);
        i += 1;
    }

    if (pos_is_letter(&p) == 0) && (pos_is_digit(&p) == 0) {
        1
    } else {
        0
    }
}