use std::collections::HashSet;
use swc_core::ecma::{
  ast::*,
  atoms::{js_word, Atom},
  visit::{noop_fold_type, Fold, FoldWith},
  transforms::testing::test,
};
use swc_common::DUMMY_SP;

/// Before using this plugin, you should inject runtime code below.
/// 
/// ```js
/// const RefreshRuntime = require('react-refresh/runtime');
/// 
/// const hmrContext = {};
/// const createHmrContext = (id) => {
///   const state = {
///     timeout: null,
///     accepted: false,
///     disposed: false,
///   };
///
///   const hot = {
///     accept: () => {
///       if (state.disposed) {
///         throw new Error('HMR module was disposed');
///       }
///   
///       if (state.accepted) {
///         throw new Error('HMR already accepted');
///       }
/// 
///       state.accepted = true;
///       state.timeout = setTimeout(() => {
///         state.timeout = null;
///         RefreshRuntime.performReactRefresh();
///       }, 50);
///     },
///     dispose: () => {
///       state.disposed = true;
///     },
///   };
/// 
///   if (hmrContext[id]) {
///     hmrContext[id].dispose();
///   }
/// 
///   hmrContext[id] = hot;
///
///   return hot;
/// };
/// 
/// RefreshRuntime.injectIntoGlobalHook(global);
/// global.$RefreshReg$ = () => {};
/// global.$RefreshSig$ = (typw) => () => type;
/// global.$RefreshRuntime$ = RefreshRuntime;
/// global.__hmr__ = {
///   accept: (id) => {
///     return createHmrContext(id);
///   },
/// };
/// ```
const GLOBAL: &str = "global";
const REGISTER_REF: &str = "$RefreshReg$";
const SIGNATURE_REF: &str = "$RefreshSig$";
const RUNTIME_REF: &str = "$RefreshRuntime$";
const TEMP_REGISTER_REF: &str = "__prevRefreshReg";
const TEMP_SIGNATURE_REF: &str = "__prevRefreshSig";
const SIGNATURE_FN: &str = "__s";

const REACT_REFRESH_REGISTER_FN: &str = "register";
const REACT_REFRESH_CREATE_SIGNATURE_FN: &str = "createSignatureFunctionForTransform";

const HMR_REF: &str = "__hmr__";
const HMR_ACCEPT_FN: &str = "accept";

const BUILTIN_HOOKS: &'static [&'static str] = &[
    "useState",
    "useReducer",
    "useEffect",
    "useLayoutEffect",
    "useMemo",
    "useCallback",
    "useRef",
    "useContext",
    "useImperativeHandle",
    "useDebugValue",
];

struct ComponentMeta {
    name: String,
    has_custom_hook_call: bool,
}

/// For add the empty signature function call expression into React component
/// and check if any custom hooks are used.
struct ReactRefreshRuntimeComponent {
    has_custom_hook_call: bool,
}

impl ReactRefreshRuntimeComponent {
    fn default() -> ReactRefreshRuntimeComponent {
        ReactRefreshRuntimeComponent { has_custom_hook_call: false }
    }

