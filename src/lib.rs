use anyhow::Result;

mod application;
pub use application::Application;
mod config;
pub use config::Config;
pub mod error;
pub mod utils;

pub fn run(config: Config) -> Result<()> {
    let _application: Application = config.into();

    Ok(())
}
