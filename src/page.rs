#[derive(Clone)]
pub struct MarkupPage {
    body: String,
}

impl MarkupPage {
    pub fn new(body: &str) -> MarkupPage {
        MarkupPage { body: body.to_string() }
    }
}

#[derive(Clone)]
pub struct HTMLPage {
    body: String,
}


impl HTMLPage {
    pub fn new(body: &str) -> HTMLPage {
        HTMLPage { body: body.to_string() }
    }
}

impl From<HTMLPage> for MarkupPage {
    fn from(value: HTMLPage) -> Self {
        todo!("Convert")
    }
}
