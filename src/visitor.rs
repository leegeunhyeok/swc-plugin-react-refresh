use std::collections::HashSet;
use swc_core::ecma::{
    ast::*,
    visit::Visit,
};
use crate::utils::{get_name_from_ident, is_react_component_name};

/// Visit top-level to find external and class components.
pub struct IgnoreIdentifierCollector {
    black_list: HashSet<String>,
}

impl IgnoreIdentifierCollector {
    fn default() -> IgnoreIdentifierCollector {
        IgnoreIdentifierCollector { black_list: HashSet::new() }
    }

    fn add(&mut self, identifier: String) {
        println!("ignore identifier: {:#?}", identifier);
        self.black_list.insert(identifier);
    }

    pub fn get_black_list(&self) -> HashSet<String> {
        self.black_list.to_owned()
    }
}

impl Visit for IgnoreIdentifierCollector {
    fn visit_import_specifiers(&mut self, import_specifiers: &[ImportSpecifier]) {
        for import_specifier in import_specifiers.iter() {
            // Ignore external components.
            //
            // - `import Component from '...';`
            // - `import { Component } from '...';`
            match import_specifier {
                ImportSpecifier::Named(named_import) => {
                    let identifier = get_name_from_ident(&named_import.local);
                    if is_react_component_name(&identifier) {
                        self.add(identifier);
                    }
                }
                ImportSpecifier::Default(default_import) => {
                    let identifier = get_name_from_ident(&default_import.local);
                    if is_react_component_name(&identifier) {
                        self.add(identifier);
                    }
                }
                _ => (),
            }
        }
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        // Ignore class component
        let identifier = get_name_from_ident(&class_decl.ident);
        if is_react_component_name(&identifier) {
            self.add(identifier);
        }
    }
}

pub fn black_list_collector() -> IgnoreIdentifierCollector {
    IgnoreIdentifierCollector::default()
}
