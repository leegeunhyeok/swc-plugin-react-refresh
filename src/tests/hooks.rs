use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
    no_hook_component,
    // Input codes
    r#"
    export function NoHookComponent() {
        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    export function NoHookComponent() {
        return <div>{'Hello, World'}</div>;
    }
    global.$RefreshReg$(NoHookComponent, "NoHookComponent");
    global.$RefreshRuntime$.getContext(NoHookComponent).accept();
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
    non_declare_internal_hook_component,
    // Input codes
    r#"
    export function NonDeclBuiltinHook() {
        useEffect(() => {}, []);

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function NonDeclBuiltinHook() {
        __s();
        useEffect(() => {}, []);
        return <div>{'Hello, World'}</div>;
    }
    __s(NonDeclBuiltinHook, "test:NonDeclBuiltinHook", false);
    global.$RefreshReg$(NonDeclBuiltinHook, "NonDeclBuiltinHook");
    global.$RefreshRuntime$.getContext(NonDeclBuiltinHook).accept();
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
    declare_internal_hook_component,
    // Input codes
    r#"
    export function DeclBuiltinHook() {
        const [number, setNumber] = useState(0);

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function DeclBuiltinHook() {
        __s();
        const [number, setNumber] = useState(0);
        return <div>{'Hello, World'}</div>;
    }
    __s(DeclBuiltinHook, "test:DeclBuiltinHook", false);
    global.$RefreshReg$(DeclBuiltinHook, "DeclBuiltinHook");
    global.$RefreshRuntime$.getContext(DeclBuiltinHook).accept();
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
    mixed_builtin_hooks_component,
    // Input codes
    r#"
    export function MixedBuiltinHooks() {
        const [number, setNumber] = useState(0);

        useMemo(() => 0);
        useCallback(() => {});

        useEffect(() => {}, []);
        useLayoutEffect(() => {}, []);

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function MixedBuiltinHooks() {
        __s();
        const [number, setNumber] = useState(0);
        useMemo(() => 0);
        useCallback(() => {});
        useEffect(() => {}, []);
        useLayoutEffect(() => {}, []);
        return <div>{'Hello, World'}</div>;
    }
    __s(MixedBuiltinHooks, "test:MixedBuiltinHooks", false);
    global.$RefreshReg$(MixedBuiltinHooks, "MixedBuiltinHooks");
    global.$RefreshRuntime$.getContext(MixedBuiltinHooks).accept();
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
    non_declare_custom_hook_component,
    // Input codes
    r#"
    export function NonDeclCustomHook() {
        useMyCustomHook();

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function NonDeclCustomHook() {
        __s();
        useMyCustomHook();
        return <div>{'Hello, World'}</div>;
    }
    __s(NonDeclCustomHook, "test:NonDeclCustomHook", true);
    global.$RefreshReg$(NonDeclCustomHook, "NonDeclCustomHook");
    global.$RefreshRuntime$.getContext(NonDeclCustomHook).accept();
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
    declare_custom_hook_component,
    // Input codes
    r#"
    export function DeclCustomHook() {
        const hookValue = useMyCustomHookDecl();

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function DeclCustomHook() {
        __s();
        const hookValue = useMyCustomHookDecl();
        return <div>{'Hello, World'}</div>;
    }
    __s(DeclCustomHook, "test:DeclCustomHook", true);
    global.$RefreshReg$(DeclCustomHook, "DeclCustomHook");
    global.$RefreshRuntime$.getContext(DeclCustomHook).accept();
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
    mixed_custom_hooks_component,
    // Input codes
    r#"
    export function MixedCustomHooks() {
        useMyCustomHook();

        const hookValue = useMyCustomHookDecl();

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function MixedCustomHooks() {
        __s();
        useMyCustomHook();
        const hookValue = useMyCustomHookDecl();
        return <div>{'Hello, World'}</div>;
    }
    __s(MixedCustomHooks, "test:MixedCustomHooks", true);
    global.$RefreshReg$(MixedCustomHooks, "MixedCustomHooks");
    global.$RefreshRuntime$.getContext(MixedCustomHooks).accept();
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
    mixed_hooks_component,
    // Input codes
    r#"
    export function MixedHooks() {
        // builtin hooks
        const [number, setNumber] = useState(0);
        useMemo(() => 0);
        useCallback(() => {});

        // custom hooks
        useMyCustomHook();
        const hookValue = useMyCustomHookDecl();

        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function MixedHooks() {
        __s();
        const [number, setNumber] = useState(0);
        useMemo(() => 0);
        useCallback(() => {});
        useMyCustomHook();
        const hookValue = useMyCustomHookDecl();
        return <div>{'Hello, World'}</div>;
    }
    __s(MixedHooks, "test:MixedHooks", true);
    global.$RefreshReg$(MixedHooks, "MixedHooks");
    global.$RefreshRuntime$.getContext(MixedHooks).accept();
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
    builtin_hook_from_member,
    // Input codes
    r#"
    export function BuiltinMemberHook() {
        const [number, setNumber] = React.useState(0);
        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function BuiltinMemberHook() {
        __s();
        const [number, setNumber] = React.useState(0);
        return <div>{'Hello, World'}</div>;
    }
    __s(BuiltinMemberHook, "test:BuiltinMemberHook", false);
    global.$RefreshReg$(BuiltinMemberHook, "BuiltinMemberHook");
    global.$RefreshRuntime$.getContext(BuiltinMemberHook).accept();
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
    custom_hook_from_member,
    // Input codes
    r#"
    export function CustomMemberHook() {
        const res = App.useCustomHook();
        return <div>{'Hello, World'}</div>;
    }
    "#,
    // Output
    r#"
    var __prevRefreshReg = global.$RefreshReg$;
    var __prevRefreshSig = global.$RefreshSig$;
    global.$RefreshReg$ = global.$RefreshRuntime$.getRegisterFunction();
    global.$RefreshSig$ = global.$RefreshRuntime$.getCreateSignatureFunction();
    var __s = global.$RefreshSig$();
    export function CustomMemberHook() {
        __s();
        const res = App.useCustomHook();
        return <div>{'Hello, World'}</div>;
    }
    __s(CustomMemberHook, "test:CustomMemberHook", true);
    global.$RefreshReg$(CustomMemberHook, "CustomMemberHook");
    global.$RefreshRuntime$.getContext(CustomMemberHook).accept();
    global.$RefreshReg$ = __prevRefreshReg;
    global.$RefreshSig$ = __prevRefreshSig;
    "#
);
