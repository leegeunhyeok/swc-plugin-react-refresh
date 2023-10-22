use swc_core::ecma::{
    ast::*,
    atoms::Atom,
};
use swc_common::{Span, DUMMY_SP};

/// Check provided name is valid React component name.
/// 
/// Returns `true` if name starts with capitalize.
/// 
/// - MyComponent: `true`
/// - myComponent: `false`
pub fn is_react_component_name(name: &String) -> bool {
    name.chars().nth(0).unwrap().is_uppercase()
}

/// Get symbol name from `Ident`.
pub fn get_name_from_ident(ident: &Ident) -> String {
    ident.sym.to_string()
}

/// Returns an identify.
pub fn ident(sym: Atom) -> Ident {
    Ident::new(sym, DUMMY_SP)
}

/// Returns an identify by string.
pub fn ident_str(sym: &String, span: Span) -> Ident {
    Ident::new(sym.to_owned().into(), span)
}

/// Returns an identify expression expression.
pub fn ident_expr(sym: Atom) -> Expr {
    Expr::Ident(ident(sym))
}

/// Returns an identify expression by string.
pub fn ident_str_expr(sym: &String, span: Span) -> Expr {
    Expr::Ident(ident_str(sym.into(), span))
}

/// Returns an string literal expression.
pub fn str_expr(value: &String) -> Expr {
    Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: value.to_owned().into(),
        raw: None,
    }))
}

/// Returns an bool literal expression.
pub fn bool_expr(value: bool) -> Expr {
    Expr::Lit(Lit::Bool(value.into()))
}

/// Returns an function argument expression.
pub fn arg_expr(expr: Expr) -> ExprOrSpread {
    ExprOrSpread {
        expr: Box::new(expr),
        spread: None,
    }
}

/// Returns an expression that call function with arguments.
///
/// Code: `obj.prop`
pub fn obj_prop_expr(obj: Expr, prop: Ident) -> Expr {
    Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(obj),
        prop: MemberProp::Ident(prop),
    })
}

/// Returns an expression that assign right to left.
///
/// Code: `left = right`
pub fn assign_expr(left: Expr, right: Expr) -> Expr {
    Expr::Assign(AssignExpr {
        span: DUMMY_SP,
        op: AssignOp::Assign,
        left: PatOrExpr::Expr(Box::new(left)),
        right: Box::new(right),
    })
}

/// Returns an expression that call function with arguments.
///
/// Code: `callee(arg1, arg2, ...)`
pub fn call_expr(callee: Expr, args:  Vec<ExprOrSpread>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(callee)),
        args: args,
        type_args: None,
    })
}

/// Returns a statement that declare variable and assign.
///
/// Code: `var name = init;`;
pub fn decl_var_and_assign_stmt(name: Ident, init: Expr) -> Stmt {
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Var,
        declare: false,
        decls: vec![
            VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent {
                    id: name,
                    type_ann: None,
                }),
                init: Some(Box::new(init)),
                definite: false,
            },
        ],
    })))
}

/// Returns expr statement.
pub fn to_stmt(expr: Expr) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(expr),
    })
}
