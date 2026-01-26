use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub enum BYTECODE_GENERATOR_CODES {
    BYTECODE_GENERATOR_OK = 0,
    BYTECODE_GENERATOR_NO_LOCAL_VARIABLE = -1,
    BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE = -2,
    BYTECODE_GENERATOR_INVALID_BREAK = -3,
    BYTECODE_GENERATOR_INVALID_CONTINUE = -4,
}

impl From<i32> for BYTECODE_GENERATOR_CODES {
    fn from(code: i32) -> Self {
        match code {
            0 => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK,
            -1 => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_NO_LOCAL_VARIABLE,
            -2 => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_ALREADY_HAVE_LOCAL_VARIABLE,
            -3 => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_BREAK,
            -4 => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_INVALID_CONTINUE,
            _ => BYTECODE_GENERATOR_CODES::BYTECODE_GENERATOR_OK,
        }
    }
}

pub struct BYTECODE_ERROR {
    pub pos: POS,
    pub code: i32,
}

impl Default for BYTECODE_ERROR {
    fn default() -> Self {
        Self {
            pos: POS::default(),
            code: 0,
        }
    }
}

pub struct POS {
    pub line: usize,
    pub pos: usize,
}

impl Default for POS {
    fn default() -> Self {
        Self {
            line: 0,
            pos: 0,
        }
    }
}

pub enum BC_HEAP_OP {
    BC_ARRAY_INDEX,
    BC_OBJECT_FIELD,
}

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

pub enum CONSTANT_TYPE {
    CONSTANT_TYPE_INTEGER,
    CONSTANT_TYPE_DOUBLE,
    CONSTANT_TYPE_FIELDREF,
    CONSTANT_TYPE_FUNCTIONREF,
}

pub struct CONSTANT {
    pub int_cnst: i64,
    pub double_cnst: f64,
    pub str_cnst: [u8; 32],
    pub type_: CONSTANT_TYPE,
}

impl Default for CONSTANT {
    fn default() -> Self {
        Self {
            int_cnst: 0,
            double_cnst: 0.0,
            str_cnst: [0; 32],
            type_: CONSTANT_TYPE::CONSTANT_TYPE_INTEGER,
        }
    }
}

impl Clone for CONSTANT {
    fn clone(&self) -> Self {
        Self {
            int_cnst: self.int_cnst,
            double_cnst: self.double_cnst,
            str_cnst: self.str_cnst,
            type_: match self.type_ {
                CONSTANT_TYPE::CONSTANT_TYPE_INTEGER => CONSTANT_TYPE::CONSTANT_TYPE_INTEGER,
                CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE => CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE,
                CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF => CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF,
                CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF => CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF,
            },
        }
    }
}

impl PartialEq for CONSTANT {
    fn eq(&self, other: &Self) -> bool {
        self.int_cnst == other.int_cnst
            && (self.double_cnst - other.double_cnst).abs() < f64::EPSILON
            && self.str_cnst == other.str_cnst
    }
}

pub struct BYTECODE {
    pub op_codes: Vec<usize>,
    pub constant_pool: Vec<CONSTANT>,
    pub poss: Vec<BYTECODE_POS>,
}

impl Default for BYTECODE {
    fn default() -> Self {
        Self {
            op_codes: Vec::new(),
            constant_pool: Vec::new(),
            poss: Vec::new(),
        }
    }
}

pub struct BYTECODE_POS {
    pub line: usize,
    pub pos: usize,
}

pub type bytecode_type_t = Box<BYTECODE>;

pub struct BYTECODE_GENERATOR {
    pub ast: Option<&'static UNIT_AST>,
    pub bc: Option<Box<BYTECODE>>,
    pub locals: Vec<LOCAL_VARIABLE>,
    pub scope_depth: usize,
    pub err: BYTECODE_ERROR,
}

impl Default for BYTECODE_GENERATOR {
    fn default() -> Self {
        Self {
            ast: None,
            bc: None,
            locals: Vec::new(),
            scope_depth: 0,
            err: BYTECODE_ERROR::default(),
        }
    }
}

pub type bytecode_generator_type_t = Box<BYTECODE_GENERATOR>;

pub struct LOCAL_VARIABLE {
    pub name: [u8; 32],
    pub depth: usize,
}

impl Default for LOCAL_VARIABLE {
    fn default() -> Self {
        Self {
            name: [0; 32],
            depth: 0,
        }
    }
}

pub struct UNIT_AST {
    pub functions: Vec<Box<FUNCTION_DECL_AST>>,
}

pub struct FUNCTION_DECL_AST {
    pub body: Box<BODY_AST>,
}

pub struct BODY_AST {
    pub stmts: Vec<Box<STMT_AST>>,
}

