pub struct GitlabHttpClient<'a> {
    token: &'a String,
    base_url: &'a String,
}

impl<'a> GitlabHttpClient<'a> {
    pub fn new(token: &'a String, base_url: &'a String) -> Self {
        Self { token, base_url }
    }

    pub fn get_merge_request_diff(&self) {}

    pub fn get_merge_requests(&self) {}
}
