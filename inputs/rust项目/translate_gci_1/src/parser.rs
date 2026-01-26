use crate::utils::*;
use crate::lexer::*;
use crate::ast::*;
pub type lexer_type_t = usize;

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
#[derive(Default, Clone)]
pub struct FRAG_STARTING {
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct FRAG {
    pub starting: FRAG_STARTING,
}

#[repr(C)]
#[derive(Clone)]
pub struct TOKEN {
    pub token_type: TOKEN_TYPE,
    pub str_val: String,
    pub int_val: i64,
    pub frag: FRAG,
}

impl Default for TOKEN {
    fn default() -> Self {
        TOKEN {
            token_type: TOKEN_TYPE::default(),
            str_val: String::new(),
            int_val: 0,
            frag: FRAG::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PARSER_CODES {
    PARSER_OK = 0,
    PARSER_INVALID_TOKEN = -1,
}

impl Default for PARSER_CODES {
    fn default() -> Self {
        PARSER_CODES::PARSER_OK
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PARSER_ERROR {
    pub exp_toks: [TOKEN_TYPE; 16],
    pub get_tok: TOKEN_TYPE,
    pub line: usize,
    pub pos: usize,
}

impl Default for PARSER_ERROR {
    fn default() -> Self {
        PARSER_ERROR {
            exp_toks: [TOKEN_TYPE::TOKEN_TYPE_EOF; 16],
            get_tok: TOKEN_TYPE::default(),
            line: 0,
            pos: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PARSER {
    pub lexer: lexer_type_t,
    pub tok: Option<Box<TOKEN>>,
    pub err: PARSER_ERROR,
}

impl Default for PARSER {
    fn default() -> Self {
        PARSER {
            lexer: 0,
            tok: None,
            err: PARSER_ERROR::default(),
        }
    }
}

pub type parser_type_t = Box<PARSER>;

pub fn SAFE_CALLOC<T: Default>() -> Box<T> {
    Box::new(T::default())
}

pub fn SAFE_FREE<T>(_v: Option<Box<T>>) {}

pub fn PUSH_BACK<T>(vec: &mut Vec<T>, item: T) {
    vec.push(item);
}

pub fn token_free(_t: Option<Box<TOKEN>>) {}

pub fn lexer_next_token(_lexer: lexer_type_t, _tok: &mut Option<Box<TOKEN>>) {}

pub fn create_ident_ast(_s: &str, _line: usize, _pos: usize) -> Option<Box<IDENT_AST>> {
    None
}
pub fn ident_ast_free(_a: Option<Box<IDENT_AST>>) {}

pub fn create_variable_part_ast(_a: Option<Box<IDENT_AST>>, _t: AST_VARIABLE_PART_TYPE) -> Option<Box<VARIABLE_PART_AST>> {
    None
}
pub fn variable_part_ast_free(_a: Option<Box<VARIABLE_PART_AST>>) {}

pub fn logical_or_expr_ast_read(_p: &mut PARSER, _out: &mut Option<Box<LOGICAL_OR_EXPR_AST>>) -> PARSER_CODES {
    PARSER_CODES::PARSER_OK
}
pub fn logical_or_expr_ast_free(_a: Option<Box<LOGICAL_OR_EXPR_AST>>) {}

pub fn create_variable_ast(_ident: Option<Box<IDENT_AST>>, _parts: Vec<Box<VARIABLE_PART_AST>>, _parts_len: usize, _line: usize, _pos: usize) -> Option<Box<VARIABLE_AST>> {
    None
}
pub fn variable_ast_free(_a: Option<Box<VARIABLE_AST>>) {}

pub fn create_formal_parameters_list_ast(_idents: Vec<Box<IDENT_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<FORMAL_PARAMETERS_LIST_AST>> {
    None
}
pub fn formal_parameters_list_ast_free(_a: Option<Box<FORMAL_PARAMETERS_LIST_AST>>) {}

pub fn create_number_ast(_val: i64, _line: usize, _pos: usize) -> Option<Box<NUMBER_AST>> {
    None
}
pub fn number_ast_free(_a: Option<Box<NUMBER_AST>>) {}

pub fn create_args_list_ast(_assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<ARGS_LIST_AST>> {
    None
}
pub fn args_list_ast_free(_a: Option<Box<ARGS_LIST_AST>>) {}

pub fn create_function_call_ast(_name: Option<Box<IDENT_AST>>, _args: Option<Box<ARGS_LIST_AST>>, _line: usize, _pos: usize) -> Option<Box<FUNCTION_CALL_AST>> {
    None
}
pub fn function_call_ast_free(_a: Option<Box<FUNCTION_CALL_AST>>) {}

pub fn create_has_property_expr_ast(_obj: Option<Box<VARIABLE_AST>>, _ident: Option<Box<IDENT_AST>>, _line: usize, _pos: usize) -> Option<Box<HAS_PROPERTY_EXPR_AST>> {
    None
}
pub fn has_property_expr_ast_free(_a: Option<Box<HAS_PROPERTY_EXPR_AST>>) {}

pub fn create_len_expr_ast(_arr: Option<Box<VARIABLE_AST>>, _line: usize, _pos: usize) -> Option<Box<LEN_EXPR_AST>> {
    None
}
pub fn len_expr_ast_free(_a: Option<Box<LEN_EXPR_AST>>) {}

pub fn create_primary_expr_ast(_v: Option<Box<dyn std::any::Any>>, _t: AST_PRIMARY_EXPR_TYPE) -> Option<Box<PRIMARY_EXPR_AST>> {
    None
}
pub fn primary_expr_ast_free(_a: Option<Box<PRIMARY_EXPR_AST>>) {}

pub fn create_left_unary_expr_ast(_op: AST_LEFT_UNARY_OP, _p: Option<Box<PRIMARY_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<LEFT_UNARY_EXPR_AST>> {
    None
}
pub fn left_unary_expr_ast_free(_a: Option<Box<LEFT_UNARY_EXPR_AST>>) {}

pub fn create_multiplicative_expr_ast(_a: Vec<Box<LEFT_UNARY_EXPR_AST>>, _ops: Vec<AST_MULTIPLICATIVE_OP>, _len: usize, _line: usize, _pos: usize) -> Option<Box<MULTIPLICATIVE_EXPR_AST>> {
    None
}
pub fn multiplicative_expr_ast_free(_a: Option<Box<MULTIPLICATIVE_EXPR_AST>>) {}

pub fn create_additive_expr_ast(_a: Vec<Box<MULTIPLICATIVE_EXPR_AST>>, _ops: Vec<AST_ADDITIVE_OP>, _len: usize, _line: usize, _pos: usize) -> Option<Box<ADDITIVE_EXPR_AST>> {
    None
}
pub fn additive_expr_ast_free(_a: Option<Box<ADDITIVE_EXPR_AST>>) {}

pub fn create_relational_expr_ast(_left: Option<Box<ADDITIVE_EXPR_AST>>, _op: AST_REL_OP, _right: Option<Box<ADDITIVE_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<RELATIONAL_EXPR_AST>> {
    None
}
pub fn relational_expr_ast_free(_a: Option<Box<RELATIONAL_EXPR_AST>>) {}

pub fn create_eq_expr_ast(_left: Option<Box<RELATIONAL_EXPR_AST>>, _op: AST_EQ_OP, _right: Option<Box<RELATIONAL_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<EQ_EXPR_AST>> {
    None
}
pub fn eq_expr_ast_free(_a: Option<Box<EQ_EXPR_AST>>) {}

pub fn create_logical_and_expr_ast(_a: Vec<Box<EQ_EXPR_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<LOGICAL_AND_EXPR_AST>> {
    None
}
pub fn logical_and_expr_ast_free(_a: Option<Box<LOGICAL_AND_EXPR_AST>>) {}

pub fn create_logical_or_expr_ast(_a: Vec<Box<LOGICAL_AND_EXPR_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<LOGICAL_OR_EXPR_AST>> {
    None
}
pub fn logical_or_expr_ast_free(_a: Option<Box<LOGICAL_OR_EXPR_AST>>) {}

pub fn create_property_ast(_ident: Option<Box<IDENT_AST>>, _assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<PROPERTY_AST>> {
    None
}
pub fn property_ast_free(_a: Option<Box<PROPERTY_AST>>) {}

pub fn create_object_literal_ast(_properties: Vec<Box<PROPERTY_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<OBJECT_LITERAL_AST>> {
    None
}
pub fn object_literal_ast_free(_a: Option<Box<OBJECT_LITERAL_AST>>) {}

pub fn create_array_literal_ast(_args: Option<Box<ARGS_LIST_AST>>, _line: usize, _pos: usize) -> Option<Box<ARRAY_LITERAL_AST>> {
    None
}
pub fn array_literal_ast_free(_a: Option<Box<ARRAY_LITERAL_AST>>) {}

pub fn create_assignment_expr_ast(_v: Option<Box<dyn std::any::Any>>, _t: AST_ASSIGNMENT_EXPR_TYPE) -> Option<Box<ASSIGNMENT_EXPR_AST>> {
    None
}
pub fn assignment_expr_ast_free(_a: Option<Box<ASSIGNMENT_EXPR_AST>>) {}

pub fn create_decl_stmt_ast(_ident: Option<Box<IDENT_AST>>, _assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<DECL_STMT_AST>> {
    None
}
pub fn decl_stmt_ast_free(_a: Option<Box<DECL_STMT_AST>>) {}

pub fn create_if_stmt_ast(_cond: Option<Box<LOGICAL_OR_EXPR_AST>>, _if_body: Option<Box<BODY_AST>>, _else_body: Option<Box<BODY_AST>>, _line: usize, _pos: usize) -> Option<Box<IF_STMT_AST>> {
    None
}
pub fn if_stmt_ast_free(_a: Option<Box<IF_STMT_AST>>) {}

pub fn create_while_stmt_ast(_cond: Option<Box<LOGICAL_OR_EXPR_AST>>, _body: Option<Box<BODY_AST>>, _line: usize, _pos: usize) -> Option<Box<WHILE_STMT_AST>> {
    None
}
pub fn while_stmt_ast_free(_a: Option<Box<WHILE_STMT_AST>>) {}

pub fn create_break_stmt_ast(_line: usize, _pos: usize) -> Option<Box<BREAK_STMT_AST>> {
    None
}
pub fn break_stmt_ast_free(_a: Option<Box<BREAK_STMT_AST>>) {}

pub fn create_continue_stmt_ast(_line: usize, _pos: usize) -> Option<Box<CONTINUE_STMT_AST>> {
    None
}
pub fn continue_stmt_ast_free(_a: Option<Box<CONTINUE_STMT_AST>>) {}

pub fn create_append_stmt_ast(_obj: Option<Box<VARIABLE_AST>>, _ident: Option<Box<IDENT_AST>>, _line: usize, _pos: usize) -> Option<Box<APPEND_STMT_AST>> {
    None
}
pub fn append_stmt_ast_free(_a: Option<Box<APPEND_STMT_AST>>) {}

pub fn create_delete_stmt_ast(_obj: Option<Box<VARIABLE_AST>>, _ident: Option<Box<IDENT_AST>>, _line: usize, _pos: usize) -> Option<Box<DELETE_STMT_AST>> {
    None
}
pub fn delete_stmt_ast_free(_a: Option<Box<DELETE_STMT_AST>>) {}

pub fn create_return_stmt_ast(_assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>>, _line: usize, _pos: usize) -> Option<Box<RETURN_STMT_AST>> {
    None
}
pub fn return_stmt_ast_free(_a: Option<Box<RETURN_STMT_AST>>) {}

pub fn create_stmt_ast(_v: Option<Box<dyn std::any::Any>>, _t: AST_STMT_TYPE) -> Option<Box<STMT_AST>> {
    None
}
pub fn stmt_ast_free(_a: Option<Box<STMT_AST>>) {}

pub fn create_body_ast(_stmts: Vec<Box<STMT_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<BODY_AST>> {
    None
}
pub fn body_ast_free(_a: Option<Box<BODY_AST>>) {}

pub fn create_function_decl_ast(_name: Option<Box<IDENT_AST>>, _formal: Option<Box<FORMAL_PARAMETERS_LIST_AST>>, _body: Option<Box<BODY_AST>>, _line: usize, _pos: usize) -> Option<Box<FUNCTION_DECL_AST>> {
    None
}
pub fn function_decl_ast_free(_a: Option<Box<FUNCTION_DECL_AST>>) {}

pub fn create_unit_ast(_functions: Vec<Box<FUNCTION_DECL_AST>>, _len: usize, _line: usize, _pos: usize) -> Option<Box<UNIT_AST>> {
    None
}
pub fn unit_ast_free(_a: Option<Box<UNIT_AST>>) {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_VARIABLE_PART_TYPE {
    AST_VARIABLE_PART_TYPE_FIELD,
    AST_VARIABLE_PART_TYPE_INDEX,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_PRIMARY_EXPR_TYPE {
    AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL,
    AST_PRIMARY_EXPR_TYPE_VARIABLE,
    AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY,
    AST_PRIMARY_EXPR_TYPE_LEN,
    AST_PRIMARY_EXPR_TYPE_NUMBER,
    AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_LEFT_UNARY_OP {
    AST_LEFT_UNARY_OP_PLUS,
    AST_LEFT_UNARY_OP_MINUS,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_MULTIPLICATIVE_OP {
    AST_MULTIPLICATIVE_OP_MUL,
    AST_MULTIPLICATIVE_OP_DIV,
    AST_MULTIPLICATIVE_OP_MOD,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_ADDITIVE_OP {
    AST_ADDITIVE_OP_PLUS,
    AST_ADDITIVE_OP_MINUS,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_REL_OP {
    AST_REL_OP_LT,
    AST_REL_OP_GT,
    AST_REL_OP_LE,
    AST_REL_OP_GE,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_EQ_OP {
    AST_EQ_OP_EQEQ,
    AST_EQ_OP_NEQ,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_ASSIGNMENT_EXPR_TYPE {
    AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL,
    AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL,
    AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AST_STMT_TYPE {
    AST_STMT_TYPE_DECL,
    AST_STMT_TYPE_FUNCTION_CALL,
    AST_STMT_TYPE_IF,
    AST_STMT_TYPE_WHILE,
    AST_STMT_TYPE_BREAK,
    AST_STMT_TYPE_CONTINUE,
    AST_STMT_TYPE_APPEND,
    AST_STMT_TYPE_DELETE,
    AST_STMT_TYPE_RETURN,
    AST_STMT_TYPE_ASSIGN,
}

pub enum AST_VARIABLE_PART {}
pub enum IDENT_AST {}
pub enum VARIABLE_PART_AST {}
pub enum VARIABLE_AST {}
pub enum FORMAL_PARAMETERS_LIST_AST {}
pub enum NUMBER_AST {}
pub enum ARGS_LIST_AST {}
pub enum FUNCTION_CALL_AST {}
pub enum HAS_PROPERTY_EXPR_AST {}
pub enum LEN_EXPR_AST {}
pub enum PRIMARY_EXPR_AST {}
pub enum LEFT_UNARY_EXPR_AST {}
pub enum MULTIPLICATIVE_EXPR_AST {}
pub enum ADDITIVE_EXPR_AST {}
pub enum RELATIONAL_EXPR_AST {}
pub enum EQ_EXPR_AST {}
pub enum LOGICAL_AND_EXPR_AST {}
pub enum LOGICAL_OR_EXPR_AST {}
pub enum PROPERTY_AST {}
pub enum OBJECT_LITERAL_AST {}
pub enum ARRAY_LITERAL_AST {}
pub enum ASSIGNMENT_EXPR_AST {}
pub enum DECL_STMT_AST {}
pub enum IF_STMT_AST {}
pub enum WHILE_STMT_AST {}
pub enum BREAK_STMT_AST {}
pub enum CONTINUE_STMT_AST {}
pub enum APPEND_STMT_AST {}
pub enum DELETE_STMT_AST {}
pub enum RETURN_STMT_AST {}
pub enum STMT_AST {}
pub enum BODY_AST {}
pub enum FUNCTION_DECL_AST {}
pub enum UNIT_AST {}

pub fn create_parser() -> parser_type_t {
    SAFE_CALLOC::<PARSER>()
}

pub fn parser_conf(parser: &mut parser_type_t, lexer: lexer_type_t) {
    parser.lexer = lexer;
}

pub fn set_parser_error(parser: &mut PARSER, toks: &[TOKEN_TYPE]) {
    let mut i = 0usize;
    while i < toks.len() && i < 16 {
        parser.err.exp_toks[i] = toks[i];
        i += 1;
    }
    if i < 16 {
        parser.err.exp_toks[i] = TOKEN_TYPE::TOKEN_TYPE_EOF;
    } else {
        parser.err.exp_toks[15] = TOKEN_TYPE::TOKEN_TYPE_EOF;
    }
    if let Some(ref t) = parser.tok {
        parser.err.get_tok = t.token_type;
        parser.err.line = t.frag.starting.line;
        parser.err.pos = t.frag.starting.pos;
    } else {
        parser.err.get_tok = TOKEN_TYPE::TOKEN_TYPE_EOF;
        parser.err.line = 0;
        parser.err.pos = 0;
    }
}

pub fn ident_ast_read(parser: &mut PARSER, ident: &mut Option<Box<IDENT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_IDENT) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_IDENT]);
        *ident = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }

    if let Some(t) = &parser.tok {
        let new_ident = create_ident_ast(&t.str_val, t.frag.starting.line, t.frag.starting.pos);
        *ident = new_ident;
    } else {
        *ident = None;
    }

    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    PARSER_CODES::PARSER_OK
}

pub fn variable_part_ast_read(parser: &mut PARSER, variable_part: &mut Option<Box<VARIABLE_PART_AST>>) -> PARSER_CODES {
    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_DOT) => {
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            let mut field: Option<Box<IDENT_AST>> = None;
            let r = ident_ast_read(parser, &mut field);
            if r != PARSER_CODES::PARSER_OK {
                *variable_part = None;
                return r;
            }
            *variable_part = create_variable_part_ast(field, AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_FIELD);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_LBRACKET) => {
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            let mut index: Option<Box<LOGICAL_OR_EXPR_AST>> = None;
            let r = logical_or_expr_ast_read(parser, &mut index);
            if r != PARSER_CODES::PARSER_OK {
                *variable_part = None;
                return r;
            }

            if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACKET) {
                logical_or_expr_ast_free(index);
                set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RBRACKET]);
                *variable_part = None;
                return PARSER_CODES::PARSER_INVALID_TOKEN;
            }

            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            *variable_part = create_variable_part_ast(index.map(|_| IDENT_AST {}), AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_INDEX);
            PARSER_CODES::PARSER_OK
        }
        _ => {
            set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_DOT, TOKEN_TYPE::TOKEN_TYPE_LBRACKET]);
            *variable_part = None;
            PARSER_CODES::PARSER_INVALID_TOKEN
        }
    }
}

pub fn variable_ast_read(parser: &mut PARSER, variable: &mut Option<Box<VARIABLE_AST>>, ident: Option<Box<IDENT_AST>>) -> PARSER_CODES {
    let mut parts: Vec<Box<VARIABLE_PART_AST>> = Vec::new();

    while matches!(parser.tok.as_ref().map(|t| t.token_type), Some(TOKEN_TYPE::TOKEN_TYPE_DOT) | Some(TOKEN_TYPE::TOKEN_TYPE_LBRACKET)) {
        let mut part: Option<Box<VARIABLE_PART_AST>> = None;
        let r = variable_part_ast_read(parser, &mut part);
        if r != PARSER_CODES::PARSER_OK {
            for p in parts.into_iter() {
                variable_part_ast_free(Some(p));
            }
            *variable = None;
            return r;
        }
        if let Some(pv) = part {
            PUSH_BACK(&mut parts, pv);
        }
    }

    *variable = create_variable_ast(ident, parts, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));

    PARSER_CODES::PARSER_OK
}

pub fn formal_parameters_list_ast_read(parser: &mut PARSER, formal_parameters_list: &mut Option<Box<FORMAL_PARAMETERS_LIST_AST>>) -> PARSER_CODES {
    let mut idents: Vec<Box<IDENT_AST>> = Vec::new();

    let mut ident_opt: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut ident_opt);
    if r != PARSER_CODES::PARSER_OK {
        *formal_parameters_list = None;
        return r;
    }
    if let Some(i) = ident_opt {
        PUSH_BACK(&mut idents, i);
    }

    while parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut ident2: Option<Box<IDENT_AST>> = None;
        r = ident_ast_read(parser, &mut ident2);
        if r != PARSER_CODES::PARSER_OK {
            for it in idents.into_iter() {
                ident_ast_free(Some(it));
            }
            *formal_parameters_list = None;
            return r;
        }
        if let Some(i2) = ident2 {
            PUSH_BACK(&mut idents, i2);
        }
    }

    *formal_parameters_list = create_formal_parameters_list_ast(idents, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn number_ast_read(parser: &mut PARSER, number: &mut Option<Box<NUMBER_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_NUMBER) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_NUMBER]);
        *number = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }

    if let Some(t) = &parser.tok {
        *number = create_number_ast(t.int_val, t.frag.starting.line, t.frag.starting.pos);
    } else {
        *number = None;
    }

    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    PARSER_CODES::PARSER_OK
}

pub fn args_list_ast_read(parser: &mut PARSER, args_list: &mut Option<Box<ARGS_LIST_AST>>) -> PARSER_CODES {
    let mut assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>> = Vec::new();

    let mut assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>> = None;
    let mut r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r != PARSER_CODES::PARSER_OK {
        *args_list = None;
        return r;
    }
    if let Some(ae) = assignment_expr {
        PUSH_BACK(&mut assignment_exprs, ae);
    }

    while parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut assignment_expr2: Option<Box<ASSIGNMENT_EXPR_AST>> = None;
        r = assignment_expr_ast_read(parser, &mut assignment_expr2);
        if r != PARSER_CODES::PARSER_OK {
            for a in assignment_exprs.into_iter() {
                assignment_expr_ast_free(Some(a));
            }
            *args_list = None;
            return r;
        }
        if let Some(a2) = assignment_expr2 {
            PUSH_BACK(&mut assignment_exprs, a2);
        }
    }

    *args_list = create_args_list_ast(assignment_exprs, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn function_call_ast_read(parser: &mut PARSER, function_call: &mut Option<Box<FUNCTION_CALL_AST>>, function_name: Option<Box<IDENT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *function_call = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut args_list: Option<Box<ARGS_LIST_AST>> = None;
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        let r = args_list_ast_read(parser, &mut args_list);
        if r != PARSER_CODES::PARSER_OK {
            *function_call = None;
            return r;
        }
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        if args_list.is_some() {
            args_list_ast_free(args_list);
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *function_call = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *function_call = create_function_call_ast(function_name, args_list, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn has_property_expr_ast_read(parser: &mut PARSER, has_property_expr: &mut Option<Box<HAS_PROPERTY_EXPR_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY]);
        *has_property_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *has_property_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r != PARSER_CODES::PARSER_OK {
        *has_property_expr = None;
        return r;
    }

    let mut obj: Option<Box<VARIABLE_AST>> = None;
    r = variable_ast_read(parser, &mut obj, ident.take());
    if r != PARSER_CODES::PARSER_OK {
        if ident.is_some() {
            ident_ast_free(ident);
        }
        *has_property_expr = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
        variable_ast_free(obj);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_COMMA]);
        *has_property_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident2: Option<Box<IDENT_AST>> = None;
    r = ident_ast_read(parser, &mut ident2);
    if r != PARSER_CODES::PARSER_OK {
        variable_ast_free(obj);
        *has_property_expr = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        variable_ast_free(obj);
        ident_ast_free(ident2);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *has_property_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *has_property_expr = create_has_property_expr_ast(obj, ident2, line, pos);
    PARSER_CODES::PARSER_OK
}

pub fn len_expr_ast_read(parser: &mut PARSER, len_expr: &mut Option<Box<LEN_EXPR_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LEN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LEN]);
        *len_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *len_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r != PARSER_CODES::PARSER_OK {
        *len_expr = None;
        return r;
    }

    let mut arr: Option<Box<VARIABLE_AST>> = None;
    r = variable_ast_read(parser, &mut arr, ident);
    if r != PARSER_CODES::PARSER_OK {
        ident_ast_free(ident);
        *len_expr = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        ident_ast_free(ident);
        variable_ast_free(arr);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *len_expr = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *len_expr = create_len_expr_ast(arr, line, pos);
    PARSER_CODES::PARSER_OK
}

pub fn primary_expr_ast_read(parser: &mut PARSER, primary_expr: &mut Option<Box<PRIMARY_EXPR_AST>>) -> PARSER_CODES {
    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_IDENT) => {
            let ident = parser.tok.as_ref().map(|t| create_ident_ast(&t.str_val, t.frag.starting.line, t.frag.starting.pos)).flatten();
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            match parser.tok.as_ref().map(|t| t.token_type) {
                Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) => {
                    let mut function_call: Option<Box<FUNCTION_CALL_AST>> = None;
                    let r = function_call_ast_read(parser, &mut function_call, ident);
                    if r != PARSER_CODES::PARSER_OK {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        *primary_expr = None;
                        return r;
                    }
                    *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL);
                }
                _ => {
                    let mut var_name: Option<Box<VARIABLE_AST>> = None;
                    let r = variable_ast_read(parser, &mut var_name, ident);
                    if r != PARSER_CODES::PARSER_OK {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        *primary_expr = None;
                        return r;
                    }
                    *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_VARIABLE);
                }
            }
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY) => {
            let mut has_property_expr: Option<Box<HAS_PROPERTY_EXPR_AST>> = None;
            let r = has_property_expr_ast_read(parser, &mut has_property_expr);
            if r != PARSER_CODES::PARSER_OK {
                *primary_expr = None;
                return r;
            }
            *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_LEN) => {
            let mut len_expr: Option<Box<LEN_EXPR_AST>> = None;
            let r = len_expr_ast_read(parser, &mut len_expr);
            if r != PARSER_CODES::PARSER_OK {
                *primary_expr = None;
                return r;
            }
            *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LEN);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_NUMBER) => {
            let mut number: Option<Box<NUMBER_AST>> = None;
            let r = number_ast_read(parser, &mut number);
            if r != PARSER_CODES::PARSER_OK {
                *primary_expr = None;
                return r;
            }
            *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_NUMBER);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) => {
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            let mut logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>> = None;
            let r = logical_or_expr_ast_read(parser, &mut logical_or_expr);
            if r != PARSER_CODES::PARSER_OK {
                *primary_expr = None;
                return r;
            }

            if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
                logical_or_expr_ast_free(logical_or_expr);
                set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
                *primary_expr = None;
                return PARSER_CODES::PARSER_INVALID_TOKEN;
            }
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            *primary_expr = create_primary_expr_ast(None, AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR);
            PARSER_CODES::PARSER_OK
        }
        _ => {
            set_parser_error(parser, &[
                TOKEN_TYPE::TOKEN_TYPE_IDENT,
                TOKEN_TYPE::TOKEN_TYPE_NUMBER,
                TOKEN_TYPE::TOKEN_TYPE_LPAREN,
                TOKEN_TYPE::TOKEN_TYPE_LEN,
                TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY,
            ]);
            *primary_expr = None;
            PARSER_CODES::PARSER_INVALID_TOKEN
        }
    }
}

pub fn left_unary_expr_ast_read(parser: &mut PARSER, left_unary_expr: &mut Option<Box<LEFT_UNARY_EXPR_AST>>) -> PARSER_CODES {
    let mut op = AST_LEFT_UNARY_OP::AST_LEFT_UNARY_OP_PLUS;

    if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_PLUS) {
        op = AST_LEFT_UNARY_OP::AST_LEFT_UNARY_OP_PLUS;
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);
    } else if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_MINUS) {
        op = AST_LEFT_UNARY_OP::AST_LEFT_UNARY_OP_MINUS;
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);
    }

    let mut primary_expr: Option<Box<PRIMARY_EXPR_AST>> = None;
    let r = primary_expr_ast_read(parser, &mut primary_expr);
    if r != PARSER_CODES::PARSER_OK {
        *left_unary_expr = None;
        return r;
    }

    *left_unary_expr = create_left_unary_expr_ast(op, primary_expr, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn multiplicative_expr_ast_read(parser: &mut PARSER, multiplicative_expr: &mut Option<Box<MULTIPLICATIVE_EXPR_AST>>) -> PARSER_CODES {
    let mut left_unary_exprs: Vec<Box<LEFT_UNARY_EXPR_AST>> = Vec::new();
    let mut ops: Vec<AST_MULTIPLICATIVE_OP> = Vec::new();

    let mut left_unary_expr: Option<Box<LEFT_UNARY_EXPR_AST>> = None;
    let mut r = left_unary_expr_ast_read(parser, &mut left_unary_expr);
    if r != PARSER_CODES::PARSER_OK {
        *multiplicative_expr = None;
        return r;
    }
    if let Some(lu) = left_unary_expr {
        PUSH_BACK(&mut left_unary_exprs, lu);
    }

    while matches!(parser.tok.as_ref().map(|t| t.token_type), Some(TOKEN_TYPE::TOKEN_TYPE_MUL) | Some(TOKEN_TYPE::TOKEN_TYPE_DIV) | Some(TOKEN_TYPE::TOKEN_TYPE_MOD)) {
        if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_MUL) {
            PUSH_BACK(&mut ops, AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_MUL);
        } else if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_DIV) {
            PUSH_BACK(&mut ops, AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_DIV);
        } else if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_MOD) {
            PUSH_BACK(&mut ops, AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_MOD);
        }
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut left_unary_expr2: Option<Box<LEFT_UNARY_EXPR_AST>> = None;
        r = left_unary_expr_ast_read(parser, &mut left_unary_expr2);
        if r != PARSER_CODES::PARSER_OK {
            for le in left_unary_exprs.into_iter() {
                left_unary_expr_ast_free(Some(le));
            }
            *multiplicative_expr = None;
            return r;
        }
        if let Some(lu2) = left_unary_expr2 {
            PUSH_BACK(&mut left_unary_exprs, lu2);
        }
    }