pub enum STMT_AST {
    Decl(Box<DECL_STMT_AST>),
    Assign(Box<ASSIGN_STMT_AST>),
    FunctionCall(Box<FUNCTION_CALL_STMT_AST>),
    If(Box<IF_STMT_AST>),
    While(Box<WHILE_STMT_AST>),
    Break(Box<BREAK_STMT_AST>),
    Continue(Box<CONTINUE_STMT_AST>),
    Append(Box<APPEND_STMT_AST>),
    Delete(Box<DELETE_STMT_AST>),
    Return(Box<RETURN_STMT_AST>),
}

pub struct DECL_STMT_AST {
    pub line: usize,
    pub pos: usize,
    pub new_var_name: Box<IDENTIFIER>,
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
}

pub struct ASSIGN_STMT_AST {
    pub assignment: Box<ASSIGNMENT_EXPR_AST>,
}

pub struct FUNCTION_CALL_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct IF_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub if_body: Box<BODY_AST>,
    pub else_body: Option<Box<BODY_AST>>,
}

pub struct WHILE_STMT_AST {
    pub condition: Box<LOGICAL_OR_EXPR_AST>,
    pub body: Box<BODY_AST>,
}

pub struct BREAK_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct CONTINUE_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct APPEND_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct DELETE_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct RETURN_STMT_AST {
    pub result: Option<Box<ASSIGNMENT_EXPR_AST>>,
}

pub enum ASSIGNMENT_EXPR_AST {
    ObjectLiteral(Box<OBJECT_LITERAL_AST>),
    ArrayLiteral(Box<ARRAY_LITERAL_AST>),
    LogicalOrExpr(Box<LOGICAL_OR_EXPR_AST>),
}

pub struct OBJECT_LITERAL_AST {
    pub properties: Vec<Box<PROPERTY_AST>>,
}

pub struct PROPERTY_AST {
    pub key: Box<IDENTIFIER>,
    pub value: Box<ASSIGNMENT_EXPR_AST>,
}

pub struct ARRAY_LITERAL_AST {
    pub args_list: Option<Box<ASSIGNMENT_EXPRS_LIST>>,
}

pub struct ASSIGNMENT_EXPRS_LIST {
    pub assignment_exprs: Vec<Box<ASSIGNMENT_EXPR_AST>>,
}

pub struct LOGICAL_OR_EXPR_AST {
    pub and_exprs: Vec<Box<LOGICAL_AND_EXPR_AST>>,
}

pub struct LOGICAL_AND_EXPR_AST {
    pub eq_exprs: Vec<Box<EQ_EXPR_AST>>,
}

pub struct EQ_EXPR_AST {
    pub left: Box<RELATIONAL_EXPR_AST>,
    pub right: Option<Box<RELATIONAL_EXPR_AST>>,
    pub eq_op: i32,
}

pub struct RELATIONAL_EXPR_AST {
    pub left: Box<ADDITIVE_EXPR_AST>,
    pub right: Option<Box<ADDITIVE_EXPR_AST>>,
    pub rel_op: i32,
}

pub struct ADDITIVE_EXPR_AST {
    pub muls: Vec<Box<MULTIPLICATIVE_EXPR_AST>>,
    pub ops: Vec<i32>,
}

pub struct MULTIPLICATIVE_EXPR_AST {
    pub lues: Vec<Box<LEFT_UNARY_EXPR_AST>>,
    pub ops: Vec<i32>,
}

pub struct LEFT_UNARY_EXPR_AST {
    pub expr: Box<PRIMARY_EXPR_AST>,
    pub op: i32,
}

pub enum PRIMARY_EXPR_AST {
    HasProperty(Box<HAS_PROPERTY_EXPR_AST>),
    Len(Box<LEN_EXPR_AST>),
    FunctionCall(Box<FUNCTION_CALL_AST>),
    Variable(Box<VARIABLE_AST>),
    Number(Box<NUMBER_AST>),
    LogicalOrExpr(Box<LOGICAL_OR_EXPR_AST>),
}

pub struct HAS_PROPERTY_EXPR_AST {
    pub obj: Box<VARIABLE_AST>,
    pub ident: Box<IDENTIFIER>,
}

pub struct LEN_EXPR_AST {
    pub arr: Box<VARIABLE_AST>,
}

pub struct FUNCTION_CALL_AST {
    pub line: usize,
    pub pos: usize,
}

pub struct NUMBER_AST {
    pub number: i64,
}

pub struct VARIABLE_AST {
    pub line: usize,
    pub pos: usize,
    pub ident: Box<IDENTIFIER>,
    pub parts: Vec<Box<VARIABLE_PART_AST>>,
}

pub struct VARIABLE_PART_AST {
    pub type_: i32,
    pub field: Option<Box<IDENTIFIER>>,
    pub index: Option<Box<LOGICAL_OR_EXPR_AST>>,
}

