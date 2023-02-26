use std::path::PathBuf;

use anyhow::Result;

pub fn does_crate_exist(target: &str) -> Result<bool> {
    let response = reqwest::blocking::get(format!("https://docs.rs/{}", target))?;
    let body = response.text()?;
    Ok(!body.contains("The requested crate does not exist"))
}

pub fn get_cache_dir(target: &str) -> Result<PathBuf> {
    #[cfg(target_os = "linux")]
    return Ok(PathBuf::from(format!(
        "{}/.cache/{}",
        std::env::var("HOME")?,
        target
    )));
}
