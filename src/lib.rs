use anyhow::Result;

mod page_manager;
pub use page_manager::PageManager;
mod config;
pub use config::Config;
pub mod error;
pub mod page;
pub mod utils;

pub fn run(config: Config) -> Result<()> {
    if !utils::does_crate_exist(&config.target)? {
        return Err(error::Error::CrateDoesNotExist(config.target).into());
    }

    let _application = PageManager::new(config);

    Ok(())
}
