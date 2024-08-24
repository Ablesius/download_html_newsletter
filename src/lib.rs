pub struct Link {
    url: String,
}

impl Link {
    pub fn new(s: &str) -> Self {
        Link { url: s.to_string() }
    }
    pub fn download(&self) -> Result<String, reqwest::Error> {
        let body = reqwest::blocking::get(&self.url)?.text()?;

        Ok(body)
    }
}