    *multiplicative_expr = create_multiplicative_expr_ast(left_unary_exprs, ops, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn additive_expr_ast_read(parser: &mut PARSER, additive_expr: &mut Option<Box<ADDITIVE_EXPR_AST>>) -> PARSER_CODES {
    let mut multiplicative_exprs: Vec<Box<MULTIPLICATIVE_EXPR_AST>> = Vec::new();
    let mut ops: Vec<AST_ADDITIVE_OP> = Vec::new();

    let mut multiplicative_expr: Option<Box<MULTIPLICATIVE_EXPR_AST>> = None;
    let mut r = multiplicative_expr_ast_read(parser, &mut multiplicative_expr);
    if r != PARSER_CODES::PARSER_OK {
        *additive_expr = None;
        return r;
    }
    if let Some(me) = multiplicative_expr {
        PUSH_BACK(&mut multiplicative_exprs, me);
    }

    while matches!(parser.tok.as_ref().map(|t| t.token_type), Some(TOKEN_TYPE::TOKEN_TYPE_PLUS) | Some(TOKEN_TYPE::TOKEN_TYPE_MINUS)) {
        if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_PLUS) {
            PUSH_BACK(&mut ops, AST_ADDITIVE_OP::AST_ADDITIVE_OP_PLUS);
        } else {
            PUSH_BACK(&mut ops, AST_ADDITIVE_OP::AST_ADDITIVE_OP_MINUS);
        }
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut multiplicative_expr2: Option<Box<MULTIPLICATIVE_EXPR_AST>> = None;
        r = multiplicative_expr_ast_read(parser, &mut multiplicative_expr2);
        if r != PARSER_CODES::PARSER_OK {
            for me in multiplicative_exprs.into_iter() {
                multiplicative_expr_ast_free(Some(me));
            }
            *additive_expr = None;
            return r;
        }
        if let Some(me2) = multiplicative_expr2 {
            PUSH_BACK(&mut multiplicative_exprs, me2);
        }
    }

    *additive_expr = create_additive_expr_ast(multiplicative_exprs, ops, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn relational_expr_ast_read(parser: &mut PARSER, relational_expr: &mut Option<Box<RELATIONAL_EXPR_AST>>) -> PARSER_CODES {
    let mut left: Option<Box<ADDITIVE_EXPR_AST>> = None;
    let mut rel_op = AST_REL_OP::AST_REL_OP_LT;
    let mut right: Option<Box<ADDITIVE_EXPR_AST>> = None;

    let mut r = additive_expr_ast_read(parser, &mut left);
    if r != PARSER_CODES::PARSER_OK {
        *relational_expr = None;
        return r;
    }

    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_LT) => rel_op = AST_REL_OP::AST_REL_OP_LT,
        Some(TOKEN_TYPE::TOKEN_TYPE_GT) => rel_op = AST_REL_OP::AST_REL_OP_GT,
        Some(TOKEN_TYPE::TOKEN_TYPE_LE) => rel_op = AST_REL_OP::AST_REL_OP_LE,
        Some(TOKEN_TYPE::TOKEN_TYPE_GE) => rel_op = AST_REL_OP::AST_REL_OP_GE,
        _ => {}
    }

    if matches!(parser.tok.as_ref().map(|t| t.token_type), Some(TOKEN_TYPE::TOKEN_TYPE_LT) | Some(TOKEN_TYPE::TOKEN_TYPE_GT) | Some(TOKEN_TYPE::TOKEN_TYPE_LE) | Some(TOKEN_TYPE::TOKEN_TYPE_GE)) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        r = additive_expr_ast_read(parser, &mut right);
        if r != PARSER_CODES::PARSER_OK {
            additive_expr_ast_free(left);
            *relational_expr = None;
            return r;
        }
    }

