use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub enum Status {
    UPLOADED,
    FINISHED,
    ABORTED
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "UPLOADED" => Status::UPLOADED,
            "FINISHED" => Status::FINISHED,
            "ABORTED" => Status::ABORTED,
            e => panic!("Could not convert status: {}", e)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UploadStatus {
    code: String,
    status: Status
}

impl UploadStatus {
    pub fn new(code: String, status: Status) -> Self {
        Self { code, status }
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status
    }
}


#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Clone, Debug)]
pub struct UploadResult {
    errors: usize,
    warnings: usize,
    skipped: usize,
    total: usize,
    result: Vec<String>,
}

impl UploadResult {
    pub fn new(errors: usize, warnings: usize, skipped: usize, total: usize, result: Vec<String>) -> Self {
        Self { errors, warnings, skipped, total, result }
    }
}
