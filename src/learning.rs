use crate::config::Config;
use crate::question::Question;
use crate::{config, helper};
use rand::seq::SliceRandom;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct LearnState {
    pub current_bin: u64,
    pub correct: u64,
    pub wrong: u64,
    pub marked: bool,
    pub rounds_since_wrong: u64,
    pub time_last_answer: u64,
}

impl LearnState {
    pub fn new() -> LearnState {
        LearnState {
            current_bin: 0,
            correct: 0,
            wrong: 0,
            marked: false,
            rounds_since_wrong: 0,
            time_last_answer: 0,
        }
    }
}

pub fn handle_correct_answer(
    learning: &mut HashMap<String, LearnState>,
    identifier: &str,
    config: &config::Config,
) {
    let learn_state = learning
        .entry(identifier.to_string())
        .or_insert(LearnState::new());
    learn_state.current_bin += 1;
    if learn_state.current_bin > config.max_learn_bin {
        learn_state.current_bin = config.max_learn_bin;
    }
    learn_state.correct += 1;
    learn_state.rounds_since_wrong += 1;
    learn_state.time_last_answer = helper::get_current_unixtime_in_sec();
}

pub fn handle_wrong_answer(learning: &mut HashMap<String, LearnState>, identifier: &str) {
    let learn_state = learning
        .entry(identifier.to_string())
        .or_insert(LearnState::new());
    learn_state.current_bin = 1;
    learn_state.wrong += 1;
    learn_state.rounds_since_wrong = 0;
    learn_state.time_last_answer = helper::get_current_unixtime_in_sec();
}

pub fn load_learning() -> HashMap<String, LearnState> {
    let config_dir_path = Path::new("./learning");
    if !config_dir_path.exists() {
        helper::ensure_dir_existence("./learning");
    }

    let config_file_path = Path::new("./learning/learning.json");
    if !config_file_path.exists() {
        let default_learning: HashMap<String, LearnState> = HashMap::new();
        let learning_json = serde_json::to_string_pretty(&default_learning).unwrap();
        helper::overwrite_file_str("./learning/learning.json", &learning_json);
    }

    let learning_text = helper::read_filetext("./learning/learning.json");
    serde_json::from_str(&learning_text).unwrap()
}

pub fn save_learning(learn_state: &HashMap<String, LearnState>) {
    let json_string = serde_json::to_string_pretty(learn_state).unwrap();
    helper::overwrite_file_str("./learning/learning.json", &json_string);
}

#[derive(PartialEq, Debug)]
pub enum Answer {
    A,
    B,
    C,
    D,
}

pub struct PrintQuestion {
    pub question: Question,
    pub answer_shuffle: Vec<Answer>,
}

impl PrintQuestion {
    pub fn get_correct_answer(&self) -> Answer {
        if self.answer_shuffle[0] == Answer::A {
            Answer::A
        } else if self.answer_shuffle[1] == Answer::A {
            Answer::B
        } else if self.answer_shuffle[2] == Answer::A {
            Answer::C
        } else {
            Answer::D
        }
    }

    pub fn get_shuffled_answer(&self, index: usize) -> String {
        match self.answer_shuffle[index] {
            Answer::A => String::from(&self.question.answer_a),
            Answer::B => String::from(&self.question.answer_b),
            Answer::C => String::from(&self.question.answer_c),
            Answer::D => String::from(&self.question.answer_d),
        }
    }

    pub fn get_shuffled_picture(&self, index: usize) -> String {
        match self.answer_shuffle[index] {
            Answer::A => String::from(&self.question.picture_a),
            Answer::B => String::from(&self.question.picture_b),
            Answer::C => String::from(&self.question.picture_c),
            Answer::D => String::from(&self.question.picture_d),
        }
    }
}

pub fn get_next_print_question(
    eligible_questions: &Vec<Question>,
    learning: &mut HashMap<String, LearnState>,
    config: &Config,
) -> PrintQuestion {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut index = rng.gen_range(0..eligible_questions.len() - 1);
    let mut counter = 0;
    let mut ignore_preferences = false;
    loop {
        let question = &eligible_questions[index];
        let learn_state = learning
            .entry(String::from(&question.identifier))
            .or_insert(LearnState::new());

        let mut is_eligible = false;
        if !ignore_preferences {
            let is_marked = (learn_state.marked) && config.prefer_marked;
            let is_wrong = (learn_state.wrong > 0) && config.prefer_wrong;
            let is_new = ((learn_state.wrong + learn_state.correct) == 0) && config.prefer_new;
            if is_marked || is_wrong || is_new {
                is_eligible = true;
            }
        } else {
            let threshold = match learn_state.current_bin {
                1 => 5,
                2 => 40,
                3 => 70,
                4 => 80,
                5 => 85,
                _ => 0,
            };
            is_eligible = rng.gen_range(0..=100) > threshold;
        }

        if is_eligible {
            let mut answer_shuffle = vec![Answer::A, Answer::B, Answer::C, Answer::D];
            answer_shuffle.shuffle(&mut rng);

            return PrintQuestion {
                question: question.clone(),
                answer_shuffle,
            };
        }

        index = (index + 1) % eligible_questions.len();

        counter += 1;
        ignore_preferences = counter >= eligible_questions.len();
    }
}
