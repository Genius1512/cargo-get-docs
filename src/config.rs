use anyhow::Result;

pub struct Config {
    pub target: String,
}

impl Config {
    pub fn new(target: &str) -> Result<Config> {
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
