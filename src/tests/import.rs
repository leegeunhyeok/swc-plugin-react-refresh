use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

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