    /// Returns a statement that call the signature function without arguments.
    ///
    /// Code: `__s();`
    fn get_signature_call_stmt(&self) -> Stmt {
        // __s()
        let call_signature_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(
                Expr::Ident(Ident::new(js_word!(SIGNATURE_FN), DUMMY_SP)))
            ),
            args: vec![],
            type_args: None,
        });

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(call_signature_expr),
        })
    }

    fn find_custom_hook_call_from_stmt(&self, stmt: &Stmt) -> bool {
        let mut has_custom_hook = false;
        // There is two type of call hooks.
        //
        // 1. Call hook only (eg: `useCallback()`)
        // 2. Call hook and assign value to variable (eg: `const [...] = useState(0)`)
        if let Some(call_expr) = stmt.as_expr().and_then(|expr_stmt| expr_stmt.expr.as_call()) {
            has_custom_hook = has_custom_hook || self.is_custom_hook_call(call_expr);
        } else if let Some(var_decl_stmt) = stmt.as_decl().and_then(|decl_stmt| decl_stmt.as_var()) {
            has_custom_hook = var_decl_stmt.decls.iter().any(|decl| {
                decl.init.as_ref().and_then(|init_expr| init_expr.as_call()).map_or(false, |call_expr| {
                    self.is_custom_hook_call(call_expr)
                })
            });
        }
        has_custom_hook
    }

    fn is_custom_hook_call(&self, call_expr: &CallExpr) -> bool {
        if let Some(callee_expr) = call_expr.callee.as_expr() {
            // Check if this expression is hook like a `React.useXXX()`
            if let Some(ident) = callee_expr.as_ident() {
                let hook_name = ident.sym.to_string();
                if BUILTIN_HOOKS.contains(&hook_name.as_str()) {
                    return false;
                } else if hook_name.starts_with("use") {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Fold for ReactRefreshRuntimeComponent {
    fn fold_block_stmt(&mut self, mut block_stmt: BlockStmt) -> BlockStmt {
        for stmt in block_stmt.stmts.iter() {
            // Explore all of function call statements and check if any custom hooks are used.
            if self.find_custom_hook_call_from_stmt(stmt) {
                self.has_custom_hook_call = true;
                break;
            }
        }

        // Add `__s();` at the top inside the component.
        //
        // In `react-refresh/runtime` comment,
        // it says calling `__s()` without arguments will trigger to collect hooks.
        block_stmt.stmts.insert(0, self.get_signature_call_stmt());
        block_stmt
    }
}

/// Find React components from module.
/// And then add signature, register components and accept for HMR.
pub struct ReactRefreshRuntime {
    module_id: String,
    module_body: Vec<ModuleItem>,
    component_list: Vec<ComponentMeta>,
    component_names: HashSet<String>,
}

impl ReactRefreshRuntime {
    fn default(module_id: String) -> ReactRefreshRuntime {
        ReactRefreshRuntime {
            module_id,
            module_body: Vec::new(),
            component_list: Vec::new(),
            component_names: HashSet::new(),
        }
    }

    fn initialize_before_fold_module(&mut self) {
        self.module_body.clear();
        self.component_list.clear();
        self.component_names.clear();
    }

    /// Get symbol name from `Ident`
    fn get_name_from_ident(&self, ident: &Ident) -> String {
        ident.sym.to_string()
    }

    /// Returns id
    fn get_id(&self, identifier: &String) -> String {
        let mut owned_string = self.module_id.clone();
        owned_string.push_str(":");
        owned_string.push_str(identifier.as_str());
        owned_string
    }

    /// Check provided name is valid React component name.
    /// 
    /// Returns `true` if name starts with capitalize.
    /// 
    /// - MyComponent: `true`
    /// - myComponent: `false`
    fn is_react_component_name(&self, name: &String) -> bool {
        // starts with capital character
        name.chars().nth(0).unwrap().is_uppercase()
    }

    /// Fold with ReactRefreshRuntimeComponent if it is valid React component.
    /// 
    /// Returns `isFolded`
    fn fold_if_react_component(&mut self, module: &ModuleItem, ident: &Ident) -> bool {
        let component_name = self.get_name_from_ident(ident);

        if self.is_react_component_name(&component_name) && !self.component_names.contains(&component_name) {
            let fold_component_inner = &mut ReactRefreshRuntimeComponent::default();
            self.module_body.push(module.clone().fold_children_with(fold_component_inner));
            self.component_names.insert(component_name.to_owned());
            self.component_list.push(ComponentMeta {
                name: component_name.to_owned(),
                has_custom_hook_call: fold_component_inner.has_custom_hook_call,
            });
            return true;
        }
        false
    }

    /// Returns a statement that temporarily stores the registration function.
    /// 
    /// Code: `var __prevRefreshRef = global.$RefreshRef$;`
    /// Code: `var __prevRefreshSig = global.$RefreshSig$;`
    fn get_assign_temp_ref_fn_stmt(&self, var: Atom, prop: Atom) -> Stmt {
        // global.$RefreshXXX$
        let access_to_global_target = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
            prop: MemberProp::Ident(Ident::new(prop, DUMMY_SP)),
        });

        // var __prevRefreshXXX = {access_to_global_target};
        let assign_target_to_stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new(var, DUMMY_SP),
                        type_ann: None,
                    }),
                    definite: false,
                    init: Some(Box::new(access_to_global_target)),
                },
            ],
        })));

        assign_target_to_stmt
    }

    /// Returns a statement that define register function and override.
    ///
    /// Code:
    /// ```js
    /// global.$RefreshRuntime$ = function (type, id) {
    ///   global.$RefreshRuntime$.register(type, id);
    /// }
    /// ```
    fn get_assign_register_fn_stmt(&self) -> Stmt {
        // global.$RefreshRuntime$.register(type, id);
        let call_register_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(
                Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                        prop: MemberProp::Ident(
                            Ident::new(js_word!(RUNTIME_REF), DUMMY_SP),
                        ),
                    })),
                    prop: MemberProp::Ident(Ident::new(js_word!(REACT_REFRESH_REGISTER_FN), DUMMY_SP)),
                })),
            ),
            args: vec![
                ExprOrSpread {
                    expr: Box::new(Expr::Ident(Ident::new(js_word!("type"), DUMMY_SP))),
                    spread: None,
                },
                ExprOrSpread {
                    expr: Box::new(Expr::Ident(Ident::new(js_word!("id"), DUMMY_SP))),
                    spread: None,
                },
            ],
            type_args: None,
        });

        // function(type, id) {
        //   global.$RefreshRuntime$.register(type, id);
        // }
        let define_register_fn_expr = Expr::Fn(FnExpr {
            ident: None,
            function: Box::new(Function {
                span: DUMMY_SP,
                params: vec![
                    Param {
                        span: DUMMY_SP,
                        decorators: vec![],
                        pat: Pat::Expr(Box::new(Expr::Ident(
                            Ident::new(js_word!("type"), DUMMY_SP),
                        ))),
                    },
                    Param {
                        span: DUMMY_SP,
                        decorators: vec![],
                        pat: Pat::Expr(Box::new(Expr::Ident(
                            Ident::new(js_word!("id"), DUMMY_SP),
                        ))),
                    },
                ],
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![
                        Stmt::Expr(ExprStmt {
                            span: DUMMY_SP,
                            expr: Box::new(call_register_expr),
                        }),
                    ],
                }),
                decorators: vec![],
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        });

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(
                Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: AssignOp::Assign,
                    left: PatOrExpr::Expr(Box::new(
                        Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                            prop: MemberProp::Ident(Ident::new(js_word!(REGISTER_REF), DUMMY_SP)),
                        }),
                    )),
                    right: Box::new(define_register_fn_expr),
                }),
            ),
        })
    }

    /// Returns a statement that override the signature function variable.
    ///
    /// Code: `global.$RefreshSig$ = global.$RefreshRuntime$.createSignatureFunctionForTransform;`
    fn get_assign_signature_fn_stmt(&self) -> Stmt {
        // global.$RefreshRuntime$.createSignatureFunctionForTransform
        let access_create_signature_fn_expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                prop: MemberProp::Ident(
                    Ident::new(js_word!(RUNTIME_REF), DUMMY_SP),
                ),
            })),
            prop: MemberProp::Ident(Ident::new(js_word!(REACT_REFRESH_CREATE_SIGNATURE_FN), DUMMY_SP)),
        });

        // global.$RefreshSig$ = {access_create_signature_fn_expr};
        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: AssignOp::Assign,
                left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                    prop: MemberProp::Ident(
                        Ident::new(js_word!(SIGNATURE_REF), DUMMY_SP),
                    ),
                }))),
                right: Box::new(access_create_signature_fn_expr),
            })),
        })
    }

    /// Returns a statement that declares the signature function variable
    /// and assigns it after create the signature function.
    ///
    /// Code: `var __s = global.$RefreshSig$();`
    fn get_create_signature_fn_stmt(&self) -> Stmt {
        // global.$RefreshSig$()
        let call_signature_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                prop: MemberProp::Ident(Ident::new(js_word!(SIGNATURE_REF), DUMMY_SP)),
            }))),
            args: vec![],
            type_args: None,
        });

        // var __s = {call_signature_expr};
        Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new(js_word!(SIGNATURE_FN), DUMMY_SP),
                        type_ann: None,
                    }),
                    definite: false,
                    init: Some(Box::new(call_signature_expr)),
                },
            ],
        })))
    }

    /// Returns a statement that call the created signature function.
    ///
    /// Code: `__s(Component, "module_id", has_custom_hook_call);`
    fn get_call_signature_fn_stmt(&self, component_name: &String, has_custom_hook_call: bool) -> Stmt {
        // __s(Component, "module_id", {has_custom_hook_call})
        let call_signature_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(
                Expr::Ident(Ident::new(js_word!(SIGNATURE_FN), DUMMY_SP))),
            ),
            args: vec![
                ExprOrSpread {
                    expr: Box::new(Expr::Ident(Ident::new(component_name.to_owned().into(), DUMMY_SP))),
                    spread: None,
                },
                ExprOrSpread {
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: self.get_id(component_name).into(),
                        raw: None,
                    }))),
                    spread: None,
                },
                ExprOrSpread {
                    expr: Box::new(Expr::Lit(Lit::Bool(has_custom_hook_call.into()))),
                    spread: None,
                },
            ],
            type_args: None,
        });

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(call_signature_expr),
        })
    }

    /// Returns a statement that call the register function.
    ///
    /// Code: `global.$RefreshRef$(Component, "module_id");`
    fn get_call_register_fn_stmt(&self, component_name: &String) -> Stmt {
        // global.$RefreshRef$(Component, "module_id")
        let call_register_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                prop: MemberProp::Ident(Ident::new(js_word!(REGISTER_REF), DUMMY_SP)),
            }))),
            args: vec![
                ExprOrSpread {
                    expr: Box::new(Expr::Ident(Ident::new(component_name.to_owned().into(), DUMMY_SP))),
                    spread: None,
                },
                ExprOrSpread {
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: self.get_id(component_name).into(),
                        raw: None,
                    }))),
                    spread: None,
                },
            ],
            type_args: None,
        });

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(call_register_expr),
        })
    }

    /// Returns a statement that call the HMR accept method.
    ///
    /// Code: `global.__hmr__("module_id").accept();`
    fn get_call_accept_stmt(&self, component_name: &String) -> Stmt {
        // global.__hmr__("module_id")
        let call_hmr_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                prop: MemberProp::Ident(Ident::new(js_word!(HMR_REF), DUMMY_SP)),
            }))),
            args: vec![
                ExprOrSpread {
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: self.get_id(component_name).into(),
                        raw: None,
                    }))),
                    spread: None,
                },
            ],
            type_args: None,
        });

        // {call_hmr_expr}.accept()
        let call_accept_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(call_hmr_expr),
                prop: MemberProp::Ident(Ident::new(js_word!(HMR_ACCEPT_FN), DUMMY_SP)),
            }))),
            args: vec![],
            type_args: None,
        });

        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(call_accept_expr),
        })
    }

    /// Returns a statement that restore the registration function from temporarily variable.
    ///
    /// Code: `global.$RefreshReg$ = __prevRefreshReg;`
    /// Code: `global.$RefreshSig$ = __prevRefreshSeg;`
    ///        prop                  var
    fn get_restore_ref_fn_stmt(&self, prop: Atom, var: Atom) -> Stmt {
        // global.$RefreshReg$
        let access_to_global_target = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
            prop: MemberProp::Ident(Ident::new(prop, DUMMY_SP)),
        });

        // {access_to_global_target} = __prevRefreshReg;
        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: AssignOp::Assign,
                left: PatOrExpr::Expr(Box::new(access_to_global_target)) ,
                right: Box::new(Expr::Ident(Ident::new(var, DUMMY_SP))),
            })),
        })
    }
}