pub struct IDENTIFIER {
    pub ident: String,
}

pub fn create_constant_from_int(int_cnst: i64) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    cnst.int_cnst = int_cnst;
    cnst.type_ = CONSTANT_TYPE::CONSTANT_TYPE_INTEGER;
    cnst
}

pub fn create_constant_from_double(double_cnst: f64) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    cnst.double_cnst = double_cnst;
    cnst.type_ = CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE;
    cnst
}

pub fn create_constant_from_fieldref(str_cnst: &str) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    let bytes = str_cnst.as_bytes();
    let len = bytes.len().min(31);
    cnst.str_cnst[..len].copy_from_slice(&bytes[..len]);
    cnst.type_ = CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF;
    cnst
}

pub fn create_constant_from_functionref(str_cnst: &str) -> CONSTANT {
    let mut cnst = CONSTANT::default();
    let bytes = str_cnst.as_bytes();
    let len = bytes.len().min(31);
    cnst.str_cnst[..len].copy_from_slice(&bytes[..len]);
    cnst.type_ = CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF;
    cnst
}

fn constant_pool_push_back(bc: &mut BYTECODE, cnst: CONSTANT) -> usize {
    for (i, existing) in bc.constant_pool.iter().enumerate() {
        if *existing == cnst {
            return i;
        }
    }
    bc.constant_pool.push(cnst);
    bc.constant_pool.len() - 1
}

fn create_local_variable(name: &str, depth: usize) -> LOCAL_VARIABLE {
    let mut lv = LOCAL_VARIABLE::default();
    let bytes = name.as_bytes();
    let len = bytes.len().min(31);
    lv.name[..len].copy_from_slice(&bytes[..len]);
    lv.depth = depth;
    lv
}

fn local_variable_index(bc_gen: &BYTECODE_GENERATOR, var_name: &str) -> i32 {
    for i in (0..bc_gen.locals.len()).rev() {
        let local_name = &bc_gen.locals[i].name;
        let local_cstr = CStr::from_bytes_until_nul(local_name).unwrap_or_default();
        if local_cstr.to_string_lossy() == var_name {
            return i as i32;
        }
    }
    -1
}

pub fn create_bytecode_generator() -> bytecode_generator_type_t {
    Box::new(BYTECODE_GENERATOR::default())
}

pub fn bytecode_generator_conf(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &'static UNIT_AST,
) {
    bc_gen.ast = Some(ast);
    bc_gen.bc = Some(Box::new(BYTECODE::default()));
}

fn set_bytecode_generator_error(
    bc_gen: &mut BYTECODE_GENERATOR,
    line: usize,
    pos: usize,
    code: i32,
) {
    bc_gen.err.pos.line = line;
    bc_gen.err.pos.pos = pos;
    bc_gen.err.code = code;
}

fn property_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &PROPERTY_AST,
) -> i32 {
    let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.value);
    if r != 0 {
        return r;
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(6);
        let cnst = create_constant_from_fieldref(&ast.key.ident);
        let idx = constant_pool_push_back(bc, cnst);
        bc.op_codes.push(idx);
    }
    0
}

fn object_literal_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &OBJECT_LITERAL_AST,
) -> i32 {
    for prop in &ast.properties {
        let r = property_ast_bytecode_generate(bc_gen, prop);
        if r != 0 {
            return r;
        }
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(5);
        bc.op_codes.push(ast.properties.len());
    }
    0
}

fn array_literal_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &ARRAY_LITERAL_AST,
) -> i32 {
    if let Some(ref args_list) = ast.args_list {
        for i in (0..args_list.assignment_exprs.len()).rev() {
            let r = assignment_expr_ast_bytecode_generate(
                bc_gen,
                &args_list.assignment_exprs[i],
            );
            if r != 0 {
                return r;
            }
        }
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(7);
        if let Some(ref args_list) = ast.args_list {
            bc.op_codes.push(args_list.assignment_exprs.len());
        } else {
            bc.op_codes.push(0);
        }
    }
    0
}

fn number_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &NUMBER_AST,
) -> i32 {
    let cnst = create_constant_from_int(ast.number);
    if let Some(ref mut bc) = bc_gen.bc {
        let index = constant_pool_push_back(bc, cnst);
        bc.op_codes.push(1);
        bc.op_codes.push(index);
    }
    0
}

