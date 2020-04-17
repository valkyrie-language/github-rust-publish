use crate::{GithubError, commands::LegionArguments};
use clap::Parser;
use js_component_bindgen::{BindingsMode, InstantiationMode, TranspileOpts};
use std::collections::HashMap;

#[derive(Debug, Parser)]
pub struct CommandPolyfill {
    input: String,
    #[arg(short, long, default_value = "index")]
    name: String,
    #[arg(short, long)]
    instantiation: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    guest: bool,
}

impl CommandPolyfill {
    pub async fn run(&self, _: &LegionArguments) -> Result<(), GithubError> {
        let input = std::fs::read(&self.input)?;
        let mut map = HashMap::default();
        map.insert("wasi:*".to_string(), "@bytecodealliance/preview2-shim/*".to_string());
        map.insert("valkyrie:std-legacy/*".to_string(), "@valkyrie-language/std-legacy/*".to_string());
        // for (k, v) in self.shim {
        //     map.insert(k, v);
        // }
        let cfg = TranspileOpts {
            name: self.name.to_string(),
            no_typescript: false,
            instantiation: if self.instantiation { Some(InstantiationMode::Async) } else { None },
            import_bindings: Some(BindingsMode::Js),
            map: Some(map),
            no_nodejs_compat: false,
            base64_cutoff: 0,
            tla_compat: false,
            valid_lifting_optimization: !self.debug,
            tracing: self.debug,
            no_namespaced_exports: true,
            multi_memory: true,
            guest: self.guest,
        };
        let result = js_component_bindgen::transpile(&input, cfg)?;
        result.files;
        Ok(())
    }
}
