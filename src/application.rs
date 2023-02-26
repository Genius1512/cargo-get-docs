use std::collections::HashMap;

use anyhow::Result;

use crate::{utils, Config, Page};

pub struct Application {
    target: String,
    version: String,

    cached_pages: HashMap<String, Page>,
}

impl Application {
    pub fn new(config: Config) -> Result<Application> {
        Ok(Application {
            target: config.target,
            version: config.version,
            cached_pages: HashMap::new(),
        })
    }

    #[allow(unused)]
    fn get_page(&mut self, path: &str) -> Result<Page> {
        let page: Page = match self.cached_pages.get(&path.to_string()) {
            Some(page) => return Ok(page.clone()),
            None => match self.get_page_from_fs(path)? {
                Some(page) => page,
                None => self.get_page_from_docs_rs(path)?,
            },
        };

        self.cached_pages.insert(path.to_string(), page.clone());

        Ok(page)
    }

    fn get_page_from_fs(&self, #[allow(unused)] path: &str) -> Result<Option<Page>> {
        #[allow(unused)]
        let cache_dir = utils::get_cache_dir(&self.target)?;
        todo!("Convert path to filepath, join with cache_dir and read the page")
    }

    fn get_page_from_docs_rs(&self, path: &str) -> Result<Page> {
        let response = reqwest::blocking::get(self.get_url_from_path(path))?;
        let body = response.text()?;
        Ok(Page::HTML(body))
    }

    fn get_url_from_path(&self, path: &str) -> String {
        format!("https://docs.rs/{}/{}/{}", self.target, self.version, path)
    }
}
