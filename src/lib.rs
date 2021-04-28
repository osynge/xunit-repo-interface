#[macro_use]
extern crate serde_derive;
use std::collections::HashMap;

pub use serde::{Deserialize, Serialize};
pub use xunit_struct::errors::XunitError;
pub use xunit_struct::model::Xunit;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub content: Xunit,
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub sk: Option<String>,
    pub key_value: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub sk: Option<String>,
    pub identifier: Option<String>,
    pub human_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub sk: Option<String>,
    pub client_identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
    pub project: Project,
    pub environment: Environment,
    pub run: Run,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub project: String,
    pub environment: String,
    pub run_identifier: String,
    pub test_run: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
