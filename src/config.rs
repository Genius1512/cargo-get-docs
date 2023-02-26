use anyhow::Result;

pub struct Config {
    pub target: String,
    pub version: String,
}

impl Config {
    pub fn new(target: &str, version: &str) -> Result<Config> {
        Ok(Config {
            target: target.to_string(),
            version: version.to_string(),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            target: "anyhow".to_string(),
            version: "latest".to_string(),
        }
    }
}
