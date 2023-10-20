use serde::Deserialize;
use swc_core::ecma::{
    ast::Program,
    visit::FoldWith,
};
use swc_core::plugin::{
    plugin_transform,
    proxies::TransformPluginProgramMetadata,
    metadata::TransformPluginMetadataContextKind,
};
mod transformer;
mod visitor;
mod utils;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReactRefreshOptions {
    module_id: String,
    skip_env_check: Option<bool>,
}

#[plugin_transform]
fn swc_react_refresh_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<ReactRefreshOptions>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-plugin-react-refresh"),
    )
    .expect("invalid config for swc-plugin-react-refresh");

    let env_name = &data
        .get_context(&TransformPluginMetadataContextKind::Env)
        .unwrap_or_default();
    let is_dev = env_name.eq("development");

    if !is_dev && !config.skip_env_check.unwrap_or(false) {
        panic!("swc-plugin-react-refresh transform should only be enabled in development environment.\n\
        If you want to override this check, pass `skipEnvCheck` as plugin options.");
    }

    program.fold_with(&mut transformer::react_refresh(config.module_id))
}
