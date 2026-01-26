use std::io::Write;

// The generated code frequently uses `write!(f, "{:width$}", "", spaces_num);`.
// In Rust, `width$` expects a named argument (`width = ...`), otherwise the extra
// positional argument triggers "argument never used".
// Shadow `write!` in this module to fix that specific pattern while forwarding
// everything else to `std::write!`.
macro_rules! write {
    ($dst:expr, "{:width$}", $s:expr, $width:expr $(,)?) => {
        std::write!($dst, "{:width$}", $s, width = $width)
    };
    ($($tt:tt)*) => {
        std::write!($($tt)*)
    };
}

pub struct UNIT_AST {
    pub functions: Box<[Option<Box<FUNCTION_DECL_AST>>]>,
    pub functions_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_unit_ast(
    functions: Box<[Option<Box<FUNCTION_DECL_AST>>]>,
    functions_len: usize,
    line: usize,
    pos: usize,
) -> Box<UNIT_AST> {
    Box::new(UNIT_AST {
        functions,
        functions_len,
        line,
        pos,
    })
}

fn dump_unit_ast_to_xml_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a UNIT_AST,
    mut spaces_num: usize,
) {
    let _ = writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(f, "<unit>");
    spaces_num += 2;
    for i in 0..ast.functions_len {
        if let Some(ref func) = ast.functions[i] {
            dump_function_decl_ast_to_file_inner(f, func, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = writeln!(f, "</unit>");
}

pub fn dump_unit_ast_to_xml_file<'a>(f: &'a mut dyn Write, ast: &'a UNIT_AST) {
    dump_unit_ast_to_xml_file_inner(f, ast, 0);
}

pub fn unit_ast_free(_ast: &mut UNIT_AST) {}

pub struct FUNCTION_DECL_AST {
    pub function_name: Option<Box<IDENT_AST>>,
    pub formal_parameters_list: Option<Box<FORMAL_PARAMETERS_LIST_AST>>,
    pub body: Option<Box<BODY_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_function_decl_ast(
    function_name: Option<Box<IDENT_AST>>,
    formal_parameters_list: Option<Box<FORMAL_PARAMETERS_LIST_AST>>,
    body: Option<Box<BODY_AST>>,
    line: usize,
    pos: usize,
) -> Box<FUNCTION_DECL_AST> {
    Box::new(FUNCTION_DECL_AST {
        function_name,
        formal_parameters_list,
        body,
        line,
        pos,
    })
}

fn dump_function_decl_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a FUNCTION_DECL_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<function line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<function_name line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    if let Some(ref name) = ast.function_name {
        dump_ident_ast_to_file_inner(f, name, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</function_name>");
    if let Some(ref params) = ast.formal_parameters_list {
        dump_formal_parameters_list_ast_to_file_inner(f, params, spaces_num);
    }
    if let Some(ref body) = ast.body {
        dump_body_ast_to_file_inner(f, body, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</function>");
}

pub fn dump_function_decl_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a FUNCTION_DECL_AST) {
    dump_function_decl_ast_to_file_inner(f, ast, 0);
}

pub fn function_decl_ast_free(_ast: &mut FUNCTION_DECL_AST) {}

pub struct FORMAL_PARAMETERS_LIST_AST {
    pub params: Box<[Option<Box<IDENT_AST>>]>,
    pub params_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_formal_parameters_list_ast(
    params: Box<[Option<Box<IDENT_AST>>]>,
    params_len: usize,
    line: usize,
    pos: usize,
) -> Box<FORMAL_PARAMETERS_LIST_AST> {
    Box::new(FORMAL_PARAMETERS_LIST_AST {
        params,
        params_len,
        line,
        pos,
    })
}

fn dump_formal_parameters_list_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a FORMAL_PARAMETERS_LIST_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<formal_parameters_list line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    for i in 0..ast.params_len {
        if let Some(ref param) = ast.params[i] {
            dump_ident_ast_to_file_inner(f, param, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</formal_parameters_list>");
}

pub fn dump_formal_parameters_list_ast_to_file<'a>(
    f: &'a mut dyn Write,
    ast: &'a FORMAL_PARAMETERS_LIST_AST,
) {
    dump_formal_parameters_list_ast_to_file_inner(f, ast, 0);
}

pub fn formal_parameters_list_ast_free(_ast: &mut FORMAL_PARAMETERS_LIST_AST) {}

pub struct BODY_AST {
    pub stmts: Box<[Option<Box<STMT_AST>>]>,
    pub stmts_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_body_ast(
    stmts: Box<[Option<Box<STMT_AST>>]>,
    stmts_len: usize,
    line: usize,
    pos: usize,
) -> Box<BODY_AST> {
    Box::new(BODY_AST {
        stmts,
        stmts_len,
        line,
        pos,
    })
}

fn dump_body_ast_to_file_inner<'a>(f: &'a mut dyn Write, ast: &'a BODY_AST, mut spaces_num: usize) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<body line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    for i in 0..ast.stmts_len {
        if let Some(ref stmt) = ast.stmts[i] {
            dump_stmt_ast_to_file_inner(f, stmt, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</body>");
}

pub fn dump_body_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a BODY_AST) {
    dump_body_ast_to_file_inner(f, ast, 0);
}

pub fn body_ast_free(_ast: &mut BODY_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_STMT_TYPE {
    Decl,
    Assign,
    FunctionCall,
    If,
    While,
    Break,
    Continue,
    Append,
    Delete,
    Return,
}

pub enum StmtAstInner {
    DeclStmt(Box<DECL_STMT_AST>),
    AssignStmt(Box<ASSIGN_STMT_AST>),
    FunctionCallStmt(Box<FUNCTION_CALL_STMT_AST>),
    IfStmt(Box<IF_STMT_AST>),
    WhileStmt(Box<WHILE_STMT_AST>),
    BreakStmt(Box<BREAK_STMT_AST>),
    ContinueStmt(Box<CONTINUE_STMT_AST>),
    AppendStmt(Box<APPEND_STMT_AST>),
    DeleteStmt(Box<DELETE_STMT_AST>),
    ReturnStmt(Box<RETURN_STMT_AST>),
}

pub struct STMT_AST {
    pub inner: StmtAstInner,
    pub stmt_type: AST_STMT_TYPE,
}

pub fn create_stmt_ast(stmt_ptr: StmtAstInner, stmt_type: AST_STMT_TYPE) -> Box<STMT_AST> {
    Box::new(STMT_AST {
        inner: stmt_ptr,
        stmt_type,
    })
}

fn dump_stmt_ast_to_file_inner<'a>(f: &'a mut dyn Write, ast: &'a STMT_AST, spaces_num: usize) {
    match &ast.inner {
        StmtAstInner::DeclStmt(s) => dump_decl_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::AssignStmt(s) => dump_assign_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::FunctionCallStmt(s) => {
            dump_function_call_stmt_ast_to_file_inner(f, s, spaces_num)
        }
        StmtAstInner::IfStmt(s) => dump_if_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::WhileStmt(s) => dump_while_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::BreakStmt(s) => dump_break_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::ContinueStmt(s) => dump_continue_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::AppendStmt(s) => dump_append_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::DeleteStmt(s) => dump_delete_stmt_ast_to_file_inner(f, s, spaces_num),
        StmtAstInner::ReturnStmt(s) => dump_return_stmt_ast_to_file_inner(f, s, spaces_num),
    }
}

pub fn dump_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a STMT_AST) {
    dump_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn stmt_ast_free(_ast: &mut STMT_AST) {}

pub struct DECL_STMT_AST {
    pub new_var_name: Option<Box<IDENT_AST>>,
    pub assignment: Option<Box<ASSIGNMENT_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_decl_stmt_ast(
    new_var_name: Option<Box<IDENT_AST>>,
    assignment: Option<Box<ASSIGNMENT_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<DECL_STMT_AST> {
    Box::new(DECL_STMT_AST {
        new_var_name,
        assignment,
        line,
        pos,
    })
}

fn dump_decl_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a DECL_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<decl_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref name) = ast.new_var_name {
        dump_ident_ast_to_file_inner(f, name, spaces_num);
    }
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<op>EQ</op>");
    if let Some(ref assign) = ast.assignment {
        dump_assignment_expr_ast_to_file_inner(f, assign, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</decl_stmt>");
}

pub fn dump_decl_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a DECL_STMT_AST) {
    dump_decl_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn decl_stmt_ast_free(_ast: &mut DECL_STMT_AST) {}

pub struct ASSIGN_STMT_AST {
    pub var_name: Option<Box<VARIABLE_AST>>,
    pub assignment: Option<Box<ASSIGNMENT_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_assign_stmt_ast(
    var_name: Option<Box<VARIABLE_AST>>,
    assignment: Option<Box<ASSIGNMENT_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<ASSIGN_STMT_AST> {
    Box::new(ASSIGN_STMT_AST {
        var_name,
        assignment,
        line,
        pos,
    })
}

fn dump_assign_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a ASSIGN_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<assign_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref var) = ast.var_name {
        dump_variable_ast_to_file_inner(f, var, spaces_num);
    }
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<op>EQ</op>");
    if let Some(ref assign) = ast.assignment {
        dump_assignment_expr_ast_to_file_inner(f, assign, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</assign_stmt>");
}

pub fn dump_assign_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a ASSIGN_STMT_AST) {
    dump_assign_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn assign_stmt_ast_free(_ast: &mut ASSIGN_STMT_AST) {}

pub struct FUNCTION_CALL_STMT_AST {
    pub function_call: Option<Box<FUNCTION_CALL_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_function_call_stmt_ast(
    function_call: Option<Box<FUNCTION_CALL_AST>>,
    line: usize,
    pos: usize,
) -> Box<FUNCTION_CALL_STMT_AST> {
    Box::new(FUNCTION_CALL_STMT_AST {
        function_call,
        line,
        pos,
    })
}

fn dump_function_call_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a FUNCTION_CALL_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<function_call_stmt line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    if let Some(ref call) = ast.function_call {
        dump_function_call_ast_to_file_inner(f, call, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</function_call_stmt>");
}

pub fn dump_function_call_stmt_ast_to_file<'a>(
    f: &'a mut dyn Write,
    ast: &'a FUNCTION_CALL_STMT_AST,
) {
    dump_function_call_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn function_call_stmt_ast_free(_ast: &mut FUNCTION_CALL_STMT_AST) {}

pub struct IF_STMT_AST {
    pub condition: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub if_body: Option<Box<BODY_AST>>,
    pub else_body: Option<Box<BODY_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_if_stmt_ast(
    condition: Option<Box<LOGICAL_OR_EXPR_AST>>,
    if_body: Option<Box<BODY_AST>>,
    else_body: Option<Box<BODY_AST>>,
    line: usize,
    pos: usize,
) -> Box<IF_STMT_AST> {
    Box::new(IF_STMT_AST {
        condition,
        if_body,
        else_body,
        line,
        pos,
    })
}

fn dump_if_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a IF_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<if_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<condition>");
    spaces_num += 2;
    if let Some(ref cond) = ast.condition {
        dump_logical_or_expr_ast_to_file_inner(f, cond, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</condition>");
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<if_body>");
    spaces_num += 2;
    if let Some(ref body) = ast.if_body {
        dump_body_ast_to_file_inner(f, body, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</if_body>");
    if let Some(ref else_body) = ast.else_body {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "<else_body>");
        spaces_num += 2;
        dump_body_ast_to_file_inner(f, else_body, spaces_num);
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</else_body>");
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</if_stmt>");
}

pub fn dump_if_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a IF_STMT_AST) {
    dump_if_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn if_stmt_ast_free(_ast: &mut IF_STMT_AST) {}

pub struct WHILE_STMT_AST {
    pub condition: Option<Box<LOGICAL_OR_EXPR_AST>>,
    pub body: Option<Box<BODY_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_while_stmt_ast(
    condition: Option<Box<LOGICAL_OR_EXPR_AST>>,
    body: Option<Box<BODY_AST>>,
    line: usize,
    pos: usize,
) -> Box<WHILE_STMT_AST> {
    Box::new(WHILE_STMT_AST {
        condition,
        body,
        line,
        pos,
    })
}

fn dump_while_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a WHILE_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<while_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<condition>");
    spaces_num += 2;
    if let Some(ref cond) = ast.condition {
        dump_logical_or_expr_ast_to_file_inner(f, cond, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</condition>");
    if let Some(ref body) = ast.body {
        dump_body_ast_to_file_inner(f, body, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</while_stmt>");
}

pub fn dump_while_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a WHILE_STMT_AST) {
    dump_while_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn while_stmt_ast_free(_ast: &mut WHILE_STMT_AST) {}

pub struct BREAK_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub fn create_break_stmt_ast(line: usize, pos: usize) -> Box<BREAK_STMT_AST> {
    Box::new(BREAK_STMT_AST { line, pos })
}

fn dump_break_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    _ast: &'a BREAK_STMT_AST,
    spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<break></break>");
}

pub fn dump_break_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a BREAK_STMT_AST) {
    dump_break_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn break_stmt_ast_free(_ast: &mut BREAK_STMT_AST) {}

pub struct CONTINUE_STMT_AST {
    pub line: usize,
    pub pos: usize,
}

pub fn create_continue_stmt_ast(line: usize, pos: usize) -> Box<CONTINUE_STMT_AST> {
    Box::new(CONTINUE_STMT_AST { line, pos })
}

fn dump_continue_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    _ast: &'a CONTINUE_STMT_AST,
    spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<continue></continue>");
}

pub fn dump_continue_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a CONTINUE_STMT_AST) {
    dump_continue_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn continue_stmt_ast_free(_ast: &mut CONTINUE_STMT_AST) {}

pub struct APPEND_STMT_AST {
    pub arr: Option<Box<VARIABLE_AST>>,
    pub ident: Option<Box<IDENT_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_append_stmt_ast(
    arr: Option<Box<VARIABLE_AST>>,
    ident: Option<Box<IDENT_AST>>,
    line: usize,
    pos: usize,
) -> Box<APPEND_STMT_AST> {
    Box::new(APPEND_STMT_AST {
        arr,
        ident,
        line,
        pos,
    })
}

pub fn dump_append_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a APPEND_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<append_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref arr) = ast.arr {
        dump_variable_ast_to_file_inner(f, arr, spaces_num);
    }
    if let Some(ref ident) = ast.ident {
        dump_ident_ast_to_file_inner(f, ident, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</append_stmt>");
}

pub fn dump_append_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a APPEND_STMT_AST) {
    dump_append_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn append_stmt_ast_free(_ast: &mut APPEND_STMT_AST) {}

pub struct DELETE_STMT_AST {
    pub var: Option<Box<VARIABLE_AST>>,
    pub ident: Option<Box<IDENT_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_delete_stmt_ast(
    var: Option<Box<VARIABLE_AST>>,
    ident: Option<Box<IDENT_AST>>,
    line: usize,
    pos: usize,
) -> Box<DELETE_STMT_AST> {
    Box::new(DELETE_STMT_AST {
        var,
        ident,
        line,
        pos,
    })
}

pub fn dump_delete_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a DELETE_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<delete_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref var) = ast.var {
        dump_variable_ast_to_file_inner(f, var, spaces_num);
    }
    if let Some(ref ident) = ast.ident {
        dump_ident_ast_to_file_inner(f, ident, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</delete_stmt>");
}

pub fn dump_delete_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a DELETE_STMT_AST) {
    dump_delete_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn delete_stmt_ast_free(_ast: &mut DELETE_STMT_AST) {}

pub struct RETURN_STMT_AST {
    pub result: Option<Box<ASSIGNMENT_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_return_stmt_ast(
    result: Option<Box<ASSIGNMENT_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<RETURN_STMT_AST> {
    Box::new(RETURN_STMT_AST { result, line, pos })
}

fn dump_return_stmt_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a RETURN_STMT_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<return_stmt line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    if let Some(ref result) = ast.result {
        spaces_num += 2;
        dump_assignment_expr_ast_to_file_inner(f, result, spaces_num);
        spaces_num -= 2;
    }
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</return_stmt>");
}

pub fn dump_return_stmt_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a RETURN_STMT_AST) {
    dump_return_stmt_ast_to_file_inner(f, ast, 0);
}

pub fn return_stmt_ast_free(_ast: &mut RETURN_STMT_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_VARIABLE_PART_TYPE {
    Field,
    Index,
}

pub enum VariablePartAstInner {
    Field(Box<IDENT_AST>),
    Index(Box<LOGICAL_OR_EXPR_AST>),
}

pub struct VARIABLE_PART_AST {
    pub inner: VariablePartAstInner,
    pub var_type: AST_VARIABLE_PART_TYPE,
}

pub fn create_variable_part_ast(
    variable_part_ptr: VariablePartAstInner,
    var_type: AST_VARIABLE_PART_TYPE,
) -> Box<VARIABLE_PART_AST> {
    Box::new(VARIABLE_PART_AST {
        inner: variable_part_ptr,
        var_type,
    })
}

fn dump_variable_part_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a VARIABLE_PART_AST,
    mut spaces_num: usize,
) {
    match &ast.inner {
        VariablePartAstInner::Field(field) => {
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "<field>");
            spaces_num += 2;
            dump_ident_ast_to_file_inner(f, field, spaces_num);
            spaces_num -= 2;
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "</field>");
        }
        VariablePartAstInner::Index(index) => {
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "<index>");
            spaces_num += 2;
            dump_logical_or_expr_ast_to_file_inner(f, index, spaces_num);
            spaces_num -= 2;
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "</index>");
        }
    }
}

pub fn dump_variable_part_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a VARIABLE_PART_AST) {
    dump_variable_part_ast_to_file_inner(f, ast, 0);
}

pub fn variable_part_ast_free(_ast: &mut VARIABLE_PART_AST) {}

pub struct VARIABLE_AST {
    pub ident: Option<Box<IDENT_AST>>,
    pub parts: Box<[Option<Box<VARIABLE_PART_AST>>]>,
    pub parts_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_variable_ast(
    ident: Option<Box<IDENT_AST>>,
    parts: Box<[Option<Box<VARIABLE_PART_AST>>]>,
    parts_len: usize,
    line: usize,
    pos: usize,
) -> Box<VARIABLE_AST> {
    Box::new(VARIABLE_AST {
        ident,
        parts,
        parts_len,
        line,
        pos,
    })
}

fn dump_variable_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a VARIABLE_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<variable line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref ident) = ast.ident {
        dump_ident_ast_to_file_inner(f, ident, spaces_num);
    }
    for i in 0..ast.parts_len {
        if let Some(ref part) = ast.parts[i] {
            dump_variable_part_ast_to_file_inner(f, part, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</variable>");
}

pub fn dump_variable_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a VARIABLE_AST) {
    dump_variable_ast_to_file_inner(f, ast, 0);
}

pub fn variable_ast_free(_ast: &mut VARIABLE_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_ASSIGNMENT_EXPR_TYPE {
    ObjectLiteral,
    ArrayLiteral,
    LogicalOrExpr,
}

pub enum AssignmentExprAstInner {
    ObjectLiteral(Box<OBJECT_LITERAL_AST>),
    ArrayLiteral(Box<ARRAY_LITERAL_AST>),
    LogicalOrExpr(Box<LOGICAL_OR_EXPR_AST>),
}

// Type alias for older generated names.
pub type AssignmentExprAst = ASSIGNMENT_EXPR_AST;

pub struct ASSIGNMENT_EXPR_AST {
    pub inner: AssignmentExprAstInner,
    pub expr_type: AST_ASSIGNMENT_EXPR_TYPE,
}

pub fn create_assignment_expr_ast(
    assignment_ptr: AssignmentExprAstInner,
    expr_type: AST_ASSIGNMENT_EXPR_TYPE,
) -> Box<ASSIGNMENT_EXPR_AST> {
    Box::new(ASSIGNMENT_EXPR_AST {
        inner: assignment_ptr,
        expr_type,
    })
}

fn dump_assignment_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a ASSIGNMENT_EXPR_AST,
    spaces_num: usize,
) {
    match &ast.inner {
        AssignmentExprAstInner::ObjectLiteral(obj) => {
            dump_object_literal_ast_to_file_inner(f, obj, spaces_num)
        }
        AssignmentExprAstInner::ArrayLiteral(arr) => {
            dump_array_literal_ast_to_file_inner(f, arr, spaces_num)
        }
        AssignmentExprAstInner::LogicalOrExpr(expr) => {
            dump_logical_or_expr_ast_to_file_inner(f, expr, spaces_num)
        }
    }
}

pub fn dump_assignment_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a ASSIGNMENT_EXPR_AST) {
    dump_assignment_expr_ast_to_file_inner(f, ast, 0);
}

pub fn assignment_expr_ast_free(_ast: &mut ASSIGNMENT_EXPR_AST) {}

pub struct OBJECT_LITERAL_AST {
    pub properties: Box<[Option<Box<PROPERTY_AST>>]>,
    pub properties_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_object_literal_ast(
    properties: Box<[Option<Box<PROPERTY_AST>>]>,
    properties_len: usize,
    line: usize,
    pos: usize,
) -> Box<OBJECT_LITERAL_AST> {
    Box::new(OBJECT_LITERAL_AST {
        properties,
        properties_len,
        line,
        pos,
    })
}

fn dump_object_literal_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a OBJECT_LITERAL_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<object_literal line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    for i in 0..ast.properties_len {
        if let Some(ref prop) = ast.properties[i] {
            dump_property_ast_to_file_inner(f, prop, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</object_literal>");
}

pub fn dump_object_literal_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a OBJECT_LITERAL_AST) {
    dump_object_literal_ast_to_file_inner(f, ast, 0);
}

pub fn object_literal_ast_free(_ast: &mut OBJECT_LITERAL_AST) {}

pub struct PROPERTY_AST {
    pub key: Option<Box<IDENT_AST>>,
    pub value: Option<Box<ASSIGNMENT_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_property_ast(
    key: Option<Box<IDENT_AST>>,
    value: Option<Box<ASSIGNMENT_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<PROPERTY_AST> {
    Box::new(PROPERTY_AST {
        key,
        value,
        line,
        pos,
    })
}

fn dump_property_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a PROPERTY_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<property line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<key>");
    spaces_num += 2;
    if let Some(ref key) = ast.key {
        dump_ident_ast_to_file_inner(f, key, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</key>");
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<value>");
    spaces_num += 2;
    if let Some(ref val) = ast.value {
        dump_assignment_expr_ast_to_file_inner(f, val, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</value>");
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</property>");
}

pub fn dump_property_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a PROPERTY_AST) {
    dump_property_ast_to_file_inner(f, ast, 0);
}

pub fn property_ast_free(_ast: &mut PROPERTY_AST) {}

pub struct ARRAY_LITERAL_AST {
    pub args_list: Option<Box<ARGS_LIST_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_array_literal_ast(
    args_list: Option<Box<ARGS_LIST_AST>>,
    line: usize,
    pos: usize,
) -> Box<ARRAY_LITERAL_AST> {
    Box::new(ARRAY_LITERAL_AST {
        args_list,
        line,
        pos,
    })
}

pub fn dump_array_literal_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a ARRAY_LITERAL_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<array_literal line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    if let Some(ref args) = ast.args_list {
        dump_args_list_ast_to_file_inner(f, args, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</array_literal>");
}

pub fn dump_array_literal_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a ARRAY_LITERAL_AST) {
    dump_array_literal_ast_to_file_inner(f, ast, 0);
}

pub fn array_literal_ast_free(_ast: &mut ARRAY_LITERAL_AST) {}

pub struct LOGICAL_OR_EXPR_AST {
    pub and_exprs: Box<[Option<Box<LOGICAL_AND_EXPR_AST>>]>,
    pub and_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_logical_or_expr_ast(
    and_exprs: Box<[Option<Box<LOGICAL_AND_EXPR_AST>>]>,
    and_exprs_len: usize,
    line: usize,
    pos: usize,
) -> Box<LOGICAL_OR_EXPR_AST> {
    Box::new(LOGICAL_OR_EXPR_AST {
        and_exprs,
        and_exprs_len,
        line,
        pos,
    })
}

fn dump_logical_or_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a LOGICAL_OR_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.and_exprs_len == 1 {
        if let Some(ref expr) = ast.and_exprs[0] {
            dump_logical_and_expr_ast_to_file_inner(f, expr, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<logical_or_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        if let Some(ref expr) = ast.and_exprs[0] {
            dump_logical_and_expr_ast_to_file_inner(f, expr, spaces_num);
        }
        for i in 1..ast.and_exprs_len {
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "<op>OR</op>");
            if let Some(ref expr) = ast.and_exprs[i] {
                dump_logical_and_expr_ast_to_file_inner(f, expr, spaces_num);
            }
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</logical_or_expr>");
    }
}

pub fn dump_logical_or_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a LOGICAL_OR_EXPR_AST) {
    dump_logical_or_expr_ast_to_file_inner(f, ast, 0);
}

pub fn logical_or_expr_ast_free(_ast: &mut LOGICAL_OR_EXPR_AST) {}

pub struct LOGICAL_AND_EXPR_AST {
    pub eq_exprs: Box<[Option<Box<EQ_EXPR_AST>>]>,
    pub eq_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_logical_and_expr_ast(
    eq_exprs: Box<[Option<Box<EQ_EXPR_AST>>]>,
    eq_exprs_len: usize,
    line: usize,
    pos: usize,
) -> Box<LOGICAL_AND_EXPR_AST> {
    Box::new(LOGICAL_AND_EXPR_AST {
        eq_exprs,
        eq_exprs_len,
        line,
        pos,
    })
}

fn dump_logical_and_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a LOGICAL_AND_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.eq_exprs_len == 1 {
        if let Some(ref expr) = ast.eq_exprs[0] {
            dump_eq_expr_ast_to_file_inner(f, expr, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<logical_and_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        if let Some(ref expr) = ast.eq_exprs[0] {
            dump_eq_expr_ast_to_file_inner(f, expr, spaces_num);
        }
        for i in 1..ast.eq_exprs_len {
            let _ = write!(f, "{:width$}", "", spaces_num);
            let _ = writeln!(f, "<op>AND</op>");
            if let Some(ref expr) = ast.eq_exprs[i] {
                dump_eq_expr_ast_to_file_inner(f, expr, spaces_num);
            }
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</logical_and_expr>");
    }
}

pub fn dump_logical_and_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a LOGICAL_AND_EXPR_AST) {
    dump_logical_and_expr_ast_to_file_inner(f, ast, 0);
}

pub fn logical_and_expr_ast_free(_ast: &mut LOGICAL_AND_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_EQ_OP {
    Eqeq,
    Neq,
}

pub struct EQ_EXPR_AST {
    pub left: Option<Box<RELATIONAL_EXPR_AST>>,
    pub eq_op: AST_EQ_OP,
    pub right: Option<Box<RELATIONAL_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_eq_expr_ast(
    left: Option<Box<RELATIONAL_EXPR_AST>>,
    eq_op: AST_EQ_OP,
    right: Option<Box<RELATIONAL_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<EQ_EXPR_AST> {
    Box::new(EQ_EXPR_AST {
        left,
        eq_op,
        right,
        line,
        pos,
    })
}

fn dump_eq_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a EQ_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.right.is_none() {
        if let Some(ref left) = ast.left {
            dump_relational_expr_ast_to_file_inner(f, left, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "<eq_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
        spaces_num += 2;
        if let Some(ref left) = ast.left {
            dump_relational_expr_ast_to_file_inner(f, left, spaces_num);
        }
        let _ = write!(f, "{:width$}", "", spaces_num);
        match ast.eq_op {
            AST_EQ_OP::Eqeq => {
                let _ = writeln!(f, "<op>EQEQ</op>");
            }
            AST_EQ_OP::Neq => {
                let _ = writeln!(f, "<op>NEQ</op>");
            }
        }
        if let Some(ref right) = ast.right {
            dump_relational_expr_ast_to_file_inner(f, right, spaces_num);
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</eq_expr>");
    }
}

pub fn dump_eq_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a EQ_EXPR_AST) {
    dump_eq_expr_ast_to_file_inner(f, ast, 0);
}

pub fn eq_expr_ast_free(_ast: &mut EQ_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_REL_OP {
    Lt,
    Gt,
    Le,
    Ge,
}

pub struct RELATIONAL_EXPR_AST {
    pub left: Option<Box<ADDITIVE_EXPR_AST>>,
    pub rel_op: AST_REL_OP,
    pub right: Option<Box<ADDITIVE_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_relational_expr_ast(
    left: Option<Box<ADDITIVE_EXPR_AST>>,
    rel_op: AST_REL_OP,
    right: Option<Box<ADDITIVE_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<RELATIONAL_EXPR_AST> {
    Box::new(RELATIONAL_EXPR_AST {
        left,
        rel_op,
        right,
        line,
        pos,
    })
}

fn dump_relational_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a RELATIONAL_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.right.is_none() {
        if let Some(ref left) = ast.left {
            dump_additive_expr_ast_to_file_inner(f, left, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<relational_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        if let Some(ref left) = ast.left {
            dump_additive_expr_ast_to_file_inner(f, left, spaces_num);
        }
        let _ = write!(f, "{:width$}", "", spaces_num);
        match ast.rel_op {
            AST_REL_OP::Lt => {
                let _ = writeln!(f, "<op>LT</op>");
            }
            AST_REL_OP::Gt => {
                let _ = writeln!(f, "<op>GT</op>");
            }
            AST_REL_OP::Le => {
                let _ = writeln!(f, "<op>LE</op>");
            }
            AST_REL_OP::Ge => {
                let _ = writeln!(f, "<op>GE</op>");
            }
        }
        if let Some(ref right) = ast.right {
            dump_additive_expr_ast_to_file_inner(f, right, spaces_num);
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</relational_expr>");
    }
}

pub fn dump_relational_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a RELATIONAL_EXPR_AST) {
    dump_relational_expr_ast_to_file_inner(f, ast, 0);
}

pub fn relational_expr_ast_free(_ast: &mut RELATIONAL_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_ADDITIVE_OP {
    Plus,
    Minus,
}

pub struct ADDITIVE_EXPR_AST {
    pub muls: Box<[Option<Box<MULTIPLICATIVE_EXPR_AST>>]>,
    pub ops: Box<[AST_ADDITIVE_OP]>,
    pub muls_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_additive_expr_ast(
    muls: Box<[Option<Box<MULTIPLICATIVE_EXPR_AST>>]>,
    ops: Box<[AST_ADDITIVE_OP]>,
    muls_len: usize,
    line: usize,
    pos: usize,
) -> Box<ADDITIVE_EXPR_AST> {
    Box::new(ADDITIVE_EXPR_AST {
        muls,
        ops,
        muls_len,
        line,
        pos,
    })
}

fn dump_additive_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a ADDITIVE_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.muls_len == 1 {
        if let Some(ref mul) = ast.muls[0] {
            dump_multiplicative_expr_ast_to_file_inner(f, mul, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<additive_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        if let Some(ref mul) = ast.muls[0] {
            dump_multiplicative_expr_ast_to_file_inner(f, mul, spaces_num);
        }
        for i in 1..ast.muls_len {
            let _ = write!(f, "{:width$}", "", spaces_num);
            match ast.ops[i - 1] {
                AST_ADDITIVE_OP::Plus => {
                    let _ = writeln!(f, "<op>PLUS</op>");
                }
                AST_ADDITIVE_OP::Minus => {
                    let _ = writeln!(f, "<op>MINUS</op>");
                }
            }
            if let Some(ref mul) = ast.muls[i] {
                dump_multiplicative_expr_ast_to_file_inner(f, mul, spaces_num);
            }
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</additive_expr>");
    }
}

pub fn dump_additive_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a ADDITIVE_EXPR_AST) {
    dump_additive_expr_ast_to_file_inner(f, ast, 0);
}

pub fn additive_expr_ast_free(_ast: &mut ADDITIVE_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_MULTIPLICATIVE_OP {
    Mul,
    Div,
    Mod,
}

pub struct MULTIPLICATIVE_EXPR_AST {
    pub lues: Box<[Option<Box<LEFT_UNARY_EXPR_AST>>]>,
    pub ops: Box<[AST_MULTIPLICATIVE_OP]>,
    pub lues_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_multiplicative_expr_ast(
    lues: Box<[Option<Box<LEFT_UNARY_EXPR_AST>>]>,
    ops: Box<[AST_MULTIPLICATIVE_OP]>,
    lues_len: usize,
    line: usize,
    pos: usize,
) -> Box<MULTIPLICATIVE_EXPR_AST> {
    Box::new(MULTIPLICATIVE_EXPR_AST {
        lues,
        ops,
        lues_len,
        line,
        pos,
    })
}

fn dump_multiplicative_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a MULTIPLICATIVE_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.lues_len == 1 {
        if let Some(ref lue) = ast.lues[0] {
            dump_left_unary_expr_ast_to_file_inner(f, lue, spaces_num);
        }
    } else {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<multiplicative_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        if let Some(ref lue) = ast.lues[0] {
            dump_left_unary_expr_ast_to_file_inner(f, lue, spaces_num);
        }
        for i in 1..ast.lues_len {
            let _ = write!(f, "{:width$}", "", spaces_num);
            match ast.ops[i - 1] {
                AST_MULTIPLICATIVE_OP::Mul => {
                    let _ = writeln!(f, "<op>MUL</op>");
                }
                AST_MULTIPLICATIVE_OP::Div => {
                    let _ = writeln!(f, "<op>DIV</op>");
                }
                AST_MULTIPLICATIVE_OP::Mod => {
                    let _ = writeln!(f, "<op>MOD</op>");
                }
            }
            if let Some(ref lue) = ast.lues[i] {
                dump_left_unary_expr_ast_to_file_inner(f, lue, spaces_num);
            }
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</multiplicative_expr>");
    }
}

pub fn dump_multiplicative_expr_ast_to_file<'a>(
    f: &'a mut dyn Write,
    ast: &'a MULTIPLICATIVE_EXPR_AST,
) {
    dump_multiplicative_expr_ast_to_file_inner(f, ast, 0);
}

pub fn multiplicative_expr_ast_free(_ast: &mut MULTIPLICATIVE_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_LEFT_UNARY_OP {
    Plus,
    Minus,
}

pub struct LEFT_UNARY_EXPR_AST {
    pub op: AST_LEFT_UNARY_OP,
    pub expr: Option<Box<PRIMARY_EXPR_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_left_unary_expr_ast(
    op: AST_LEFT_UNARY_OP,
    expr: Option<Box<PRIMARY_EXPR_AST>>,
    line: usize,
    pos: usize,
) -> Box<LEFT_UNARY_EXPR_AST> {
    Box::new(LEFT_UNARY_EXPR_AST {
        op,
        expr,
        line,
        pos,
    })
}

fn dump_left_unary_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a LEFT_UNARY_EXPR_AST,
    mut spaces_num: usize,
) {
    if ast.op != AST_LEFT_UNARY_OP::Plus {
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(
            f,
            "<left_unary_expr line=\"{}\" pos=\"{}\">",
            ast.line, ast.pos
        );
        spaces_num += 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "<op>UNARY_MINUS</op>");
        if let Some(ref expr) = ast.expr {
            dump_primary_expr_ast_to_file_inner(f, expr, spaces_num);
        }
        spaces_num -= 2;
        let _ = write!(f, "{:width$}", "", spaces_num);
        let _ = writeln!(f, "</left_unary_expr>");
    } else {
        if let Some(ref expr) = ast.expr {
            dump_primary_expr_ast_to_file_inner(f, expr, spaces_num);
        }
    }
}

pub fn dump_left_unary_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a LEFT_UNARY_EXPR_AST) {
    dump_left_unary_expr_ast_to_file_inner(f, ast, 0);
}

pub fn left_unary_expr_ast_free(_ast: &mut LEFT_UNARY_EXPR_AST) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AST_PRIMARY_EXPR_TYPE {
    HasProperty,
    Len,
    FunctionCall,
    Variable,
    Number,
    LogicalOrExpr,
}

pub enum PrimaryExprAstInner {
    HasProperty(Box<HAS_PROPERTY_EXPR_AST>),
    Len(Box<LEN_EXPR_AST>),
    FunctionCall(Box<FUNCTION_CALL_AST>),
    Variable(Box<VARIABLE_AST>),
    Number(Box<NUMBER_AST>),
    LogicalOrExpr(Box<LOGICAL_OR_EXPR_AST>),
}

pub struct PRIMARY_EXPR_AST {
    pub inner: PrimaryExprAstInner,
    pub expr_type: AST_PRIMARY_EXPR_TYPE,
}

pub fn create_primary_expr_ast(
    primary_expr_ptr: PrimaryExprAstInner,
    expr_type: AST_PRIMARY_EXPR_TYPE,
) -> Box<PRIMARY_EXPR_AST> {
    Box::new(PRIMARY_EXPR_AST {
        inner: primary_expr_ptr,
        expr_type,
    })
}

fn dump_primary_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a PRIMARY_EXPR_AST,
    spaces_num: usize,
) {
    match &ast.inner {
        PrimaryExprAstInner::HasProperty(expr) => {
            dump_has_property_expr_ast_to_file_inner(f, expr, spaces_num)
        }
        PrimaryExprAstInner::Len(expr) => dump_len_expr_ast_to_file_inner(f, expr, spaces_num),
        PrimaryExprAstInner::FunctionCall(expr) => {
            dump_function_call_ast_to_file_inner(f, expr, spaces_num)
        }
        PrimaryExprAstInner::Variable(var) => dump_variable_ast_to_file_inner(f, var, spaces_num),
        PrimaryExprAstInner::Number(num) => dump_number_ast_to_file_inner(f, num, spaces_num),
        PrimaryExprAstInner::LogicalOrExpr(expr) => {
            dump_logical_or_expr_ast_to_file_inner(f, expr, spaces_num)
        }
    }
}

pub fn dump_primary_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a PRIMARY_EXPR_AST) {
    dump_primary_expr_ast_to_file_inner(f, ast, 0);
}

pub fn primary_expr_ast_free(_ast: &mut PRIMARY_EXPR_AST) {}

pub struct HAS_PROPERTY_EXPR_AST {
    pub obj: Option<Box<VARIABLE_AST>>,
    pub ident: Option<Box<IDENT_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_has_property_expr_ast(
    obj: Option<Box<VARIABLE_AST>>,
    ident: Option<Box<IDENT_AST>>,
    line: usize,
    pos: usize,
) -> Box<HAS_PROPERTY_EXPR_AST> {
    Box::new(HAS_PROPERTY_EXPR_AST {
        obj,
        ident,
        line,
        pos,
    })
}

fn dump_has_property_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a HAS_PROPERTY_EXPR_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<has_property_expr line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    if let Some(ref obj) = ast.obj {
        dump_variable_ast_to_file_inner(f, obj, spaces_num);
    }
    if let Some(ref ident) = ast.ident {
        dump_ident_ast_to_file_inner(f, ident, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</has_property_expr>");
}

pub fn dump_has_property_expr_ast_to_file<'a>(
    f: &'a mut dyn Write,
    ast: &'a HAS_PROPERTY_EXPR_AST,
) {
    dump_has_property_expr_ast_to_file_inner(f, ast, 0);
}

pub fn has_property_expr_ast_free(_ast: &mut HAS_PROPERTY_EXPR_AST) {}

pub struct LEN_EXPR_AST {
    pub arr: Option<Box<VARIABLE_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_len_expr_ast(
    arr: Option<Box<VARIABLE_AST>>,
    line: usize,
    pos: usize,
) -> Box<LEN_EXPR_AST> {
    Box::new(LEN_EXPR_AST { arr, line, pos })
}

fn dump_len_expr_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a LEN_EXPR_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<len_expr line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    if let Some(ref arr) = ast.arr {
        dump_variable_ast_to_file_inner(f, arr, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</len_expr>");
}

pub fn dump_len_expr_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a LEN_EXPR_AST) {
    dump_len_expr_ast_to_file_inner(f, ast, 0);
}

pub fn len_expr_ast_free(_ast: &mut LEN_EXPR_AST) {}

pub struct FUNCTION_CALL_AST {
    pub function_name: Option<Box<IDENT_AST>>,
    pub args_list: Option<Box<ARGS_LIST_AST>>,
    pub line: usize,
    pub pos: usize,
}

pub fn create_function_call_ast(
    function_name: Option<Box<IDENT_AST>>,
    args_list: Option<Box<ARGS_LIST_AST>>,
    line: usize,
    pos: usize,
) -> Box<FUNCTION_CALL_AST> {
    Box::new(FUNCTION_CALL_AST {
        function_name,
        args_list,
        line,
        pos,
    })
}

fn dump_function_call_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a FUNCTION_CALL_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<function_call line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<function_name line=\"{}\" pos=\"{}\">",
        ast.line, ast.pos
    );
    spaces_num += 2;
    if let Some(ref name) = ast.function_name {
        dump_ident_ast_to_file_inner(f, name, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</function_name>");
    if let Some(ref args) = ast.args_list {
        dump_args_list_ast_to_file_inner(f, args, spaces_num);
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</function_call>");
}

pub fn dump_function_call_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a FUNCTION_CALL_AST) {
    dump_function_call_ast_to_file_inner(f, ast, 0);
}

pub fn function_call_ast_free(_ast: &mut FUNCTION_CALL_AST) {}

pub struct ARGS_LIST_AST {
    pub assignment_exprs: Box<[Option<Box<ASSIGNMENT_EXPR_AST>>]>,
    pub assignment_exprs_len: usize,
    pub line: usize,
    pub pos: usize,
}

pub fn create_args_list_ast(
    assignment_exprs: Box<[Option<Box<ASSIGNMENT_EXPR_AST>>]>,
    assignment_exprs_len: usize,
    line: usize,
    pos: usize,
) -> Box<ARGS_LIST_AST> {
    Box::new(ARGS_LIST_AST {
        assignment_exprs,
        assignment_exprs_len,
        line,
        pos,
    })
}

fn dump_args_list_ast_to_file_inner<'a>(
    f: &'a mut dyn Write,
    ast: &'a ARGS_LIST_AST,
    mut spaces_num: usize,
) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "<args_list line=\"{}\" pos=\"{}\">", ast.line, ast.pos);
    spaces_num += 2;
    for i in 0..ast.assignment_exprs_len {
        if let Some(ref expr) = ast.assignment_exprs[i] {
            dump_assignment_expr_ast_to_file_inner(f, expr, spaces_num);
        }
    }
    spaces_num -= 2;
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(f, "</args_list>");
}

pub fn dump_args_list_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a ARGS_LIST_AST) {
    dump_args_list_ast_to_file_inner(f, ast, 0);
}

pub fn args_list_ast_free(_ast: &mut ARGS_LIST_AST) {}

pub struct IDENT_AST {
    pub ident: [u8; 32],
    pub line: usize,
    pub pos: usize,
}

pub fn create_ident_ast(ident: &str, line: usize, pos: usize) -> Box<IDENT_AST> {
    let mut ident_array = [0u8; 32];
    let bytes = ident.as_bytes();
    let copy_len = std::cmp::min(bytes.len(), 31);
    ident_array[..copy_len].copy_from_slice(&bytes[..copy_len]);
    ident_array[copy_len] = 0;
    Box::new(IDENT_AST {
        ident: ident_array,
        line,
        pos,
    })
}

fn dump_ident_ast_to_file_inner<'a>(f: &'a mut dyn Write, ast: &'a IDENT_AST, spaces_num: usize) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let ident_str = std::str::from_utf8(&ast.ident)
        .unwrap_or("")
        .trim_end_matches(char::from(0));
    let _ = writeln!(
        f,
        "<ident line=\"{}\" pos=\"{}\">{}</ident>",
        ast.line, ast.pos, ident_str
    );
}

pub fn dump_ident_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a IDENT_AST) {
    dump_ident_ast_to_file_inner(f, ast, 0);
}

pub fn ident_ast_free(_ast: &mut IDENT_AST) {}

pub struct NUMBER_AST {
    pub number: i64,
    pub line: usize,
    pub pos: usize,
}

pub fn create_number_ast(number: i64, line: usize, pos: usize) -> Box<NUMBER_AST> {
    Box::new(NUMBER_AST { number, line, pos })
}

fn dump_number_ast_to_file_inner<'a>(f: &'a mut dyn Write, ast: &'a NUMBER_AST, spaces_num: usize) {
    let _ = write!(f, "{:width$}", "", spaces_num);
    let _ = writeln!(
        f,
        "<number line=\"{}\" pos=\"{}\">{}</number>",
        ast.line, ast.pos, ast.number
    );
}

pub fn dump_number_ast_to_file<'a>(f: &'a mut dyn Write, ast: &'a NUMBER_AST) {
    dump_number_ast_to_file_inner(f, ast, 0);
}

pub fn number_ast_free(_ast: &mut NUMBER_AST) {}
