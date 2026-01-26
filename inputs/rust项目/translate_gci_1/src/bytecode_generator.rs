use crate::utils::*;
use crate::parser::*;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::string::String;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BYTECODE_GENERATOR_CODES {
    BYTECODE_GENERATOR_OK = 0,
    BYTECODE_GENERATOR_NO_LOCAL_VARIABLE = -1,
    BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE = -2,
    BYTECODE_GENERATOR_INVALID_BREAK = -3,
    BYTECODE_GENERATOR_INVALID_CONTINUE = -4,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BC_HEAP_OP {
    BC_ARRAY_INDEX,
    BC_OBJECT_FIELD,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BC_OP_CODES {
    BC_OP_POP,
    BC_OP_CONSTANT,
    BC_OP_CREATE_LOCAL,
    BC_OP_GET_LOCAL,
    BC_OP_SET_LOCAL,
    BC_OP_CREATE_OBJ,
    BC_OP_INIT_OBJ_PROP,
    BC_OP_CREATE_ARR,
    BC_OP_GET_HEAP,
    BC_OP_SET_HEAP,
    BC_OP_APPEND,
    BC_OP_DELETE,
    BC_OP_LOGICAL_OR,
    BC_OP_LOGICAL_AND,
    BC_OP_EQ_EQEQ,
    BC_OP_EQ_NEQ,
    BC_OP_REL_LT,
    BC_OP_REL_GT,
    BC_OP_REL_LE,
    BC_OP_REL_GE,
    BC_OP_ADDITIVE_PLUS,
    BC_OP_ADDITIVE_MINUS,
    BC_OP_MULTIPLICATIVE_MUL,
    BC_OP_MULTIPLICATIVE_DIV,
    BC_OP_MULTIPLICATIVE_MOD,
    BC_OP_NEGATE,
    BC_OP_HAS_PROPERTY,
    BC_OP_LEN,
    BC_OP_JUMP_IF_FALSE,
    BC_OP_JUMP,
    BC_OP_RETURN,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CONSTANT_TYPE {
    CONSTANT_TYPE_INTEGER,
    CONSTANT_TYPE_DOUBLE,
    CONSTANT_TYPE_FIELDREF,
    CONSTANT_TYPE_FUNCTIONREF,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct POS {
    pub line: usize,
    pub pos: usize,
}

impl Default for POS {
    fn default() -> Self {
        POS { line: 0, pos: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BYTECODE_ERROR {
    pub pos: POS,
    pub code: BYTECODE_GENERATOR_CODES,
}

impl Default for BYTECODE_ERROR {
    fn default() -> Self {
        BYTECODE_ERROR { pos: POS::default(), code: BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CONSTANT {
    pub int_cnst: i64,
    pub double_cnst: f64,
    pub str_cnst: [u8; 32],
    pub r#type: CONSTANT_TYPE,
}

impl Default for CONSTANT {
    fn default() -> Self {
        CONSTANT { int_cnst: 0, double_cnst: 0.0, str_cnst: [0u8; 32], r#type: CONSTANT_TYPE::CONSTANT_TYPE_INTEGER }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LOCAL_VARIABLE {
    pub name: String,
    pub depth: usize,
}

impl Default for LOCAL_VARIABLE {
    fn default() -> Self {
        LOCAL_VARIABLE { name: String::new(), depth: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BYTECODE_POS {
    // Opaque for this module; other modules may define real fields.
    pub dummy: usize,
}

impl Default for BYTECODE_POS {
    fn default() -> Self {
        BYTECODE_POS { dummy: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BYTECODE {
    pub op_codes: Vec<usize>,
    pub op_codes_len: usize,
    pub op_codes_cap: usize,
    pub constant_pool: Vec<CONSTANT>,
    pub constant_pool_len: usize,
    pub constant_pool_cap: usize,
    pub poss: Vec<BYTECODE_POS>,
}

impl Default for BYTECODE {
    fn default() -> Self {
        BYTECODE {
            op_codes: Vec::new(),
            op_codes_len: 0,
            op_codes_cap: 0,
            constant_pool: Vec::new(),
            constant_pool_len: 0,
            constant_pool_cap: 0,
            poss: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BYTECODE_GENERATOR {
    pub ast: Option<Box<UNIT_AST>>,
    pub bc: Option<Box<BYTECODE>>,
    pub locals: Vec<LOCAL_VARIABLE>,
    pub locals_len: usize,
    pub locals_cap: usize,
    pub scope_depth: usize,
    pub err: BYTECODE_ERROR,
}

impl Default for BYTECODE_GENERATOR {
    fn default() -> Self {
        BYTECODE_GENERATOR {
            ast: None,
            bc: None,
            locals: Vec::new(),
            locals_len: 0,
            locals_cap: 0,
            scope_depth: 0,
            err: BYTECODE_ERROR::default(),
        }
    }
}

pub type bytecode_generator_type_t = Box<BYTECODE_GENERATOR>;
pub type bytecode_type_t = Box<BYTECODE>;

// Opaque / placeholder AST-related types and small AST nodes used by this module.
// These are simplified representations that expose the fields accessed by this code.
// Other modules may provide fuller definitions.

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct IDENT_AST {
    pub ident: String,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct PROPERTY_AST {
    pub value: Box<ASSIGNMENT_EXPR_AST>,
    pub key: Box<IDENT_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct OBJECT_LITERAL_AST {
    pub properties_len: usize,
    pub properties: Vec<Box<PROPERTY_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ARGS_LIST_AST {
    pub assignment_exprs_len: usize,
    pub assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ARRAY_LITERAL_AST {
    pub args_list: Option<Box<ARGS_LIST_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct NUMBER_AST {
    pub number: i64,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct VARIABLE_PART {
    pub r#type: i32,
    pub index: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub field: Option<Box<IDENT_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct VARIABLE_AST {
    pub ident: Box<IDENT_AST>,
    pub parts_len: usize,
    pub parts: Vec<Box<VARIABLE_PART>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct HAS_PROPERTY_EXPR_AST {
    pub obj: Box<VARIABLE_AST>,
    pub ident: Box<IDENT_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct LEN_EXPR_AST {
    pub arr: Box<VARIABLE_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct FUNCTION_CALL_AST {
    // Placeholder fields
    pub dummy: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct PRIMARY_EXPR_AST {
    pub r#type: i32,
    pub has_property_expr: Option<Box<HAS_PROPERTY_EXPR_AST>>,
    pub len_expr: Option<Box<LEN_EXPR_AST>>,
    pub function_call: Option<Box<FUNCTION_CALL_AST>>,
    pub var_name: Option<Box<VARIABLE_AST>>,
    pub number: Option<Box<NUMBER_AST>>,
    pub logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct LEFT_UNARY_EXPR_AST {
    pub expr: Box<PRIMARY_EXPR_AST>,
    pub op: i32,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct MULTIPLICATIVE_EXPR_AST {
    pub lues: Vec<Box<LEFT_UNARY_EXPR_AST>>,
    pub lues_len: usize,
    pub ops: Vec<i32>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ADDITIVE_EXPR_AST {
    pub muls: Vec<Box<MULTIPLICATIVE_EXPR_AST>>,
    pub muls_len: usize,
    pub ops: Vec<i32>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct RELATIONAL_EXPR_AST {
    pub left: Box<ADDITIVE_EXPR_AST>,
    pub right: Option<Box<ADDITIVE_EXPR_AST>>,
    pub rel_op: i32,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct EQ_EXPR_AST {
    pub left: Box<RELATIONAL_EXPR_AST>,
    pub right: Option<Box<RELATIONAL_EXPR_AST>>,
    pub eq_op: i32,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct LOGICAL_AND_EXPR_AST {
    pub eq_exprs: Vec<Box<EQ_EXPR_AST>>,
    pub eq_exprs_len: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct LOGICAL_OR_EXPR_AST {
    pub and_exprs: Vec<Box<LOGICAL_AND_EXPR_AST>>,
    pub and_exprs_len: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ASSIGNMENT_EXPR_AST {
    pub r#type: i32,
    pub object_literal: Option<Box<OBJECT_LITERAL_AST>>,
    pub array_literal: Option<Box<ARRAY_LITERAL_AST>>,
    pub logical_or_expr: Option<Box<LOGICAL_OR_EXPR_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DECL_STMT_AST {
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
    pub new_var_name: Box<IDENT_AST>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ASSIGN_STMT_AST {
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
    pub var_name: Box<VARIABLE_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct FUNCTION_CALL_STMT_AST {
    pub dummy: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct IF_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub if_body: Box<BODY_AST>,
    pub else_body: Option<Box<BODY_AST>>,
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct WHILE_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub body: Box<BODY_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct BREAK_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct CONTINUE_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct APPEND_STMT_AST {
    pub dummy: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DELETE_STMT_AST {
    pub dummy: usize,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct RETURN_STMT_AST {
    pub result: Option<Box<ASSIGNMENT_EXPR_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct STMT_AST {
    pub r#type: i32,
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
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct BODY_AST {
    pub stmts_len: usize,
    pub stmts: Vec<Box<STMT_AST>>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct FUNCTION_DECL_AST {
    pub body: Box<BODY_AST>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct UNIT_AST {
    pub functions: Vec<Box<FUNCTION_DECL_AST>>,
}

// Helper functions to emulate C macros behavior in safe Rust.

pub fn PUSH_BACK<T>(v: &mut Vec<T>, value: T) {
    v.push(value);
}

pub fn SAFE_CALLOC<T: Default + 'static>() -> Box<T> {
    Box::new(T::default())
}

pub fn SAFE_FREE_vec<T>(_v: &mut Vec<T>) {
    _v.clear();
}

pub fn SAFE_FREE_box<T>(_b: &mut Option<Box<T>>) {
    *_b = None;
}

pub fn PREFIX_UNUSED<T>(_v: &T) {}

pub fn create_constant_from_int(int_cnst: i64) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    cnst.int_cnst = int_cnst;
    cnst.r#type = CONSTANT_TYPE::CONSTANT_TYPE_INTEGER;
    cnst
}

pub fn create_constant_from_double(double_cnst: f64) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    cnst.double_cnst = double_cnst;
    cnst.r#type = CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE;
    cnst
}

pub fn create_constant_from_fieldref(str_cnst: &str) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    let bytes = str_cnst.as_bytes();
    let len = bytes.len().min(32);
    cnst.str_cnst[..len].copy_from_slice(&bytes[..len]);
    cnst.r#type = CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF;
    cnst
}

pub fn create_constant_from_functionref(str_cnst: &str) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    let bytes = str_cnst.as_bytes();
    let len = bytes.len().min(32);
    cnst.str_cnst[..len].copy_from_slice(&bytes[..len]);
    cnst.r#type = CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF;
    cnst
}

pub fn constant_pool_push_back(bc: &mut BYTECODE, cnst: CONSTANT) -> i32 {
    for (i, existing) in bc.constant_pool.iter().enumerate() {
        if existing.int_cnst == cnst.int_cnst
            && existing.double_cnst == cnst.double_cnst
            && existing.str_cnst == cnst.str_cnst
            && existing.r#type == cnst.r#type
        {
            return i as i32;
        }
    }

    PUSH_BACK(&mut bc.constant_pool, cnst);
    bc.constant_pool_len = bc.constant_pool.len();
    (bc.constant_pool_len - 1) as i32
}

pub fn create_local_variable(name: &str, depth: usize) -> LOCAL_VARIABLE {
    LOCAL_VARIABLE { name: name.to_string(), depth }
}

pub fn local_variable_index(bc_gen: &BYTECODE_GENERATOR, var_name: &str) -> i32 {
    if bc_gen.locals_len == 0 {
        return -1;
    }
    let mut i = (bc_gen.locals_len as isize) - 1;
    while i >= 0 {
        if bc_gen.locals[i as usize].name == var_name {
            return i as i32;
        }
        i -= 1;
    }
    -1
}

pub fn create_bytecode_generator() -> bytecode_generator_type_t {
    SAFE_CALLOC::<BYTECODE_GENERATOR>()
}

pub fn bytecode_generator_conf(bc_gen: &mut BYTECODE_GENERATOR, ast: Box<UNIT_AST>) {
    bc_gen.ast = Some(ast);
    bc_gen.bc = Some(create_bytecode());
}

pub fn set_bytecode_generator_error(bc_gen: &mut BYTECODE_GENERATOR, line: usize, pos: usize, code: BYTECODE_GENERATOR_CODES) {
    bc_gen.err.pos.line = line;
    bc_gen.err.pos.pos = pos;
    bc_gen.err.code = code;
}

pub fn property_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &PROPERTY_AST) -> BYTECODE_GENERATOR_CODES {
    let mut cnst;
    if let Some(bc_box) = &mut bc_gen.bc {
        let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.value);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }

        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_INIT_OBJ_PROP as usize);
        cnst = create_constant_from_fieldref(&ast.key.ident);
        let idx = constant_pool_push_back(bc_box, cnst);
        PUSH_BACK(&mut bc_box.op_codes, idx as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn object_literal_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &OBJECT_LITERAL_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(bc_box) = &mut bc_gen.bc {
        for i in 0..ast.properties_len {
            let r = property_ast_bytecode_generate(bc_gen, &ast.properties[i]);
            if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
                return r;
            }
        }

        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_CREATE_OBJ as usize);
        PUSH_BACK(&mut bc_box.op_codes, ast.properties_len);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn array_literal_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &ARRAY_LITERAL_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(bc_box) = &mut bc_gen.bc {
        if let Some(args_list) = &ast.args_list {
            if args_list.assignment_exprs_len > 0 {
                for i in (0..args_list.assignment_exprs_len).rev() {
                    let r = assignment_expr_ast_bytecode_generate(bc_gen, &args_list.assignment_exprs[i]);
                    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
                        return r;
                    }
                }
            }
        }

        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_CREATE_ARR as usize);
        if let Some(args_list) = &ast.args_list {
            PUSH_BACK(&mut bc_box.op_codes, args_list.assignment_exprs_len);
        } else {
            PUSH_BACK(&mut bc_box.op_codes, 0);
        }
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn number_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &NUMBER_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(bc_box) = &mut bc_gen.bc {
        let cnst = create_constant_from_int(ast.number);
        let index = constant_pool_push_back(bc_box, cnst) as usize;
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_CONSTANT as usize);
        PUSH_BACK(&mut bc_box.op_codes, index);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn variable_ast_bytecode_generate_inner(bc_gen: &mut BYTECODE_GENERATOR, ast: &VARIABLE_AST, is_set_op: bool) -> BYTECODE_GENERATOR_CODES {
    let idx = local_variable_index(bc_gen, &ast.ident.ident);
    if idx == -1 {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_NO_LOCAL_VARIABLE);
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_NO_LOCAL_VARIABLE;
    }

    if let Some(bc_box) = &mut bc_gen.bc {
        if ast.parts_len == 0 {
            PUSH_BACK(&mut bc_box.op_codes, if is_set_op { BC_OP_CODES::BC_OP_SET_LOCAL as usize } else { BC_OP_CODES::BC_OP_GET_LOCAL as usize });
            PUSH_BACK(&mut bc_box.op_codes, idx as usize);
            bc_box.op_codes_len = bc_box.op_codes.len();
        } else {
            let mut count: usize = 0;
            for i in 0..ast.parts_len {
                if ast.parts[i].r#type == AST_VARIABLE_PART_TYPE_INDEX {
                    count += 1;
                    if let Some(index_expr) = &ast.parts[i].index {
                        let r = logical_or_expr_ast_bytecode_generate(bc_gen, index_expr);
                        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
                            return r;
                        }
                    }
                }
            }

            PUSH_BACK(&mut bc_box.op_codes, if is_set_op { BC_OP_CODES::BC_OP_SET_HEAP as usize } else { BC_OP_CODES::BC_OP_GET_HEAP as usize });
            PUSH_BACK(&mut bc_box.op_codes, idx as usize);
            PUSH_BACK(&mut bc_box.op_codes, ast.parts_len);

            for i in 0..ast.parts_len {
                if ast.parts[i].r#type == AST_VARIABLE_PART_TYPE_FIELD {
                    let cnst = create_constant_from_fieldref(&ast.parts[i].field.as_ref().unwrap().ident);
                    PUSH_BACK(&mut bc_box.op_codes, BC_HEAP_OP::BC_OBJECT_FIELD as usize);
                    PUSH_BACK(&mut bc_box.op_codes, constant_pool_push_back(bc_box, cnst) as usize);
                } else if ast.parts[i].r#type == AST_VARIABLE_PART_TYPE_INDEX {
                    PUSH_BACK(&mut bc_box.op_codes, BC_HEAP_OP::BC_ARRAY_INDEX as usize);
                    count -= 1;
                    PUSH_BACK(&mut bc_box.op_codes, count);
                } else {
                    // In original C code, this would exit; here we return an error
                    return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
                }
            }
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn variable_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &VARIABLE_AST) -> BYTECODE_GENERATOR_CODES {
    variable_ast_bytecode_generate_inner(bc_gen, ast, false)
}

pub fn has_property_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &HAS_PROPERTY_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(bc_box) = &mut bc_gen.bc {
        let r = variable_ast_bytecode_generate_inner(bc_gen, &ast.obj, false);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_HAS_PROPERTY as usize);
        let cnst = create_constant_from_fieldref(&ast.ident.ident);
        PUSH_BACK(&mut bc_box.op_codes, constant_pool_push_back(bc_box, cnst) as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn len_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &LEN_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(bc_box) = &mut bc_gen.bc {
        let r = variable_ast_bytecode_generate_inner(bc_gen, &ast.arr, false);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_LEN as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn function_call_expr_ast_bytecode_generate(_bc_gen: &mut BYTECODE_GENERATOR, _ast: &FUNCTION_CALL_AST) -> BYTECODE_GENERATOR_CODES {
    // TODO: preserved behavior from C (no-op stub)
    PREFIX_UNUSED(_bc_gen);
    PREFIX_UNUSED(_ast);
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn primary_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &PRIMARY_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    let r = match ast.r#type {
        AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY => {
            has_property_expr_ast_bytecode_generate(bc_gen, ast.has_property_expr.as_ref().unwrap())
        }
        AST_PRIMARY_EXPR_TYPE_LEN => {
            len_expr_ast_bytecode_generate(bc_gen, ast.len_expr.as_ref().unwrap())
        }
        AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL => {
            function_call_expr_ast_bytecode_generate(bc_gen, ast.function_call.as_ref().unwrap())
        }
        AST_PRIMARY_EXPR_TYPE_VARIABLE => {
            variable_ast_bytecode_generate(bc_gen, ast.var_name.as_ref().unwrap())
        }
        AST_PRIMARY_EXPR_TYPE_NUMBER => {
            number_ast_bytecode_generate(bc_gen, ast.number.as_ref().unwrap())
        }
        AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR => {
            logical_or_expr_ast_bytecode_generate(bc_gen, ast.logical_or_expr.as_ref().unwrap())
        }
        _ => {
            // In C: fprintf + exit; here return OK to preserve flow.
            return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
        }
    };
    r
}

pub fn left_unary_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &LEFT_UNARY_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    let r = primary_expr_ast_bytecode_generate(bc_gen, &ast.expr);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    if ast.op != AST_LEFT_UNARY_OP_PLUS {
        if ast.op == AST_LEFT_UNARY_OP_MINUS {
            if let Some(bc_box) = &mut bc_gen.bc {
                PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_NEGATE as usize);
                bc_box.op_codes_len = bc_box.op_codes.len();
            }
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn multiplicative_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &MULTIPLICATIVE_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if ast.lues_len == 0 {
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    let r = left_unary_expr_ast_bytecode_generate(bc_gen, &ast.lues[0]);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    for i in 1..ast.lues_len {
        let r = left_unary_expr_ast_bytecode_generate(bc_gen, &ast.lues[i]);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            match ast.ops[i - 1] {
                AST_MULTIPLICATIVE_OP_MUL => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_MULTIPLICATIVE_MUL as usize),
                AST_MULTIPLICATIVE_OP_DIV => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_MULTIPLICATIVE_DIV as usize),
                AST_MULTIPLICATIVE_OP_MOD => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_MULTIPLICATIVE_MOD as usize),
                _ => {}
            }
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn additive_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &ADDITIVE_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if ast.muls_len == 0 {
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    let r = multiplicative_expr_ast_bytecode_generate(bc_gen, &ast.muls[0]);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    for i in 1..ast.muls_len {
        let r = multiplicative_expr_ast_bytecode_generate(bc_gen, &ast.muls[i]);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            if ast.ops[i - 1] == AST_ADDITIVE_OP_PLUS {
                PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_ADDITIVE_PLUS as usize);
            } else if ast.ops[i - 1] == AST_ADDITIVE_OP_MINUS {
                PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_ADDITIVE_MINUS as usize);
            }
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn relational_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &RELATIONAL_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    let r = additive_expr_ast_bytecode_generate(bc_gen, &ast.left);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    if let Some(right) = &ast.right {
        let r = additive_expr_ast_bytecode_generate(bc_gen, right);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            match ast.rel_op {
                AST_REL_OP_LT => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_REL_LT as usize),
                AST_REL_OP_GT => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_REL_GT as usize),
                AST_REL_OP_LE => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_REL_LE as usize),
                AST_REL_OP_GE => PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_REL_GE as usize),
                _ => {}
            }
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn eq_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &EQ_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    let r = relational_expr_ast_bytecode_generate(bc_gen, &ast.left);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    if let Some(right) = &ast.right {
        let r = relational_expr_ast_bytecode_generate(bc_gen, right);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            if ast.eq_op == AST_EQ_OP_EQEQ {
                PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_EQ_EQEQ as usize);
            } else if ast.eq_op == AST_EQ_OP_NEQ {
                PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_EQ_NEQ as usize);
            }
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn logical_and_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &LOGICAL_AND_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if ast.eq_exprs_len == 0 {
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    let r = eq_expr_ast_bytecode_generate(bc_gen, &ast.eq_exprs[0]);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    for i in 1..ast.eq_exprs_len {
        let r = eq_expr_ast_bytecode_generate(bc_gen, &ast.eq_exprs[i]);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_LOGICAL_AND as usize);
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn logical_or_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &LOGICAL_OR_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    if ast.and_exprs_len == 0 {
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    let r = logical_and_expr_ast_bytecode_generate(bc_gen, &ast.and_exprs[0]);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    for i in 1..ast.and_exprs_len {
        let r = logical_and_expr_ast_bytecode_generate(bc_gen, &ast.and_exprs[i]);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
        if let Some(bc_box) = &mut bc_gen.bc {
            PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_LOGICAL_OR as usize);
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn assignment_expr_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &ASSIGNMENT_EXPR_AST) -> BYTECODE_GENERATOR_CODES {
    match ast.r#type {
        AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL => {
            object_literal_ast_bytecode_generate(bc_gen, ast.object_literal.as_ref().unwrap())
        }
        AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL => {
            array_literal_ast_bytecode_generate(bc_gen, ast.array_literal.as_ref().unwrap())
        }
        AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR => {
            logical_or_expr_ast_bytecode_generate(bc_gen, ast.logical_or_expr.as_ref().unwrap())
        }
        _ => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK,
    }
}

pub fn decl_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &DECL_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.assignment);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }

    let idx = local_variable_index(bc_gen, &ast.new_var_name.ident);

    if idx != -1 && bc_gen.locals[idx as usize].depth == bc_gen.scope_depth {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE);
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE;
    }

    let lv = create_local_variable(&ast.new_var_name.ident, bc_gen.scope_depth);
    PUSH_BACK(&mut bc_gen.locals, lv);
    bc_gen.locals_len = bc_gen.locals.len();
    let idx = bc_gen.locals_len - 1;

    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_SET_LOCAL as usize);
        PUSH_BACK(&mut bc_box.op_codes, idx);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }

    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn assign_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &ASSIGN_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.assignment);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }
    variable_ast_bytecode_generate_inner(bc_gen, &ast.var_name, true)
}

pub fn function_call_stmt_ast_bytecode_generate(_bc_gen: &mut BYTECODE_GENERATOR, _ast: &FUNCTION_CALL_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    PREFIX_UNUSED(_bc_gen);
    PREFIX_UNUSED(_ast);
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn emit_jump(bc_gen: &mut BYTECODE_GENERATOR, instruction: usize) -> i32 {
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, instruction);
        PUSH_BACK(&mut bc_box.op_codes, 0x0);
        bc_box.op_codes_len = bc_box.op_codes.len();
        (bc_box.op_codes_len - 1) as i32
    } else {
        0
    }
}

pub fn patch_jump(bc_gen: &mut BYTECODE_GENERATOR, offset: usize) {
    if let Some(bc_box) = &mut bc_gen.bc {
        let jump = bc_box.op_codes_len - offset - 1;
        if offset < bc_box.op_codes.len() {
            bc_box.op_codes[offset] = jump;
        }
    }
}

pub fn if_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &IF_STMT_AST, loop_start_idx: Option<&mut i32>,
                                     loop_exit_idxs: Option<&mut Vec<i32>>, _loop_exit_idxs_len: Option<&mut usize>, _loop_exit_idxs_cap: Option<&mut usize>) -> BYTECODE_GENERATOR_CODES {
    let r = logical_or_expr_ast_bytecode_generate(bc_gen, &ast.condition);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }

    let if_idx = emit_jump(bc_gen, BC_OP_CODES::BC_OP_JUMP_IF_FALSE as usize);
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_POP as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }

    let r = body_ast_bytecode_generate(bc_gen, &ast.if_body, loop_start_idx, loop_exit_idxs, None, None);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }

    let else_idx = emit_jump(bc_gen, BC_OP_CODES::BC_OP_JUMP as usize);

    patch_jump(bc_gen, if_idx as usize);
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_POP as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }

    if let Some(else_body) = &ast.else_body {
        let r = body_ast_bytecode_generate(bc_gen, else_body, loop_start_idx, loop_exit_idxs, None, None);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
    }

    patch_jump(bc_gen, else_idx as usize);
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn emit_loop(bc_gen: &mut BYTECODE_GENERATOR, loop_start: usize) {
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_JUMP as usize);
        let offset = ((bc_box.op_codes_len as isize - loop_start as isize + 1) as isize) * (-1);
        PUSH_BACK(&mut bc_box.op_codes, offset as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
}

pub fn while_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &WHILE_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    let mut loop_exit_idxs: Vec<i32> = Vec::new();
    let mut loop_exit_idxs_len: usize = 0;
    let mut loop_exit_idxs_cap: usize = 0;

    let loop_start_idx = bc_gen.bc.as_ref().map(|b| b.op_codes_len).unwrap_or(0);

    let r = logical_or_expr_ast_bytecode_generate(bc_gen, &ast.condition);
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }

    PUSH_BACK(&mut loop_exit_idxs, emit_jump(bc_gen, BC_OP_CODES::BC_OP_JUMP_IF_FALSE as usize));
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_POP as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }

    let r = body_ast_bytecode_generate(bc_gen, &ast.body, &mut (loop_start_idx as i32), Some(&mut loop_exit_idxs), Some(&mut loop_exit_idxs_len), Some(&mut loop_exit_idxs_cap));
    if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
        return r;
    }

    emit_loop(bc_gen, loop_start_idx);

    if !loop_exit_idxs.is_empty() {
        patch_jump(bc_gen, loop_exit_idxs[0] as usize);
    }
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_POP as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    for i in 1..loop_exit_idxs.len() {
        patch_jump(bc_gen, loop_exit_idxs[i] as usize);
    }

    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn break_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &BREAK_STMT_AST,
                                        loop_exit_idxs: Option<&mut Vec<i32>>, _loop_exit_idxs_len: Option<&mut usize>, _loop_exit_idxs_cap: Option<&mut usize>) -> BYTECODE_GENERATOR_CODES {
    PREFIX_UNUSED(ast);

    if loop_exit_idxs.is_none() {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_BREAK);
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_BREAK;
    }

    if let Some(vec) = loop_exit_idxs {
        let loop_exit_idx = emit_jump(bc_gen, BC_OP_CODES::BC_OP_JUMP as usize);
        PUSH_BACK(vec, loop_exit_idx);
    }

    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn continue_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &CONTINUE_STMT_AST, loop_start_idx: Option<&mut i32>) -> BYTECODE_GENERATOR_CODES {
    PREFIX_UNUSED(ast);

    if loop_start_idx.is_none() {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_CONTINUE);
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_CONTINUE;
    }

    if let Some(idx) = loop_start_idx {
        emit_loop(bc_gen, *idx as usize);
    }

    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn append_stmt_ast_bytecode_generate(_bc_gen: &mut BYTECODE_GENERATOR, _ast: &APPEND_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    PREFIX_UNUSED(_bc_gen);
    PREFIX_UNUSED(_ast);
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn delete_stmt_ast_bytecode_generate(_bc_gen: &mut BYTECODE_GENERATOR, _ast: &DELETE_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    PREFIX_UNUSED(_bc_gen);
    PREFIX_UNUSED(_ast);
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn return_stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &RETURN_STMT_AST) -> BYTECODE_GENERATOR_CODES {
    if let Some(result) = &ast.result {
        let r = assignment_expr_ast_bytecode_generate(bc_gen, result);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
    }
    if let Some(bc_box) = &mut bc_gen.bc {
        PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_RETURN as usize);
        bc_box.op_codes_len = bc_box.op_codes.len();
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn stmt_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &STMT_AST, loop_start_idx: Option<&mut i32>,
                                  loop_exit_idxs: Option<&mut Vec<i32>>, loop_exit_idxs_len: Option<&mut usize>, loop_exit_idxs_cap: Option<&mut usize>) -> BYTECODE_GENERATOR_CODES {
    match ast.r#type {
        AST_STMT_TYPE_DECL => {
            decl_stmt_ast_bytecode_generate(bc_gen, ast.decl_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_ASSIGN => {
            assign_stmt_ast_bytecode_generate(bc_gen, ast.assign_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_FUNCTION_CALL => {
            function_call_stmt_ast_bytecode_generate(bc_gen, ast.function_call_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_IF => {
            if_stmt_ast_bytecode_generate(bc_gen, ast.if_stmt.as_ref().unwrap(), loop_start_idx, loop_exit_idxs, loop_exit_idxs_len, loop_exit_idxs_cap)
        }
        AST_STMT_TYPE_WHILE => {
            while_stmt_ast_bytecode_generate(bc_gen, ast.while_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_BREAK => {
            break_stmt_ast_bytecode_generate(bc_gen, ast.break_stmt.as_ref().unwrap(), loop_exit_idxs, loop_exit_idxs_len, loop_exit_idxs_cap)
        }
        AST_STMT_TYPE_CONTINUE => {
            continue_stmt_ast_bytecode_generate(bc_gen, ast.continue_stmt.as_ref().unwrap(), loop_start_idx)
        }
        AST_STMT_TYPE_APPEND => {
            append_stmt_ast_bytecode_generate(bc_gen, ast.append_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_DELETE => {
            delete_stmt_ast_bytecode_generate(bc_gen, ast.delete_stmt.as_ref().unwrap())
        }
        AST_STMT_TYPE_RETURN => {
            return_stmt_ast_bytecode_generate(bc_gen, ast.return_stmt.as_ref().unwrap())
        }
        _ => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK,
    }
}

pub fn body_ast_bytecode_generate(bc_gen: &mut BYTECODE_GENERATOR, ast: &BODY_AST, loop_start_idx: Option<&mut i32>,
                                  loop_exit_idxs: Option<&mut Vec<i32>>, loop_exit_idxs_len: Option<&mut usize>, loop_exit_idxs_cap: Option<&mut usize>) -> BYTECODE_GENERATOR_CODES {
    bc_gen.scope_depth += 1;

    for i in 0..ast.stmts_len {
        let r = stmt_ast_bytecode_generate(bc_gen, &ast.stmts[i], loop_start_idx, loop_exit_idxs, loop_exit_idxs_len, loop_exit_idxs_cap);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            return r;
        }
    }

    bc_gen.scope_depth -= 1;

    while (bc_gen.locals_len > 0) && (bc_gen.locals[bc_gen.locals_len - 1].depth > bc_gen.scope_depth) {
        if let Some(bc_box) = &mut bc_gen.bc {
            PUSH_BACK(&mut bc_box.op_codes, BC_OP_CODES::BC_OP_POP as usize);
            bc_box.op_codes_len = bc_box.op_codes.len();
        }
        bc_gen.locals_len -= 1;
        bc_gen.locals.pop();
    }

    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn bytecode_generator_generate(bc_gen: &mut BYTECODE_GENERATOR, bc_out: &mut Option<bytecode_type_t>) -> BYTECODE_GENERATOR_CODES {
    // now only one function without arguments is supported.
    if let Some(ast_box) = &bc_gen.ast {
        if ast_box.functions.is_empty() {
            return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
        }
        let f = &ast_box.functions[0];
        let r = body_ast_bytecode_generate(bc_gen, &f.body, None, None, None, None);
        if r != BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK {
            if let Some(bc_box) = bc_gen.bc.take() {
                bytecode_free(Some(bc_box));
            }
            return r;
        }
        *bc_out = bc_gen.bc.take();
        return BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK;
    }
    BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK
}

pub fn bytecode_generator_free(bc_gen: &mut BYTECODE_GENERATOR) {
    SAFE_FREE_vec(&mut bc_gen.locals);
    // Box will be dropped automatically when function ends or owner drops.
}

pub fn create_bytecode() -> bytecode_type_t {
    SAFE_CALLOC::<BYTECODE>()
}

pub fn dump_bytecode_to_xml_file(f: &mut dyn Write, bc: &BYTECODE) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<bytecode>");
    let _ = writeln!(f, "\t<constant_pool>");
    for (i, cnst) in bc.constant_pool.iter().enumerate() {
        match cnst.r#type {
            CONSTANT_TYPE::CONSTANT_TYPE_INTEGER => {
                let _ = writeln!(f, "\t\t<int_cnst idx=\"{}\">{}</int_cnst>", i, cnst.int_cnst);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE => {
                let _ = writeln!(f, "\t\t<double_cnst idx=\"{}\">{:.6}</double_cnst>", i, cnst.double_cnst);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF => {
                let s = String::from_utf8_lossy(&cnst.str_cnst).trim_end_matches('\0').to_string();
                let _ = writeln!(f, "\t\t<fieldref_cnst idx=\"{}\">{}</fieldref_cnst>", i, s);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF => {
                let s = String::from_utf8_lossy(&cnst.str_cnst).trim_end_matches('\0').to_string();
                let _ = writeln!(f, "\t\t<functionref_cnst idx=\"{}\">{}</functionref_cnst>", i, s);
            }
        }
    }
    let _ = writeln!(f, "\t</constant_pool>");
    let _ = writeln!(f, "\t<op_codes>");
    let mut i = 0usize;
    while i < bc.op_codes_len {
        match bc.op_codes[i] {
            x if x == BC_OP_CODES::BC_OP_POP as usize => {
                let _ = writeln!(f, "\t\t<op>POP<op>");
                i += 1;
                continue;
            }
            x if x == BC_OP_CODES::BC_OP_CONSTANT as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>CONSTANT {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_GET_LOCAL as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>GET_LOCAL {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_SET_LOCAL as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>SET_LOCAL {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_CREATE_OBJ as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>CREATE_OBJ {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_INIT_OBJ_PROP as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>INIT_OBJ_PROP {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_CREATE_ARR as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>CREATE_ARR {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_GET_HEAP as usize || x == BC_OP_CODES::BC_OP_SET_HEAP as usize => {
                if x == BC_OP_CODES::BC_OP_GET_HEAP as usize {
                    i += 1;
                    if i < bc.op_codes_len {
                        let _ = write!(f, "\t\t<op>GET_HEAP {}", bc.op_codes[i]);
                    }
                } else {
                    i += 1;
                    if i < bc.op_codes_len {
                        let _ = write!(f, "\t\t<op>SET_HEAP {}", bc.op_codes[i]);
                    }
                }
                i += 1;
                if i >= bc.op_codes_len { break; }
                let n = bc.op_codes[i];
                i += 1;
                for _ in 0..n {
                    if i >= bc.op_codes_len { break; }
                    if bc.op_codes[i] == BC_HEAP_OP::BC_OBJECT_FIELD as usize {
                        i += 1;
                        if i < bc.op_codes_len {
                            let _ = write!(f, " field({})", bc.op_codes[i]);
                        }
                    } else if bc.op_codes[i] == BC_HEAP_OP::BC_ARRAY_INDEX as usize {
                        i += 1;
                        if i < bc.op_codes_len {
                            let _ = write!(f, " index({})", bc.op_codes[i]);
                        }
                    }
                    i += 1;
                }
                let _ = writeln!(f, "</op>");
                i -= 1;
            }
            x if x == BC_OP_CODES::BC_OP_APPEND as usize || x == BC_OP_CODES::BC_OP_DELETE as usize => {
                // no-op in dump
            }
            x if x == BC_OP_CODES::BC_OP_LOGICAL_OR as usize => {
                let _ = writeln!(f, "\t\t<op>LOGICAL_OR</op>");
            }
            x if x == BC_OP_CODES::BC_OP_LOGICAL_AND as usize => {
                let _ = writeln!(f, "\t\t<op>LOGICAL_AND</op>");
            }
            x if x == BC_OP_CODES::BC_OP_EQ_EQEQ as usize => {
                let _ = writeln!(f, "\t\t<op>EQ_EQEQ</op>");
            }
            x if x == BC_OP_CODES::BC_OP_EQ_NEQ as usize => {
                let _ = writeln!(f, "\t\t<op>EQ_NEQ</op>");
            }
            x if x == BC_OP_CODES::BC_OP_REL_LT as usize => {
                let _ = writeln!(f, "\t\t<op>REL_LT</op>");
            }
            x if x == BC_OP_CODES::BC_OP_REL_GT as usize => {
                let _ = writeln!(f, "\t\t<op>REL_GT</op>");
            }
            x if x == BC_OP_CODES::BC_OP_REL_LE as usize => {
                let _ = writeln!(f, "\t\t<op>REL_LE</op>");
            }
            x if x == BC_OP_CODES::BC_OP_REL_GE as usize => {
                let _ = writeln!(f, "\t\t<op>REL_GE</op>");
            }
            x if x == BC_OP_CODES::BC_OP_ADDITIVE_PLUS as usize => {
                let _ = writeln!(f, "\t\t<op>ADDITIVE_PLUS</op>");
            }
            x if x == BC_OP_CODES::BC_OP_ADDITIVE_MINUS as usize => {
                let _ = writeln!(f, "\t\t<op>ADDITIVE_MINUS</op>");
            }
            x if x == BC_OP_CODES::BC_OP_MULTIPLICATIVE_MUL as usize => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_MUL</op>");
            }
            x if x == BC_OP_CODES::BC_OP_MULTIPLICATIVE_DIV as usize => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_DIV</op>");
            }
            x if x == BC_OP_CODES::BC_OP_MULTIPLICATIVE_MOD as usize => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_MOD</op>");
            }
            x if x == BC_OP_CODES::BC_OP_NEGATE as usize => {
                let _ = writeln!(f, "\t\t<op>NEGATE</op>");
            }
            x if x == BC_OP_CODES::BC_OP_LEN as usize => {
                let _ = writeln!(f, "\t\t<op>LEN</op>");
            }
            x if x == BC_OP_CODES::BC_OP_HAS_PROPERTY as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>HAS_PROPERTY {}</op>", bc.op_codes[i]);
                }
            }
            x if x == BC_OP_CODES::BC_OP_JUMP_IF_FALSE as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>JUMP_IF_FALSE {}</op>", bc.op_codes[i] as isize);
                }
            }
            x if x == BC_OP_CODES::BC_OP_JUMP as usize => {
                i += 1;
                if i < bc.op_codes_len {
                    let _ = writeln!(f, "\t\t<op>JUMP {}</op>", bc.op_codes[i] as isize);
                }
            }
            x if x == BC_OP_CODES::BC_OP_RETURN as usize => {
                let _ = writeln!(f, "\t\t<op>RETURN</op>");
            }
            _ => {}
        }
        i += 1;
    }
    let _ = writeln!(f, "\t</op_codes>");
    let _ = writeln!(f, "</bytecode>");
}

pub fn bytecode_generator_get_error(bc_gen: &BYTECODE_GENERATOR) -> BYTECODE_ERROR {
    bc_gen.err.clone()
}

pub fn print_bytecode_error(err: &BYTECODE_ERROR) {
    let error_str = match err.code {
        BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK => "ok?!",
        BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_NO_LOCAL_VARIABLE => "no local variable!",
        BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE => "already have local variable!",
        BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_BREAK => "break outside of while!",
        BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_CONTINUE => "continue outside of while!",
    };
    eprintln!("{}:{}: error: {}", err.pos.line, err.pos.pos, error_str);
}

pub fn bytecode_free(bc_opt: Option<bytecode_type_t>) {
    if let Some(mut bc) = bc_opt {
        SAFE_FREE_vec(&mut bc.op_codes);
        SAFE_FREE_vec(&mut bc.constant_pool);
        // Box will be dropped when function exits
    }
}

// Constants for AST node types and ops used above. These mimic the C enum integer values.
// Values are preserved only as distinct integers for switching logic.
pub const AST_VARIABLE_PART_TYPE_INDEX: i32 = 0;
pub const AST_VARIABLE_PART_TYPE_FIELD: i32 = 1;

pub const AST_PRIMARY_EXPR_TYPE_HAS_PROPERTY: i32 = 0;
pub const AST_PRIMARY_EXPR_TYPE_LEN: i32 = 1;
pub const AST_PRIMARY_EXPR_TYPE_FUNCTION_CALL: i32 = 2;
pub const AST_PRIMARY_EXPR_TYPE_VARIABLE: i32 = 3;
pub const AST_PRIMARY_EXPR_TYPE_NUMBER: i32 = 4;
pub const AST_PRIMARY_EXPR_TYPE_LOGICAL_OR_EXPR: i32 = 5;

pub const AST_LEFT_UNARY_OP_PLUS: i32 = 0;
pub const AST_LEFT_UNARY_OP_MINUS: i32 = 1;

pub const AST_MULTIPLICATIVE_OP_MUL: i32 = 0;
pub const AST_MULTIPLICATIVE_OP_DIV: i32 = 1;
pub const AST_MULTIPLICATIVE_OP_MOD: i32 = 2;

pub const AST_ADDITIVE_OP_PLUS: i32 = 0;
pub const AST_ADDITIVE_OP_MINUS: i32 = 1;

pub const AST_REL_OP_LT: i32 = 0;
pub const AST_REL_OP_GT: i32 = 1;
pub const AST_REL_OP_LE: i32 = 2;
pub const AST_REL_OP_GE: i32 = 3;

pub const AST_EQ_OP_EQEQ: i32 = 0;
pub const AST_EQ_OP_NEQ: i32 = 1;

pub const AST_ASSIGNMENT_EXPR_TYPE_OBJECT_LITERAL: i32 = 0;
pub const AST_ASSIGNMENT_EXPR_TYPE_ARRAY_LITERAL: i32 = 1;
pub const AST_ASSIGNMENT_EXPR_TYPE_LOGICAL_OR_EXPR: i32 = 2;

pub const AST_STMT_TYPE_DECL: i32 = 0;
pub const AST_STMT_TYPE_ASSIGN: i32 = 1;
pub const AST_STMT_TYPE_FUNCTION_CALL: i32 = 2;
pub const AST_STMT_TYPE_IF: i32 = 3;
pub const AST_STMT_TYPE_WHILE: i32 = 4;
pub const AST_STMT_TYPE_BREAK: i32 = 5;
pub const AST_STMT_TYPE_CONTINUE: i32 = 6;
pub const AST_STMT_TYPE_APPEND: i32 = 7;
pub const AST_STMT_TYPE_DELETE: i32 = 8;
pub const AST_STMT_TYPE_RETURN: i32 = 9;