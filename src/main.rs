//! Main file of Funkfragenhelfer.
//!
//! Here, all Funkfragenhelfer modules are loaded and the GUI is started.
//! For more about the specific modules, look up their comments. Note that
//! only the "GUI" module contains egui code, all other modules are GUI-framework
//! agnostic.

// Make sure that no console occurs under Windows in the release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Load Funkfragenhelfer modules
mod config;
mod gui;
mod helper;
mod learning;
mod question;

// Start Funkfragenhelger
fn main() {
    let config = config::load_config();
    let learn_states: learning::LearnStates = learning::load_learning();
    let questions_json_str = helper::read_filetext("./resources/ffh_questions.json");
    let questions: Vec<question::Question> = serde_json::from_str(&questions_json_str).unwrap();

    gui::run(config, learn_states, questions).unwrap();
}
