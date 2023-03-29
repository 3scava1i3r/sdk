//! Code for creating SNS configurations
use fn_error_context::context;
use std::ffi::OsString;
use std::path::Path;

use crate::lib::call_bundled::wsl_call_bundled;
use crate::lib::error::DfxResult;
use crate::util::wsl_path;
use crate::Environment;

/// Ceates an SNS configuration template.
#[context("Failed to create sns config at {}.", path.display())]
pub fn create_config(env: &dyn Environment, path: &Path) -> DfxResult {
    let args = vec![
        OsString::from("init-config-file"),
        OsString::from("--init-config-file-path"),
        OsString::from(wsl_path(path)?),
        OsString::from("new"),
    ];
    wsl_call_bundled(env, "sns", &args)?;
    Ok(())
}
