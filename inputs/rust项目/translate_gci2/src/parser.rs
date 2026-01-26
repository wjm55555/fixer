use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

// Bring token variants like `TOKEN_TYPE_FUNCTION` into scope.
use crate::lexer::TOKEN_TYPE::*;

// Type aliases to match the original translator naming.
type UnitAst = UNIT_AST;
type FunctionDeclAst = FUNCTION_DECL_AST;
type IdentAst = IDENT_AST;
type FormalParametersListAst = FORMAL_PARAMETERS_LIST_AST;
type VariablePartAst = VARIABLE_PART_AST;
type VariableAst = VARIABLE_AST;
type NumberAst = NUMBER_AST;
type LenExprAst = LEN_EXPR_AST;
type ArgsListAst = ARGS_LIST_AST;
type FunctionCallAst = FUNCTION_CALL_AST;
type HasPropertyExprAst = HAS_PROPERTY_EXPR_AST;
type PrimaryExprAst = PRIMARY_EXPR_AST;
type LeftUnaryExprAst = LEFT_UNARY_EXPR_AST;
type MultiplicativeExprAst = MULTIPLICATIVE_EXPR_AST;
type AdditiveExprAst = ADDITIVE_EXPR_AST;
type RelationalExprAst = RELATIONAL_EXPR_AST;
type EqExprAst = EQ_EXPR_AST;
type LogicalAndExprAst = LOGICAL_AND_EXPR_AST;
type LogicalOrExprAst = LOGICAL_OR_EXPR_AST;
type AssignmentExprAst = ASSIGNMENT_EXPR_AST;
type PropertyAst = PROPERTY_AST;
type ObjectLiteralAst = OBJECT_LITERAL_AST;
type ArrayLiteralAst = ARRAY_LITERAL_AST;
type DeclStmtAst = DECL_STMT_AST;
type BodyAst = BODY_AST;
type StmtAst = STMT_AST;
type IfStmtAst = IF_STMT_AST;
type WhileStmtAst = WHILE_STMT_AST;
type BreakStmtAst = BREAK_STMT_AST;
type ContinueStmtAst = CONTINUE_STMT_AST;
type AppendStmtAst = APPEND_STMT_AST;
type DeleteStmtAst = DELETE_STMT_AST;
type ReturnStmtAst = RETURN_STMT_AST;
type AstMultiplicativeOp = AST_MULTIPLICATIVE_OP;
type AstAdditiveOp = AST_ADDITIVE_OP;

// Allow using `AstMultiplicativeOp::AstMultiplicativeOpXxx` style.
impl AstMultiplicativeOp {
    pub const AstMultiplicativeOpMul: AstMultiplicativeOp = AST_MULTIPLICATIVE_OP::Mul;
    pub const AstMultiplicativeOpDiv: AstMultiplicativeOp = AST_MULTIPLICATIVE_OP::Div;
    pub const AstMultiplicativeOpMod: AstMultiplicativeOp = AST_MULTIPLICATIVE_OP::Mod;
}

// Allow using `AstAdditiveOp::AstAdditiveOpXxx` style.
impl AstAdditiveOp {
    pub const AstAdditiveOpPlus: AstAdditiveOp = AST_ADDITIVE_OP::Plus;
    pub const AstAdditiveOpMinus: AstAdditiveOp = AST_ADDITIVE_OP::Minus;
}
type AstVariablePartType = AST_VARIABLE_PART_TYPE;
type TokenType = TOKEN_TYPE;
type Token = TOKEN;

// Allow using `TokenType::TokenTypeXxx` style.
impl TokenType {
    pub const TokenTypeFunction: TokenType = TOKEN_TYPE_FUNCTION;
    pub const TokenTypeLet: TokenType = TOKEN_TYPE_LET;
    pub const TokenTypeIf: TokenType = TOKEN_TYPE_IF;
    pub const TokenTypeElse: TokenType = TOKEN_TYPE_ELSE;
    pub const TokenTypeWhile: TokenType = TOKEN_TYPE_WHILE;
    pub const TokenTypeBreak: TokenType = TOKEN_TYPE_BREAK;
    pub const TokenTypeContinue: TokenType = TOKEN_TYPE_CONTINUE;
    pub const TokenTypeAppend: TokenType = TOKEN_TYPE_APPEND;
    pub const TokenTypeDelete: TokenType = TOKEN_TYPE_DELETE;
    pub const TokenTypeHasProperty: TokenType = TOKEN_TYPE_HAS_PROPERTY;
    pub const TokenTypeLen: TokenType = TOKEN_TYPE_LEN;
    pub const TokenTypeReturn: TokenType = TOKEN_TYPE_RETURN;
    pub const TokenTypeIdent: TokenType = TOKEN_TYPE_IDENT;
    pub const TokenTypeOr: TokenType = TOKEN_TYPE_OR;
    pub const TokenTypeAnd: TokenType = TOKEN_TYPE_AND;
    pub const TokenTypeEqeq: TokenType = TOKEN_TYPE_EQEQ;
    pub const TokenTypeNeq: TokenType = TOKEN_TYPE_NEQ;
    pub const TokenTypeLt: TokenType = TOKEN_TYPE_LT;
    pub const TokenTypeGt: TokenType = TOKEN_TYPE_GT;
    pub const TokenTypeLe: TokenType = TOKEN_TYPE_LE;
    pub const TokenTypeGe: TokenType = TOKEN_TYPE_GE;
    pub const TokenTypeEq: TokenType = TOKEN_TYPE_EQ;
    pub const TokenTypePlus: TokenType = TOKEN_TYPE_PLUS;
    pub const TokenTypeMinus: TokenType = TOKEN_TYPE_MINUS;
    pub const TokenTypeMul: TokenType = TOKEN_TYPE_MUL;
    pub const TokenTypeDiv: TokenType = TOKEN_TYPE_DIV;
    pub const TokenTypeMod: TokenType = TOKEN_TYPE_MOD;
    pub const TokenTypeLparen: TokenType = TOKEN_TYPE_LPAREN;
    pub const TokenTypeRparen: TokenType = TOKEN_TYPE_RPAREN;
    pub const TokenTypeNumber: TokenType = TOKEN_TYPE_NUMBER;
    pub const TokenTypeLbracket: TokenType = TOKEN_TYPE_LBRACKET;
    pub const TokenTypeRbracket: TokenType = TOKEN_TYPE_RBRACKET;
    pub const TokenTypeLbrace: TokenType = TOKEN_TYPE_LBRACE;
    pub const TokenTypeRbrace: TokenType = TOKEN_TYPE_RBRACE;
    pub const TokenTypeComma: TokenType = TOKEN_TYPE_COMMA;
    pub const TokenTypeSemi: TokenType = TOKEN_TYPE_SEMI;
    pub const TokenTypeDot: TokenType = TOKEN_TYPE_DOT;
    pub const TokenTypeColon: TokenType = TOKEN_TYPE_COLON;
    pub const TokenTypeEof: TokenType = TOKEN_TYPE_EOF;
    pub const TokenTypeUnknown: TokenType = TOKEN_TYPE_UNKNOWN;
}

