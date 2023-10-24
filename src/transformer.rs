use crate::{
    utils::{
        arg_expr, assign_expr, bool_expr, call_expr, decl_var_and_assign_stmt, get_name_from_ident,
        ident, ident_expr, ident_str_expr, is_componentish_name, obj_prop_expr, str_expr, to_stmt,
    },
    visitor,
};
use std::collections::HashSet;
use swc_common::Span;
use swc_core::ecma::{
    ast::*,
    atoms::{js_word, Atom},
    visit::{noop_fold_type, Fold, FoldWith, VisitWith},
};

const GLOBAL: &str = "global";
const REGISTER_REF: &str = "$RefreshReg$";
const SIGNATURE_REF: &str = "$RefreshSig$";
const RUNTIME_REF: &str = "$RefreshRuntime$";
const RUNTIME_GET_REGISTER_FN: &str = "getRegisterFunction";
const RUNTIME_GET_SIGNATURE_FN: &str = "getCreateSignatureFunction";
const RUNTIME_GET_CONTEXT_FN: &str = "getContext";
const CONTEXT_ACCEPT_FN: &str = "accept";
const TEMP_REGISTER_REF: &str = "__prevRefreshReg";
const TEMP_SIGNATURE_REF: &str = "__prevRefreshSig";
const SIGNATURE_FN: &str = "__s";

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
    span: Span,
    name: String,
    builtin_hook_count: i32,
    custom_hook_count: i32,
}

/// For add the empty signature function call expression into React component
/// and check if any custom hooks are used.
struct ReactRefreshRuntimeComponent {
    is_empty: bool,
    builtin_hook_count: i32,
    custom_hook_count: i32,
}

impl ReactRefreshRuntimeComponent {
    fn default() -> ReactRefreshRuntimeComponent {
        ReactRefreshRuntimeComponent {
            is_empty: false,
            builtin_hook_count: 0,
            custom_hook_count: 0,
        }
    }

    /// Returns a statement that call the signature function without arguments.
    ///
    /// Code: `__s();`
    fn get_signature_call_stmt(&self) -> Stmt {
        to_stmt(call_expr(ident_expr(js_word!(SIGNATURE_FN)), vec![]))
    }

    fn find_hook_call_from_stmt(&mut self, stmt: &Stmt) {
        // There is two type of call hooks.
        //
        // 1. Call hook only (eg: `useCallback()`)
        // 2. Call hook and assign value to variable (eg: `const [...] = useState(0)`)
        if let Some(call_expr) = stmt
            .as_expr()
            .and_then(|expr_stmt| expr_stmt.expr.as_call())
        {
            self.count_hook(call_expr);
        } else if let Some(var_decl_stmt) = stmt.as_decl().and_then(|decl_stmt| decl_stmt.as_var())
        {
            for decl in var_decl_stmt.decls.iter() {
                if let Some(call_expr) =
                    decl.init.as_ref().and_then(|init_expr| init_expr.as_call())
                {
                    self.count_hook(call_expr)
                }
            }
        }
    }

    fn count_hook(&mut self, call_expr: &CallExpr) {
        if let Some(callee_expr) = call_expr.callee.as_expr() {
            // Check if this expression is hook like a `React.useXXX()`.
            if let Some(ident) = callee_expr.as_ident() {
                let hook_name = ident.sym.to_string();
                if BUILTIN_HOOKS.contains(&hook_name.as_str()) {
                    self.builtin_hook_count += 1;
                } else if hook_name.starts_with("use") {
                    self.custom_hook_count += 1;
                }
            }
        }
    }
}