impl Fold for ReactRefreshRuntime {
    noop_fold_type!();

    fn fold_module(&mut self, module: Module) -> Module {
        self.initialize_before_fold_module();
        let mut is_folded: bool;

        for module in module.body.iter() {
            is_folded = false;

            // 1. Find variable declare statements and check it is React component.
            //    - `const MyComponent = () => {};`
            //    - `const MyComponent = function() {};`
            //
            // 2. Find function declare statements and check it is React component.
            //    - `function MyComponent() {}`
            //
            // 3. Find React component exports.
            //    - `export const MyComponent = () => {};`
            //    - `export function MyComponent() {};`
            //    - `export { NamedA, NamedB, NamedC };`
            //    - `export default function MyComponent() {};`
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = module {
                for decl in var_decl.decls.iter() {
                    if let Pat::Ident(ident) = &decl.name {
                        is_folded = self.fold_if_react_component(module, ident);
                    }
                }
            } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) = module {
                is_folded = self.fold_if_react_component(module, &fn_decl.ident);
            } else if let ModuleItem::ModuleDecl(module_decl) = module {
                if let Some(named_export) = module_decl.as_export_decl() {
                    match &named_export.decl {
                        Decl::Var(named_var_export) => {
                            if let Some(named_var_ident) = named_var_export.decls.get(0).and_then(
                                |d| d.name.as_ident(),
                            ) {
                                is_folded = self.fold_if_react_component(module, &named_var_ident.id);
                            }
                        }
                        Decl::Fn(named_fn_export) => {
                            is_folded = self.fold_if_react_component(module, &named_fn_export.ident);
                        }
                        _ => (),
                    }
                } else if let Some(default_export) = module_decl.as_export_default_expr() {
                    if let Some(default_export_ident) = default_export.expr.as_ident() {
                        is_folded = self.fold_if_react_component(module, &default_export_ident);
                    }
                }
            }

            // 3. If React component not found, use original statement.
            if !is_folded {
                self.module_body.push(module.clone());
            }
        }

