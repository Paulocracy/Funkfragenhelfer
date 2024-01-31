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
}

pub fn handle_correct_answer(
    learning: &mut HashMap<String, LearnState>,
    identifier: &str,
    config: &config::Config,
) {
    let current_bin = match learning.get(identifier) {
        Some(learn_state) => learn_state.current_bin,
        None => 1,
    };
    if current_bin > config.max_learn_bin {
        return;
    }
    learning
        .entry(identifier.to_string())
        .or_insert(LearnState { current_bin: 0 })
        .current_bin += 1;
}

pub fn handle_wrong_answer(learning: &mut HashMap<String, LearnState>, identifier: &str) {
    let current_bin = match learning.get(identifier) {
        Some(learn_state) => learn_state.current_bin,
        None => 2,
    };
    if current_bin <= 1 {
        return;
    }
    learning
        .entry(identifier.to_string())
        .or_insert(LearnState { current_bin: 1 })
        .current_bin = 1;
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
    println!("C");
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
    learning: &HashMap<String, LearnState>,
) -> PrintQuestion {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut counter = rng.gen_range(0..eligible_questions.len() - 1);
    loop {
        let question = &eligible_questions[counter];
        let learn_state = learning
            .get(&question.identifier)
            .unwrap_or(&LearnState { current_bin: 1 });

        let threshold = match learn_state.current_bin {
            1 => 5,
            2 => 40,
            3 => 70,
            4 => 80,
            5 => 85,
            _ => 95,
        };

        if rng.gen_range(0..=100) > threshold {
            let mut answer_shuffle = vec![Answer::A, Answer::B, Answer::C, Answer::D];
            answer_shuffle.shuffle(&mut rng);

            return PrintQuestion {
                question: question.clone(),
                answer_shuffle,
            };
        }

        if counter >= eligible_questions.len() - 1 {
            counter = 0;
        } else {
            counter += 1;
        }
    }
}