    *relational_expr = create_relational_expr_ast(left, rel_op, right, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn eq_expr_ast_read(parser: &mut PARSER, eq_expr: &mut Option<Box<EQ_EXPR_AST>>) -> PARSER_CODES {
    let mut left: Option<Box<RELATIONAL_EXPR_AST>> = None;
    let mut eq_op = AST_EQ_OP::AST_EQ_OP_EQEQ;
    let mut right: Option<Box<RELATIONAL_EXPR_AST>> = None;

    let mut r = relational_expr_ast_read(parser, &mut left);
    if r != PARSER_CODES::PARSER_OK {
        *eq_expr = None;
        return r;
    }

    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_EQEQ) => eq_op = AST_EQ_OP::AST_EQ_OP_EQEQ,
        Some(TOKEN_TYPE::TOKEN_TYPE_NEQ) => eq_op = AST_EQ_OP::AST_EQ_OP_NEQ,
        _ => {}
    }

    if matches!(parser.tok.as_ref().map(|t| t.token_type), Some(TOKEN_TYPE::TOKEN_TYPE_EQEQ) | Some(TOKEN_TYPE::TOKEN_TYPE_NEQ)) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        r = relational_expr_ast_read(parser, &mut right);
        if r != PARSER_CODES::PARSER_OK {
            relational_expr_ast_free(left);
            *eq_expr = None;
            return r;
        }
    }