impl Fold for ReactRefreshRuntimeComponent {
    fn fold_block_stmt(&mut self, mut block_stmt: BlockStmt) -> BlockStmt {
        self.is_empty = block_stmt.stmts.len() == 0;

        for stmt in block_stmt.stmts.iter() {
            // Explore all of function call statements.
            self.find_hook_call_from_stmt(stmt)
        }

        // If no hook call found, do nothing.
        if self.builtin_hook_count + self.custom_hook_count == 0 {
            return block_stmt;
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
    black_list: HashSet<String>,
}

impl ReactRefreshRuntime {
    fn default(module_id: String) -> ReactRefreshRuntime {
        ReactRefreshRuntime {
            module_id,
            module_body: Vec::new(),
            component_list: Vec::new(),
            component_names: HashSet::new(),
            black_list: HashSet::new(),
        }
    }

    fn initialize_before_fold_module(&mut self) {
        self.module_body.clear();
        self.component_list.clear();
        self.component_names.clear();
        self.black_list.clear();
    }

    fn prepare_before_fold_module(&mut self, module: &Module) {
        let mut collector = visitor::black_list_collector();
        module.visit_with(&mut collector);
        self.black_list = collector.get_black_list();
    }

    /// Returns id
    fn get_id(&self, identifier: &String) -> String {
        let mut owned_string = self.module_id.to_owned();
        owned_string.push_str(":");
        owned_string.push_str(identifier.as_str());
        owned_string
    }

    /// Fold with ReactRefreshRuntimeComponent if it is valid React component.
    ///
    /// Returns `true` when folded and otherwise returns `false`
    fn fold_if_react_component(&mut self, module: &ModuleItem, ident: &Ident) -> bool {
        let component_name = get_name_from_ident(ident);
        if is_componentish_name(&component_name)
            && !self.component_names.contains(&component_name)
            && !self.black_list.contains(&component_name)
        {
            let component = &mut ReactRefreshRuntimeComponent::default();
            let component_stmt = module.to_owned().fold_children_with(component);

            if !component.is_empty {
                self.module_body.push(component_stmt);
                self.component_names.insert(component_name.to_owned());
                self.component_list.push(ComponentMeta {
                    span: ident.span,
                    name: component_name.to_owned(),
                    builtin_hook_count: component.builtin_hook_count,
                    custom_hook_count: component.custom_hook_count,
                });
                return true;
            }
        }
        false
    }

    /// Fold with ReactRefreshRuntimeComponent if it is valid React component.
    ///
    /// Returns `true` when folded and otherwise returns `false`
    fn fold_var_declarator(&mut self, module: &ModuleItem, var_decl: &VarDeclarator) -> bool {
        if let (Some(ident), Some(init_expr)) = (var_decl.name.as_ident(), var_decl.init.to_owned())
        {
            match *init_expr {
                Expr::Fn(_) => {
                    return self.fold_if_react_component(module, ident);
                }
                Expr::Arrow(_) => {
                    return self.fold_if_react_component(module, ident);
                }
                Expr::Call(_) => {
                    return self.fold_if_react_component(module, ident);
                }
                _ => (),
            }
        }
        false
    }

    /// Returns a statement that temporarily stores the registration function.
    ///
    /// Code: `var __prevRefreshRef = global.$RefreshRef$;`
    /// Code: `var __prevRefreshSig = global.$RefreshSig$;`
    fn get_assign_temp_ref_fn_stmt(&self, var_name: Atom, prop: Atom) -> Stmt {
        decl_var_and_assign_stmt(
            ident(var_name),
            obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(prop)),
        )
    }

    /// Returns a statement that create register function and override.
    ///
    /// Code: `global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();`
    fn get_assign_register_fn_stmt(&self) -> Stmt {
        let left = obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(REGISTER_REF)));
        let right = call_expr(
            obj_prop_expr(
                obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(RUNTIME_REF))),
                ident(js_word!(RUNTIME_GET_REGISTER_FN)),
            ),
            vec![],
        );
        to_stmt(assign_expr(left, right))
    }

    /// Returns a statement that override the signature function variable.
    ///
    /// Code: `global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();`
    fn get_assign_signature_fn_stmt(&self) -> Stmt {
        let left = obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(SIGNATURE_REF)));
        let right = call_expr(
            obj_prop_expr(
                obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(RUNTIME_REF))),
                ident(js_word!(RUNTIME_GET_SIGNATURE_FN)),
            ),
            vec![],
        );
        to_stmt(assign_expr(left, right))
    }

    /// Returns a statement that declares the signature function variable
    /// and assigns it after create the signature function.
    ///
    /// Code: `var __s = global.$RefreshSig$();`
    fn get_create_signature_fn_stmt(&self) -> Stmt {
        decl_var_and_assign_stmt(
            ident(js_word!(SIGNATURE_FN)),
            call_expr(
                obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(SIGNATURE_REF))),
                vec![],
            ),
        )
    }

    /// Returns a statement that call the created signature function.
    ///
    /// Code: `__s(Component, "module_id", has_custom_hook_call);`
    fn get_call_signature_fn_stmt(
        &self,
        component_name: &String,
        span: Span,
        has_custom_hook_call: bool,
    ) -> Stmt {
        to_stmt(call_expr(
            ident_expr(js_word!(SIGNATURE_FN)),
            vec![
                arg_expr(ident_str_expr(component_name, span)),
                arg_expr(str_expr(&self.get_id(component_name))),
                arg_expr(bool_expr(has_custom_hook_call)),
            ],
        ))
    }

    /// Returns a statement that call the register function.
    ///
    /// Code: `global.$RefreshRef$(Component, "Component");`
    fn get_call_register_fn_stmt(&self, component_name: &String, span: Span) -> Stmt {
        to_stmt(call_expr(
            obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(REGISTER_REF))),
            vec![
                arg_expr(ident_str_expr(component_name, span)),
                arg_expr(str_expr(component_name)),
            ],
        ))
    }

    /// Returns a statement that call the HMR accept method.
    ///
    /// Code: `global.$RefreshRuntime$.getContext().accept(Component);`
    fn get_call_accept_stmt(&self, component_name: &String, span: Span) -> Stmt {
        let call_get_ctx_fn = call_expr(
            obj_prop_expr(
                obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(RUNTIME_REF))),
                ident(js_word!(RUNTIME_GET_CONTEXT_FN)),
            ),
            vec![arg_expr(ident_str_expr(component_name, span))],
        );

        to_stmt(call_expr(
            obj_prop_expr(call_get_ctx_fn, ident(js_word!(CONTEXT_ACCEPT_FN))),
            vec![],
        ))
    }

    /// Returns a statement that restore the registration function from temporarily variable.
    ///
    /// Code: `global.$RefreshReg$ = __prevRefreshReg;`
    /// Code: `global.$RefreshSig$ = __prevRefreshSeg;`
    fn get_restore_ref_fn_stmt(&self, prop: Atom, var_name: Atom) -> Stmt {
        to_stmt(assign_expr(
            obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(prop)),
            ident_expr(var_name),
        ))
    }
}

