/// Enum representing necessary headers for requests
pub enum Headers {
    ContentTypeAppJson,
    /// Marker for content-endpoints (content.dropboxapi.com). When present the
    /// service macro suppresses the JSON body — args travel in `Dropbox-API-Arg`.
    /// Binary body supply (for upload endpoints) is a deliberate follow-up.
    ContentTypeAppOctetStream,
    TestAuthorization,
    DropboxApiArg(String),
    /// Marker for download-class endpoints that return metadata in the
    /// `Dropbox-API-Result` response header and binary body separately. When
    /// present the service macro parses the response payload from the header
    /// instead of the body. Exposing the binary body to callers is a follow-up.
    DropboxApiResult,
}

impl Headers {
    pub fn get_str(&self) -> (&str, &str) {
        match self {
            Headers::ContentTypeAppJson => ("Content-type", "application/json"),
            Headers::ContentTypeAppOctetStream => ("Content-Type", "application/octet-stream"),
            Headers::TestAuthorization => ("Authorization", "Bearer user"),
            Headers::DropboxApiArg(path) => ("Dropbox-API-Arg", path),
            Headers::DropboxApiResult => ("Dropbox-API-Result", ""),
        }
    }
}
