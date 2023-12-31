#![warn(clippy::unwrap_used, clippy::expect_used)]

mod error;
pub mod models;
mod tasks;

use crate::error::CaughtError;
use crate::models::Config;

#[cfg(not(tarpaulin_include))]
pub fn generate_mod_with_json_default(
    temp_dir: &str,
    version: &str,
    target_dir: &str,
    sender: std::sync::mpsc::Sender<String>,
) -> Result<(), CaughtError> {
    generate_mod_with_json("config.json", temp_dir, version, target_dir, sender)
}

#[cfg(not(tarpaulin_include))]
pub fn generate_mod_with_json(
    cfg_path: &str,
    temp_dir: &str,
    version: &str,
    target_dir: &str,
    sender: std::sync::mpsc::Sender<String>,
) -> Result<(), CaughtError> {
    let config = tasks::read_config(cfg_path)?;
    generate_mod(config, temp_dir, version, target_dir, sender)
}

pub fn generate_mod(
    config: Config,
    temp_dir: &str,
    version: &str,
    target_dir: &str,
    sender: std::sync::mpsc::Sender<String>,
) -> Result<(), CaughtError> {
    tasks::generate(config, temp_dir, target_dir, version, sender)
}