    *eq_expr = create_eq_expr_ast(left, eq_op, right, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn logical_and_expr_ast_read(parser: &mut PARSER, logical_and_expr: &mut Option<Box<LOGICAL_AND_EXPR_AST>>) -> PARSER_CODES {
    let mut eq_exprs: Vec<Box<EQ_EXPR_AST>> = Vec::new();

    let mut eq_expr: Option<Box<EQ_EXPR_AST>> = None;
    let mut r = eq_expr_ast_read(parser, &mut eq_expr);
    if r != PARSER_CODES::PARSER_OK {
        *logical_and_expr = None;
        return r;
    }
    if let Some(e) = eq_expr {
        PUSH_BACK(&mut eq_exprs, e);
    }

    while parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_AND) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut eq_expr2: Option<Box<EQ_EXPR_AST>> = None;
        r = eq_expr_ast_read(parser, &mut eq_expr2);
        if r != PARSER_CODES::PARSER_OK {
            for e in eq_exprs.into_iter() {
                eq_expr_ast_free(Some(e));
            }
            *logical_and_expr = None;
            return r;
        }
        if let Some(e2) = eq_expr2 {
            PUSH_BACK(&mut eq_exprs, e2);
        }
    }

    *logical_and_expr = create_logical_and_expr_ast(eq_exprs, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn logical_or_expr_ast_read(parser: &mut PARSER, logical_or_expr: &mut Option<Box<LOGICAL_OR_EXPR_AST>>) -> PARSER_CODES {
    let mut logical_and_exprs: Vec<Box<LOGICAL_AND_EXPR_AST>> = Vec::new();

    let mut logical_and_expr: Option<Box<LOGICAL_AND_EXPR_AST>> = None;
    let mut r = logical_and_expr_ast_read(parser, &mut logical_and_expr);
    if r != PARSER_CODES::PARSER_OK {
        *logical_or_expr = None;
        return r;
    }
    if let Some(l) = logical_and_expr {
        PUSH_BACK(&mut logical_and_exprs, l);
    }

    while parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_OR) {
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut logical_and_expr2: Option<Box<LOGICAL_AND_EXPR_AST>> = None;
        r = logical_and_expr_ast_read(parser, &mut logical_and_expr2);
        if r != PARSER_CODES::PARSER_OK {
            for l in logical_and_exprs.into_iter() {
                logical_and_expr_ast_free(Some(l));
            }
            *logical_or_expr = None;
            return r;
        }
        if let Some(la) = logical_and_expr2 {
            PUSH_BACK(&mut logical_and_exprs, la);
        }
    }

    *logical_or_expr = create_logical_or_expr_ast(logical_and_exprs, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn property_ast_read(parser: &mut PARSER, property: &mut Option<Box<PROPERTY_AST>>) -> PARSER_CODES {
    let mut ident: Option<Box<IDENT_AST>> = None;
    let mut assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>> = None;

    let mut r = ident_ast_read(parser, &mut ident);
    if r != PARSER_CODES::PARSER_OK {
        *property = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_COLON) {
        ident_ast_free(ident);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_COLON]);
        *property = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r != PARSER_CODES::PARSER_OK {
        ident_ast_free(ident);
        *property = None;
        return r;
    }

    *property = create_property_ast(ident, assignment_expr, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn object_literal_ast_read(parser: &mut PARSER, object_literal: &mut Option<Box<OBJECT_LITERAL_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LBRACE) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LBRACE]);
        *object_literal = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut properties: Vec<Box<PROPERTY_AST>> = Vec::new();

    while parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACE) {
        let mut property_opt: Option<Box<PROPERTY_AST>> = None;
        let r = property_ast_read(parser, &mut property_opt);
        if r != PARSER_CODES::PARSER_OK {
            for p in properties.into_iter() {
                property_ast_free(Some(p));
            }
            *object_literal = None;
            return r;
        }
        if let Some(pv) = property_opt {
            PUSH_BACK(&mut properties, pv);
        }

        if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_RBRACE) {
            break;
        }