impl Fold for ReactRefreshRuntime {
    noop_fold_type!();

    fn fold_module(&mut self, module: Module) -> Module {
        self.initialize_before_fold_module();
        self.prepare_before_fold_module(&module);
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
                let decl: Option<&VarDeclarator> = var_decl.decls.get(0);
                let is_single_decl = var_decl.decls.len() == 1;
                if let Some(var_decl) = decl {
                    if is_single_decl {
                        is_folded = self.fold_var_declarator(module, var_decl);
                    }
                }
            } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) = module {
                is_folded = self.fold_if_react_component(module, &fn_decl.ident);
            } else if let ModuleItem::ModuleDecl(module_decl) = module {
                if let Some(named_export) = module_decl.as_export_decl() {
                    match &named_export.decl {
                        Decl::Var(named_var_export) => {
                            for var_decl in named_var_export.decls.iter() {
                                is_folded = self.fold_var_declarator(module, var_decl);
                            }
                        }
                        Decl::Fn(named_fn_export) => {
                            is_folded =
                                self.fold_if_react_component(module, &named_fn_export.ident);
                        }
                        _ => (),
                    }
                } else if let Some(default_export) = module_decl.as_export_default_decl() {
                    if let Some(fn_expr) = default_export.decl.as_fn_expr() {
                        if let Some(fn_ident) = &fn_expr.ident {
                            is_folded = self.fold_if_react_component(module, fn_ident);
                        }
                    }
                }
            }

            // 4. If React component not found, use original statement.
            if !is_folded {
                self.module_body.push(module.to_owned());
            }
        }

        let has_defined_component = self.component_names.len() > 0;
        let mut should_assign_sig_fn = false;

        // If some React component defined, insert the code below at the top.
        //
        // var __prevRefreshReg = global.$RefreshReg$;
        // var __prevRefreshSig = global.$RefreshSig$;
        if has_defined_component {
            self.module_body.insert(
                0,
                ModuleItem::Stmt(self.get_assign_temp_ref_fn_stmt(
                    js_word!(TEMP_REGISTER_REF),
                    js_word!(REGISTER_REF),
                )),
            );
            self.module_body.insert(
                1,
                ModuleItem::Stmt(self.get_assign_temp_ref_fn_stmt(
                    js_word!(TEMP_SIGNATURE_REF),
                    js_word!(SIGNATURE_REF),
                )),
            );
            self.module_body
                .insert(2, ModuleItem::Stmt(self.get_assign_register_fn_stmt()));
        }

        // Append the code below at the bottom.
        // - call signature
        // - registration
        // - accept (= performReactRefresh)
        //
        // _s(Component, "module_id");
        // global.$RefreshReg$(Component, "Component");
        // global.$RefreshRuntime$.getContext(Component).accept();
        for component in self.component_list.iter() {
            let has_hook = component.builtin_hook_count + component.custom_hook_count > 0;
            should_assign_sig_fn = should_assign_sig_fn || has_hook;
            if has_hook {
                self.module_body
                    .push(ModuleItem::Stmt(self.get_call_signature_fn_stmt(
                        &component.name,
                        component.span,
                        component.custom_hook_count > 0,
                    )));
            }
            self.module_body.push(ModuleItem::Stmt(
                self.get_call_register_fn_stmt(&component.name, component.span),
            ));
            self.module_body.push(ModuleItem::Stmt(
                self.get_call_accept_stmt(&component.name, component.span),
            ));
        }

        // If any components use hooks, define a signature function.
        //
        // global.$RefreshSig$ = global.$RefreshRuntime$.createSignatureFunctionForTransform;
        // var __s = global.$RefreshSig$();
        if has_defined_component && should_assign_sig_fn {
            self.module_body
                .insert(3, ModuleItem::Stmt(self.get_assign_signature_fn_stmt()));
            self.module_body
                .insert(4, ModuleItem::Stmt(self.get_create_signature_fn_stmt()));
        }

        // Finally, restore original react-refresh functions.
        //
        // global.$RefreshReg$ = __prevRefreshReg;
        // global.$RefreshSig$ = __prevRefreshSig;
        if has_defined_component {
            self.module_body.push(ModuleItem::Stmt(
                self.get_restore_ref_fn_stmt(js_word!(REGISTER_REF), js_word!(TEMP_REGISTER_REF)),
            ));
            self.module_body
                .push(ModuleItem::Stmt(self.get_restore_ref_fn_stmt(
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

#[cfg(test)]
#[path = "./tests/unit.rs"]
mod tests;