fn variable_ast_bytecode_generate_inner(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &VARIABLE_AST,
    is_set_op: i32,
) -> i32 {
    let idx = local_variable_index(bc_gen, &ast.ident.ident);
    if idx == -1 {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, -1);
        return -1;
    }

    if ast.parts.is_empty() {
        if let Some(ref mut bc) = bc_gen.bc {
            if is_set_op != 0 {
                bc.op_codes.push(4);
            } else {
                bc.op_codes.push(3);
            }
            bc.op_codes.push(idx as usize);
        }
    } else {
        let mut count = 0;
        for part in &ast.parts {
            if part.type_ == 0 {
                count += 1;
                if let Some(ref index) = part.index {
                    logical_or_expr_ast_bytecode_generate(bc_gen, index);
                }
            }
        }

        if let Some(ref mut bc) = bc_gen.bc {
            if is_set_op != 0 {
                bc.op_codes.push(10);
            } else {
                bc.op_codes.push(8);
            }
            bc.op_codes.push(idx as usize);
            bc.op_codes.push(ast.parts.len());

            for part in &ast.parts {
                if part.type_ == 1 {
                    if let Some(ref field) = part.field {
                        let cnst = create_constant_from_fieldref(&field.ident);
                        bc.op_codes.push(1);
                        let idx = constant_pool_push_back(bc, cnst);
                        bc.op_codes.push(idx);
                    }
                } else if part.type_ == 0 {
                    bc.op_codes.push(0);
                    count -= 1;
                    bc.op_codes.push(count);
                }
            }
        }
    }
    0
}

fn variable_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &VARIABLE_AST,
) -> i32 {
    variable_ast_bytecode_generate_inner(bc_gen, ast, 0)
}

fn has_property_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &HAS_PROPERTY_EXPR_AST,
) -> i32 {
    let r = variable_ast_bytecode_generate_inner(bc_gen, &ast.obj, 0);
    if r != 0 {
        return r;
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(26);
        let cnst = create_constant_from_fieldref(&ast.ident.ident);
        let idx = constant_pool_push_back(bc, cnst);
        bc.op_codes.push(idx);
    }
    0
}

fn len_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &LEN_EXPR_AST,
) -> i32 {
    let r = variable_ast_bytecode_generate_inner(bc_gen, &ast.arr, 0);
    if r != 0 {
        return r;
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(27);
    }
    0
}

fn function_call_expr_ast_bytecode_generate(
    _bc_gen: &mut BYTECODE_GENERATOR,
    _ast: &FUNCTION_CALL_AST,
) -> i32 {
    0
}

fn primary_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &PRIMARY_EXPR_AST,
) -> i32 {
    match ast {
        PRIMARY_EXPR_AST::HasProperty(has_prop) => {
            has_property_expr_ast_bytecode_generate(bc_gen, has_prop)
        }
        PRIMARY_EXPR_AST::Len(len_expr) => {
            len_expr_ast_bytecode_generate(bc_gen, len_expr)
        }
        PRIMARY_EXPR_AST::FunctionCall(fn_call) => {
            function_call_expr_ast_bytecode_generate(bc_gen, fn_call)
        }
        PRIMARY_EXPR_AST::Variable(var) => {
            variable_ast_bytecode_generate(bc_gen, var)
        }
        PRIMARY_EXPR_AST::Number(num) => {
            number_ast_bytecode_generate(bc_gen, num)
        }
        PRIMARY_EXPR_AST::LogicalOrExpr(expr) => {
            logical_or_expr_ast_bytecode_generate(bc_gen, expr)
        }
    }
}

fn left_unary_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &LEFT_UNARY_EXPR_AST,
) -> i32 {
    let r = primary_expr_ast_bytecode_generate(bc_gen, &ast.expr);
    if r != 0 {
        return r;
    }

    if ast.op != 0 {
        if ast.op == 1 {
            if let Some(ref mut bc) = bc_gen.bc {
                bc.op_codes.push(25);
            }
        }
    }
    0
}

fn multiplicative_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &MULTIPLICATIVE_EXPR_AST,
) -> i32 {
    if ast.lues.is_empty() {
        return 0;
    }

    let r = left_unary_expr_ast_bytecode_generate(bc_gen, &ast.lues[0]);
    if r != 0 {
        return r;
    }

    for i in 1..ast.lues.len() {
        let r = left_unary_expr_ast_bytecode_generate(bc_gen, &ast.lues[i]);
        if r != 0 {
            return r;
        }

        if i > 0 && i - 1 < ast.ops.len() {
            if let Some(ref mut bc) = bc_gen.bc {
                match ast.ops[i - 1] {
                    0 => bc.op_codes.push(22),
                    1 => bc.op_codes.push(23),
                    2 => bc.op_codes.push(24),
                    _ => {}
                }
            }
        }
    }
    0
}

