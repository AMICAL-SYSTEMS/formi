use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use phf_codegen::Map;
use syn::{Ident, LitStr, Token, parse::Parse, parse::ParseStream, visit::Visit};

fn main() {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&out_path).unwrap());

    let core_dir = PathBuf::from("src/core");
    let mut calls = Vec::new();
    let mut modules = Vec::new();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/core");
    println!("cargo:rerun-if-changed=src/core.rs");

    for entry in fs::read_dir(&core_dir).unwrap() {
        let path = entry.unwrap().path();
        assert_eq!(path.extension().unwrap(), "rs");

        println!("cargo:rerun-if-changed={}", path.display());
        modules.push(
            path.file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        );
        calls.extend(find_define_word_calls(&path));
    }

    modules.sort();
    for module in modules {
        let mod_path = core_dir
            .join(format!("{module}.rs"))
            .canonicalize()
            .unwrap();
        writeln!(file, "#[path = \"{}\"]", mod_path.display()).unwrap();
        writeln!(file, "pub mod r#{};", module).unwrap();
    }

    let map_entries: Vec<(String, String)> = calls
        .into_iter()
        .map(|call| {
            let value = format!(
                "<crate::core::r#{}::{} as crate::core::FixedToken>::execute",
                call.module, call.type_ident
            );
            (call.token, value)
        })
        .collect();

    let mut map = Map::new();
    for (token, value) in &map_entries {
        map.entry(token, value);
    }

    writeln!(
        &mut file,
        "pub static FIXED_TOKENS_MAP: FixedTokensMap = {};",
        map.build()
    )
    .unwrap();
}

#[derive(Debug)]
struct DefineWordCall {
    module: String,
    type_ident: String,
    token: String,
}

fn find_define_word_calls(path: &Path) -> Vec<DefineWordCall> {
    let module = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap()
        .to_string();
    let content = fs::read_to_string(path).unwrap();
    let syntax = syn::parse_file(&content).unwrap();

    let mut visitor = DefineWordVisitor {
        module_name: module,
        calls: Vec::new(),
    };
    visitor.visit_file(&syntax);

    visitor.calls
}

struct DefineWordVisitor {
    module_name: String,
    calls: Vec<DefineWordCall>,
}

impl<'ast> Visit<'ast> for DefineWordVisitor {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let args = syn::parse2::<DefineWordArgs>(mac.tokens.clone()).unwrap();
        self.calls.push(DefineWordCall {
            module: self.module_name.clone(),
            type_ident: args.type_ident.to_string(),
            token: args.token.value(),
        });

        syn::visit::visit_macro(self, mac);
    }
}

struct DefineWordArgs {
    type_ident: Ident,
    token: LitStr,
}

impl Parse for DefineWordArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let type_ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let token: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let _: proc_macro2::TokenStream = input.parse()?;

        Ok(Self { type_ident, token })
    }
}
