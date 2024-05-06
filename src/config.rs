//! Includes the struct and associated implementations
//! for Funkragenhelfer's main configuration, including
//! the selected question categories and question filters.

// IMPORTS SECTION //
use crate::helper;
use serde::{Deserialize, Serialize};
use std::path::Path;

// STRUCT SECTION //
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Include question category V (Verordnungen)?
    pub include_v: bool,
    /// Include question category B (Betriebliches)?
    pub include_b: bool,
    /// Include question category N (Technik Klasse N)?
    pub include_n: bool,
    /// Include question category E (Technik Klasse E)?
    pub include_e: bool,
    /// Include question category A (Technik Klasse A)?
    pub include_a: bool,
    /// Filepath to learning.json (currently fixed)
    pub learning_filepath: String,
    /// Maximal learning "bin" (see learning module for more)
    pub max_learn_bin: u64,
    /// Filter for wrongly answered questions?
    pub prefer_wrong: bool,
    /// Filter for user-marked questions?
    pub prefer_marked: bool,
    /// Filter for questions that weren't asked yet?
    pub prefer_new: bool,
    /// Path to questions.json (currently fixed)
    pub questions_filepath: String,
}

impl Config {
    /// Return a Config with default values, as if someone
    /// would learn for class E without filters.
    fn new() -> Config {
        Config {
            include_v: true,
            include_b: true,
            include_n: true,
            include_e: false,
            include_a: false,
            learning_filepath: String::from("./learning/learning.json"),
            max_learn_bin: 5,
            prefer_wrong: false,
            prefer_marked: false,
            prefer_new: false,
            questions_filepath: String::from("./questions/questions.json"),
        }
    }

    /// Save the config in the config.json (the directory's existence
    /// was ensured beforehand with this module's load_config()).
    pub fn save(&self) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        helper::overwrite_file_str("./config/config.json", &json_str);
    }

    /// Returns whether or not all categories were set off (used by the GUI to prevent that
    /// a user shuts off all questions).
    pub fn all_includes_false(&self) -> bool {
        !self.include_a
            && !self.include_b
            && !self.include_e
            && !self.include_n
            && !self.include_v
    }
}

// PUBLIC FUNCTION SECTION //
/// Loads the config.json Config, or, if it doesn't exist, creates
/// a new such JSON with default values.
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