fn additive_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &ADDITIVE_EXPR_AST,
) -> i32 {
    if ast.muls.is_empty() {
        return 0;
    }

    let r = multiplicative_expr_ast_bytecode_generate(bc_gen, &ast.muls[0]);
    if r != 0 {
        return r;
    }

    for i in 1..ast.muls.len() {
        let r = multiplicative_expr_ast_bytecode_generate(bc_gen, &ast.muls[i]);
        if r != 0 {
            return r;
        }

        if i > 0 && i - 1 < ast.ops.len() {
            if let Some(ref mut bc) = bc_gen.bc {
                if ast.ops[i - 1] == 0 {
                    bc.op_codes.push(20);
                } else if ast.ops[i - 1] == 1 {
                    bc.op_codes.push(21);
                }
            }
        }
    }
    0
}

fn relational_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &RELATIONAL_EXPR_AST,
) -> i32 {
    let r = additive_expr_ast_bytecode_generate(bc_gen, &ast.left);
    if r != 0 {
        return r;
    }

    if let Some(ref right) = ast.right {
        let r = additive_expr_ast_bytecode_generate(bc_gen, right);
        if r != 0 {
            return r;
        }

        if let Some(ref mut bc) = bc_gen.bc {
            match ast.rel_op {
                0 => bc.op_codes.push(16),
                1 => bc.op_codes.push(17),
                2 => bc.op_codes.push(18),
                3 => bc.op_codes.push(19),
                _ => {}
            }
        }
    }
    0
}

fn eq_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &EQ_EXPR_AST,
) -> i32 {
    let r = relational_expr_ast_bytecode_generate(bc_gen, &ast.left);
    if r != 0 {
        return r;
    }

    if let Some(ref right) = ast.right {
        let r = relational_expr_ast_bytecode_generate(bc_gen, right);
        if r != 0 {
            return r;
        }

        if let Some(ref mut bc) = bc_gen.bc {
            if ast.eq_op == 0 {
                bc.op_codes.push(14);
            } else if ast.eq_op == 1 {
                bc.op_codes.push(15);
            }
        }
    }
    0
}

fn logical_and_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &LOGICAL_AND_EXPR_AST,
) -> i32 {
    if ast.eq_exprs.is_empty() {
        return 0;
    }

    let r = eq_expr_ast_bytecode_generate(bc_gen, &ast.eq_exprs[0]);
    if r != 0 {
        return r;
    }

    for i in 1..ast.eq_exprs.len() {
        let r = eq_expr_ast_bytecode_generate(bc_gen, &ast.eq_exprs[i]);
        if r != 0 {
            return r;
        }

        if let Some(ref mut bc) = bc_gen.bc {
            bc.op_codes.push(13);
        }
    }
    0
}

fn logical_or_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &LOGICAL_OR_EXPR_AST,
) -> i32 {
    if ast.and_exprs.is_empty() {
        return 0;
    }

    let r = logical_and_expr_ast_bytecode_generate(bc_gen, &ast.and_exprs[0]);
    if r != 0 {
        return r;
    }

    for i in 1..ast.and_exprs.len() {
        let r = logical_and_expr_ast_bytecode_generate(bc_gen, &ast.and_exprs[i]);
        if r != 0 {
            return r;
        }

        if let Some(ref mut bc) = bc_gen.bc {
            bc.op_codes.push(12);
        }
    }
    0
}

fn assignment_expr_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &ASSIGNMENT_EXPR_AST,
) -> i32 {
    match ast {
        ASSIGNMENT_EXPR_AST::ObjectLiteral(obj_lit) => {
            object_literal_ast_bytecode_generate(bc_gen, obj_lit)
        }
        ASSIGNMENT_EXPR_AST::ArrayLiteral(arr_lit) => {
            array_literal_ast_bytecode_generate(bc_gen, arr_lit)
        }
        ASSIGNMENT_EXPR_AST::LogicalOrExpr(expr) => {
            logical_or_expr_ast_bytecode_generate(bc_gen, expr)
        }
    }
}

fn decl_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &DECL_STMT_AST,
) -> i32 {
    let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.assignment);
    if r != 0 {
        return r;
    }

    let idx = local_variable_index(bc_gen, &ast.new_var_name.ident);
    if idx != -1 && bc_gen.locals[idx as usize].depth == bc_gen.scope_depth {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, -2);
        return -2;
    }

    let lv = create_local_variable(&ast.new_var_name.ident, bc_gen.scope_depth);
    bc_gen.locals.push(lv);
    let idx = bc_gen.locals.len() - 1;

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(4);
        bc.op_codes.push(idx);
    }
    0
}

fn assign_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &ASSIGN_STMT_AST,
) -> i32 {
    let r = assignment_expr_ast_bytecode_generate(bc_gen, &ast.assignment);
    if r != 0 {
        return r;
    }
    0
}

pub fn function_call_stmt_ast_bytecode_generate(
    _bc_gen: &mut BYTECODE_GENERATOR,
    _ast: &FUNCTION_CALL_STMT_AST,
) -> i32 {
    0
}

