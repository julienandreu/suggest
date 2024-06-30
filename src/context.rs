use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Context {
    name: String,
    description: String,
    version: String,
}

impl Context {
    pub fn load() -> Option<Self> {
        let current_path = env::current_dir().unwrap();
        let filename = "package.json";
        let path = current_path.join(filename);

        if let Ok(content) = fs::read_to_string(path) {
            return Some(serde_json::from_str(&content).expect("Unable to read package.json"));
        }

        None
    }
}
