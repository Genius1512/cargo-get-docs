use anyhow::Result;

mod application;
pub use application::Application;
mod config;
pub use config::Config;
pub mod error;
pub mod utils;

pub fn run(config: Config) -> Result<()> {
    if !utils::does_crate_exist(&config.target)? {
        return Err(error::Error::CrateDoesNotExist(config.target).into());
    }

    let _application = Application::new(config);

    Ok(())
}