fn emit_jump(bc_gen: &mut BYTECODE_GENERATOR, instruction: usize) -> i32 {
    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(instruction);
        bc.op_codes.push(0);
        (bc.op_codes.len() - 1) as i32
    } else {
        -1
    }
}

fn patch_jump(bc_gen: &mut BYTECODE_GENERATOR, offset: usize) {
    if let Some(ref mut bc) = bc_gen.bc {
        let jump = (bc.op_codes.len() as i32) - (offset as i32) - 1;
        if offset < bc.op_codes.len() {
            bc.op_codes[offset] = jump as usize;
        }
    }
}

fn if_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &IF_STMT_AST,
    loop_start_idx: &mut Option<i32>,
    loop_exit_idxs: &mut Option<Vec<i32>>,
) -> i32 {
    let r = logical_or_expr_ast_bytecode_generate(bc_gen, &ast.condition);
    if r != 0 {
        return r;
    }

    let if_idx = emit_jump(bc_gen, 28) as usize;

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(0);
    }

    let r = body_ast_bytecode_generate(bc_gen, &ast.if_body, loop_start_idx, loop_exit_idxs);
    if r != 0 {
        return r;
    }

    let else_idx = emit_jump(bc_gen, 29) as usize;
    patch_jump(bc_gen, if_idx);

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(0);
    }

    if let Some(ref else_body) = ast.else_body {
        let r = body_ast_bytecode_generate(bc_gen, else_body, loop_start_idx, loop_exit_idxs);
        if r != 0 {
            return r;
        }
    }

    patch_jump(bc_gen, else_idx);
    0
}

fn emit_loop(bc_gen: &mut BYTECODE_GENERATOR, loop_start: usize) {
    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(29);
        let offset = ((bc.op_codes.len() as i32) - (loop_start as i32) + 1) * (-1);
        bc.op_codes.push(offset as usize);
    }
}

fn while_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &WHILE_STMT_AST,
) -> i32 {
    let loop_start_idx = if let Some(ref bc) = bc_gen.bc {
        bc.op_codes.len()
    } else {
        0
    };

    let r = logical_or_expr_ast_bytecode_generate(bc_gen, &ast.condition);
    if r != 0 {
        return r;
    }

    let mut loop_exit_idxs: Vec<i32> = Vec::new();
    let exit_idx = emit_jump(bc_gen, 28);
    if exit_idx >= 0 {
        loop_exit_idxs.push(exit_idx as i32);
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(0);
    }

    let mut loop_start = Some(loop_start_idx as i32);
    let mut loop_exits = Some(loop_exit_idxs.clone());

    let r = body_ast_bytecode_generate(bc_gen, &ast.body, &mut loop_start, &mut loop_exits);
    if r != 0 {
        return r;
    }

    if let Some(ref loop_exits) = loop_exits {
        loop_exit_idxs = loop_exits.clone();
    }

    emit_loop(bc_gen, loop_start_idx);
    
    if !loop_exit_idxs.is_empty() {
        patch_jump(bc_gen, loop_exit_idxs[0] as usize);
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(0);
    }

    for i in 1..loop_exit_idxs.len() {
        patch_jump(bc_gen, loop_exit_idxs[i] as usize);
    }

    0
}

fn break_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &BREAK_STMT_AST,
    loop_exit_idxs: &mut Option<Vec<i32>>,
) -> i32 {
    if loop_exit_idxs.is_none() {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, -3);
        return -3;
    }

    let loop_exit_idx = emit_jump(bc_gen, 29);
    if let Some(ref mut idxs) = loop_exit_idxs {
        idxs.push(loop_exit_idx);
    }
    0
}

fn continue_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &CONTINUE_STMT_AST,
    loop_start_idx: &mut Option<i32>,
) -> i32 {
    if let Some(start_idx) = loop_start_idx {
        emit_loop(bc_gen, *start_idx as usize);
        0
    } else {
        set_bytecode_generator_error(bc_gen, ast.line, ast.pos, -4);
        -4
    }
}

fn append_stmt_ast_bytecode_generate(
    _bc_gen: &mut BYTECODE_GENERATOR,
    _ast: &APPEND_STMT_AST,
) -> i32 {
    0
}

fn delete_stmt_ast_bytecode_generate(
    _bc_gen: &mut BYTECODE_GENERATOR,
    _ast: &DELETE_STMT_AST,
) -> i32 {
    0
}

fn return_stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &RETURN_STMT_AST,
) -> i32 {
    if let Some(ref result) = ast.result {
        let r = assignment_expr_ast_bytecode_generate(bc_gen, result);
        if r != 0 {
            return r;
        }
    }

    if let Some(ref mut bc) = bc_gen.bc {
        bc.op_codes.push(30);
    }
    0
}

