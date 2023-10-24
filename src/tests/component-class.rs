use super::react_refresh;
use swc_core::ecma::transforms::testing::test;

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
