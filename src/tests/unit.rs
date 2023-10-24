use swc_core::ecma::transforms::testing::test;

use super::react_refresh;

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

// Arrow function expression components
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

// HoC
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

// External components
test!(
    swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| react_refresh(String::from("test")),
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
    |_| react_refresh(String::from("test")),
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
    |_| react_refresh(String::from("test")),
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

// Hooks
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

// Common
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

// Edge cases
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
