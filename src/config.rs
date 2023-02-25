use anyhow::Result;

use crate::error;

pub struct Config {
    pub target: String,
}

impl Config {
    pub fn new(target: &str) -> Result<Config> {
        if !crate::utils::does_crate_exist(target)? {
            return Err(error::Error::CrateDoesNotExist(target.to_string()).into());
        }

        Ok(Config {
            target: target.into(),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            target: "anyhow".to_string(),
        }
    }
}