        if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
            for p in properties.into_iter() {
                property_ast_free(Some(p));
            }
            set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_COMMA]);
            *object_literal = None;
            return PARSER_CODES::PARSER_INVALID_TOKEN;
        }
        token_free(parser.tok.take());
        lexer_next_token(parser.lexer, &mut parser.tok);
    }

    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *object_literal = create_object_literal_ast(properties, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn array_literal_ast_read(parser: &mut PARSER, array_literal: &mut Option<Box<ARRAY_LITERAL_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LBRACKET) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LBRACKET]);
        *array_literal = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut args_list: Option<Box<ARGS_LIST_AST>> = None;
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACKET) {
        let r = args_list_ast_read(parser, &mut args_list);
        if r != PARSER_CODES::PARSER_OK {
            *array_literal = None;
            return r;
        }
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACKET) {
        if args_list.is_some() {
            args_list_ast_free(args_list);
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RBRACKET]);
        *array_literal = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *array_literal = create_array_literal_ast(args_list, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn assignment_expr_ast_read(parser: &mut PARSER, assignment_expr: &mut Option<Box<ASSIGNMENT_EXPR_AST>>) -> PARSER_CODES {
    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_LBRACE) => {
            let mut object_literal: Option<Box<OBJECT_LITERAL_AST>> = None;
            let r = object_literal_ast_read(parser, &mut object_literal);
            if r != PARSER_CODES::PARSER_OK {
                *assignment_expr = None;
                return r;
            }
            *assignment_expr = create_assignment_expr_ast(object_literal.map(|_| Box::new(())), AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_LBRACKET) => {
            let mut array_literal: Option<Box<ARRAY_LITERAL_AST>> = None;
            let r = array_literal_ast_read(parser, &mut array_literal);
            if r != PARSER_CODES::PARSER_OK {
                *assignment_expr = None;
                return r;
            }
            *assignment_expr = create_assignment_expr_ast(array_literal.map(|_| Box::new(())), AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL);
            PARSER_CODES::PARSER_OK
        }
        _ => {
            let mut logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>> = None;
            let r = logical_or_expr_ast_read(parser, &mut logical_or_expr);
            if r != PARSER_CODES::PARSER_OK {
                *assignment_expr = None;
                return r;
            }
            *assignment_expr = create_assignment_expr_ast(logical_or_expr.map(|_| Box::new(())), AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR);
            PARSER_CODES::PARSER_OK
        }
    }
}

