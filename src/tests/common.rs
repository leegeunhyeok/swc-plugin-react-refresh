use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    non_component,
    // Input codes
    r#"
    const NAME = 'react-refresh';
    export const TIMEOUT = 5000;
    const styles = StyleSheet.create({});
    "#,
    // Output
    r#"
    const NAME = 'react-refresh';
    export const TIMEOUT = 5000;
    const styles = StyleSheet.create({});
    "#
);

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    multiple_components,
    // Input codes
    r#"
    export const MultipleA = () => {
        return <div>{'Hello, World'}</div>;
    };

    export const MultipleB = () => {
        return <div>{'Hello, World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const MultipleA = () => {
        return <div>{'Hello, World'}</div>;
    };
    export const MultipleB = () => {
        return <div>{'Hello, World'}</div>;
    };
    global.$RefreshReg$(MultipleA, "MultipleA");
    global.$RefreshRuntime$.getContext(MultipleA).accept();
    global.$RefreshReg$(MultipleB, "MultipleB");
    global.$RefreshRuntime$.getContext(MultipleB).accept();
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
    invalid_hook_call_in_component,
    // Input codes
    r#"
    export const NotHook = () => {
        notValidHook();

        return <div>{'Hello, World'}</div>;
    };
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export const NotHook = () => {
        notValidHook();
        return <div>{'Hello, World'}</div>;
    };
    global.$RefreshReg$(NotHook, "NotHook");
    global.$RefreshRuntime$.getContext(NotHook).accept();
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
    multiple_variable_declares,
    // Input codes
    r#"
    var A, B, C = () => {
        return <div>{'Hello, World'}</div>;
    };
    "#,
    // Output
    r#"
    var A, B, C = () => {
        return <div>{'Hello, World'}</div>;
    };
    "#
);
