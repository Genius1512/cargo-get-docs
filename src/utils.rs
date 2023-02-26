use anyhow::Result;

pub fn does_crate_exist(target: &str) -> Result<bool> {
    let response = reqwest::blocking::get(format!("https://docs.rs/{}", target))?;
    let body = response.text()?;
    Ok(!body.contains("The requested crate does not exist"))
}