pub fn decl_stmt_ast_read(parser: &mut PARSER, decl_stmt: &mut Option<Box<DECL_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LET) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LET]);
        *decl_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut var_name: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut var_name);
    if r != PARSER_CODES::PARSER_OK {
        *decl_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_EQ) {
        ident_ast_free(var_name);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_EQ]);
        *decl_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>> = None;
    r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r != PARSER_CODES::PARSER_OK {
        ident_ast_free(var_name);
        *decl_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        assignment_expr_ast_free(assignment_expr);
        ident_ast_free(var_name);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *decl_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *decl_stmt = create_decl_stmt_ast(var_name, assignment_expr, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn if_stmt_ast_read(parser: &mut PARSER, if_stmt: &mut Option<Box<IF_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_IF) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_IF]);
        *if_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *if_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut condition: Option<Box<LOGICAL_OR_EXPR_AST>> = None;
    let mut r = logical_or_expr_ast_read(parser, &mut condition);
    if r != PARSER_CODES::PARSER_OK {
        *if_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        logical_or_expr_ast_free(condition);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *if_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut if_body: Option<Box<BODY_AST>> = None;
    r = body_ast_read(parser, &mut if_body);
    if r != PARSER_CODES::PARSER_OK {
        logical_or_expr_ast_free(condition);
        *if_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_ELSE) {
        *if_stmt = create_if_stmt_ast(condition, if_body, None, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
        return PARSER_CODES::PARSER_OK;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_LBRACE) {
        let mut else_body: Option<Box<BODY_AST>> = None;
        r = body_ast_read(parser, &mut else_body);
        if r != PARSER_CODES::PARSER_OK {
            body_ast_free(if_body);
            logical_or_expr_ast_free(condition);
            *if_stmt = None;
            return r;
        }
        *if_stmt = create_if_stmt_ast(condition, if_body, else_body, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
        PARSER_CODES::PARSER_OK
    } else if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_IF) {
        let mut stmts: Vec<Box<STMT_AST>> = Vec::new();
        let mut if_stmt_inner: Option<Box<IF_STMT_AST>> = None;
        r = if_stmt_ast_read(parser, &mut if_stmt_inner);
        if r != PARSER_CODES::PARSER_OK {
            body_ast_free(if_body);
            logical_or_expr_ast_free(condition);
            *if_stmt = None;
            return r;
        }
        let stmt = create_stmt_ast(if_stmt_inner.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_IF);
        if let Some(s) = stmt {
            PUSH_BACK(&mut stmts, s);
        }
        let else_body = create_body_ast(stmts, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
        *if_stmt = create_if_stmt_ast(condition, if_body, else_body, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
        PARSER_CODES::PARSER_OK
    } else {
        body_ast_free(if_body);
        logical_or_expr_ast_free(condition);
        *if_stmt = None;
        PARSER_CODES::PARSER_INVALID_TOKEN
    }
}

pub fn while_stmt_ast_read(parser: &mut PARSER, while_stmt: &mut Option<Box<WHILE_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_WHILE) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_WHILE]);
        *while_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *while_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut condition: Option<Box<LOGICAL_OR_EXPR_AST>> = None;
    let mut r = logical_or_expr_ast_read(parser, &mut condition);
    if r != PARSER_CODES::PARSER_OK {
        *while_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        logical_or_expr_ast_free(condition);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *while_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut body: Option<Box<BODY_AST>> = None;
    r = body_ast_read(parser, &mut body);
    if r != PARSER_CODES::PARSER_OK {
        logical_or_expr_ast_free(condition);
        *while_stmt = None;
        return r;
    }

    *while_stmt = create_while_stmt_ast(condition, body, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn break_stmt_ast_read(parser: &mut PARSER, break_stmt: &mut Option<Box<BREAK_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_BREAK) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_BREAK]);
        *break_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *break_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *break_stmt = create_break_stmt_ast(line, pos);
    PARSER_CODES::PARSER_OK
}

pub fn continue_stmt_ast_read(parser: &mut PARSER, continue_stmt: &mut Option<Box<CONTINUE_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_CONTINUE) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_CONTINUE]);
        *continue_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *continue_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *continue_stmt = create_continue_stmt_ast(line, pos);
    PARSER_CODES::PARSER_OK
}