// Allow using `AstVariablePartType::AstVariablePartTypeXxx` style.
impl AstVariablePartType {
    pub const AstVariablePartTypeField: AstVariablePartType = AST_VARIABLE_PART_TYPE::Field;
    pub const AstVariablePartTypeIndex: AstVariablePartType = AST_VARIABLE_PART_TYPE::Index;
}

pub struct PARSER {
    lexer: lexer_type_t,
    tok: Option<Box<TOKEN>>,
    err: PARSER_ERROR,
}

#[derive(Clone, Copy)]
pub enum PARSER_CODES {
    ParserOk = 0,
    ParserInvalidToken = -1,
}

pub const PARSER_OK: i32 = 0;
pub const PARSER_INVALID_TOKEN: i32 = -1;

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
            exp_toks: [TOKEN_TYPE_EOF; 16],
            get_tok: TOKEN_TYPE_EOF,
            line: 0,
            pos: 0,
        }
    }
}

pub type ParserType = Box<PARSER>;
pub type parser_type_t = *mut PARSER;

pub fn create_parser() -> ParserType {
    Box::new(PARSER {
        lexer: std::ptr::null_mut(),
        tok: None,
        err: PARSER_ERROR::default(),
    })
}

pub fn parser_conf(parser: &mut PARSER, lexer: &LEXER) {
    parser.lexer = lexer as *const LEXER as lexer_type_t;
}

fn set_parser_error(parser: &mut PARSER, n: usize, args: &[TOKEN_TYPE]) {
    for i in 0..n {
        parser.err.exp_toks[i] = args[i];
    }
    parser.err.exp_toks[n] = TOKEN_TYPE_EOF;
    if let Some(ref tok) = parser.tok {
        parser.err.get_tok = tok.token_type;
        parser.err.line = tok.frag.starting.line;
        parser.err.pos = tok.frag.starting.pos;
    }
}

fn ident_ast_read(parser: &mut PARSER, ident: &mut Option<Box<IDENT_AST>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TOKEN_TYPE_IDENT {
            set_parser_error(parser, 1, &[TOKEN_TYPE_IDENT]);
            *ident = None;
            return PARSER_CODES::ParserInvalidToken;
        }

        *ident = Some(Box::new(create_ident_ast(
            &tok.str_val,
            tok.frag.starting.line,
            tok.frag.starting.pos,
        )));
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);
    PARSER_CODES::ParserOk
}

fn variable_part_ast_read(
    parser: &mut PARSER,
    variable_part: &mut Option<Box<VariablePartAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeDot => {
                if let Some(tok) = parser.tok.take() {
                    token_free(Some(tok));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                let mut field = None;
                let r = ident_ast_read(parser, &mut field);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *variable_part = None;
                    return r;
                }

                if let Some(f) = field {
                    *variable_part = Some(Box::new(create_variable_part_ast(
                        f,
                        AstVariablePartType::AstVariablePartTypeField,
                    )));
                }
                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeLbracket => {
                if let Some(tok) = parser.tok.take() {
                    token_free(Some(tok));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                let mut index = None;
                let r = logical_or_expr_ast_read(parser, &mut index);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *variable_part = None;
                    return r;
                }

                if let Some(ref tok) = parser.tok {
                    if tok.token_type != TokenType::TokenTypeRbracket {
                        if let Some(idx) = index {
                            logical_or_expr_ast_free(Some(idx));
                        }
                        set_parser_error(parser, 1, &[TokenType::TokenTypeRbracket]);
                        *variable_part = None;
                        return PARSER_CODES::ParserInvalidToken;
                    }
                }

                if let Some(tok) = parser.tok.take() {
                    token_free(Some(tok));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                if let Some(idx) = index {
                    *variable_part = Some(Box::new(create_variable_part_ast(
                        idx,
                        AstVariablePartType::AstVariablePartTypeIndex,
                    )));
                }
                PARSER_CODES::ParserOk
            }
            _ => {
                set_parser_error(
                    parser,
                    2,
                    &[TokenType::TokenTypeDot, TokenType::TokenTypeLbracket],
                );
                *variable_part = None;
                PARSER_CODES::ParserInvalidToken
            }
        }
    } else {
        *variable_part = None;
        PARSER_CODES::ParserInvalidToken
    }
}

fn variable_ast_read(
    parser: &mut PARSER,
    variable: &mut Option<Box<VariableAst>>,
    ident: Box<IdentAst>,
) -> PARSER_CODES {
    let mut parts: Vec<Box<VariablePartAst>> = Vec::new();

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type != TokenType::TokenTypeDot
                && tok.token_type != TokenType::TokenTypeLbracket
            {
                break;
            }
        } else {
            break;
        }

        let mut part = None;
        let r = variable_part_ast_read(parser, &mut part);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for p in parts {
                variable_part_ast_free(Some(p));
            }
            *variable = None;
            return r;
        }

        if let Some(p) = part {
            parts.push(p);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *variable = Some(Box::new(create_variable_ast(ident, parts, line, pos)));
    PARSER_CODES::ParserOk
}

fn formal_parameters_list_ast_read(
    parser: &mut PARSER,
    formal_parameters_list: &mut Option<Box<FormalParametersListAst>>,
) -> PARSER_CODES {
    let mut idents: Vec<Box<IdentAst>> = Vec::new();

    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *formal_parameters_list = None;
        return r;
    }

    if let Some(id) = ident {
        idents.push(id);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type != TokenType::TokenTypeComma {
                break;
            }
        } else {
            break;
        }

        if let Some(tok) = parser.tok.take() {
            token_free(Some(tok));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut ident = None;
        r = ident_ast_read(parser, &mut ident);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for id in idents {
                ident_ast_free(Some(id));
            }
            *formal_parameters_list = None;
            return r;
        }

        if let Some(id) = ident {
            idents.push(id);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *formal_parameters_list = Some(Box::new(create_formal_parameters_list_ast(
        idents, line, pos,
    )));
    PARSER_CODES::ParserOk
}

fn number_ast_read(parser: &mut PARSER, number: &mut Option<Box<NumberAst>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeNumber {
            set_parser_error(parser, 1, &[TokenType::TokenTypeNumber]);
            *number = None;
            return PARSER_CODES::ParserInvalidToken;
        }

        *number = Some(Box::new(create_number_ast(
            tok.int_val,
            tok.frag.starting.line,
            tok.frag.starting.pos,
        )));
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);
    PARSER_CODES::ParserOk
}

