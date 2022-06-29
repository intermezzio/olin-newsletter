use serde;
use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};

pub fn load() -> ConfigVars {
    let path = "./src/config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: ConfigVars = serde_json::from_str(&data).expect("Unable to parse");
    
    res
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigVars {
    pub newsapi: Option<String>,
    pub weatherbit: Option<String>
}