fn stmt_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &STMT_AST,
    loop_start_idx: &mut Option<i32>,
    loop_exit_idxs: &mut Option<Vec<i32>>,
) -> i32 {
    match ast {
        STMT_AST::Decl(decl) => decl_stmt_ast_bytecode_generate(bc_gen, decl),
        STMT_AST::Assign(assign) => assign_stmt_ast_bytecode_generate(bc_gen, assign),
        STMT_AST::FunctionCall(fn_call) => function_call_stmt_ast_bytecode_generate(bc_gen, fn_call),
        STMT_AST::If(if_stmt) => if_stmt_ast_bytecode_generate(bc_gen, if_stmt, loop_start_idx, loop_exit_idxs),
        STMT_AST::While(while_stmt) => while_stmt_ast_bytecode_generate(bc_gen, while_stmt),
        STMT_AST::Break(break_stmt) => break_stmt_ast_bytecode_generate(bc_gen, break_stmt, loop_exit_idxs),
        STMT_AST::Continue(continue_stmt) => continue_stmt_ast_bytecode_generate(bc_gen, continue_stmt, loop_start_idx),
        STMT_AST::Append(append) => append_stmt_ast_bytecode_generate(bc_gen, append),
        STMT_AST::Delete(delete) => delete_stmt_ast_bytecode_generate(bc_gen, delete),
        STMT_AST::Return(ret) => return_stmt_ast_bytecode_generate(bc_gen, ret),
    }
}

fn body_ast_bytecode_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    ast: &BODY_AST,
    loop_start_idx: &mut Option<i32>,
    loop_exit_idxs: &mut Option<Vec<i32>>,
) -> i32 {
    bc_gen.scope_depth += 1;

    for stmt in &ast.stmts {
        let r = stmt_ast_bytecode_generate(bc_gen, stmt, loop_start_idx, loop_exit_idxs);
        if r != 0 {
            return r;
        }
    }

    bc_gen.scope_depth -= 1;

    while !bc_gen.locals.is_empty()
        && bc_gen.locals[bc_gen.locals.len() - 1].depth > bc_gen.scope_depth
    {
        if let Some(ref mut bc) = bc_gen.bc {
            bc.op_codes.push(0);
        }
        bc_gen.locals.pop();
    }

    0
}

pub fn bytecode_generator_generate(
    bc_gen: &mut BYTECODE_GENERATOR,
    bc: &mut Option<Box<BYTECODE>>,
) -> i32 {
    if let Some(ref ast) = bc_gen.ast {
        if !ast.functions.is_empty() {
            let f = &ast.functions[0];
            let mut loop_start_idx: Option<i32> = None;
            let mut loop_exit_idxs: Option<Vec<i32>> = None;

            let r = body_ast_bytecode_generate(bc_gen, &f.body, &mut loop_start_idx, &mut loop_exit_idxs);
            if r != 0 {
                if let Some(ref mut b) = bc_gen.bc {
                    bytecode_free(b);
                }
                return r;
            }

            *bc = bc_gen.bc.take();
            return 0;
        }
    }

    if let Some(ref mut b) = bc_gen.bc {
        bytecode_free(b);
    }
    -1
}

pub fn bytecode_generator_free(bc_gen: &mut BYTECODE_GENERATOR) {
    bc_gen.locals.clear();
}

pub fn create_bytecode() -> Box<BYTECODE> {
    Box::new(BYTECODE::default())
}

