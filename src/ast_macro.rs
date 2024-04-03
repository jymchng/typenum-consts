use std::env;

use crate::debug_eprintln;
use proc_macro2::Span;
use std::path::PathBuf;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{LitStr, Macro, Token};

const CARGO_BUILD_TARGET_DIR: &str = "CARGO_BUILD_TARGET_DIR";
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const CARGO_TARGET_DIR: &str = "CARGO_TARGET_DIR";

const CARGO_ENV_DIRS: [&str; 3] = [CARGO_BUILD_TARGET_DIR, CARGO_MANIFEST_DIR, CARGO_TARGET_DIR];

impl EnvArgs {
    fn check_string_cannot_be_empty(
        &self,
        string: &str,
        error_span: Span,
        name: &str,
    ) -> Result<()> {
        if string.is_empty() {
            return Err(Error::new(error_span, format! {"{name} cannot be empty"}));
        }
        Ok(())
    }

    fn get_env_path(&self, mut env_path: PathBuf) -> core::result::Result<PathBuf, [PathBuf; 3]> {
        let mut err_array: [PathBuf; 3] = [PathBuf::new(), PathBuf::new(), PathBuf::new()];
        for (idx, cargo_env_dir) in CARGO_ENV_DIRS.iter().enumerate() {
            if let Ok(manifest_dir) = env::var(cargo_env_dir) {
                let manifest_path = PathBuf::from(manifest_dir);
                env_path = manifest_path.join(env_path);
                err_array[idx] = env_path.clone();
                debug_eprintln!("`env_path` = {env_path:?}; `manifest_path` = {manifest_path:?}");
                if env_path.exists() {
                    return Ok(env_path);
                }
            };
        }
        Err(err_array)
    }
    pub fn read_env_value(&self) -> Result<String> {
        let litkey = &self.key;
        let litfile_path = &self.file_path;
        let key = litkey.value();
        self.check_string_cannot_be_empty(&key, litkey.span(), "key")?;
        if let Some(litpath) = litfile_path {
            let path = &litpath.value();
            self.check_string_cannot_be_empty(path, litpath.span(), "path")?;
            let env_path = PathBuf::from(path);
            match self.get_env_path(env_path.clone()) {
                Ok(env_path) => {
                    dotenv::from_path(&env_path).map_err(|e| {
                        Error::new(
                            litpath.span(),
                            format!(
                                "failed to read file = `{}`: err: {}",
                                &env_path.display(),
                                e
                            ),
                        )
                    })?;
                    env::var(&key).map_err(|e| {
                        Error::new(
                            litkey.span(),
                            format!(
                                "failed to read `{}` key from `{}` file, err = {e}",
                                key,
                                &env_path.display()
                            ),
                        )
                    })
                }
                Err(err) => {
                    Err(Error::new(
                        litpath.span(),
                        format!("failed to locate the file = `{}`, the 3 file paths which are searched for are `{err:?}`", &env_path.display()),
                    ))
                }
            }
        } else {
            env::var(&key).map_err(|e| {
                Error::new(
                    litkey.span(),
                    format!(
                        "failed to read `{}` key from the environment, err = {e}",
                        key
                    ),
                )
            })
        }
    }
}

pub(crate) enum AllowedMacros {
    Env(EnvArgs),
}

impl AllowedMacros {
    pub(crate) fn which_macro(the_macro: &Macro) -> Result<Self> {
        let macro_ident = the_macro.path.require_ident()?;
        match macro_ident.to_string().as_str() {
            "env" => Ok(Self::Env(the_macro.parse_body::<EnvArgs>()?)),
            other => Err(Error::new(
                macro_ident.span(),
                format!("currently only support `env` macro-like invocation, it currently does not support `{}`", other),
            )),
        }
    }

    pub(crate) fn invoke_macro(&self) -> Result<String> {
        match self {
            Self::Env(env_args) => Ok(env_args.read_env_value()?),
        }
    }
}

pub(crate) struct EnvArgs {
    key: LitStr,
    file_path: Option<LitStr>,
}

impl Parse for EnvArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitStr = input.parse()?;
        let file_path = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(EnvArgs { key, file_path })
    }
}
