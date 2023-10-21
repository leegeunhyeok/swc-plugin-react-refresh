use std::collections::HashSet;
use swc_core::ecma::{
  ast::*,
  atoms::{js_word, Atom},
  visit::{noop_fold_type, VisitWith, Fold, FoldWith},
  transforms::testing::test,
};
use crate::{utils::{
    ident,
    ident_expr,
    ident_str_expr,
    str_expr,
    bool_expr,
    arg_expr,
    obj_prop_expr,
    assign_expr,
    call_expr,
    decl_var_and_assign_stmt,
    to_stmt,
    get_name_from_ident,
    is_react_component_name,
}, visitor};

const GLOBAL: &str = "global";
const REGISTER_REF: &str = "$RefreshReg$";
const SIGNATURE_REF: &str = "$RefreshSig$";
const RUNTIME_REF: &str = "$RefreshRuntime$";
const RUNTIME_GET_REGISTER_FN: &str = "getRegisterFunction";
const RUNTIME_GET_SIGNATURE_FN: &str = "getCreateSignatureFunction";
const TEMP_REGISTER_REF: &str = "__prevRefreshReg";
const TEMP_SIGNATURE_REF: &str = "__prevRefreshSig";
const SIGNATURE_FN: &str = "__s";

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
        to_stmt(call_expr(ident_expr(js_word!(SIGNATURE_FN)), vec![]))
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
        let mut is_custom_hook = false;
        if let Some(callee_expr) = call_expr.callee.as_expr() {
            // Check if this expression is hook like a `React.useXXX()`.
            if let Some(ident) = callee_expr.as_ident() {
                let hook_name = ident.sym.to_string();
                if !BUILTIN_HOOKS.contains(&hook_name.as_str()) && hook_name.starts_with("use") {
                    is_custom_hook = true;
                }
            }
        }
        is_custom_hook
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
    fn get_id(&self, identifier: String) -> String {
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

        if !is_react_component_name(&component_name) {
            return false;
        }

        if !(self.component_names.contains(&component_name) || self.black_list.contains(&component_name)) {
            let fold_component_inner = &mut ReactRefreshRuntimeComponent::default();
            self.module_body.push(module.to_owned().fold_children_with(fold_component_inner));
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
                obj_prop_expr(
                    ident_expr(js_word!(GLOBAL)),
                    ident(js_word!(SIGNATURE_REF)),
                ),
                vec![],
            ),
        )
    }

    /// Returns a statement that call the created signature function.
    ///
    /// Code: `__s(Component, "module_id", has_custom_hook_call);`
    fn get_call_signature_fn_stmt(&self, component_name: String, has_custom_hook_call: bool) -> Stmt {
        to_stmt(call_expr(
            ident_expr(js_word!(SIGNATURE_FN)),
            vec![
                arg_expr(ident_str_expr(&component_name)),
                arg_expr(str_expr(&self.get_id(component_name))),
                arg_expr(bool_expr(has_custom_hook_call)),
            ],
        ))
    }

    /// Returns a statement that call the register function.
    ///
    /// Code: `global.$RefreshRef$(Component, "module_id");`
    fn get_call_register_fn_stmt(&self, component_name: String) -> Stmt {
        to_stmt(call_expr(
            obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(REGISTER_REF))),
            vec![
                arg_expr(ident_str_expr(&component_name)),
                arg_expr(str_expr(&self.get_id(component_name))),
            ],
        ))
    }

    /// Returns a statement that call the HMR accept method.
    ///
    /// Code: `global.__hmr__(Component, "module_id").accept();`
    fn get_call_accept_stmt(&self, component_name: String) -> Stmt {
        to_stmt(call_expr(
            obj_prop_expr(
                call_expr(
                    obj_prop_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(HMR_REF))),
                    vec![
                        arg_expr(ident_str_expr(&component_name)),
                        arg_expr(str_expr(&self.get_id(component_name))),
                    ],
                ),
                ident(js_word!(HMR_ACCEPT_FN)),
            ),
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

            // 4. If React component not found, use original statement.
            if !is_folded {
                self.module_body.push(module.to_owned());
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
                meta.name.to_owned(),
                meta.has_custom_hook_call,
            )));
            self.module_body.push(ModuleItem::Stmt(self.get_call_register_fn_stmt(meta.name.to_owned())));
            self.module_body.push(ModuleItem::Stmt(self.get_call_accept_stmt(meta.name.to_owned())));
        }

        // Finally, restore original react-refresh functions.
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
    const ArrowComponent = () => {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    const ArrowComponent = ()=>{
        __s();
        return <div>{'Hello World'}</div>;
    };
    __s(ArrowComponent, "test:ArrowComponent", false);
    global.$RefreshReg$(ArrowComponent, "test:ArrowComponent");
    global.__hmr__(ArrowComponent, "test:ArrowComponent").accept();
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
    const ArrowComponentDefaultFromVar = () => {
        return <div>{'Hello World'}</div>;
    };

    export default ArrowComponentDefaultFromVar;
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    const ArrowComponentDefaultFromVar = ()=>{
        __s();
        return <div>{'Hello World'}</div>;
    };
    export default ArrowComponentDefaultFromVar;
    __s(ArrowComponentDefaultFromVar, "test:ArrowComponentDefaultFromVar", false);
    global.$RefreshReg$(ArrowComponentDefaultFromVar, "test:ArrowComponentDefaultFromVar");
    global.__hmr__(ArrowComponentDefaultFromVar, "test:ArrowComponentDefaultFromVar").accept();
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
    external_component_default_import,
    // Input codes
    r#"
    import RootComponent from 'app/core';

    export { RootComponent };
    "#,
    // Output
    r#"
    import RootComponent from 'app/core';

    export { RootComponent };
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| ReactRefreshRuntime::default(String::from("test")),
    external_component_named_import,
    // Input codes
    r#"
    import { Button, Text } from 'app/design-system';

    export { Button, Text };
    "#,
    // Output
    r#"
    import { Button, Text } from 'app/design-system';

    export { Button, Text };
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| ReactRefreshRuntime::default(String::from("test")),
    class_component,
    // Input codes
    r#"
    class ClassComponent extends React.Component {
        render () {
            return <div>{'Hello, World'}</div>;
        }
    }
    "#,
    // Output
    r#"
    class ClassComponent extends React.Component {
        render () {
            return <div>{'Hello, World'}</div>;
        }
    }
    "#
);