fn args_list_ast_read(
    parser: &mut PARSER,
    args_list: &mut Option<Box<ArgsListAst>>,
) -> PARSER_CODES {
    let mut assignment_exprs: Vec<Box<AssignmentExprAst>> = Vec::new();

    let mut assignment_expr = None;
    let mut r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *args_list = None;
        return r;
    }

    if let Some(ae) = assignment_expr {
        assignment_exprs.push(ae);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type != TokenType::TokenTypeComma {
                break;
            }
        } else {
            break;
        }

        if let Some(tok) = parser.tok.take() {
            token_free(Some(tok));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut assignment_expr = None;
        r = assignment_expr_ast_read(parser, &mut assignment_expr);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for ae in assignment_exprs {
                assignment_expr_ast_free(Some(ae));
            }
            *args_list = None;
            return r;
        }

        if let Some(ae) = assignment_expr {
            assignment_exprs.push(ae);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *args_list = Some(Box::new(create_args_list_ast(assignment_exprs, line, pos)));
    PARSER_CODES::ParserOk
}

fn function_call_ast_read(
    parser: &mut PARSER,
    function_call: &mut Option<Box<FunctionCallAst>>,
    function_name: Box<IdentAst>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *function_call = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut args_list = None;

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            let r = args_list_ast_read(parser, &mut args_list);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                *function_call = None;
                return r;
            }
        }
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(al) = args_list {
                args_list_ast_free(Some(al));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *function_call = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *function_call = Some(Box::new(create_function_call_ast(
        function_name,
        args_list,
        line,
        pos,
    )));
    PARSER_CODES::ParserOk
}

