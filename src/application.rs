use anyhow::Result;

use crate::Config;

pub struct Application {
    target: String,
    version: String,
}

impl Application {
    pub fn new(config: Config) -> Result<Application> {
        Ok(Application {
            target: config.target,
            version: config.version,
        })
    }

    fn get_html_page(&self, path: &str) -> Result<String> {
        let response = reqwest::blocking::get(self.get_url_from_path(path))?;
        let body = response.text()?;
        Ok(body)
    }

    fn get_url_from_path(&self, path: &str) -> String {
        format!("https://docs.rs/{}/{}/{}", self.target, self.version, path)
    }
}
