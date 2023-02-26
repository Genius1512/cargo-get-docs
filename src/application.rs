use anyhow::Result;

use crate::Config;

pub struct Application {
    target: String,
}

impl Application {
    pub fn new(config: Config) -> Result<Application> {
        Ok(Application {
            target: config.target,
        })
    }
}
