use anyhow::Result;

mod config;
pub use config::Config;
pub mod error;
pub mod utils;

pub fn run(_config: Config) -> Result<()> {
    Ok(())
}
