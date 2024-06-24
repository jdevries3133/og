use anyhow::{Error, Result};
use std::env;

pub fn get_var(var_name: &str) -> Result<String> {
    env::var(var_name)
        .map_err(|e| Error::msg(format!("error fetching {var_name}: {e}")))
}
