use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct KaspiCategory {
    pub code: String,
    pub title: String,
}

impl fmt::Display for KaspiCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.code, self.title)
    }
}