        let has_defined_component = self.component_names.len() > 0;

        // If some React component defined, insert the code below at the top.
        //
        // var __prevRefreshReg = global.$RefreshReg$;
        // var __prevRefreshSig = global.$RefreshSig$;
        // global.$RefreshSig$ = global.$RefreshRuntime$.createSignatureFunctionForTransform;
        // var __s = global.$RefreshSig$();
        if has_defined_component {
            self.module_body.insert(0, ModuleItem::Stmt(self.get_assign_temp_ref_fn_stmt(
                js_word!(TEMP_REGISTER_REF),
                js_word!(REGISTER_REF),
            )));
            self.module_body.insert(1, ModuleItem::Stmt(self.get_assign_temp_ref_fn_stmt(
                js_word!(TEMP_SIGNATURE_REF),
                js_word!(SIGNATURE_REF),
            )));
            self.module_body.insert(2, ModuleItem::Stmt(self.get_assign_register_fn_stmt()));
            self.module_body.insert(3, ModuleItem::Stmt(self.get_assign_signature_fn_stmt()));
            self.module_body.insert(4, ModuleItem::Stmt(self.get_create_signature_fn_stmt()));
        }

        // Append the code below at the bottom.
        // - call signature
        // - registration
        // - accept (= performReactRefresh)
        //
        // _s(Component, "module_id");
        // global.$RefreshReg$(Component, "module_id");
        // global.__hmr__("module_id_here").accept();
        for meta in self.component_list.iter() {
            self.module_body.push(ModuleItem::Stmt(self.get_call_signature_fn_stmt(
                &meta.name,
                meta.has_custom_hook_call,
            )));
            self.module_body.push(ModuleItem::Stmt(self.get_call_register_fn_stmt(&meta.name)));
            self.module_body.push(ModuleItem::Stmt(self.get_call_accept_stmt(&meta.name)));
        }

