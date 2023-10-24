use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component,
    // Input codes
    r#"
    function Component() {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    function Component() {
        return <div>{'Hello World'}</div>;
    };
    global.$RefreshReg$(Component, "Component");
    global.$RefreshRuntime$.getContext(Component).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_default_export,
    // Input codes
    r#"
    export default function() {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    export default function() {
        return <div>{'Hello World'}</div>;
    };
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_default_export_with_name,
    // Input codes
    r#"
    export default function ComponentDefault() {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export default function ComponentDefault() {
        return <div>{'Hello World'}</div>;
    };
    global.$RefreshReg$(ComponentDefault, "ComponentDefault");
    global.$RefreshRuntime$.getContext(ComponentDefault).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_default_export_from_var,
    // Input codes
    r#"
    function ComponentDefaultFromVar() {
        return <div>{'Hello World'}</div>;
    };

    export default ComponentDefaultFromVar;
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    function ComponentDefaultFromVar() {
        return <div>{'Hello World'}</div>;
    };
    export default ComponentDefaultFromVar;
    global.$RefreshReg$(ComponentDefaultFromVar, "ComponentDefaultFromVar");
    global.$RefreshRuntime$.getContext(ComponentDefaultFromVar).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_named_export,
    // Input codes
    r#"
    function ComponentNamedExport() {
        return <div>{'Hello World'}</div>;
    };

    export { ComponentNamedExport };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    function ComponentNamedExport() {
        return <div>{'Hello World'}</div>;
    };
    export { ComponentNamedExport };
    global.$RefreshReg$(ComponentNamedExport, "ComponentNamedExport");
    global.$RefreshRuntime$.getContext(ComponentNamedExport).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_named_export_as_rename,
    // Input codes
    r#"
    function ComponentNamedExportAs() {
        return <div>{'Hello World'}</div>;
    };

    export { ComponentNamedExportAs as Rename };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    function ComponentNamedExportAs() {
        return <div>{'Hello World'}</div>;
    };
    export { ComponentNamedExportAs as Rename };
    global.$RefreshReg$(ComponentNamedExportAs, "ComponentNamedExportAs");
    global.$RefreshRuntime$.getContext(ComponentNamedExportAs).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    function_component_named_export_with_declare,
    // Input codes
    r#"
    export function ComponentNamedExportDeclare() {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export function ComponentNamedExportDeclare() {
        return <div>{'Hello World'}</div>;
    };
    global.$RefreshReg$(ComponentNamedExportDeclare, "ComponentNamedExportDeclare");
    global.$RefreshRuntime$.getContext(ComponentNamedExportDeclare).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);
