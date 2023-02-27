use std::collections::HashMap;

use anyhow::Result;

use crate::{utils, Config, page::{HTMLPage, MarkupPage}};

pub struct PageManager {
    target: String,
    version: String,

    cached_pages: HashMap<String, MarkupPage>,
}

impl PageManager {
    pub fn new(config: Config) -> Result<PageManager> {
        Ok(PageManager {
            target: config.target,
            version: config.version,
            cached_pages: HashMap::new(),
        })
    }

    #[allow(unused)]
    fn get_page(&mut self, path: &str) -> Result<MarkupPage> {
        let page: MarkupPage = match self.cached_pages.get(&path.to_string()) {
            Some(page) => return Ok(page.clone()),
            None => match self.get_page_from_fs(path)? {
                Some(page) => page,
                None => self.get_page_from_docs_rs(path)?.into(),
            },
        };

        self.cached_pages.insert(path.to_string(), page.clone());

        Ok(page)
    }

    fn get_page_from_fs(&self, #[allow(unused)] path: &str) -> Result<Option<MarkupPage>> {
        #[allow(unused)]
        let cache_dir = utils::get_cache_dir(&self.target)?;
        todo!("Convert path to filepath, join with cache_dir and read the page")
    }

    fn get_page_from_docs_rs(&self, path: &str) -> Result<HTMLPage> {
        let response = reqwest::blocking::get(self.get_url_from_path(path))?;
        let body = response.text()?;
        Ok(HTMLPage::new(&body))
    }

    fn get_url_from_path(&self, path: &str) -> String {
        format!("https://docs.rs/{}/{}/{}", self.target, self.version, path)
    }
}
