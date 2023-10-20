use swc_core::ecma::ast::Ident;

/// Check provided name is valid React component name.
/// 
/// Returns `true` if name starts with capitalize.
/// 
/// - MyComponent: `true`
/// - myComponent: `false`
pub fn is_react_component_name(name: &String) -> bool {
  name.chars().nth(0).unwrap().is_uppercase()
}

/// Get symbol name from `Ident`
pub fn get_name_from_ident(ident: &Ident) -> String {
  ident.sym.to_string()
}
