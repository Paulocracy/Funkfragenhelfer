use crate::helper;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub include_v: bool,
    pub include_b: bool,
    pub include_n: bool,
    pub include_e: bool,
    pub include_a: bool,
    pub learning_filepath: String,
    pub max_learn_bin: u64,
    pub questions_filepath: String,
}

impl Config {
    fn new() -> Config {
        Config {
            include_v: true,
            include_b: true,
            include_n: true,
            include_e: false,
            include_a: false,
            learning_filepath: String::from("./learning/learning.json"),
            max_learn_bin: 5,
            questions_filepath: String::from("./questions/questions.json"),
        }
    }

    pub fn save(&self) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        helper::overwrite_file_str("./config/config.json", &json_str);
    }

    pub fn all_includes_false(&self) -> bool {
        return !self.include_a
            && !self.include_b
            && !self.include_e
            && !self.include_n
            && !self.include_v;
    }
}

pub fn load_config() -> Config {
    let config_dir_path = Path::new("./config");
    if !config_dir_path.exists() {
        helper::ensure_dir_existence("./config");
    }
    let config_file_path = Path::new("./config/config.json");
    if !config_file_path.exists() {
        let default_config = Config::new();
        let config_json = serde_json::to_string_pretty(&default_config).unwrap();
        helper::overwrite_file_str("./config/config.json", &config_json);
    }
    let config_text = helper::read_filetext("./config/config.json");
    serde_json::from_str(&config_text).unwrap()
}