pub fn append_stmt_ast_read(parser: &mut PARSER, append_stmt: &mut Option<Box<APPEND_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_APPEND) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_APPEND]);
        *append_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *append_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r != PARSER_CODES::PARSER_OK {
        *append_stmt = None;
        return r;
    }

    let mut obj: Option<Box<VARIABLE_AST>> = None;
    r = variable_ast_read(parser, &mut obj, ident);
    if r != PARSER_CODES::PARSER_OK {
        if let Some(i) = ident {
            ident_ast_free(Some(i));
        }
        *append_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
        variable_ast_free(obj);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_COMMA]);
        *append_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident2: Option<Box<IDENT_AST>> = None;
    r = ident_ast_read(parser, &mut ident2);
    if r != PARSER_CODES::PARSER_OK {
        variable_ast_free(obj);
        *append_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        variable_ast_free(obj);
        if ident2.is_some() {
            ident_ast_free(ident2);
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *append_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *append_stmt = create_append_stmt_ast(obj, ident2, line, pos);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        append_stmt_ast_free(*append_stmt);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *append_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    PARSER_CODES::PARSER_OK
}

pub fn delete_stmt_ast_read(parser: &mut PARSER, delete_stmt: &mut Option<Box<DELETE_STMT_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_DELETE) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_DELETE]);
        *delete_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    let line = parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0);
    let pos = parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0);
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *delete_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r != PARSER_CODES::PARSER_OK {
        *delete_stmt = None;
        return r;
    }

    let mut obj: Option<Box<VARIABLE_AST>> = None;
    r = variable_ast_read(parser, &mut obj, ident);
    if r != PARSER_CODES::PARSER_OK {
        if let Some(i) = ident {
            ident_ast_free(Some(i));
        }
        *delete_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_COMMA) {
        variable_ast_free(obj);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_COMMA]);
        *delete_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident2: Option<Box<IDENT_AST>> = None;
    r = ident_ast_read(parser, &mut ident2);
    if r != PARSER_CODES::PARSER_OK {
        variable_ast_free(obj);
        *delete_stmt = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        variable_ast_free(obj);
        if ident2.is_some() {
            ident_ast_free(ident2);
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *delete_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *delete_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *delete_stmt = create_delete_stmt_ast(obj, ident2, line, pos);
    PARSER_CODES::PARSER_OK
}

pub fn return_stmt_ast_read(parser: &mut PARSER, return_stmt: &mut Option<Box<RETURN_STMT_AST>>) -> PARSER_CODES {
    // In original C code there is an empty check for TOKEN_TYPE_RETURN; mimic behavior by advancing.
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>> = None;

    if parser.tok.as_ref().map(|t| t.token_type) == Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        assignment_expr = None;
    } else {
        let r = assignment_expr_ast_read(parser, &mut assignment_expr);
        if r != PARSER_CODES::PARSER_OK {
            *return_stmt = None;
            return r;
        }
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
        if assignment_expr.is_some() {
            assignment_expr_ast_free(assignment_expr);
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
        *return_stmt = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }

    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *return_stmt = create_return_stmt_ast(assignment_expr, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn stmt_ast_read(parser: &mut PARSER, stmt: &mut Option<Box<STMT_AST>>) -> PARSER_CODES {
    match parser.tok.as_ref().map(|t| t.token_type) {
        Some(TOKEN_TYPE::TOKEN_TYPE_LET) => {
            let mut decl_stmt: Option<Box<DECL_STMT_AST>> = None;
            let r = decl_stmt_ast_read(parser, &mut decl_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(decl_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_DECL);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_IDENT) => {
            let ident = parser.tok.as_ref().map(|t| create_ident_ast(&t.str_val, t.frag.starting.line, t.frag.starting.pos)).flatten();
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);

            match parser.tok.as_ref().map(|t| t.token_type) {
                Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) => {
                    let mut function_call: Option<Box<FUNCTION_CALL_AST>> = None;
                    let r = function_call_ast_read(parser, &mut function_call, ident);
                    if r != PARSER_CODES::PARSER_OK {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        *stmt = None;
                        return r;
                    }
                    let function_call_stmt = create_function_call_ast(None, None, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
                    *stmt = create_stmt_ast(function_call_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_FUNCTION_CALL);
                    // ensure semicolon below
                }
                _ => {
                    let mut var_name: Option<Box<VARIABLE_AST>> = None;
                    let mut assignment_expr: Option<Box<ASSIGNMENT_EXPR_AST>> = None;
                    let r = variable_ast_read(parser, &mut var_name, ident);
                    if r != PARSER_CODES::PARSER_OK {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        *stmt = None;
                        return r;
                    }

                    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_EQ) {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        variable_ast_free(var_name);
                        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_EQ]);
                        *stmt = None;
                        return PARSER_CODES::PARSER_INVALID_TOKEN;
                    }
                    token_free(parser.tok.take());
                    lexer_next_token(parser.lexer, &mut parser.tok);

                    let r2 = assignment_expr_ast_read(parser, &mut assignment_expr);
                    if r2 != PARSER_CODES::PARSER_OK {
                        if let Some(i) = ident {
                            ident_ast_free(Some(i));
                        }
                        variable_ast_free(var_name);
                        *stmt = None;
                        return r2;
                    }

                    let assign_stmt = create_decl_stmt_ast(None, assignment_expr, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
                    *stmt = create_stmt_ast(assign_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_ASSIGN);
                }
            }

            if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_SEMI) {
                if let Some(s) = stmt.take() {
                    stmt_ast_free(Some(s));
                }
                set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_SEMI]);
                *stmt = None;
                return PARSER_CODES::PARSER_INVALID_TOKEN;
            }
            token_free(parser.tok.take());
            lexer_next_token(parser.lexer, &mut parser.tok);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_IF) => {
            let mut if_stmt: Option<Box<IF_STMT_AST>> = None;
            let r = if_stmt_ast_read(parser, &mut if_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(if_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_IF);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_WHILE) => {
            let mut while_stmt: Option<Box<WHILE_STMT_AST>> = None;
            let r = while_stmt_ast_read(parser, &mut while_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(while_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_WHILE);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_BREAK) => {
            let mut break_stmt: Option<Box<BREAK_STMT_AST>> = None;
            let r = break_stmt_ast_read(parser, &mut break_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(break_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_BREAK);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_CONTINUE) => {
            let mut continue_stmt: Option<Box<CONTINUE_STMT_AST>> = None;
            let r = continue_stmt_ast_read(parser, &mut continue_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(continue_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_CONTINUE);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_APPEND) => {
            let mut append_stmt: Option<Box<APPEND_STMT_AST>> = None;
            let r = append_stmt_ast_read(parser, &mut append_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(append_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_APPEND);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_DELETE) => {
            let mut delete_stmt: Option<Box<DELETE_STMT_AST>> = None;
            let r = delete_stmt_ast_read(parser, &mut delete_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(delete_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_DELETE);
            PARSER_CODES::PARSER_OK
        }
        Some(TOKEN_TYPE::TOKEN_TYPE_RETURN) => {
            let mut return_stmt: Option<Box<RETURN_STMT_AST>> = None;
            let r = return_stmt_ast_read(parser, &mut return_stmt);
            if r != PARSER_CODES::PARSER_OK {
                *stmt = None;
                return r;
            }
            *stmt = create_stmt_ast(return_stmt.map(|_| Box::new(())), AST_STMT_TYPE::AST_STMT_TYPE_RETURN);
            PARSER_CODES::PARSER_OK
        }
        _ => {
            set_parser_error(parser, &[
                TOKEN_TYPE::TOKEN_TYPE_LET,
                TOKEN_TYPE::TOKEN_TYPE_IDENT,
                TOKEN_TYPE::TOKEN_TYPE_IF,
                TOKEN_TYPE::TOKEN_TYPE_WHILE,
                TOKEN_TYPE::TOKEN_TYPE_BREAK,
                TOKEN_TYPE::TOKEN_TYPE_CONTINUE,
                TOKEN_TYPE::TOKEN_TYPE_APPEND,
                TOKEN_TYPE::TOKEN_TYPE_DELETE,
                TOKEN_TYPE::TOKEN_TYPE_RETURN,
            ]);
            *stmt = None;
            PARSER_CODES::PARSER_INVALID_TOKEN
        }
    }
}

pub fn body_ast_read(parser: &mut PARSER, body: &mut Option<Box<BODY_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LBRACE) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LBRACE]);
        *body = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut stmts: Vec<Box<STMT_AST>> = Vec::new();

    while parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACE) {
        let mut stmt_opt: Option<Box<STMT_AST>> = None;
        let r = stmt_ast_read(parser, &mut stmt_opt);
        if r != PARSER_CODES::PARSER_OK {
            for s in stmts.into_iter() {
                stmt_ast_free(Some(s));
            }
            *body = None;
            return r;
        }
        if let Some(sv) = stmt_opt {
            PUSH_BACK(&mut stmts, sv);
        }
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RBRACE) {
        for s in stmts.into_iter() {
            stmt_ast_free(Some(s));
        }
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RBRACE]);
        *body = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    *body = create_body_ast(stmts, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn function_decl_ast_read(parser: &mut PARSER, function: &mut Option<Box<FUNCTION_DECL_AST>>) -> PARSER_CODES {
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_FUNCTION) {
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_FUNCTION]);
        *function = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut function_name: Option<Box<IDENT_AST>> = None;
    let mut r = ident_ast_read(parser, &mut function_name);
    if r != PARSER_CODES::PARSER_OK {
        *function = None;
        return r;
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_LPAREN) {
        ident_ast_free(function_name);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_LPAREN]);
        *function = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut formal_parameters_list: Option<Box<FORMAL_PARAMETERS_LIST_AST>> = None;
    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        r = formal_parameters_list_ast_read(parser, &mut formal_parameters_list);
        if r != PARSER_CODES::PARSER_OK {
            ident_ast_free(function_name);
            *function = None;
            return r;
        }
    }

    if parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_RPAREN) {
        if formal_parameters_list.is_some() {
            formal_parameters_list_ast_free(formal_parameters_list);
        }
        ident_ast_free(function_name);
        set_parser_error(parser, &[TOKEN_TYPE::TOKEN_TYPE_RPAREN]);
        *function = None;
        return PARSER_CODES::PARSER_INVALID_TOKEN;
    }
    token_free(parser.tok.take());
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut body: Option<Box<BODY_AST>> = None;
    r = body_ast_read(parser, &mut body);
    if r != PARSER_CODES::PARSER_OK {
        if formal_parameters_list.is_some() {
            formal_parameters_list_ast_free(formal_parameters_list);
        }
        ident_ast_free(function_name);
        *function = None;
        return r;
    }

    *function = create_function_decl_ast(function_name, formal_parameters_list, body, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn unit_ast_read(parser: &mut PARSER, unit: &mut Option<Box<UNIT_AST>>) -> PARSER_CODES {
    let mut functions: Vec<Box<FUNCTION_DECL_AST>> = Vec::new();

    while parser.tok.as_ref().map(|t| t.token_type) != Some(TOKEN_TYPE::TOKEN_TYPE_EOF) {
        let mut function: Option<Box<FUNCTION_DECL_AST>> = None;
        let r = function_decl_ast_read(parser, &mut function);
        if r != PARSER_CODES::PARSER_OK {
            for f in functions.into_iter() {
                function_decl_ast_free(Some(f));
            }
            *unit = None;
            return r;
        }
        if let Some(fv) = function {
            PUSH_BACK(&mut functions, fv);
        }
    }

    *unit = create_unit_ast(functions, 0, parser.tok.as_ref().map(|t| t.frag.starting.line).unwrap_or(0), parser.tok.as_ref().map(|t| t.frag.starting.pos).unwrap_or(0));
    PARSER_CODES::PARSER_OK
}

pub fn parser_parse(parser: &mut parser_type_t, unit: &mut Option<Box<UNIT_AST>>) -> PARSER_CODES {
    lexer_next_token(parser.lexer, &mut parser.tok);
    let r = unit_ast_read(parser, unit);
    token_free(parser.tok.take());
    r
}

pub fn parser_free(_parser: Option<parser_type_t>) {
    // Drop occurs automatically
}

pub fn parser_get_error(parser: &parser_type_t) -> PARSER_ERROR {
    parser.err.clone()
}

pub fn token_type_to_str(type_: TOKEN_TYPE, buf: &mut [u8], buflen: usize) {
    let s = match type_ {
        TOKEN_TYPE::TOKEN_TYPE_FUNCTION => "FUNCTION",
        TOKEN_TYPE::TOKEN_TYPE_LET => "LET",
        TOKEN_TYPE::TOKEN_TYPE_IF => "IF",
        TOKEN_TYPE::TOKEN_TYPE_ELSE => "ELSE",
        TOKEN_TYPE::TOKEN_TYPE_WHILE => "WHILE",
        TOKEN_TYPE::TOKEN_TYPE_BREAK => "BREAK",
        TOKEN_TYPE::TOKEN_TYPE_CONTINUE => "CONTINUE",
        TOKEN_TYPE::TOKEN_TYPE_APPEND => "APPEND",
        TOKEN_TYPE::TOKEN_TYPE_DELETE => "DELETE",
        TOKEN_TYPE::TOKEN_TYPE_HAS_PROPERTY => "HAS_PROPERTY",
        TOKEN_TYPE::TOKEN_TYPE_LEN => "LEN",
        TOKEN_TYPE::TOKEN_TYPE_RETURN => "RETURN",
        TOKEN_TYPE::TOKEN_TYPE_IDENT => "IDENT",
        TOKEN_TYPE::TOKEN_TYPE_OR => "OR",
        TOKEN_TYPE::TOKEN_TYPE_AND => "AND",
        TOKEN_TYPE::TOKEN_TYPE_EQEQ => "EQEQ",
        TOKEN_TYPE::TOKEN_TYPE_NEQ => "NEQ",
        TOKEN_TYPE::TOKEN_TYPE_LT => "LT",
        TOKEN_TYPE::TOKEN_TYPE_GT => "GT",
        TOKEN_TYPE::TOKEN_TYPE_LE => "LE",
        TOKEN_TYPE::TOKEN_TYPE_GE => "GE",
        TOKEN_TYPE::TOKEN_TYPE_EQ => "EQ",
        TOKEN_TYPE::TOKEN_TYPE_PLUS => "PLUS",
        TOKEN_TYPE::TOKEN_TYPE_MINUS => "MINUS",
        TOKEN_TYPE::TOKEN_TYPE_MUL => "MUL",
        TOKEN_TYPE::TOKEN_TYPE_DIV => "DIV",
        TOKEN_TYPE::TOKEN_TYPE_MOD => "MOD",
        TOKEN_TYPE::TOKEN_TYPE_LPAREN => "LPAREN",
        TOKEN_TYPE::TOKEN_TYPE_RPAREN => "RPAREN",
        TOKEN_TYPE::TOKEN_TYPE_NUMBER => "NUMBER",
        TOKEN_TYPE::TOKEN_TYPE_LBRACKET => "LBRACKET",
        TOKEN_TYPE::TOKEN_TYPE_RBRACKET => "RBRACKET",
        TOKEN_TYPE::TOKEN_TYPE_LBRACE => "LBRACE",
        TOKEN_TYPE::TOKEN_TYPE_RBRACE => "RBRACE",
        TOKEN_TYPE::TOKEN_TYPE_COMMA => "COMMA",
        TOKEN_TYPE::TOKEN_TYPE_SEMI => "SEMI",
        TOKEN_TYPE::TOKEN_TYPE_DOT => "DOT",
        TOKEN_TYPE::TOKEN_TYPE_COLON => "COLON",
        TOKEN_TYPE::TOKEN_TYPE_EOF => "EOF",
        TOKEN_TYPE::TOKEN_TYPE_UNKNOWN => "UNKNOWN",
    };
    let bytes = s.as_bytes();
    let len = std::cmp::min(buflen, bytes.len());
    if len > 0 && buf.len() >= len {
        buf[..len].copy_from_slice(&bytes[..len]);
    }
}

pub fn print_parser_error(err: &PARSER_ERROR) {
    let mut tmp = [0u8; 128];
    token_type_to_str(err.get_tok, &mut tmp, tmp.len());
    let got = String::from_utf8_lossy(&tmp).trim_end_matches(char::from(0)).to_string();
    eprintln!("{}:{}: error: invalid token: \"{}\"; expected tokens: ", err.line, err.pos, got);

    let mut i = 0usize;
    while i < 16 {
        if err.exp_toks[i] == TOKEN_TYPE::TOKEN_TYPE_EOF {
            break;
        }
        let mut tmp2 = [0u8; 128];
        token_type_to_str(err.exp_toks[i], &mut tmp2, tmp2.len());
        let s = String::from_utf8_lossy(&tmp2).trim_end_matches(char::from(0)).to_string();
        eprint!("\"{}\" ", s);
        i += 1;
    }
    eprintln!();
}