        // Finally, restore original react-refresh util function references.
        //
        // global.$RefreshReg$ = __prevRefreshReg;
        // global.$RefreshSig$ = __prevRefreshSig;
        if has_defined_component {
            self.module_body.push(ModuleItem::Stmt(self.get_restore_ref_fn_stmt(
                js_word!(REGISTER_REF),
                js_word!(TEMP_REGISTER_REF)
            )));
            self.module_body.push(ModuleItem::Stmt(self.get_restore_ref_fn_stmt(
                js_word!(SIGNATURE_REF),
                js_word!(TEMP_SIGNATURE_REF),
            )));
        }

        Module {
            body: self.module_body.to_owned(),
            ..module
        }
    }
}

pub fn react_refresh(module_id: String) -> ReactRefreshRuntime {
    ReactRefreshRuntime::default(module_id)
}

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| ReactRefreshRuntime::default(String::from("test")),
    arrow_function_component,
    // Input codes
    r#"
    const Component = () => {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = function(type, id) {
        global.$RefreshRuntime$.register(type, id);
    };
    global.$RefreshSig$ = global.$RefreshRuntime$.createSignatureFunctionForTransform;
    var __s = global.$RefreshSig$();
    const Component = ()=>{
        __s();
        return <div>{'Hello World'}</div>;
    };
    __s(Component, "test:Component", false);
    global.$RefreshReg$(Component, "test:Component");
    global.__hmr__("test:Component").accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| ReactRefreshRuntime::default(String::from("test")),
    arrow_function_component_default_export,
    // Input codes
    r#"
    export default () => {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    export default (()=>{
        return <div>{'Hello World'}</div>;
    });
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| ReactRefreshRuntime::default(String::from("test")),
    arrow_function_component_default_export_from_var,
    // Input codes
    r#"
    const Component = () => {
        return <div>{'Hello World'}</div>;
    };

    export default Component;
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = function(type, id) {
        global.$RefreshRuntime$.register(type, id);
    };
    global.$RefreshSig$ = global.$RefreshRuntime$.createSignatureFunctionForTransform;
    var __s = global.$RefreshSig$();
    const Component = ()=>{
        __s();
        return <div>{'Hello World'}</div>;
    };
    export default Component;
    __s(Component, "test:Component", false);
    global.$RefreshReg$(Component, "test:Component");
    global.__hmr__("test:Component").accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);
