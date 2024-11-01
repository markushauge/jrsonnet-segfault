use std::path::PathBuf;

use jrsonnet_evaluator::{trace::PathResolver, FileImportResolver, State};
use jrsonnet_stdlib::{ContextInitializer, YamlFormat};

fn main() {
    let state = State::default();
    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    state.set_context_initializer(ctx);
    state.set_import_resolver(FileImportResolver::new(vec![PathBuf::from("manifests")]));
    let val = state.import("./manifests/main.jsonnet").unwrap();
    let yaml_format = YamlFormat::std_to_yaml(false, false);
    val.as_arr().unwrap().iter().for_each(|v| {
        v.unwrap().manifest(&yaml_format).unwrap();
    });
}
