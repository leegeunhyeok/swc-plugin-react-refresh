use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    builtin_hoc_component_anonymous,
    // Input codes
    r#"
    const MemoComponentA = React.memo(() => {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    const MemoComponentA = React.memo(()=>{
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(MemoComponentA, "MemoComponentA");
    global.$RefreshRuntime$.getContext(MemoComponentA).accept();
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
    builtin_hoc_ident_component,
    // Input codes
    r#"
    const MemoComponentB = React.memo(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    const MemoComponentB = React.memo(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(MemoComponentB, "MemoComponentB");
    global.$RefreshRuntime$.getContext(MemoComponentB).accept();
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
    builtin_hoc_fn_only,
    // Input codes
    r#"
    const ForwardedComponent = forwardedRef(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    const ForwardedComponent = forwardedRef(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(ForwardedComponent, "ForwardedComponent");
    global.$RefreshRuntime$.getContext(ForwardedComponent).accept();
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
    builtin_hoc_component_anonymous_with_named_export,
    // Input codes
    r#"
    export const MemoComponentA = React.memo(() => {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const MemoComponentA = React.memo(()=>{
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(MemoComponentA, "MemoComponentA");
    global.$RefreshRuntime$.getContext(MemoComponentA).accept();
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
    builtin_hoc_ident_component_with_named_export,
    // Input codes
    r#"
    export const MemoComponentB = React.memo(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const MemoComponentB = React.memo(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(MemoComponentB, "MemoComponentB");
    global.$RefreshRuntime$.getContext(MemoComponentB).accept();
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
    builtin_hoc_fn_only_with_named_export,
    // Input codes
    r#"
    export const ForwardedComponent = forwardedRef(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const ForwardedComponent = forwardedRef(function OriginComponent() {
        return <div>{'Hello World'}</div>;
    });
    global.$RefreshReg$(ForwardedComponent, "ForwardedComponent");
    global.$RefreshRuntime$.getContext(ForwardedComponent).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);