pub fn dump_bytecode_to_xml_file(f: &mut std::fs::File, bc: &BYTECODE) {
    use std::io::Write;

    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<bytecode>");
    let _ = writeln!(f, "\t<constant_pool>");

    for (i, cnst) in bc.constant_pool.iter().enumerate() {
        match &cnst.type_ {
            CONSTANT_TYPE::CONSTANT_TYPE_INTEGER => {
                let _ = writeln!(f, "\t\t<int_cnst idx=\"{}\">{}</int_cnst>", i, cnst.int_cnst);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_DOUBLE => {
                let _ = writeln!(f, "\t\t<double_cnst idx=\"{}\">{}</double_cnst>", i, cnst.double_cnst);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_FIELDREF => {
                let s = String::from_utf8_lossy(
                    &cnst.str_cnst[..cnst.str_cnst.iter().position(|&b| b == 0).unwrap_or(32)],
                );
                let _ = writeln!(f, "\t\t<fieldref_cnst idx=\"{}\">{}</fieldref_cnst>", i, s);
            }
            CONSTANT_TYPE::CONSTANT_TYPE_FUNCTIONREF => {
                let s = String::from_utf8_lossy(
                    &cnst.str_cnst[..cnst.str_cnst.iter().position(|&b| b == 0).unwrap_or(32)],
                );
                let _ = writeln!(f, "\t\t<functionref_cnst idx=\"{}\">{}</functionref_cnst>", i, s);
            }
        }
    }

    let _ = writeln!(f, "\t</constant_pool>");
    let _ = writeln!(f, "\t<op_codes>");

    let mut i = 0;
    while i < bc.op_codes.len() {
        match bc.op_codes[i] {
            0 => {
                let _ = writeln!(f, "\t\t<op>POP</op>");
            }
            1 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>CONSTANT {}</op>", bc.op_codes[i]);
                }
            }
            3 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>GET_LOCAL {}</op>", bc.op_codes[i]);
                }
            }
            4 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>SET_LOCAL {}</op>", bc.op_codes[i]);
                }
            }
            5 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>CREATE_OBJ {}</op>", bc.op_codes[i]);
                }
            }
            6 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>INIT_OBJ_PROP {}</op>", bc.op_codes[i]);
                }
            }
            7 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>CREATE_ARR {}</op>", bc.op_codes[i]);
                }
            }
            8 | 10 => {
                if bc.op_codes[i] == 8 {
                    i += 1;
                    if i < bc.op_codes.len() {
                        let _ = write!(f, "\t\t<op>GET_HEAP {}", bc.op_codes[i]);
                    }
                } else {
                    i += 1;
                    if i < bc.op_codes.len() {
                        let _ = write!(f, "\t\t<op>SET_HEAP {}", bc.op_codes[i]);
                    }
                }
                i += 1;
                if i < bc.op_codes.len() {
                    let n = bc.op_codes[i];
                    i += 1;
                    for _ in 0..n {
                        if i < bc.op_codes.len() {
                            if bc.op_codes[i] == 1 {
                                i += 1;
                                if i < bc.op_codes.len() {
                                    let _ = write!(f, " field({})", bc.op_codes[i]);
                                }
                            } else if bc.op_codes[i] == 0 {
                                i += 1;
                                if i < bc.op_codes.len() {
                                    let _ = write!(f, " index({})", bc.op_codes[i]);
                                }
                            }
                            i += 1;
                        }
                    }
                    i -= 1;
                }
                let _ = writeln!(f, "</op>");
            }
            12 => {
                let _ = writeln!(f, "\t\t<op>LOGICAL_OR</op>");
            }
            13 => {
                let _ = writeln!(f, "\t\t<op>LOGICAL_AND</op>");
            }
            14 => {
                let _ = writeln!(f, "\t\t<op>EQ_EQEQ</op>");
            }
            15 => {
                let _ = writeln!(f, "\t\t<op>EQ_NEQ</op>");
            }
            16 => {
                let _ = writeln!(f, "\t\t<op>REL_LT</op>");
            }
            17 => {
                let _ = writeln!(f, "\t\t<op>REL_GT</op>");
            }
            18 => {
                let _ = writeln!(f, "\t\t<op>REL_LE</op>");
            }
            19 => {
                let _ = writeln!(f, "\t\t<op>REL_GE</op>");
            }
            20 => {
                let _ = writeln!(f, "\t\t<op>ADDITIVE_PLUS</op>");
            }
            21 => {
                let _ = writeln!(f, "\t\t<op>ADDITIVE_MINUS</op>");
            }
            22 => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_MUL</op>");
            }
            23 => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_DIV</op>");
            }
            24 => {
                let _ = writeln!(f, "\t\t<op>MULTIPLICATIVE_MOD</op>");
            }
            25 => {
                let _ = writeln!(f, "\t\t<op>NEGATE</op>");
            }
            27 => {
                let _ = writeln!(f, "\t\t<op>LEN</op>");
            }
            26 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>HAS_PROPERTY {}</op>", bc.op_codes[i]);
                }
            }
            28 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>JUMP_IF_FALSE {}</op>", bc.op_codes[i] as i32);
                }
            }
            29 => {
                i += 1;
                if i < bc.op_codes.len() {
                    let _ = writeln!(f, "\t\t<op>JUMP {}</op>", bc.op_codes[i] as i32);
                }
            }
            30 => {
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
    BYTECODE_ERROR {
        pos: POS {
            line: bc_gen.err.pos.line,
            pos: bc_gen.err.pos.pos,
        },
        code: bc_gen.err.code,
    }
}

pub fn print_bytecode_error(err: &BYTECODE_ERROR) {
    let error_str = match err.code {
        0 => "ok?!",
        -1 => "no local variable!",
        -2 => "already have local variable!",
        -3 => "break outside of while!",
        -4 => "continue outside of while!",
        _ => "unknown error!",
    };

    eprintln!(
        "{}:{}: error: {}",
        err.pos.line, err.pos.pos, error_str
    );
}

pub fn bytecode_free(_bc: &mut BYTECODE) {
}