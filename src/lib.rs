mod api;

pub use api::*;

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(url: impl AsRef<str>) -> Self {
        Self {
            base_url: url.as_ref().to_string(),
        }
    }

    pub fn upload_results(&self, run: BenchmarkData) -> Result<(), ureq::Error> {
        let ret = ureq::put(&format!("{}/rebenchdb/results", self.base_url)).send_json(&run)?;
        dbg!(ret);

        Ok(())
    }
}
