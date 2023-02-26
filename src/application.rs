use std::collections::HashMap;

use anyhow::Result;

use crate::Config;

pub struct Application {
    target: String,

    pages_cache: HashMap<String, String>,
}

impl Application {
    #[allow(unused)]
    fn get_page(&mut self, url: &str) -> Result<String> {
        let page: String = match self.pages_cache.get(&url.to_string()) {
            Some(page) => return Ok(page.clone()),
            None => match self.get_page_from_fs_cache(url) {
                Some(page) => page,
                None => self.get_page_from_docs_rs(url)?,
            },
        };

        self.pages_cache.insert(url.to_string(), page.clone());

        Ok(page)
    }

    fn get_page_from_fs_cache(&self, _url: &str) -> Option<String> {
        todo!("Get from file system")
    }

    fn get_page_from_docs_rs(&self, _url: &str) -> Result<String> {
        todo!("Get from docs.rs")
    }
}

impl From<Config> for Application {
    fn from(value: Config) -> Self {
        Application {
            target: value.target,
            pages_cache: HashMap::new(),
        }
    }
}
