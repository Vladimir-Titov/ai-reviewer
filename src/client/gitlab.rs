pub struct GitlabHttpClient<'a> {
    token: &'a String,
    base_url: &'a String,
}

impl<'a> GitlabHttpClient<'a> {
    pub fn new(token: &'a String, base_url: &'a String) -> Self {
        Self { token, base_url }
    }

    pub fn get_merge_request_diff(&self, project_id: u16, mr_id: u16) -> Result<(), ureq::Error> {
        let url: String = format!(
            "{}/api/v4/projects/{project_id}/merge_requests/{mr_id}/diffs",
            self.base_url,
        );
        let resp: serde_json::Value = ureq::get(&url)
            .set("PRIVATE-TOKEN", self.token)
            .call()?
            .into_json()?;
        println!("{}", resp);
        Ok(())
    }
}
