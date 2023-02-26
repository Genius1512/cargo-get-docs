use anyhow::Result;

mod application;
pub use application::Application;
mod config;
pub use config::Config;
pub mod error;

pub fn run(config: Config) -> Result<()> {
    let _application = Application::new(config);

    Ok(())
}
