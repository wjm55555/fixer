use translate_gci::lexer_priv::*;
use translate_gci::lexer::*;
use translate_gci::pos::*;

pub fn create_token() -> Box<TOKEN> {
    Box::new(TOKEN::default())
}

pub fn token_read_keyword(
    tok: &mut Box<TOKEN>,
    pos: &POS,
    tok_type: TOKEN_TYPE,
    keyword: &str,
    keyword_len: usize,
) {
    let mut p = *pos;
    let mut i = 0;
    
    let mut new_tok = create_token();
    new_tok.token_type = tok_type;
    new_tok.group_type = GROUP_TYPE_KEYWORDS;
    
    while i != keyword_len {
        p = pos_next(&p);
        i += 1;
    }
    
    new_tok.frag.starting = *pos;
    new_tok.frag.following = p;
    
    *tok = new_tok;
}

pub fn token_read_number(tok: &mut Box<TOKEN>, pos: &POS) -> i32 {
    let mut r = LEXER_OK;
    let mut p = *pos;
    
    let mut new_tok = create_token();
    new_tok.token_type = TOKEN_TYPE_NUMBER;
    new_tok.group_type = GROUP_TYPE_NUMBERS;
    
    while pos_is_digit(&p) {
        p = pos_next(&p);
    }
    
    if pos_get_code(&p) == '.' as u8 {
        r = LEXER_INVALID_TOKEN;
        p = pos_next(&p);
        while pos_is_digit(&p) {
            r = LEXER_OK;
            p = pos_next(&p);
        }
    }
    
    new_tok.frag.starting = *pos;
    new_tok.frag.following = p;
    
    let start_idx = new_tok.frag.starting.index;
    let end_idx = new_tok.frag.following.index;
    let program = new_tok.frag.starting.program;
    
    if start_idx <= end_idx && end_idx <= program.len() {
        if let Ok(s) = std::str::from_utf8(&program[start_idx..end_idx]) {
            if let Ok(val) = s.parse::<i64>() {
                new_tok.int_val = val;
            }
        }
    }
    
    *tok = new_tok;
    r
}

pub fn token_read_ident(tok: &mut Box<TOKEN>, pos: &POS) {
    let mut p = *pos;
    
    let mut new_tok = create_token();
    new_tok.token_type = TOKEN_TYPE_IDENT;
    new_tok.group_type = GROUP_TYPE_IDENTS;
    
    while pos_is_digit(&p) || pos_is_letter(&p) {
        p = pos_next(&p);
    }
    
    new_tok.frag.starting = *pos;
    new_tok.frag.following = p;
    
    let start_idx = new_tok.frag.starting.index;
    let end_idx = new_tok.frag.following.index;
    let program = new_tok.frag.starting.program;
    
    if start_idx <= end_idx && end_idx <= program.len() {
        if let Ok(s) = std::str::from_utf8(&program[start_idx..end_idx]) {
            if s.len() < new_tok.str_val.len() {
                new_tok.str_val = s.to_string();
            }
        }
    }
    
    *tok = new_tok;
}

pub fn token_read_op(
    tok: &mut Box<TOKEN>,
    tok_type: TOKEN_TYPE,
    starting: &POS,
    following: &POS,
) {
    let mut new_tok = create_token();
    new_tok.token_type = tok_type;
    new_tok.group_type = GROUP_TYPE_OPS;
    new_tok.frag.starting = *starting;
    new_tok.frag.following = *following;
    
    *tok = new_tok;
}

pub fn token_read_unknown(tok: &mut Box<TOKEN>, pos: &POS) {
    let mut p = *pos;
    
    let mut new_tok = create_token();
    
    while pos_is_unknown(&p) {
        p = pos_next(&p);
    }
    
    new_tok.token_type = TOKEN_TYPE_UNKNOWN;
    new_tok.group_type = GROUP_TYPE_AUX;
    new_tok.frag.starting = *pos;
    new_tok.frag.following = p;
    
    *tok = new_tok;
}

pub fn token_to_xml_string(tok: &TOKEN, str_buf: &mut String) {
    let group_type_name = match tok.group_type {
        GROUP_TYPE_KEYWORDS => "keyword",
        GROUP_TYPE_IDENTS => "ident",
        GROUP_TYPE_NUMBERS => "number",
        GROUP_TYPE_OPS => "operator",
        GROUP_TYPE_AUX => "aux",
        _ => "unknown",
    };
    
    let start_idx = tok.frag.starting.index;
    let end_idx = tok.frag.following.index;
    let program = tok.frag.starting.program;
    
    if start_idx <= end_idx && end_idx <= program.len() {
        if let Ok(content) = std::str::from_utf8(&program[start_idx..end_idx]) {
            str_buf.clear();
            str_buf.push_str(&format!(
                "<{} line=\"{}\" pos=\"{}\">{}</{}>",
                group_type_name,
                tok.frag.starting.line,
                tok.frag.starting.pos,
                content,
                group_type_name
            ));
        }
    }
}

pub fn TOKEN_GEN_STR(tok: &TOKEN, group_type: &str, str_buf: &mut String) {
    let start_idx = tok.frag.starting.index;
    let end_idx = tok.frag.following.index;
    let program = tok.frag.starting.program;
    let _len = end_idx - start_idx;
    
    if start_idx <= end_idx && end_idx <= program.len() {
        if let Ok(content) = std::str::from_utf8(&program[start_idx..end_idx]) {
            str_buf.clear();
            str_buf.push_str(&format!(
                "<{} line=\"{}\" pos=\"{}\">{}</{}>",
                group_type,
                tok.frag.starting.line,
                tok.frag.starting.pos,
                content,
                group_type
            ));
        }
    }
}

pub fn token_free(_tok: Box<TOKEN>) {
}