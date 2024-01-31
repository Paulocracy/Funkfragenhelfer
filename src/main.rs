#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod gui;
mod helper;
mod learning;
mod question;

fn main() {
    let config = config::load_config();
    let learning = learning::load_learning();
    let questions_json_str = helper::read_filetext("./resources/ffh_questions.json");
    let questions: Vec<question::Question> = serde_json::from_str(&questions_json_str).unwrap();

    gui::run(config, learning, questions);
}
