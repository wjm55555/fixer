use crate::utils::*;
use std::fs::File;
use std::io::Write;

pub type FILE = File;

pub fn INC_SPACES_NUM() {}
pub fn DEC_SPACES_NUM() {}
pub fn PUT_SPACES() {}

#[repr(C)]
#[derive(Default)]
pub struct UNIT_AST {
    pub functions: Vec<Box<FUNCTION_DECL_AST>>,
    pub functions_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct FUNCTION_DECL_AST {
    pub function_name: Box<IDENT_AST>,
    pub formal_parameters_list: Option<Box<FORMAL_PARAMETERS_LIST_AST>>,
    pub body: Box<BODY_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct FORMAL_PARAMETERS_LIST_AST {
    pub params: Vec<Box<IDENT_AST>>,
    pub params_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct BODY_AST {
    pub stmts: Vec<Box<STMT_AST>>,
    pub stmts_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct STMT_AST {
    pub decl_stmt: Option<Box<DECL_STMT_AST>>,
    pub assign_stmt: Option<Box<ASSIGN_STMT_AST>>,
    pub function_call_stmt: Option<Box<FUNCTION_CALL_STMT_AST>>,
    pub if_stmt: Option<Box<IF_STMT_AST>>,
    pub while_stmt: Option<Box<WHILE_STMT_AST>>,
    pub break_stmt: Option<Box<BREAK_STMT_AST>>,
    pub continue_stmt: Option<Box<CONTINUE_STMT_AST>>,
    pub append_stmt: Option<Box<APPEND_STMT_AST>>,
    pub delete_stmt: Option<Box<DELETE_STMT_AST>>,
    pub return_stmt: Option<Box<RETURN_STMT_AST>>,
    pub type_: AST_STMT_TYPE,
}

#[repr(C)]
#[derive(Default)]
pub struct DECL_STMT_AST {
    pub new_var_name: Box<IDENT_AST>,
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct ASSIGN_STMT_AST {
    pub var_name: Box<VARIABLE_AST>,
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct FUNCTION_CALL_STMT_AST {
    pub function_call: Box<FUNCTION_CALL_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct IF_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub if_body: Box<BODY_AST>,
    pub else_body: Option<Box<BODY_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct WHILE_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub body: Box<BODY_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct BREAK_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct CONTINUE_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct APPEND_STMT_AST {
    pub arr: Box<VARIABLE_AST>,
    pub ident: Box<IDENT_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct DELETE_STMT_AST {
    pub var: Box<VARIABLE_AST>,
    pub ident: Box<IDENT_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct RETURN_STMT_AST {
    pub result: Option<Box<ASSIGNMENT_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct VARIABLE_AST {
    pub ident: Box<IDENT_AST>,
    pub parts: Vec<Box<VARIABLE_PART_AST>>,
    pub parts_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct VARIABLE_PART_AST {
    pub field: Option<Box<IDENT_AST>>,
    pub index: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub type_: AST_VARIABLE_PART_TYPE,
}

#[repr(C)]
#[derive(Default)]
pub struct ASSIGNMENT_EXPR_AST {
    pub object_literal: Option<Box<OBJECT_LITERAL_AST>>,
    pub array_literal: Option<Box<ARRAY_LITERAL_AST>>,
    pub logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub type_: AST_ASSIGNMENT_EXPR_TYPE,
}

#[repr(C)]
#[derive(Default)]
pub struct OBJECT_LITERAL_AST {
    pub properties: Vec<Box<PROPERTY_AST>>,
    pub properties_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct PROPERTY_AST {
    pub key: Box<IDENT_AST>,
    pub value: Box<ASSIGNMENT_EXPR_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct ARRAY_LITERAL_AST {
    pub args_list: Option<Box<ARGS_LIST_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct LOGICAL_OR_EXPR_AST {
    pub and_exprs: Vec<Box<LOGICAL_AND_EXPR_AST>>,
    pub and_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct LOGICAL_AND_EXPR_AST {
    pub eq_exprs: Vec<Box<EQ_EXPR_AST>>,
    pub eq_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct EQ_EXPR_AST {
    pub left: Box<RELATIONAL_EXPR_AST>,
    pub eq_op: AST_EQ_OP,
    pub right: Option<Box<RELATIONAL_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct RELATIONAL_EXPR_AST {
    pub left: Box<ADDITIVE_EXPR_AST>,
    pub rel_op: AST_REL_OP,
    pub right: Option<Box<ADDITIVE_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct ADDITIVE_EXPR_AST {
    pub muls: Vec<Box<MULTIPLICATIVE_EXPR_AST>>,
    pub ops: Vec<AST_ADDITIVE_OP>,
    pub muls_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct MULTIPLICATIVE_EXPR_AST {
    pub lues: Vec<Box<LEFT_UNARY_EXPR_AST>>,
    pub ops: Vec<AST_MULTIPLICATIVE_OP>,
    pub lues_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct LEFT_UNARY_EXPR_AST {
    pub op: AST_LEFT_UNARY_OP,
    pub expr: Box<PRIMARY_EXPR_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct PRIMARY_EXPR_AST {
    pub has_property_expr: Option<Box<HAS_PROPERTY_EXPR_AST>>,
    pub len_expr: Option<Box<LEN_EXPR_AST>>,
    pub function_call: Option<Box<FUNCTION_CALL_AST>>,
    pub var_name: Option<Box<VARIABLE_AST>>,
    pub number: Option<Box<NUMBER_AST>>,
    pub logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub type_: AST_PRIMARY_EXPR_TYPE,
}

#[repr(C)]
#[derive(Default)]
pub struct HAS_PROPERTY_EXPR_AST {
    pub obj: Box<VARIABLE_AST>,
    pub ident: Box<IDENT_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct LEN_EXPR_AST {
    pub arr: Box<VARIABLE_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct FUNCTION_CALL_AST {
    pub function_name: Box<IDENT_AST>,
    pub args_list: Option<Box<ARGS_LIST_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct ARGS_LIST_AST {
    pub assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>>,
    pub assignment_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct IDENT_AST {
    pub ident: String,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct NUMBER_AST {
    pub number: i64,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_STMT_TYPE {
    AST_STMT_TYPE_DECL,
    AST_STMT_TYPE_ASSIGN,
    AST_STMT_TYPE_FUNCTION_CALL,
    AST_STMT_TYPE_IF,
    AST_STMT_TYPE_WHILE,
    AST_STMT_TYPE_BREAK,
    AST_STMT_TYPE_CONTINUE,
    AST_STMT_TYPE_APPEND,
    AST_STMT_TYPE_DELETE,
    AST_STMT_TYPE_RETURN,
}

impl Default for AST_STMT_TYPE {
    fn default() -> Self {
        AST_STMT_TYPE::AST_STMT_TYPE_DECL
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_VARIABLE_PART_TYPE {
    AST_VARIABLE_PART_TYPE_FIELD,
    AST_VARIABLE_PART_TYPE_INDEX,
}

impl Default for AST_VARIABLE_PART_TYPE {
    fn default() -> Self {
        AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_FIELD
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_ASSIGNMENT_EXPR_TYPE {
    AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL,
    AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL,
    AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR,
}

impl Default for AST_ASSIGNMENT_EXPR_TYPE {
    fn default() -> Self {
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_EQ_OP {
    AST_EQ_OP_EQEQ,
    AST_EQ_OP_NEQ,
}

impl Default for AST_EQ_OP {
    fn default() -> Self {
        AST_EQ_OP::AST_EQ_OP_EQEQ
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_REL_OP {
    AST_REL_OP_LT,
    AST_REL_OP_GT,
    AST_REL_OP_LE,
    AST_REL_OP_GE,
}

impl Default for AST_REL_OP {
    fn default() -> Self {
        AST_REL_OP::AST_REL_OP_LT
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_ADDITIVE_OP {
    AST_ADDITIVE_OP_PLUS,
    AST_ADDITIVE_OP_MINUS,
}

impl Default for AST_ADDITIVE_OP {
    fn default() -> Self {
        AST_ADDITIVE_OP::AST_ADDITIVE_OP_PLUS
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_MULTIPLICATIVE_OP {
    AST_MULTIPLICATIVE_OP_MUL,
    AST_MULTIPLICATIVE_OP_DIV,
    AST_MULTIPLICATIVE_OP_MOD,
}

impl Default for AST_MULTIPLICATIVE_OP {
    fn default() -> Self {
        AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_MUL
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_LEFT_UNARY_OP {
    AST_LEFT_UNARY_OP_PLUS,
    AST_LEFT_UNARY_OP_MINUS,
}

impl Default for AST_LEFT_UNARY_OP {
    fn default() -> Self {
        AST_LEFT_UNARY_OP::AST_LEFT_UNARY_OP_PLUS
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AST_PRIMARY_EXPR_TYPE {
    AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY,
    AST_PRIMARY_EXPR_TYPE_LEN,
    AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL,
    AST_PRIMARY_EXPR_TYPE_VARIABLE,
    AST_PRIMARY_EXPR_TYPE_NUMBER,
    AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR,
}

impl Default for AST_PRIMARY_EXPR_TYPE {
    fn default() -> Self {
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_VARIABLE
    }
}

pub fn create_unit_ast(functions: Vec<Box<FUNCTION_DECL_AST>>, functions_len: usize, line: usize, pos: usize) -> Box<UNIT_AST> {
    let unit_ast = Box::new(UNIT_AST {
        functions,
        functions_len,
        line,
        pos,
    });
    unit_ast
}

pub fn dump_unit_ast_to_xml_file_inner(f: &mut FILE, ast: &UNIT_AST, spaces_num: usize) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<unit>");
    INC_SPACES_NUM();
    for i in 0..ast.functions_len {
        if let Some(func) = ast.functions.get(i) {
            dump_function_decl_ast_to_file_inner(f, func.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    let _ = writeln!(f, "</unit>");
}

pub fn dump_unit_ast_to_xml_file(f: &mut FILE, ast: &UNIT_AST) {
    return dump_unit_ast_to_xml_file_inner(f, ast, 0);
}

pub fn unit_ast_free(_ast: Box<UNIT_AST>) {
    // Rely on Rust drop for cleanup
}

pub fn create_function_decl_ast(function_name: Box<IDENT_AST>, formal_parameters_list: Option<Box<FORMAL_PARAMETERS_LIST_AST>>, body: Box<BODY_AST>, line: usize, pos: usize) -> Box<FUNCTION_DECL_AST> {
    Box::new(FUNCTION_DECL_AST {
        function_name,
        formal_parameters_list,
        body,
        line,
        pos,
    })
}

pub fn dump_function_decl_ast_to_file_inner(f: &mut FILE, ast: &FUNCTION_DECL_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<function line=\"{}\" pos=\"{}\">", ast.line, ast.pos);

    INC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "<function_name line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_ident_ast_to_file_inner(f, ast.function_name.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</function_name>");
    if let Some(ref params) = ast.formal_parameters_list {
        dump_formal_parameters_list_ast_to_file_inner(f, params.as_ref(), spaces_num);
    }
    dump_body_ast_to_file_inner(f, ast.body.as_ref(), spaces_num);
    DEC_SPACES_NUM();

    PUT_SPACES(); let _ = writeln!(f, "</function>");
}

pub fn dump_function_decl_ast_to_file(f: &mut FILE, ast: &FUNCTION_DECL_AST) {
    return dump_function_decl_ast_to_file_inner(f, ast, 0);
}

pub fn function_decl_ast_free(_ast: Box<FUNCTION_DECL_AST>) {
    // rely on drop
}

pub fn create_formal_parameters_list_ast(params: Vec<Box<IDENT_AST>>, params_len: usize, line: usize, pos: usize) -> Box<FORMAL_PARAMETERS_LIST_AST> {
    Box::new(FORMAL_PARAMETERS_LIST_AST {
        params,
        params_len,
        line,
        pos,
    })
}

pub fn dump_formal_parameters_list_ast_to_file_inner(f: &mut FILE, ast: &FORMAL_PARAMETERS_LIST_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<formal_parameters_list line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    for i in 0..ast.params_len {
        if let Some(p) = ast.params.get(i) {
            dump_ident_ast_to_file_inner(f, p.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</formal_parameters_list>");
}

pub fn dump_formal_parameters_list_ast_to_file(f: &mut FILE, ast: &FORMAL_PARAMETERS_LIST_AST) {
    dump_formal_parameters_list_ast_to_file_inner(f, ast, 0);
}

pub fn formal_parameters_list_ast_free(_ast: Box<FORMAL_PARAMETERS_LIST_AST>) {
    // rely on drop
}

pub fn create_body_ast(stmts: Vec<Box<STMT_AST>>, stmts_len: usize, line: usize, pos: usize) -> Box<BODY_AST> {
    Box::new(BODY_AST {
        stmts,
        stmts_len,
        line,
        pos,
    })
}

pub fn dump_body_ast_to_file_inner(f: &mut FILE, ast: &BODY_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<body line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    for i in 0..ast.stmts_len {
        if let Some(s) = ast.stmts.get(i) {
            dump_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</body>");
}

pub fn dump_body_ast_to_file(f: &mut FILE, ast: &BODY_AST) {
    dump_body_ast_to_file_inner(f, ast, 0);
}

pub fn body_ast_free(_ast: Box<BODY_AST>) {
    // rely on drop
}

pub fn create_stmt_ast(stmt_ptr: Box<dyn std::any::Any>, stmt_type: AST_STMT_TYPE) -> Box<STMT_AST> {
    let mut stmt = STMT_AST::default();
    stmt.type_ = stmt_type;
    match stmt_type {
        AST_STMT_TYPE::AST_STMT_TYPE_DECL => {
            if let Ok(v) = stmt_ptr.downcast::<DECL_STMT_AST>() {
                stmt.decl_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_ASSIGN => {
            if let Ok(v) = stmt_ptr.downcast::<ASSIGN_STMT_AST>() {
                stmt.assign_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_FUNCTION_CALL => {
            if let Ok(v) = stmt_ptr.downcast::<FUNCTION_CALL_STMT_AST>() {
                stmt.function_call_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_IF => {
            if let Ok(v) = stmt_ptr.downcast::<IF_STMT_AST>() {
                stmt.if_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_WHILE => {
            if let Ok(v) = stmt_ptr.downcast::<WHILE_STMT_AST>() {
                stmt.while_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_BREAK => {
            if let Ok(v) = stmt_ptr.downcast::<BREAK_STMT_AST>() {
                stmt.break_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_CONTINUE => {
            if let Ok(v) = stmt_ptr.downcast::<CONTINUE_STMT_AST>() {
                stmt.continue_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_APPEND => {
            if let Ok(v) = stmt_ptr.downcast::<DELETE_STMT_AST>() {
                stmt.delete_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_DELETE => {
            if let Ok(v) = stmt_ptr.downcast::<APPEND_STMT_AST>() {
                stmt.append_stmt = Some(v);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_RETURN => {
            if let Ok(v) = stmt_ptr.downcast::<RETURN_STMT_AST>() {
                stmt.return_stmt = Some(v);
            }
        }
    }
    Box::new(stmt)
}

pub fn dump_stmt_ast_to_file_inner(f: &mut FILE, ast: &STMT_AST, spaces_num: usize) {
    match ast.type_ {
        AST_STMT_TYPE::AST_STMT_TYPE_DECL => {
            if let Some(ref s) = ast.decl_stmt {
                dump_decl_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_ASSIGN => {
            if let Some(ref s) = ast.assign_stmt {
                dump_assign_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_FUNCTION_CALL => {
            if let Some(ref s) = ast.function_call_stmt {
                dump_function_call_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_IF => {
            if let Some(ref s) = ast.if_stmt {
                dump_if_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_WHILE => {
            if let Some(ref s) = ast.while_stmt {
                dump_while_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_BREAK => {
            if let Some(ref s) = ast.break_stmt {
                dump_break_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_APPEND => {
            if let Some(ref s) = ast.append_stmt {
                dump_append_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_CONTINUE => {
            if let Some(ref s) = ast.continue_stmt {
                dump_continue_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_DELETE => {
            if let Some(ref s) = ast.delete_stmt {
                dump_delete_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
        AST_STMT_TYPE::AST_STMT_TYPE_RETURN => {
            if let Some(ref s) = ast.return_stmt {
                dump_return_stmt_ast_to_file_inner(f, s.as_ref(), spaces_num);
            }
        }
    }
}

pub fn dump_stmt_ast_to_file(f: &mut FILE, ast: &STMT_AST) {
    dump_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn stmt_ast_free(_ast: Box<STMT_AST>) {
    // rely on drop
}

pub fn create_decl_stmt_ast(new_var_name: Box<IDENT_AST>, assignment: Box<ASSIGNMENT_EXPR_AST>, line: usize, pos: usize) -> Box<DECL_STMT_AST> {
    Box::new(DECL_STMT_AST {
        new_var_name,
        assignment,
        line,
        pos,
    })
}

pub fn dump_decl_stmt_ast_to_file_inner(f: &mut FILE, ast: &DECL_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<decl_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_ident_ast_to_file_inner(f, ast.new_var_name.as_ref(), spaces_num);
    PUT_SPACES(); let _ = writeln!(f, "<op>EQ</op>");
    dump_assignment_expr_ast_to_file_inner(f, ast.assignment.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</decl_stmt>");
}

pub fn dump_decl_stmt_ast_to_file(f: &mut FILE, ast: &DECL_STMT_AST) {
    dump_decl_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn decl_stmt_ast_free(_ast: Box<DECL_STMT_AST>) {
    // rely on drop
}

pub fn create_assign_stmt_ast(var_name: Box<VARIABLE_AST>, assignment: Box<ASSIGNMENT_EXPR_AST>, line: usize, pos: usize) -> Box<ASSIGN_STMT_AST> {
    Box::new(ASSIGN_STMT_AST {
        var_name,
        assignment,
        line,
        pos,
    })
}

pub fn dump_assign_stmt_ast_to_file_inner(f: &mut FILE, ast: &ASSIGN_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<assign_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_variable_ast_to_file_inner(f, ast.var_name.as_ref(), spaces_num);
    PUT_SPACES(); let _ = writeln!(f, "<op>EQ</op>");
    dump_assignment_expr_ast_to_file_inner(f, ast.assignment.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</assign_stmt>");
}

pub fn dump_assign_stmt_ast_to_file(f: &mut FILE, ast: &ASSIGN_STMT_AST) {
    dump_assign_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn assign_stmt_ast_free(_ast: Box<ASSIGN_STMT_AST>) {
    // rely on drop
}

pub fn create_function_call_stmt_ast(function_call: Box<FUNCTION_CALL_AST>, line: usize, pos: usize) -> Box<FUNCTION_CALL_STMT_AST> {
    Box::new(FUNCTION_CALL_STMT_AST {
        function_call,
        line,
        pos,
    })
}

pub fn dump_function_call_stmt_ast_to_file_inner(f: &mut FILE, ast: &FUNCTION_CALL_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<function_call_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_function_call_ast_to_file_inner(f, ast.function_call.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</function_call_stmt>");
}

pub fn dump_function_call_stmt_ast_to_file(f: &mut FILE, ast: &FUNCTION_CALL_STMT_AST) {
    dump_function_call_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn function_call_stmt_ast_free(_ast: Box<FUNCTION_CALL_STMT_AST>) {
    // rely on drop
}

pub fn create_if_stmt_ast(condition: Box<LOGICAL_OR_EXPR_AST>, if_body: Box<BODY_AST>, else_body: Option<Box<BODY_AST>>, line: usize, pos: usize) -> Box<IF_STMT_AST> {
    Box::new(IF_STMT_AST {
        condition,
        if_body,
        else_body,
        line,
        pos,
    })
}

pub fn dump_if_stmt_ast_to_file_inner(f: &mut FILE, ast: &IF_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<if_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "<condition>");
    INC_SPACES_NUM();
    dump_logical_or_expr_ast_to_file_inner(f, ast.condition.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</condition>");

    PUT_SPACES(); let _ = writeln!(f, "<if_body>");
    INC_SPACES_NUM();
    dump_body_ast_to_file_inner(f, ast.if_body.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</if_body>");

    if let Some(ref else_b) = ast.else_body {
        PUT_SPACES(); let _ = writeln!(f, "<else_body>");
        INC_SPACES_NUM();
        dump_body_ast_to_file_inner(f, else_b.as_ref(), spaces_num);
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</else_body>");
    }

    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</if_stmt>");
}

pub fn dump_if_stmt_ast_to_file(f: &mut FILE, ast: &IF_STMT_AST) {
    dump_if_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn if_stmt_ast_free(_ast: Box<IF_STMT_AST>) {
    // rely on drop
}

pub fn create_while_stmt_ast(condition: Box<LOGICAL_OR_EXPR_AST>, body: Box<BODY_AST>, line: usize, pos: usize) -> Box<WHILE_STMT_AST> {
    Box::new(WHILE_STMT_AST {
        condition,
        body,
        line,
        pos,
    })
}

pub fn dump_while_stmt_ast_to_file_inner(f: &mut FILE, ast: &WHILE_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<while_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "<condition>");
    INC_SPACES_NUM();
    dump_logical_or_expr_ast_to_file_inner(f, ast.condition.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</condition>");

    dump_body_ast_to_file_inner(f, ast.body.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</while_stmt>");
}

pub fn dump_while_stmt_ast_to_file(f: &mut FILE, ast: &WHILE_STMT_AST) {
    dump_while_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn while_stmt_ast_free(_ast: Box<WHILE_STMT_AST>) {
    // rely on drop
}

pub fn create_break_stmt_ast(line: usize, pos: usize) -> Box<BREAK_STMT_AST> {
    Box::new(BREAK_STMT_AST { line, pos })
}

pub fn dump_break_stmt_ast_to_file_inner(_f: &mut FILE, _ast: &BREAK_STMT_AST, _spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(_f, "<break></break>");
}

pub fn dump_break_stmt_ast_to_file(f: &mut FILE, ast: &BREAK_STMT_AST) {
    dump_break_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn break_stmt_ast_free(_ast: Box<BREAK_STMT_AST>) {
    // rely on drop
}

pub fn create_continue_stmt_ast(line: usize, pos: usize) -> Box<CONTINUE_STMT_AST> {
    Box::new(CONTINUE_STMT_AST { line, pos })
}

pub fn dump_continue_stmt_ast_to_file_inner(_f: &mut FILE, _ast: &CONTINUE_STMT_AST, _spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(_f, "<continue></continue>");
}

pub fn dump_continue_stmt_ast_to_file(f: &mut FILE, ast: &CONTINUE_STMT_AST) {
    dump_continue_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn continue_stmt_ast_free(_ast: Box<CONTINUE_STMT_AST>) {
    // rely on drop
}

pub fn create_append_stmt_ast(arr: Box<VARIABLE_AST>, ident: Box<IDENT_AST>, line: usize, pos: usize) -> Box<APPEND_STMT_AST> {
    Box::new(APPEND_STMT_AST {
        arr,
        ident,
        line,
        pos,
    })
}

pub fn dump_append_stmt_ast_to_file_inner(f: &mut FILE, ast: &APPEND_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<append_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_variable_ast_to_file_inner(f, ast.arr.as_ref(), spaces_num);
    dump_ident_ast_to_file_inner(f, ast.ident.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</append_stmt>");
}

pub fn dump_append_stmt_ast_to_file(f: &mut FILE, ast: &APPEND_STMT_AST) {
    dump_append_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn append_stmt_ast_free(_ast: Box<APPEND_STMT_AST>) {
    // rely on drop
}

pub fn create_delete_stmt_ast(var: Box<VARIABLE_AST>, ident: Box<IDENT_AST>, line: usize, pos: usize) -> Box<DELETE_STMT_AST> {
    Box::new(DELETE_STMT_AST {
        var,
        ident,
        line,
        pos,
    })
}

pub fn dump_delete_stmt_ast_to_file_inner(f: &mut FILE, ast: &DELETE_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<delete_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_variable_ast_to_file_inner(f, ast.var.as_ref(), spaces_num);
    dump_ident_ast_to_file_inner(f, ast.ident.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</delete_stmt>");
}

pub fn dump_delete_stmt_ast_to_file(f: &mut FILE, ast: &DELETE_STMT_AST) {
    dump_delete_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn delete_stmt_ast_free(_ast: Box<DELETE_STMT_AST>) {
    // rely on drop
}

pub fn create_return_stmt_ast(result: Option<Box<ASSIGNMENT_EXPR_AST>>, line: usize, pos: usize) -> Box<RETURN_STMT_AST> {
    Box::new(RETURN_STMT_AST { result, line, pos })
}

pub fn dump_return_stmt_ast_to_file_inner(f: &mut FILE, ast: &RETURN_STMT_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<return_stmt> line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    if ast.result.is_some() {
        INC_SPACES_NUM();
        if let Some(ref r) = ast.result {
            dump_assignment_expr_ast_to_file_inner(f, r.as_ref(), spaces_num);
        }
        DEC_SPACES_NUM();
    }
    PUT_SPACES(); let _ = writeln!(f, "</return_stmt>");
}

pub fn dump_return_stmt_ast_to_file(f: &mut FILE, ast: &RETURN_STMT_AST) {
    dump_return_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn return_stmt_ast_free(_ast: Box<RETURN_STMT_AST>) {
    // rely on drop
}

pub fn create_variable_part_ast(variable_part_ptr: Box<dyn std::any::Any>, type_: AST_VARIABLE_PART_TYPE) -> Box<VARIABLE_PART_AST> {
    let mut variable_part = VARIABLE_PART_AST::default();
    variable_part.type_ = type_;
    match type_ {
        AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_FIELD => {
            if let Ok(v) = variable_part_ptr.downcast::<IDENT_AST>() {
                variable_part.field = Some(v);
            }
        }
        AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_INDEX => {
            if let Ok(v) = variable_part_ptr.downcast::<LOGICAL_OR_EXPR_AST>() {
                variable_part.index = Some(v);
            }
        }
    }
    Box::new(variable_part)
}

pub fn dump_variable_part_ast_to_file_inner(f: &mut FILE, ast: &VARIABLE_PART_AST, spaces_num: usize) {
    match ast.type_ {
        AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_FIELD => {
            PUT_SPACES(); let _ = writeln!(f, "<field>");
            INC_SPACES_NUM();
            if let Some(ref id) = ast.field {
                dump_ident_ast_to_file_inner(f, id.as_ref(), spaces_num);
            }
            DEC_SPACES_NUM();
            PUT_SPACES(); let _ = writeln!(f, "</field>");
        }
        AST_VARIABLE_PART_TYPE::AST_VARIABLE_PART_TYPE_INDEX => {
            PUT_SPACES(); let _ = writeln!(f, "<index>");
            INC_SPACES_NUM();
            if let Some(ref idx) = ast.index {
                dump_logical_or_expr_ast_to_file_inner(f, idx.as_ref(), spaces_num);
            }
            DEC_SPACES_NUM();
            PUT_SPACES(); let _ = writeln!(f, "</index>");
        }
    }
}

pub fn dump_variable_part_ast_to_file(f: &mut FILE, ast: &VARIABLE_PART_AST) {
    dump_variable_part_ast_to_file_inner(f, ast, 0);
}

pub fn variable_part_ast_free(_ast: Box<VARIABLE_PART_AST>) {
    // rely on drop
}

pub fn create_variable_ast(ident: Box<IDENT_AST>, parts: Vec<Box<VARIABLE_PART_AST>>, parts_len: usize, line: usize, pos: usize) -> Box<VARIABLE_AST> {
    Box::new(VARIABLE_AST {
        ident,
        parts,
        parts_len,
        line,
        pos,
    })
}

pub fn dump_variable_ast_to_file_inner(f: &mut FILE, ast: &VARIABLE_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<variable line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_ident_ast_to_file_inner(f, ast.ident.as_ref(), spaces_num);
    for i in 0..ast.parts_len {
        if let Some(p) = ast.parts.get(i) {
            dump_variable_part_ast_to_file_inner(f, p.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</variable>");
}

pub fn dump_variable_ast_to_file(f: &mut FILE, ast: &VARIABLE_AST) {
    dump_variable_ast_to_file_inner(f, ast, 0);
}

pub fn variable_ast_free(_ast: Box<VARIABLE_AST>) {
    // rely on drop
}

pub fn create_assignment_expr_ast(assignment_ptr: Box<dyn std::any::Any>, type_: AST_ASSIGNMENT_EXPR_TYPE) -> Box<ASSIGNMENT_EXPR_AST> {
    let mut assignment_expr = ASSIGNMENT_EXPR_AST::default();
    assignment_expr.type_ = type_;
    match type_ {
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL => {
            if let Ok(v) = assignment_ptr.downcast::<OBJECT_LITERAL_AST>() {
                assignment_expr.object_literal = Some(v);
            }
        }
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL => {
            if let Ok(v) = assignment_ptr.downcast::<ARRAY_LITERAL_AST>() {
                assignment_expr.array_literal = Some(v);
            }
        }
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR => {
            if let Ok(v) = assignment_ptr.downcast::<LOGICAL_OR_EXPR_AST>() {
                assignment_expr.logical_or_expr = Some(v);
            }
        }
    }
    Box::new(assignment_expr)
}

pub fn dump_assignment_expr_ast_to_file_inner(f: &mut FILE, ast: &ASSIGNMENT_EXPR_AST, spaces_num: usize) {
    match ast.type_ {
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL => {
            if let Some(ref obj) = ast.object_literal {
                dump_object_literal_ast_to_file_inner(f, obj.as_ref(), spaces_num);
            }
        }
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL => {
            if let Some(ref arr) = ast.array_literal {
                dump_array_literal_ast_to_file_inner(f, arr.as_ref(), spaces_num);
            }
        }
        AST_ASSIGNMENT_EXPR_TYPE::AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR => {
            if let Some(ref lor) = ast.logical_or_expr {
                dump_logical_or_expr_ast_to_file_inner(f, lor.as_ref(), spaces_num);
            }
        }
    }
}

pub fn dump_assignment_expr_ast_to_file(f: &mut FILE, ast: &ASSIGNMENT_EXPR_AST) {
    dump_assignment_expr_ast_to_file_inner(f, ast, 0);
}

pub fn assignment_expr_ast_free(_ast: Box<ASSIGNMENT_EXPR_AST>) {
    // rely on drop
}

pub fn create_object_literal_ast(properties: Vec<Box<PROPERTY_AST>>, properties_len: usize, line: usize, pos: usize) -> Box<OBJECT_LITERAL_AST> {
    Box::new(OBJECT_LITERAL_AST {
        properties,
        properties_len,
        line,
        pos,
    })
}

pub fn dump_object_literal_ast_to_file_inner(f: &mut FILE, ast: &OBJECT_LITERAL_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<object_literal line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    for i in 0..ast.properties_len {
        if let Some(p) = ast.properties.get(i) {
            dump_property_ast_to_file_inner(f, p.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</object_literal>");
}

pub fn dump_object_literal_ast_to_file(f: &mut FILE, ast: &OBJECT_LITERAL_AST) {
    dump_object_literal_ast_to_file_inner(f, ast, 0);
}

pub fn object_literal_ast_free(_ast: Box<OBJECT_LITERAL_AST>) {
    // rely on drop
}

pub fn create_property_ast(key: Box<IDENT_AST>, value: Box<ASSIGNMENT_EXPR_AST>, line: usize, pos: usize) -> Box<PROPERTY_AST> {
    Box::new(PROPERTY_AST { key, value, line, pos })
}

pub fn dump_property_ast_to_file_inner(f: &mut FILE, ast: &PROPERTY_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<property line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "<key>");
    INC_SPACES_NUM();
    dump_ident_ast_to_file_inner(f, ast.key.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</key>");

    PUT_SPACES(); let _ = writeln!(f, "<value>");
    INC_SPACES_NUM();
    dump_assignment_expr_ast_to_file_inner(f, ast.value.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</value>");
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</property>");
}

pub fn dump_property_ast_to_file(f: &mut FILE, ast: &PROPERTY_AST) {
    dump_property_ast_to_file_inner(f, ast, 0);
}

pub fn property_ast_free(_ast: Box<PROPERTY_AST>) {
    // rely on drop
}

pub fn create_array_literal_ast(args_list: Option<Box<ARGS_LIST_AST>>, line: usize, pos: usize) -> Box<ARRAY_LITERAL_AST> {
    Box::new(ARRAY_LITERAL_AST { args_list, line, pos })
}

pub fn dump_array_literal_ast_to_file_inner(f: &mut FILE, ast: &ARRAY_LITERAL_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<array_literal line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    if let Some(ref args) = ast.args_list {
        dump_args_list_ast_to_file_inner(f, args.as_ref(), spaces_num);
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</array_literal>");
}

pub fn dump_array_literal_ast_to_file(f: &mut FILE, ast: &ARRAY_LITERAL_AST) {
    dump_array_literal_ast_to_file_inner(f, ast, 0);
}

pub fn array_literal_ast_free(_ast: Box<ARRAY_LITERAL_AST>) {
    // rely on drop
}

pub fn create_logical_or_expr_ast(and_exprs: Vec<Box<LOGICAL_AND_EXPR_AST>>, and_exprs_len: usize, line: usize, pos: usize) -> Box<LOGICAL_OR_EXPR_AST> {
    Box::new(LOGICAL_OR_EXPR_AST {
        and_exprs,
        and_exprs_len,
        line,
        pos,
    })
}

pub fn dump_logical_or_expr_ast_to_file_inner(f: &mut FILE, ast: &LOGICAL_OR_EXPR_AST, spaces_num: usize) {
    if ast.and_exprs_len == 1 {
        if let Some(e) = ast.and_exprs.get(0) {
            dump_logical_and_expr_ast_to_file_inner(f, e.as_ref(), spaces_num);
        }
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<logical_or_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        if let Some(first) = ast.and_exprs.get(0) {
            dump_logical_and_expr_ast_to_file_inner(f, first.as_ref(), spaces_num);
        }
        for i in 1..ast.and_exprs_len {
            PUT_SPACES(); let _ = writeln!(f, "<op>OR</op>");
            if let Some(e) = ast.and_exprs.get(i) {
                dump_logical_and_expr_ast_to_file_inner(f, e.as_ref(), spaces_num);
            }
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</logical_or_expr>");
    }
}

pub fn dump_logical_or_expr_ast_to_file(f: &mut FILE, ast: &LOGICAL_OR_EXPR_AST) {
    dump_logical_or_expr_ast_to_file_inner(f, ast, 0);
}

pub fn logical_or_expr_ast_free(_ast: Box<LOGICAL_OR_EXPR_AST>) {
    // rely on drop
}

pub fn create_logical_and_expr_ast(eq_exprs: Vec<Box<EQ_EXPR_AST>>, eq_exprs_len: usize, line: usize, pos: usize) -> Box<LOGICAL_AND_EXPR_AST> {
    Box::new(LOGICAL_AND_EXPR_AST {
        eq_exprs,
        eq_exprs_len,
        line,
        pos,
    })
}

pub fn dump_logical_and_expr_ast_to_file_inner(f: &mut FILE, ast: &LOGICAL_AND_EXPR_AST, spaces_num: usize) {
    if ast.eq_exprs_len == 1 {
        if let Some(e) = ast.eq_exprs.get(0) {
            dump_eq_expr_ast_to_file_inner(f, e.as_ref(), spaces_num);
        }
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<logical_and_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        if let Some(first) = ast.eq_exprs.get(0) {
            dump_eq_expr_ast_to_file_inner(f, first.as_ref(), spaces_num);
        }
        for i in 1..ast.eq_exprs_len {
            PUT_SPACES(); let _ = writeln!(f, "<op>AND</op>");
            if let Some(e) = ast.eq_exprs.get(i) {
                dump_eq_expr_ast_to_file_inner(f, e.as_ref(), spaces_num);
            }
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</logical_and_expr>");
    }
}

pub fn dump_logical_and_expr_ast_to_file(f: &mut FILE, ast: &LOGICAL_AND_EXPR_AST) {
    dump_logical_and_expr_ast_to_file_inner(f, ast, 0);
}

pub fn logical_and_expr_ast_free(_ast: Box<LOGICAL_AND_EXPR_AST>) {
    // rely on drop
}

pub fn create_eq_expr_ast(left: Box<RELATIONAL_EXPR_AST>, eq_op: AST_EQ_OP, right: Option<Box<RELATIONAL_EXPR_AST>>, line: usize, pos: usize) -> Box<EQ_EXPR_AST> {
    Box::new(EQ_EXPR_AST { left, eq_op, right, line, pos })
}

pub fn dump_eq_expr_ast_to_file_inner(f: &mut FILE, ast: &EQ_EXPR_AST, spaces_num: usize) {
    if ast.right.is_none() {
        dump_relational_expr_ast_to_file_inner(f, ast.left.as_ref(), spaces_num);
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<eq_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        dump_relational_expr_ast_to_file_inner(f, ast.left.as_ref(), spaces_num);
        if ast.eq_op as u32 == AST_EQ_OP::AST_EQ_OP_EQEQ as u32 {
            PUT_SPACES(); let _ = writeln!(f, "<op>EQEQ</op>");
        } else if ast.eq_op as u32 == AST_EQ_OP::AST_EQ_OP_NEQ as u32 {
            PUT_SPACES(); let _ = writeln!(f, "<op>NEQ</op>");
        }
        if let Some(ref r) = ast.right {
            dump_relational_expr_ast_to_file_inner(f, r.as_ref(), spaces_num);
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</eq_expr>");
    }
}

pub fn dump_eq_expr_ast_to_file(f: &mut FILE, ast: &EQ_EXPR_AST) {
    dump_eq_expr_ast_to_file_inner(f, ast, 0);
}

pub fn eq_expr_ast_free(_ast: Box<EQ_EXPR_AST>) {
    // rely on drop
}

pub fn create_relational_expr_ast(left: Box<ADDITIVE_EXPR_AST>, rel_op: AST_REL_OP, right: Option<Box<ADDITIVE_EXPR_AST>>, line: usize, pos: usize) -> Box<RELATIONAL_EXPR_AST> {
    Box::new(RELATIONAL_EXPR_AST { left, rel_op, right, line, pos })
}

pub fn dump_relational_expr_ast_to_file_inner(f: &mut FILE, ast: &RELATIONAL_EXPR_AST, spaces_num: usize) {
    if ast.right.is_none() {
        dump_additive_expr_ast_to_file_inner(f, ast.left.as_ref(), spaces_num);
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<relational_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        dump_additive_expr_ast_to_file_inner(f, ast.left.as_ref(), spaces_num);
        match ast.rel_op {
            AST_REL_OP::AST_REL_OP_LT => { PUT_SPACES(); let _ = writeln!(f, "<op>LT</op>"); }
            AST_REL_OP::AST_REL_OP_GT => { PUT_SPACES(); let _ = writeln!(f, "<op>GT</op>"); }
            AST_REL_OP::AST_REL_OP_LE => { PUT_SPACES(); let _ = writeln!(f, "<op>LE</op>"); }
            AST_REL_OP::AST_REL_OP_GE => { PUT_SPACES(); let _ = writeln!(f, "<op>GE</op>"); }
        }
        if let Some(ref r) = ast.right {
            dump_additive_expr_ast_to_file_inner(f, r.as_ref(), spaces_num);
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</relational_expr>");
    }
}

pub fn dump_relational_expr_ast_to_file(f: &mut FILE, ast: &RELATIONAL_EXPR_AST) {
    dump_relational_expr_ast_to_file_inner(f, ast, 0);
}

pub fn relational_expr_ast_free(_ast: Box<RELATIONAL_EXPR_AST>) {
    // rely on drop
}

pub fn create_additive_expr_ast(muls: Vec<Box<MULTIPLICATIVE_EXPR_AST>>, ops: Vec<AST_ADDITIVE_OP>, muls_len: usize, line: usize, pos: usize) -> Box<ADDITIVE_EXPR_AST> {
    Box::new(ADDITIVE_EXPR_AST { muls, ops, muls_len, line, pos })
}

pub fn dump_additive_expr_ast_to_file_inner(f: &mut FILE, ast: &ADDITIVE_EXPR_AST, spaces_num: usize) {
    if ast.muls_len == 1 {
        if let Some(m) = ast.muls.get(0) {
            dump_multiplicative_expr_ast_to_file_inner(f, m.as_ref(), spaces_num);
        }
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<additive_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        if let Some(first) = ast.muls.get(0) {
            dump_multiplicative_expr_ast_to_file_inner(f, first.as_ref(), spaces_num);
        }
        for i in 1..ast.muls_len {
            if let Some(op) = ast.ops.get(i - 1) {
                if *op as u32 == AST_ADDITIVE_OP::AST_ADDITIVE_OP_PLUS as u32 {
                    PUT_SPACES(); let _ = writeln!(f, "<op>PLUS</op>");
                } else if *op as u32 == AST_ADDITIVE_OP::AST_ADDITIVE_OP_MINUS as u32 {
                    PUT_SPACES(); let _ = writeln!(f, "<op>MINUS</op>");
                }
            }
            if let Some(m) = ast.muls.get(i) {
                dump_multiplicative_expr_ast_to_file_inner(f, m.as_ref(), spaces_num);
            }
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</additive_expr>");
    }
}

pub fn dump_additive_expr_ast_to_file(f: &mut FILE, ast: &ADDITIVE_EXPR_AST) {
    dump_additive_expr_ast_to_file_inner(f, ast, 0);
}

pub fn additive_expr_ast_free(_ast: Box<ADDITIVE_EXPR_AST>) {
    // rely on drop
}

pub fn create_multiplicative_expr_ast(lues: Vec<Box<LEFT_UNARY_EXPR_AST>>, ops: Vec<AST_MULTIPLICATIVE_OP>, lues_len: usize, line: usize, pos: usize) -> Box<MULTIPLICATIVE_EXPR_AST> {
    Box::new(MULTIPLICATIVE_EXPR_AST { lues, ops, lues_len, line, pos })
}

pub fn dump_multiplicative_expr_ast_to_file_inner(f: &mut FILE, ast: &MULTIPLICATIVE_EXPR_AST, spaces_num: usize) {
    if ast.lues_len == 1 {
        if let Some(l) = ast.lues.get(0) {
            dump_left_unary_expr_ast_to_file_inner(f, l.as_ref(), spaces_num);
        }
    } else {
        PUT_SPACES(); let _ = writeln!(f, "<multiplicative_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        if let Some(first) = ast.lues.get(0) {
            dump_left_unary_expr_ast_to_file_inner(f, first.as_ref(), spaces_num);
        }
        for i in 1..ast.lues_len {
            if let Some(op) = ast.ops.get(i - 1) {
                if *op as u32 == AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_MUL as u32 {
                    PUT_SPACES(); let _ = writeln!(f, "<op>MUL</op>");
                } else if *op as u32 == AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_DIV as u32 {
                    PUT_SPACES(); let _ = writeln!(f, "<op>DIV</op>");
                } else if *op as u32 == AST_MULTIPLICATIVE_OP::AST_MULTIPLICATIVE_OP_MOD as u32 {
                    PUT_SPACES(); let _ = writeln!(f, "<op>MOD</op>");
                }
            }
            if let Some(l) = ast.lues.get(i) {
                dump_left_unary_expr_ast_to_file_inner(f, l.as_ref(), spaces_num);
            }
        }
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</multiplicative_expr>");
    }
}

pub fn dump_multiplicative_expr_ast_to_file(f: &mut FILE, ast: &MULTIPLICATIVE_EXPR_AST) {
    dump_multiplicative_expr_ast_to_file_inner(f, ast, 0);
}

pub fn multiplicative_expr_ast_free(_ast: Box<MULTIPLICATIVE_EXPR_AST>) {
    // rely on drop
}

pub fn create_left_unary_expr_ast(op: AST_LEFT_UNARY_OP, expr: Box<PRIMARY_EXPR_AST>, line: usize, pos: usize) -> Box<LEFT_UNARY_EXPR_AST> {
    Box::new(LEFT_UNARY_EXPR_AST { op, expr, line, pos })
}

pub fn dump_left_unary_expr_ast_to_file_inner(f: &mut FILE, ast: &LEFT_UNARY_EXPR_AST, spaces_num: usize) {
    if ast.op != AST_LEFT_UNARY_OP::AST_LEFT_UNARY_OP_PLUS {
        PUT_SPACES(); let _ = writeln!(f, "<left_unary_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        INC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "<op>UNARY_MINUS</op>");
        dump_primary_expr_ast_to_file_inner(f, ast.expr.as_ref(), spaces_num);
        DEC_SPACES_NUM();
        PUT_SPACES(); let _ = writeln!(f, "</left_unary_expr>");
    } else {
        dump_primary_expr_ast_to_file_inner(f, ast.expr.as_ref(), spaces_num);
    }
}

pub fn dump_left_unary_expr_ast_to_file(f: &mut FILE, ast: &LEFT_UNARY_EXPR_AST) {
    dump_left_unary_expr_ast_to_file_inner(f, ast, 0);
}

pub fn left_unary_expr_ast_free(_ast: Box<LEFT_UNARY_EXPR_AST>) {
    // rely on drop
}

pub fn create_primary_expr_ast(primary_expr_ptr: Box<dyn std::any::Any>, type_: AST_PRIMARY_EXPR_TYPE) -> Box<PRIMARY_EXPR_AST> {
    let mut primary_expr = PRIMARY_EXPR_AST::default();
    primary_expr.type_ = type_;
    match type_ {
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY => {
            if let Ok(v) = primary_expr_ptr.downcast::<HAS_PROPERTY_EXPR_AST>() {
                primary_expr.has_property_expr = Some(v);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LEN => {
            if let Ok(v) = primary_expr_ptr.downcast::<LEN_EXPR_AST>() {
                primary_expr.len_expr = Some(v);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL => {
            if let Ok(v) = primary_expr_ptr.downcast::<FUNCTION_CALL_AST>() {
                primary_expr.function_call = Some(v);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_VARIABLE => {
            if let Ok(v) = primary_expr_ptr.downcast::<VARIABLE_AST>() {
                primary_expr.var_name = Some(v);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_NUMBER => {
            if let Ok(v) = primary_expr_ptr.downcast::<NUMBER_AST>() {
                primary_expr.number = Some(v);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR => {
            if let Ok(v) = primary_expr_ptr.downcast::<LOGICAL_OR_EXPR_AST>() {
                primary_expr.logical_or_expr = Some(v);
            }
        }
    }
    Box::new(primary_expr)
}

pub fn dump_primary_expr_ast_to_file_inner(f: &mut FILE, ast: &PRIMARY_EXPR_AST, spaces_num: usize) {
    match ast.type_ {
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY => {
            if let Some(ref h) = ast.has_property_expr {
                dump_has_property_expr_ast_to_file_inner(f, h.as_ref(), spaces_num);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LEN => {
            if let Some(ref l) = ast.len_expr {
                dump_len_expr_ast_to_file_inner(f, l.as_ref(), spaces_num);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL => {
            if let Some(ref fc) = ast.function_call {
                dump_function_call_ast_to_file_inner(f, fc.as_ref(), spaces_num);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_VARIABLE => {
            if let Some(ref v) = ast.var_name {
                dump_variable_ast_to_file_inner(f, v.as_ref(), spaces_num);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_NUMBER => {
            if let Some(ref n) = ast.number {
                dump_number_ast_to_file_inner(f, n.as_ref(), spaces_num);
            }
        }
        AST_PRIMARY_EXPR_TYPE::AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR => {
            if let Some(ref lo) = ast.logical_or_expr {
                dump_logical_or_expr_ast_to_file_inner(f, lo.as_ref(), spaces_num);
            }
        }
    }
}

pub fn dump_primary_expr_ast_to_file(f: &mut FILE, ast: &PRIMARY_EXPR_AST) {
    dump_primary_expr_ast_to_file_inner(f, ast, 0);
}

pub fn primary_expr_ast_free(_ast: Box<PRIMARY_EXPR_AST>) {
    // rely on drop
}

pub fn create_has_property_expr_ast(obj: Box<VARIABLE_AST>, ident: Box<IDENT_AST>, line: usize, pos: usize) -> Box<HAS_PROPERTY_EXPR_AST> {
    Box::new(HAS_PROPERTY_EXPR_AST { obj, ident, line, pos })
}

pub fn dump_has_property_expr_ast_to_file_inner(f: &mut FILE, ast: &HAS_PROPERTY_EXPR_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<has_property_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_variable_ast_to_file_inner(f, ast.obj.as_ref(), spaces_num);
    dump_ident_ast_to_file_inner(f, ast.ident.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</has_property_expr>");
}

pub fn dump_has_property_expr_ast_to_file(f: &mut FILE, ast: &HAS_PROPERTY_EXPR_AST) {
    dump_has_property_expr_ast_to_file_inner(f, ast, 0);
}

pub fn has_property_expr_ast_free(_ast: Box<HAS_PROPERTY_EXPR_AST>) {
    // rely on drop
}

pub fn create_len_expr_ast(arr: Box<VARIABLE_AST>, line: usize, pos: usize) -> Box<LEN_EXPR_AST> {
    Box::new(LEN_EXPR_AST { arr, line, pos })
}

pub fn dump_len_expr_ast_to_file_inner(f: &mut FILE, ast: &LEN_EXPR_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<len_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_variable_ast_to_file_inner(f, ast.arr.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</len_expr>");
}

pub fn dump_len_expr_ast_to_file(f: &mut FILE, ast: &LEN_EXPR_AST) {
    dump_len_expr_ast_to_file_inner(f, ast, 0);
}

pub fn len_expr_ast_free(_ast: Box<LEN_EXPR_AST>) {
    // rely on drop
}

pub fn create_function_call_ast(function_name: Box<IDENT_AST>, args_list: Option<Box<ARGS_LIST_AST>>, line: usize, pos: usize) -> Box<FUNCTION_CALL_AST> {
    Box::new(FUNCTION_CALL_AST { function_name, args_list, line, pos })
}

pub fn dump_function_call_ast_to_file_inner(f: &mut FILE, ast: &FUNCTION_CALL_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<function_call line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "<function_name line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    dump_ident_ast_to_file_inner(f, ast.function_name.as_ref(), spaces_num);
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</function_name>");
    if let Some(ref args) = ast.args_list {
        dump_args_list_ast_to_file_inner(f, args.as_ref(), spaces_num);
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</function_call>");
}

pub fn dump_function_call_ast_to_file(f: &mut FILE, ast: &FUNCTION_CALL_AST) {
    dump_function_call_ast_to_file_inner(f, ast, 0);
}

pub fn function_call_ast_free(_ast: Box<FUNCTION_CALL_AST>) {
    // rely on drop
}

pub fn create_args_list_ast(assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>>, assignment_exprs_len: usize, line: usize, pos: usize) -> Box<ARGS_LIST_AST> {
    Box::new(ARGS_LIST_AST { assignment_exprs, assignment_exprs_len, line, pos })
}

pub fn dump_args_list_ast_to_file_inner(f: &mut FILE, ast: &ARGS_LIST_AST, spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<args_list line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    INC_SPACES_NUM();
    for i in 0..ast.assignment_exprs_len {
        if let Some(a) = ast.assignment_exprs.get(i) {
            dump_assignment_expr_ast_to_file_inner(f, a.as_ref(), spaces_num);
        }
    }
    DEC_SPACES_NUM();
    PUT_SPACES(); let _ = writeln!(f, "</args_list>");
}

pub fn dump_args_list_ast_to_file(f: &mut FILE, ast: &ARGS_LIST_AST) {
    dump_args_list_ast_to_file_inner(f, ast, 0);
}

pub fn args_list_ast_free(_ast: Box<ARGS_LIST_AST>) {
    // rely on drop
}

pub fn create_ident_ast(ident: &str, line: usize, pos: usize) -> Box<IDENT_AST> {
    let mut s = String::new();
    s.push_str(ident);
    Box::new(IDENT_AST { ident: s, line, pos })
}

pub fn dump_ident_ast_to_file_inner(f: &mut FILE, ast: &IDENT_AST, _spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<ident line=\"{}\" pos=\"{}\">{}</ident>", ast.line, ast.pos, ast.ident);
}

pub fn dump_ident_ast_to_file(f: &mut FILE, ast: &IDENT_AST) {
    dump_ident_ast_to_file_inner(f, ast, 0);
}

pub fn ident_ast_free(_ast: Box<IDENT_AST>) {
    // rely on drop
}

pub fn create_number_ast(number: i64, line: usize, pos: usize) -> Box<NUMBER_AST> {
    Box::new(NUMBER_AST { number, line, pos })
}

pub fn dump_number_ast_to_file_inner(f: &mut FILE, ast: &NUMBER_AST, _spaces_num: usize) {
    PUT_SPACES(); let _ = writeln!(f, "<number line=\"{}\" pos=\"{}\">{}</number>", ast.line, ast.pos, ast.number);
}

pub fn dump_number_ast_to_file(f: &mut FILE, ast: &NUMBER_AST) {
    dump_number_ast_to_file_inner(f, ast, 0);
}

pub fn number_ast_free(_ast: Box<NUMBER_AST>) {
    // rely on drop
}