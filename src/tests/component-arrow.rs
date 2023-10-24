use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
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
    const ArrowComponent = ()=>{
        return <div>{'Hello World'}</div>;
    };
    global.$RefreshReg$(ArrowComponent, "ArrowComponent");
    global.$RefreshRuntime$.getContext(ArrowComponent).accept();
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
    |_| react_refresh(String::from("test")),
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
    const ArrowComponentDefaultFromVar = ()=>{
        return <div>{'Hello World'}</div>;
    };
    export default ArrowComponentDefaultFromVar;
    global.$RefreshReg$(ArrowComponentDefaultFromVar, "ArrowComponentDefaultFromVar");
    global.$RefreshRuntime$.getContext(ArrowComponentDefaultFromVar).accept();
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
    arrow_function_component_named_export,
    // Input codes
    r#"
    const ArrowComponentNamedExport = () => {
        return <div>{'Hello World'}</div>;
    };

    export { ArrowComponentNamedExport };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    const ArrowComponentNamedExport = ()=>{
        return <div>{'Hello World'}</div>;
    };
    export { ArrowComponentNamedExport };
    global.$RefreshReg$(ArrowComponentNamedExport, "ArrowComponentNamedExport");
    global.$RefreshRuntime$.getContext(ArrowComponentNamedExport).accept();
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
    arrow_function_component_named_export_as_rename,
    // Input codes
    r#"
    const ArrowComponentNamedExportAs = () => {
        return <div>{'Hello World'}</div>;
    };

    export { ArrowComponentNamedExportAs as Rename };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    const ArrowComponentNamedExportAs = ()=>{
        return <div>{'Hello World'}</div>;
    };
    export { ArrowComponentNamedExportAs as Rename };
    global.$RefreshReg$(ArrowComponentNamedExportAs, "ArrowComponentNamedExportAs");
    global.$RefreshRuntime$.getContext(ArrowComponentNamedExportAs).accept();
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
    arrow_function_component_named_export_with_declare,
    // Input codes
    r#"
    export const ArrowComponentNamedExportDeclare = () => {
        return <div>{'Hello World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const ArrowComponentNamedExportDeclare = ()=>{
        return <div>{'Hello World'}</div>;
    };
    global.$RefreshReg$(ArrowComponentNamedExportDeclare, "ArrowComponentNamedExportDeclare");
    global.$RefreshRuntime$.getContext(ArrowComponentNamedExportDeclare).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);
