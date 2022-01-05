use serde::Serialize;

pub mod file;
pub mod remove;
pub mod upload;

/// Response struct for all handlers
#[derive(Debug, Serialize)]
pub struct Response {
    /// Hashes that have been created, updated or deleted
    hashes: Vec<String>,
    /// Hashes that have been skipped due to some reasons
    skipped: Vec<String>,
}

impl Response {
    /// Creating a new response
    pub fn new(hashes: Vec<String>, skipped: Vec<String>) -> Response {
        Response { hashes, skipped }
    }
}
