use std::env;

use crate::debug_eprintln;
use proc_macro2::Span;
use std::path::PathBuf;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{Ident, LitStr, Token};

impl EnvMacro {
    fn check_string_cannot_be_empty(
        &self,
        string: &String,
        error_span: Span,
        name: &str,
    ) -> Result<()> {
        if string.is_empty() {
            return Err(Error::new(error_span, format! {"{name} cannot be empty"}));
        }
        Ok(())
    }
    pub fn read_env_value(&self) -> Result<String> {
        // Check if the file path is provided
        let litkey = &self.key;
        let litfile_path = &self.file_path;
        let key = litkey.value();
        self.check_string_cannot_be_empty(&key, litkey.span(), "key")?;
        if let Some(litpath) = litfile_path {
            let path = &litpath.value();
            self.check_string_cannot_be_empty(path, litpath.span(), "path")?;
            let mut env_path = PathBuf::from(path);
            if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                let manifest_path = PathBuf::from(manifest_dir);
                env_path = manifest_path.join(env_path);
                debug_eprintln!("`env_path` = {env_path:?}; `manifest_path` = {manifest_path:?}");
            };

            dotenv::from_path(&env_path).map_err(|e| {
                Error::new(
                    litpath.span(),
                    format!("failed to read file = `{}`: err: {}", env_path.display(), e),
                )
            })?;
            #[allow(clippy::needless_return)]
            return env::var(&key).map_err(|e| {
                Error::new(
                    litkey.span(),
                    format!(
                        "failed to read `{}` key from `{}` file, err = {e}",
                        key,
                        env_path.display()
                    ),
                )
            });
        } else {
            #[allow(clippy::needless_return)]
            return env::var(&key).map_err(|e| {
                Error::new(
                    litkey.span(),
                    format!(
                        "failed to read `{}` key from the environment, err = {e}",
                        key
                    ),
                )
            });
        }
    }
}

pub(crate) struct EnvMacro {
    pub(crate) key: LitStr,
    pub(crate) file_path: Option<LitStr>,
}

impl Parse for EnvMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        if name != "env" {
            return Err(Error::new(
                name.span(),
                format!(
                    "only accepting `env!{{...}}` macro within the procedural macro, `{}` was passed in",
                    name
                ),
            ));
        };

        let _: Token![!] = input.parse()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        let key: LitStr = content.parse()?;
        let lookahead = content.lookahead1();
        if !lookahead.peek(Token![,]) {
            if !input.is_empty() {
                input.parse::<Token![;]>()?;
            }
            return Ok(EnvMacro {
                key,
                file_path: None,
            });
        }
        let _: Token![,] = content.parse()?;
        let file_path = if content.is_empty() {
            None
        } else {
            Some(content.parse()?)
        };
        if !input.is_empty() {
            input.parse::<Token![;]>()?;
        }
        Ok(EnvMacro { key, file_path })
    }
}
// Extract the key and optional file path from an invocation of env! macro.
