use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadStatus {
    code: String,
    status: String
}

impl UploadStatus {
    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResult {
    errors: usize,
    warnings: usize,
    skipped: usize,
    total: usize,
    result: Value,
}