fn has_property_expr_ast_read(
    parser: &mut PARSER,
    has_property_expr: &mut Option<Box<HasPropertyExprAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeHasProperty {
            set_parser_error(parser, 1, &[TokenType::TokenTypeHasProperty]);
            *has_property_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *has_property_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *has_property_expr = None;
        return r;
    }

    let ident = ident.unwrap();
    let mut obj = None;
    r = variable_ast_read(parser, &mut obj, ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *has_property_expr = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeComma {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeComma]);
            *has_property_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(o) = obj {
            variable_ast_free(Some(o));
        }
        *has_property_expr = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *has_property_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *has_property_expr = Some(Box::new(create_has_property_expr_ast(
        obj.unwrap(),
        ident.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn len_expr_ast_read(parser: &mut PARSER, len_expr: &mut Option<Box<LenExprAst>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLen]);
            *len_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *len_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *len_expr = None;
        return r;
    }

    let ident = ident.unwrap();
    let mut arr = None;
    r = variable_ast_read(parser, &mut arr, ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *len_expr = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(a) = arr {
                variable_ast_free(Some(a));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *len_expr = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *len_expr = Some(Box::new(create_len_expr_ast(arr.unwrap(), line, pos)));

    PARSER_CODES::ParserOk
}

fn primary_expr_ast_read(
    parser: &mut PARSER,
    primary_expr: &mut Option<Box<PrimaryExprAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeIdent => {
                let ident = Box::new(create_ident_ast(
                    &tok.str_val,
                    tok.frag.starting.line,
                    tok.frag.starting.pos,
                ));

                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                if let Some(ref tok) = parser.tok {
                    if tok.token_type == TokenType::TokenTypeLparen {
                        let mut function_call = None;
                        let r = function_call_ast_read(parser, &mut function_call, ident);
                        if r as i32 != PARSER_CODES::ParserOk as i32 {
                            *primary_expr = None;
                            return r;
                        }

                        *primary_expr = Some(Box::new(create_primary_expr_ast(
                            function_call.unwrap(),
                            AstPrimaryExprType::AstPrimaryExprTypeFunctionCall,
                        )));

                        return PARSER_CODES::ParserOk;
                    }
                }

                let mut var_name = None;
                let r = variable_ast_read(parser, &mut var_name, ident);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *primary_expr = None;
                    return r;
                }

                *primary_expr = Some(Box::new(create_primary_expr_ast(
                    var_name.unwrap(),
                    AstPrimaryExprType::AstPrimaryExprTypeVariable,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeHasProperty => {
                let mut has_property_expr = None;
                let r = has_property_expr_ast_read(parser, &mut has_property_expr);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *primary_expr = None;
                    return r;
                }

                *primary_expr = Some(Box::new(create_primary_expr_ast(
                    has_property_expr.unwrap(),
                    AstPrimaryExprType::AstPrimaryExprTypeHasProperty,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeLen => {
                let mut len_expr = None;
                let r = len_expr_ast_read(parser, &mut len_expr);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *primary_expr = None;
                    return r;
                }

                *primary_expr = Some(Box::new(create_primary_expr_ast(
                    len_expr.unwrap(),
                    AstPrimaryExprType::AstPrimaryExprTypeLen,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeNumber => {
                let mut number = None;
                let r = number_ast_read(parser, &mut number);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *primary_expr = None;
                    return r;
                }

                *primary_expr = Some(Box::new(create_primary_expr_ast(
                    number.unwrap(),
                    AstPrimaryExprType::AstPrimaryExprTypeNumber,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeLparen => {
                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                let mut logical_or_expr = None;
                let r = logical_or_expr_ast_read(parser, &mut logical_or_expr);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *primary_expr = None;
                    return r;
                }

                if let Some(ref tok) = parser.tok {
                    if tok.token_type != TokenType::TokenTypeRparen {
                        if let Some(loe) = logical_or_expr {
                            logical_or_expr_ast_free(Some(loe));
                        }
                        set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
                        *primary_expr = None;
                        return PARSER_CODES::ParserInvalidToken;
                    }
                }

                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                *primary_expr = Some(Box::new(create_primary_expr_ast(
                    logical_or_expr.unwrap(),
                    AstPrimaryExprType::AstPrimaryExprTypeLogicalOrExpr,
                )));

                PARSER_CODES::ParserOk
            }
            _ => {
                set_parser_error(
                    parser,
                    5,
                    &[
                        TokenType::TokenTypeIdent,
                        TokenType::TokenTypeNumber,
                        TokenType::TokenTypeLparen,
                        TokenType::TokenTypeLen,
                        TokenType::TokenTypeHasProperty,
                    ],
                );
                *primary_expr = None;
                PARSER_CODES::ParserInvalidToken
            }
        }
    } else {
        *primary_expr = None;
        PARSER_CODES::ParserInvalidToken
    }
}

fn left_unary_expr_ast_read(
    parser: &mut PARSER,
    left_unary_expr: &mut Option<Box<LeftUnaryExprAst>>,
) -> PARSER_CODES {
    let mut op = AstLeftUnaryOp::AstLeftUnaryOpPlus;

    if let Some(ref tok) = parser.tok {
        if tok.token_type == TokenType::TokenTypePlus {
            op = AstLeftUnaryOp::AstLeftUnaryOpPlus;
            if let Some(t) = parser.tok.take() {
                token_free(Some(t));
            }
            lexer_next_token(parser.lexer, &mut parser.tok);
        } else if tok.token_type == TokenType::TokenTypeMinus {
            op = AstLeftUnaryOp::AstLeftUnaryOpMinus;
            if let Some(t) = parser.tok.take() {
                token_free(Some(t));
            }
            lexer_next_token(parser.lexer, &mut parser.tok);
        }
    }

    let mut primary_expr = None;
    let r = primary_expr_ast_read(parser, &mut primary_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *left_unary_expr = None;
        return r;
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *left_unary_expr = Some(Box::new(create_left_unary_expr_ast(
        op,
        primary_expr.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn multiplicative_expr_ast_read(
    parser: &mut PARSER,
    multiplicative_expr: &mut Option<Box<MultiplicativeExprAst>>,
) -> PARSER_CODES {
    let mut left_unary_exprs: Vec<Box<LeftUnaryExprAst>> = Vec::new();
    let mut ops: Vec<AstMultiplicativeOp> = Vec::new();

    let mut left_unary_expr = None;
    let mut r = left_unary_expr_ast_read(parser, &mut left_unary_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *multiplicative_expr = None;
        return r;
    }

    if let Some(lue) = left_unary_expr {
        left_unary_exprs.push(lue);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            match tok.token_type {
                TokenType::TokenTypeMul => {
                    ops.push(AstMultiplicativeOp::AstMultiplicativeOpMul);
                }
                TokenType::TokenTypeDiv => {
                    ops.push(AstMultiplicativeOp::AstMultiplicativeOpDiv);
                }
                TokenType::TokenTypeMod => {
                    ops.push(AstMultiplicativeOp::AstMultiplicativeOpMod);
                }
                _ => break,
            }

            if let Some(t) = parser.tok.take() {
                token_free(Some(t));
            }
            lexer_next_token(parser.lexer, &mut parser.tok);

            let mut left_unary_expr = None;
            r = left_unary_expr_ast_read(parser, &mut left_unary_expr);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                for lue in left_unary_exprs {
                    left_unary_expr_ast_free(Some(lue));
                }
                *multiplicative_expr = None;
                return r;
            }

            if let Some(lue) = left_unary_expr {
                left_unary_exprs.push(lue);
            }
        } else {
            break;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *multiplicative_expr = Some(Box::new(create_multiplicative_expr_ast(
        left_unary_exprs,
        ops,
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn additive_expr_ast_read(
    parser: &mut PARSER,
    additive_expr: &mut Option<Box<AdditiveExprAst>>,
) -> PARSER_CODES {
    let mut multiplicative_exprs: Vec<Box<MultiplicativeExprAst>> = Vec::new();
    let mut ops: Vec<AstAdditiveOp> = Vec::new();

    let mut multiplicative_expr = None;
    let mut r = multiplicative_expr_ast_read(parser, &mut multiplicative_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *additive_expr = None;
        return r;
    }

    if let Some(me) = multiplicative_expr {
        multiplicative_exprs.push(me);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            match tok.token_type {
                TokenType::TokenTypePlus => {
                    ops.push(AstAdditiveOp::AstAdditiveOpPlus);
                }
                TokenType::TokenTypeMinus => {
                    ops.push(AstAdditiveOp::AstAdditiveOpMinus);
                }
                _ => break,
            }

            if let Some(t) = parser.tok.take() {
                token_free(Some(t));
            }
            lexer_next_token(parser.lexer, &mut parser.tok);

            let mut multiplicative_expr = None;
            r = multiplicative_expr_ast_read(parser, &mut multiplicative_expr);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                for me in multiplicative_exprs {
                    multiplicative_expr_ast_free(Some(me));
                }
                *additive_expr = None;
                return r;
            }

            if let Some(me) = multiplicative_expr {
                multiplicative_exprs.push(me);
            }
        } else {
            break;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *additive_expr = Some(Box::new(create_additive_expr_ast(
        multiplicative_exprs,
        ops,
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn relational_expr_ast_read(
    parser: &mut PARSER,
    relational_expr: &mut Option<Box<RelationalExprAst>>,
) -> PARSER_CODES {
    let mut left = None;
    let mut r = additive_expr_ast_read(parser, &mut left);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *relational_expr = None;
        return r;
    }

    let mut rel_op = AstRelOp::AstRelOpLt;
    let mut right = None;

    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeLt => {
                rel_op = AstRelOp::AstRelOpLt;
            }
            TokenType::TokenTypeGt => {
                rel_op = AstRelOp::AstRelOpGt;
            }
            TokenType::TokenTypeLe => {
                rel_op = AstRelOp::AstRelOpLe;
            }
            TokenType::TokenTypeGe => {
                rel_op = AstRelOp::AstRelOpGe;
            }
            _ => {
                let line = if let Some(ref tok) = parser.tok {
                    tok.frag.starting.line
                } else {
                    0
                };
                let pos = if let Some(ref tok) = parser.tok {
                    tok.frag.starting.pos
                } else {
                    0
                };

                *relational_expr = Some(Box::new(create_relational_expr_ast(
                    left.unwrap(),
                    rel_op,
                    None,
                    line,
                    pos,
                )));

                return PARSER_CODES::ParserOk;
            }
        }

        if let Some(t) = parser.tok.take() {
            token_free(Some(t));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        r = additive_expr_ast_read(parser, &mut right);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            if let Some(l) = left {
                additive_expr_ast_free(Some(l));
            }
            *relational_expr = None;
            return r;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *relational_expr = Some(Box::new(create_relational_expr_ast(
        left.unwrap(),
        rel_op,
        right,
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn eq_expr_ast_read(parser: &mut PARSER, eq_expr: &mut Option<Box<EqExprAst>>) -> PARSER_CODES {
    let mut left = None;
    let mut r = relational_expr_ast_read(parser, &mut left);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *eq_expr = None;
        return r;
    }

    let mut eq_op = AstEqOp::AstEqOpEqeq;
    let mut right = None;

    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeEqeq => {
                eq_op = AstEqOp::AstEqOpEqeq;
            }
            TokenType::TokenTypeNeq => {
                eq_op = AstEqOp::AstEqOpNeq;
            }
            _ => {
                let line = if let Some(ref tok) = parser.tok {
                    tok.frag.starting.line
                } else {
                    0
                };
                let pos = if let Some(ref tok) = parser.tok {
                    tok.frag.starting.pos
                } else {
                    0
                };

                *eq_expr = Some(Box::new(create_eq_expr_ast(
                    left.unwrap(),
                    eq_op,
                    None,
                    line,
                    pos,
                )));

                return PARSER_CODES::ParserOk;
            }
        }

        if let Some(t) = parser.tok.take() {
            token_free(Some(t));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        r = relational_expr_ast_read(parser, &mut right);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            if let Some(l) = left {
                relational_expr_ast_free(Some(l));
            }
            *eq_expr = None;
            return r;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *eq_expr = Some(Box::new(create_eq_expr_ast(
        left.unwrap(),
        eq_op,
        right,
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn logical_and_expr_ast_read(
    parser: &mut PARSER,
    logical_and_expr: &mut Option<Box<LogicalAndExprAst>>,
) -> PARSER_CODES {
    let mut eq_exprs: Vec<Box<EqExprAst>> = Vec::new();

    let mut eq_expr = None;
    let mut r = eq_expr_ast_read(parser, &mut eq_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *logical_and_expr = None;
        return r;
    }

    if let Some(ee) = eq_expr {
        eq_exprs.push(ee);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type != TokenType::TokenTypeAnd {
                break;
            }
        } else {
            break;
        }

        if let Some(t) = parser.tok.take() {
            token_free(Some(t));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut eq_expr = None;
        r = eq_expr_ast_read(parser, &mut eq_expr);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for ee in eq_exprs {
                eq_expr_ast_free(Some(ee));
            }
            *logical_and_expr = None;
            return r;
        }

        if let Some(ee) = eq_expr {
            eq_exprs.push(ee);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *logical_and_expr = Some(Box::new(create_logical_and_expr_ast(eq_exprs, line, pos)));

    PARSER_CODES::ParserOk
}

fn logical_or_expr_ast_read(
    parser: &mut PARSER,
    logical_or_expr: &mut Option<Box<LogicalOrExprAst>>,
) -> PARSER_CODES {
    let mut logical_and_exprs: Vec<Box<LogicalAndExprAst>> = Vec::new();

    let mut logical_and_expr = None;
    let mut r = logical_and_expr_ast_read(parser, &mut logical_and_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *logical_or_expr = None;
        return r;
    }

    if let Some(lae) = logical_and_expr {
        logical_and_exprs.push(lae);
    }

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type != TokenType::TokenTypeOr {
                break;
            }
        } else {
            break;
        }

        if let Some(t) = parser.tok.take() {
            token_free(Some(t));
        }
        lexer_next_token(parser.lexer, &mut parser.tok);

        let mut logical_and_expr = None;
        r = logical_and_expr_ast_read(parser, &mut logical_and_expr);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for lae in logical_and_exprs {
                logical_and_expr_ast_free(Some(lae));
            }
            *logical_or_expr = None;
            return r;
        }

        if let Some(lae) = logical_and_expr {
            logical_and_exprs.push(lae);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *logical_or_expr = Some(Box::new(create_logical_or_expr_ast(
        logical_and_exprs,
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn property_ast_read(parser: &mut PARSER, property: &mut Option<Box<PropertyAst>>) -> PARSER_CODES {
    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *property = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeColon {
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeColon]);
            *property = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut assignment_expr = None;
    r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(id) = ident {
            ident_ast_free(Some(id));
        }
        *property = None;
        return r;
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *property = Some(Box::new(create_property_ast(
        ident.unwrap(),
        assignment_expr.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn object_literal_ast_read(
    parser: &mut PARSER,
    object_literal: &mut Option<Box<ObjectLiteralAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLbrace {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLbrace]);
            *object_literal = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut properties: Vec<Box<PropertyAst>> = Vec::new();

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type == TokenType::TokenTypeRbrace {
                break;
            }
        } else {
            break;
        }

        let mut property = None;
        let r = property_ast_read(parser, &mut property);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for p in properties {
                property_ast_free(Some(p));
            }
            *object_literal = None;
            return r;
        }

        if let Some(p) = property {
            properties.push(p);
        }

        if let Some(ref tok) = parser.tok {
            if tok.token_type == TokenType::TokenTypeRbrace {
                break;
            }

            if tok.token_type != TokenType::TokenTypeComma {
                for p in properties {
                    property_ast_free(Some(p));
                }
                set_parser_error(parser, 1, &[TokenType::TokenTypeComma]);
                *object_literal = None;
                return PARSER_CODES::ParserInvalidToken;
            }

            if let Some(t) = parser.tok.take() {
                token_free(Some(t));
            }
            lexer_next_token(parser.lexer, &mut parser.tok);
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *object_literal = Some(Box::new(create_object_literal_ast(properties, line, pos)));

    PARSER_CODES::ParserOk
}

fn array_literal_ast_read(
    parser: &mut PARSER,
    array_literal: &mut Option<Box<ArrayLiteralAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLbracket {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLbracket]);
            *array_literal = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut args_list = None;

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRbracket {
            let r = args_list_ast_read(parser, &mut args_list);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                *array_literal = None;
                return r;
            }
        }
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRbracket {
            if let Some(al) = args_list {
                args_list_ast_free(Some(al));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRbracket]);
            *array_literal = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *array_literal = Some(Box::new(create_array_literal_ast(args_list, line, pos)));

    PARSER_CODES::ParserOk
}

fn assignment_expr_ast_read(
    parser: &mut PARSER,
    assignment_expr: &mut Option<Box<AssignmentExprAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeLbrace => {
                let mut object_literal = None;
                let r = object_literal_ast_read(parser, &mut object_literal);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *assignment_expr = None;
                    return r;
                }

                *assignment_expr = Some(Box::new(create_assignment_expr_ast(
                    object_literal.unwrap(),
                    AstAssignmentExprType::AstAssignmentExprTypeObjectLiteral,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeLbracket => {
                let mut array_literal = None;
                let r = array_literal_ast_read(parser, &mut array_literal);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *assignment_expr = None;
                    return r;
                }

                *assignment_expr = Some(Box::new(create_assignment_expr_ast(
                    array_literal.unwrap(),
                    AstAssignmentExprType::AstAssignmentExprTypeArrayLiteral,
                )));

                PARSER_CODES::ParserOk
            }
            _ => {
                let mut logical_or_expr = None;
                let r = logical_or_expr_ast_read(parser, &mut logical_or_expr);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *assignment_expr = None;
                    return r;
                }

                *assignment_expr = Some(Box::new(create_assignment_expr_ast(
                    logical_or_expr.unwrap(),
                    AstAssignmentExprType::AstAssignmentExprTypeLogicalOrExpr,
                )));

                PARSER_CODES::ParserOk
            }
        }
    } else {
        *assignment_expr = None;
        PARSER_CODES::ParserInvalidToken
    }
}

fn decl_stmt_ast_read(
    parser: &mut PARSER,
    decl_stmt: &mut Option<Box<DeclStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLet {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLet]);
            *decl_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut var_name = None;
    let mut r = ident_ast_read(parser, &mut var_name);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *decl_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeEq {
            if let Some(vn) = var_name {
                ident_ast_free(Some(vn));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeEq]);
            *decl_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut assignment_expr = None;
    r = assignment_expr_ast_read(parser, &mut assignment_expr);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(vn) = var_name {
            ident_ast_free(Some(vn));
        }
        *decl_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            if let Some(vn) = var_name {
                ident_ast_free(Some(vn));
            }
            if let Some(ae) = assignment_expr {
                assignment_expr_ast_free(Some(ae));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *decl_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *decl_stmt = Some(Box::new(create_decl_stmt_ast(
        var_name.unwrap(),
        assignment_expr.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn if_stmt_ast_read(parser: &mut PARSER, if_stmt: &mut Option<Box<IfStmtAst>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeIf {
            set_parser_error(parser, 1, &[TokenType::TokenTypeIf]);
            *if_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *if_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut condition = None;
    let mut r = logical_or_expr_ast_read(parser, &mut condition);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *if_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(cond) = condition {
                logical_or_expr_ast_free(Some(cond));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *if_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut if_body = None;
    r = body_ast_read(parser, &mut if_body);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(cond) = condition {
            logical_or_expr_ast_free(Some(cond));
        }
        *if_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeElse {
            let line = if let Some(ref tok) = parser.tok {
                tok.frag.starting.line
            } else {
                0
            };
            let pos = if let Some(ref tok) = parser.tok {
                tok.frag.starting.pos
            } else {
                0
            };

            *if_stmt = Some(Box::new(create_if_stmt_ast(
                condition.unwrap(),
                if_body.unwrap(),
                None,
                line,
                pos,
            )));
            return PARSER_CODES::ParserOk;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type == TokenType::TokenTypeLbrace {
            let mut else_body = None;
            r = body_ast_read(parser, &mut else_body);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                if let Some(ib) = if_body {
                    body_ast_free(Some(ib));
                }
                if let Some(cond) = condition {
                    logical_or_expr_ast_free(Some(cond));
                }
                *if_stmt = None;
                return r;
            }

            let line = if let Some(ref tok) = parser.tok {
                tok.frag.starting.line
            } else {
                0
            };
            let pos = if let Some(ref tok) = parser.tok {
                tok.frag.starting.pos
            } else {
                0
            };

            *if_stmt = Some(Box::new(create_if_stmt_ast(
                condition.unwrap(),
                if_body.unwrap(),
                else_body,
                line,
                pos,
            )));
            return PARSER_CODES::ParserOk;
        } else if tok.token_type == TokenType::TokenTypeIf {
            let mut if_stmt_inner = None;
            r = if_stmt_ast_read(parser, &mut if_stmt_inner);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                if let Some(ib) = if_body {
                    body_ast_free(Some(ib));
                }
                if let Some(cond) = condition {
                    logical_or_expr_ast_free(Some(cond));
                }
                *if_stmt = None;
                return r;
            }

            let mut stmts: Vec<Box<StmtAst>> = Vec::new();
            let stmt = Box::new(create_stmt_ast(
                if_stmt_inner.unwrap(),
                AstStmtType::AstStmtTypeIf,
            ));
            stmts.push(stmt);

            let line = if let Some(ref tok) = parser.tok {
                tok.frag.starting.line
            } else {
                0
            };
            let pos = if let Some(ref tok) = parser.tok {
                tok.frag.starting.pos
            } else {
                0
            };

            let else_body = Some(Box::new(create_body_ast(stmts, line, pos)));

            *if_stmt = Some(Box::new(create_if_stmt_ast(
                condition.unwrap(),
                if_body.unwrap(),
                else_body,
                line,
                pos,
            )));
            return PARSER_CODES::ParserOk;
        }
    }

    if let Some(ib) = if_body {
        body_ast_free(Some(ib));
    }
    if let Some(cond) = condition {
        logical_or_expr_ast_free(Some(cond));
    }
    *if_stmt = None;
    PARSER_CODES::ParserInvalidToken
}

fn while_stmt_ast_read(
    parser: &mut PARSER,
    while_stmt: &mut Option<Box<WhileStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeWhile {
            set_parser_error(parser, 1, &[TokenType::TokenTypeWhile]);
            *while_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *while_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut condition = None;
    let mut r = logical_or_expr_ast_read(parser, &mut condition);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *while_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(cond) = condition {
                logical_or_expr_ast_free(Some(cond));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *while_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut body = None;
    r = body_ast_read(parser, &mut body);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(cond) = condition {
            logical_or_expr_ast_free(Some(cond));
        }
        *while_stmt = None;
        return r;
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *while_stmt = Some(Box::new(create_while_stmt_ast(
        condition.unwrap(),
        body.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn break_stmt_ast_read(
    parser: &mut PARSER,
    break_stmt: &mut Option<Box<BreakStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeBreak {
            set_parser_error(parser, 1, &[TokenType::TokenTypeBreak]);
            *break_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *break_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *break_stmt = Some(Box::new(create_break_stmt_ast(line, pos)));

    PARSER_CODES::ParserOk
}

fn continue_stmt_ast_read(
    parser: &mut PARSER,
    continue_stmt: &mut Option<Box<ContinueStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeContinue {
            set_parser_error(parser, 1, &[TokenType::TokenTypeContinue]);
            *continue_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *continue_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *continue_stmt = Some(Box::new(create_continue_stmt_ast(line, pos)));

    PARSER_CODES::ParserOk
}

fn append_stmt_ast_read(
    parser: &mut PARSER,
    append_stmt: &mut Option<Box<AppendStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeAppend {
            set_parser_error(parser, 1, &[TokenType::TokenTypeAppend]);
            *append_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *append_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *append_stmt = None;
        return r;
    }

    let ident = ident.unwrap();
    let mut obj = None;
    r = variable_ast_read(parser, &mut obj, ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *append_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeComma {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeComma]);
            *append_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(o) = obj {
            variable_ast_free(Some(o));
        }
        *append_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *append_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *append_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *append_stmt = Some(Box::new(create_append_stmt_ast(
        obj.unwrap(),
        ident.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn delete_stmt_ast_read(
    parser: &mut PARSER,
    delete_stmt: &mut Option<Box<DeleteStmtAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeDelete {
            set_parser_error(parser, 1, &[TokenType::TokenTypeDelete]);
            *delete_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *delete_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    let mut r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *delete_stmt = None;
        return r;
    }

    let ident = ident.unwrap();
    let mut obj = None;
    r = variable_ast_read(parser, &mut obj, ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *delete_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeComma {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeComma]);
            *delete_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut ident = None;
    r = ident_ast_read(parser, &mut ident);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(o) = obj {
            variable_ast_free(Some(o));
        }
        *delete_stmt = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *delete_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            if let Some(o) = obj {
                variable_ast_free(Some(o));
            }
            if let Some(id) = ident {
                ident_ast_free(Some(id));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *delete_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    *delete_stmt = Some(Box::new(create_delete_stmt_ast(
        obj.unwrap(),
        ident.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn return_stmt_ast_read(
    parser: &mut PARSER,
    return_stmt: &mut Option<Box<ReturnStmtAst>>,
) -> PARSER_CODES {
    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut assignment_expr = None;

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            let r = assignment_expr_ast_read(parser, &mut assignment_expr);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                *return_stmt = None;
                return r;
            }
        }
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeSemi {
            if let Some(ae) = assignment_expr {
                assignment_expr_ast_free(Some(ae));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
            *return_stmt = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *return_stmt = Some(Box::new(create_return_stmt_ast(assignment_expr, line, pos)));

    PARSER_CODES::ParserOk
}

fn stmt_ast_read(parser: &mut PARSER, stmt: &mut Option<Box<StmtAst>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        match tok.token_type {
            TokenType::TokenTypeLet => {
                let mut decl_stmt = None;
                let r = decl_stmt_ast_read(parser, &mut decl_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    decl_stmt.unwrap(),
                    AstStmtType::AstStmtTypeDecl,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeIdent => {
                let ident = Box::new(create_ident_ast(
                    &tok.str_val,
                    tok.frag.starting.line,
                    tok.frag.starting.pos,
                ));

                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                if let Some(ref tok) = parser.tok {
                    if tok.token_type == TokenType::TokenTypeLparen {
                        let mut function_call = None;
                        let r = function_call_ast_read(parser, &mut function_call, ident);
                        if r as i32 != PARSER_CODES::ParserOk as i32 {
                            *stmt = None;
                            return r;
                        }

                        let function_call_stmt = Box::new(create_function_call_stmt_ast(
                            function_call.unwrap(),
                            if let Some(ref t) = parser.tok {
                                t.frag.starting.line
                            } else {
                                0
                            },
                            if let Some(ref t) = parser.tok {
                                t.frag.starting.pos
                            } else {
                                0
                            },
                        ));

                        *stmt = Some(Box::new(create_stmt_ast(
                            function_call_stmt,
                            AstStmtType::AstStmtTypeFunctionCall,
                        )));

                        if let Some(ref tok) = parser.tok {
                            if tok.token_type != TokenType::TokenTypeSemi {
                                if let Some(s) = stmt.take() {
                                    stmt_ast_free(Some(s));
                                }
                                set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
                                *stmt = None;
                                return PARSER_CODES::ParserInvalidToken;
                            }
                        }

                        if let Some(t) = parser.tok.take() {
                            token_free(Some(t));
                        }
                        lexer_next_token(parser.lexer, &mut parser.tok);

                        return PARSER_CODES::ParserOk;
                    }
                }

                let mut var_name = None;
                let r = variable_ast_read(parser, &mut var_name, ident);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                if let Some(ref tok) = parser.tok {
                    if tok.token_type != TokenType::TokenTypeEq {
                        if let Some(vn) = var_name {
                            variable_ast_free(Some(vn));
                        }
                        set_parser_error(parser, 1, &[TokenType::TokenTypeEq]);
                        *stmt = None;
                        return PARSER_CODES::ParserInvalidToken;
                    }
                }

                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                let mut assignment_expr = None;
                let r = assignment_expr_ast_read(parser, &mut assignment_expr);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    if let Some(vn) = var_name {
                        variable_ast_free(Some(vn));
                    }
                    *stmt = None;
                    return r;
                }

                let assign_stmt = Box::new(create_assign_stmt_ast(
                    var_name.unwrap(),
                    assignment_expr.unwrap(),
                    if let Some(ref tok) = parser.tok {
                        tok.frag.starting.line
                    } else {
                        0
                    },
                    if let Some(ref tok) = parser.tok {
                        tok.frag.starting.pos
                    } else {
                        0
                    },
                ));

                *stmt = Some(Box::new(create_stmt_ast(
                    assign_stmt,
                    AstStmtType::AstStmtTypeAssign,
                )));

                if let Some(ref tok) = parser.tok {
                    if tok.token_type != TokenType::TokenTypeSemi {
                        if let Some(s) = stmt.take() {
                            stmt_ast_free(Some(s));
                        }
                        set_parser_error(parser, 1, &[TokenType::TokenTypeSemi]);
                        *stmt = None;
                        return PARSER_CODES::ParserInvalidToken;
                    }
                }

                if let Some(t) = parser.tok.take() {
                    token_free(Some(t));
                }
                lexer_next_token(parser.lexer, &mut parser.tok);

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeIf => {
                let mut if_stmt = None;
                let r = if_stmt_ast_read(parser, &mut if_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    if_stmt.unwrap(),
                    AstStmtType::AstStmtTypeIf,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeWhile => {
                let mut while_stmt = None;
                let r = while_stmt_ast_read(parser, &mut while_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    while_stmt.unwrap(),
                    AstStmtType::AstStmtTypeWhile,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeBreak => {
                let mut break_stmt = None;
                let r = break_stmt_ast_read(parser, &mut break_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    break_stmt.unwrap(),
                    AstStmtType::AstStmtTypeBreak,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeContinue => {
                let mut continue_stmt = None;
                let r = continue_stmt_ast_read(parser, &mut continue_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    continue_stmt.unwrap(),
                    AstStmtType::AstStmtTypeContinue,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeAppend => {
                let mut append_stmt = None;
                let r = append_stmt_ast_read(parser, &mut append_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    append_stmt.unwrap(),
                    AstStmtType::AstStmtTypeAppend,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeDelete => {
                let mut delete_stmt = None;
                let r = delete_stmt_ast_read(parser, &mut delete_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    delete_stmt.unwrap(),
                    AstStmtType::AstStmtTypeDelete,
                )));

                PARSER_CODES::ParserOk
            }
            TokenType::TokenTypeReturn => {
                let mut return_stmt = None;
                let r = return_stmt_ast_read(parser, &mut return_stmt);
                if r as i32 != PARSER_CODES::ParserOk as i32 {
                    *stmt = None;
                    return r;
                }

                *stmt = Some(Box::new(create_stmt_ast(
                    return_stmt.unwrap(),
                    AstStmtType::AstStmtTypeReturn,
                )));

                PARSER_CODES::ParserOk
            }
            _ => {
                set_parser_error(
                    parser,
                    9,
                    &[
                        TokenType::TokenTypeLet,
                        TokenType::TokenTypeIdent,
                        TokenType::TokenTypeIf,
                        TokenType::TokenTypeWhile,
                        TokenType::TokenTypeBreak,
                        TokenType::TokenTypeContinue,
                        TokenType::TokenTypeAppend,
                        TokenType::TokenTypeDelete,
                        TokenType::TokenTypeReturn,
                    ],
                );
                *stmt = None;
                PARSER_CODES::ParserInvalidToken
            }
        }
    } else {
        *stmt = None;
        PARSER_CODES::ParserInvalidToken
    }
}

fn body_ast_read(parser: &mut PARSER, body: &mut Option<Box<BodyAst>>) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLbrace {
            set_parser_error(parser, 1, &[TokenType::TokenTypeLbrace]);
            *body = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut stmts: Vec<Box<StmtAst>> = Vec::new();

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type == TokenType::TokenTypeRbrace {
                break;
            }
        } else {
            break;
        }

        let mut stmt = None;
        let r = stmt_ast_read(parser, &mut stmt);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for s in stmts {
                stmt_ast_free(Some(s));
            }
            *body = None;
            return r;
        }

        if let Some(s) = stmt {
            stmts.push(s);
        }
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRbrace {
            for s in stmts {
                stmt_ast_free(Some(s));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRbrace]);
            *body = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *body = Some(Box::new(create_body_ast(stmts, line, pos)));

    PARSER_CODES::ParserOk
}

fn function_decl_ast_read(
    parser: &mut PARSER,
    function: &mut Option<Box<FunctionDeclAst>>,
) -> PARSER_CODES {
    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeFunction {
            set_parser_error(parser, 1, &[TokenType::TokenTypeFunction]);
            *function = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut function_name = None;
    let mut r = ident_ast_read(parser, &mut function_name);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        *function = None;
        return r;
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeLparen {
            if let Some(fn_name) = function_name {
                ident_ast_free(Some(fn_name));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeLparen]);
            *function = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut formal_parameters_list = None;

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            r = formal_parameters_list_ast_read(parser, &mut formal_parameters_list);
            if r as i32 != PARSER_CODES::ParserOk as i32 {
                if let Some(fn_name) = function_name {
                    ident_ast_free(Some(fn_name));
                }
                *function = None;
                return r;
            }
        }
    }

    if let Some(ref tok) = parser.tok {
        if tok.token_type != TokenType::TokenTypeRparen {
            if let Some(fn_name) = function_name {
                ident_ast_free(Some(fn_name));
            }
            if let Some(fpl) = formal_parameters_list {
                formal_parameters_list_ast_free(Some(fpl));
            }
            set_parser_error(parser, 1, &[TokenType::TokenTypeRparen]);
            *function = None;
            return PARSER_CODES::ParserInvalidToken;
        }
    }

    if let Some(t) = parser.tok.take() {
        token_free(Some(t));
    }
    lexer_next_token(parser.lexer, &mut parser.tok);

    let mut body = None;
    r = body_ast_read(parser, &mut body);
    if r as i32 != PARSER_CODES::ParserOk as i32 {
        if let Some(fn_name) = function_name {
            ident_ast_free(Some(fn_name));
        }
        if let Some(fpl) = formal_parameters_list {
            formal_parameters_list_ast_free(Some(fpl));
        }
        *function = None;
        return r;
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *function = Some(Box::new(create_function_decl_ast(
        function_name.unwrap(),
        formal_parameters_list,
        body.unwrap(),
        line,
        pos,
    )));

    PARSER_CODES::ParserOk
}

fn unit_ast_read(parser: &mut PARSER, unit: &mut Option<Box<UnitAst>>) -> PARSER_CODES {
    let mut functions: Vec<Box<FunctionDeclAst>> = Vec::new();

    loop {
        if let Some(ref tok) = parser.tok {
            if tok.token_type == TokenType::TokenTypeEof {
                break;
            }
        } else {
            break;
        }

        let mut function = None;
        let r = function_decl_ast_read(parser, &mut function);
        if r as i32 != PARSER_CODES::ParserOk as i32 {
            for f in functions {
                function_decl_ast_free(Some(f));
            }
            *unit = None;
            return r;
        }

        if let Some(f) = function {
            functions.push(f);
        }
    }

    let line = if let Some(ref tok) = parser.tok {
        tok.frag.starting.line
    } else {
        0
    };
    let pos = if let Some(ref tok) = parser.tok {
        tok.frag.starting.pos
    } else {
        0
    };

    *unit = Some(Box::new(create_unit_ast(functions, line, pos)));

    PARSER_CODES::ParserOk
}

pub fn parser_parse(parser: &mut PARSER, unit: &mut Option<Box<UnitAst>>) -> i32 {
    lexer_next_token(parser.lexer, &mut parser.tok);
    let r = unit_ast_read(parser, unit);
    if let Some(tok) = parser.tok.take() {
        token_free(Some(tok));
    }
    r as i32
}

// Compatibility wrapper: many callers use `parser_free(&mut parser)`.
pub fn parser_free(_parser: &mut PARSER) {}

pub fn parser_get_error(parser: &PARSER) -> PARSER_ERROR {
    parser.err.clone()
}

pub fn token_type_to_str(token_type: TokenType) -> &'static str {
    match token_type {
        TOKEN_TYPE_FUNCTION => "FUNCTION",
        TOKEN_TYPE_LET => "LET",
        TOKEN_TYPE_IF => "IF",
        TOKEN_TYPE_ELSE => "ELSE",
        TOKEN_TYPE_WHILE => "WHILE",
        TOKEN_TYPE_BREAK => "BREAK",
        TOKEN_TYPE_CONTINUE => "CONTINUE",
        TOKEN_TYPE_APPEND => "APPEND",
        TOKEN_TYPE_DELETE => "DELETE",
        TOKEN_TYPE_HAS_PROPERTY => "HAS_PROPERTY",
        TOKEN_TYPE_LEN => "LEN",
        TOKEN_TYPE_RETURN => "RETURN",
        TOKEN_TYPE_IDENT => "IDENT",
        TOKEN_TYPE_OR => "OR",
        TOKEN_TYPE_AND => "AND",
        TOKEN_TYPE_EQEQ => "EQEQ",
        TOKEN_TYPE_NEQ => "NEQ",
        TOKEN_TYPE_LT => "LT",
        TOKEN_TYPE_GT => "GT",
        TOKEN_TYPE_LE => "LE",
        TOKEN_TYPE_GE => "GE",
        TOKEN_TYPE_EQ => "EQ",
        TOKEN_TYPE_PLUS => "PLUS",
        TOKEN_TYPE_MINUS => "MINUS",
        TOKEN_TYPE_MUL => "MUL",
        TOKEN_TYPE_DIV => "DIV",
        TOKEN_TYPE_MOD => "MOD",
        TOKEN_TYPE_LPAREN => "LPAREN",
        TOKEN_TYPE_RPAREN => "RPAREN",
        TOKEN_TYPE_NUMBER => "NUMBER",
        TOKEN_TYPE_LBRACKET => "LBRACKET",
        TOKEN_TYPE_RBRACKET => "RBRACKET",
        TOKEN_TYPE_LBRACE => "LBRACE",
        TOKEN_TYPE_RBRACE => "RBRACE",
        TOKEN_TYPE_COMMA => "COMMA",
        TOKEN_TYPE_SEMI => "SEMI",
        TOKEN_TYPE_DOT => "DOT",
        TOKEN_TYPE_COLON => "COLON",
        TOKEN_TYPE_EOF => "EOF",
        TOKEN_TYPE_UNKNOWN => "UNKNOWN",
    }
}

pub fn print_parser_error(err: &PARSER_ERROR) {
    let get_tok_str = token_type_to_str(err.get_tok);
    eprint!(
        "{}:{}: error: invalid token: \"{}\"; expected tokens: ",
        err.line, err.pos, get_tok_str
    );

    let mut i = 0;
    while err.exp_toks[i] != TokenType::TokenTypeEof {
        let exp_tok_str = token_type_to_str(err.exp_toks[i]);
        eprint!("\"{}\" ", exp_tok_str);
        i += 1;
    }
    eprintln!();
}
