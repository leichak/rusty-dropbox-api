/// Enum representing necessary headers for requests
pub enum Headers {
    ContentTypeAppJson,
    TestAuthorization,
}

impl Headers {
    pub fn get_str(&self) -> (&str, &str) {
        match self {
            Headers::ContentTypeAppJson => ("Content-type", "application/json"),
            Headers::TestAuthorization => ("Authorization", "Bearer 123456"),
        }
    }
}
