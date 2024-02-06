//! In this module, the structs and functions for the learning
//! algorithm can be found. See the comment of the "LearnState"
//! and "PrintQuestion" structs for more.

// IMPORTS SECTION //
use crate::config::Config;
use crate::question::Question;
use crate::{config, helper};
use rand::seq::SliceRandom;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ENUMS SECTION //
/// Represents the possible answers A to B.
#[derive(PartialEq, Debug)]
pub enum Answer {
    A,
    B,
    C,
    D,
}

// STRUCTS SECTION //
/// Represents the learning progress or "state" of a question.
///
/// Each question can have its own `LearnState` instance, which
/// describes, e.g., how many times the question was answered
/// correctly or wrongly. In addition, it contains the learning
/// "bin", which has a minimal value of 1 and a maximum of 5. Each
/// time the user answers a question corrcectly, the bin rises by 1 (or
/// stays at the maximum). Is the answer wrong, a question's bin is
/// set to 1 again. Now, the higher the bin, the lower the probability
/// is that the question is asked.
#[derive(Serialize, Deserialize, Debug)]
pub struct LearnState {
    /// The current learning "bin"
    pub current_bin: u64,
    /// Number of correct answers up to now
    pub correct: u64,
    /// Number of wrong answers up to now
    pub wrong: u64,
    /// Whether or not the user marked the question
    pub marked: bool,
    /// Number of times the question was asked since the user
    /// gave a wrong answer
    pub rounds_since_wrong: u64,
    /// The UNIX time when this question was last answered
    pub time_last_answer: u64,
}

impl LearnState {
    /// Creates a new `LearnState` with default values,
    /// as if this question was never asked.
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

/// Type alias for the full collection of LearnState instances.
pub type LearnStates = HashMap<String, LearnState>;

/// Representation of a print-friendly question.
///
/// In contrast to a normal Question, which is included here too,
/// this struct also constaint an "answer shuffle" which is a randomly
/// shuffled vector of the Answer instances. By following this vector,
/// a user interface can determine the random order of shown answers.
pub struct PrintQuestion {
    pub question: Question,
    pub answer_shuffle: Vec<Answer>,
}

impl PrintQuestion {
    /// Looks where the "real" answer A is in the randomly shuffled answers
    /// and returns its current Answer value. A is looked up as in
    /// the questions database of the Bundesnetzagentur, A is always
    /// the correct answer.
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

    /// Returns the question string for the given randomly shuffled answer.
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

// PUBLIC FUNCTIONS SECTION //
/// Updates the LearnState of the given question for the case that
/// it was answered correctly.
///
/// ### Arguments
/// * learning: All current LearnStates
/// * identifier: The correctly answered question's identifier (i.e., th
///   LearnStates key).
/// * config: The current Funkfragenhelfer configurarion; Determined the
///   maximal bin.
pub fn handle_correct_answer(
    learning: &mut LearnStates,
    identifier: &str,
    config: &config::Config,
) {
    // Get the LearnState of the question (or create one
    // if it doesn't exist yet)
    let learn_state = learning
        .entry(identifier.to_string())
        .or_insert(LearnState::new());
    // Set the bin 1 higher (or keep it at the maximum)
    if learn_state.current_bin < config.max_learn_bin {
        learn_state.current_bin += 1;
    }
    // Update the rest of the LearnState statistics
    learn_state.correct += 1;
    learn_state.rounds_since_wrong += 1;
    learn_state.time_last_answer = helper::get_current_unixtime_in_sec();
}

/// Updates the LearnState of the given question for the case that
/// it was answered wrongly.
///
/// ### Arguments
/// * learning: All current LearnStates
/// * identifier: The correctly answered question's identifier (i.e., th
///   LearnStates key).
pub fn handle_wrong_answer(learning: &mut LearnStates, identifier: &str) {
    // Get the LearnState of the question (or create one
    // if it doesn't exist yet)
    let learn_state = learning
        .entry(identifier.to_string())
        .or_insert(LearnState::new());
    // Set the bin to 1
    learn_state.current_bin = 1;
    // Update the rest of the LearnState statistics
    learn_state.wrong += 1;
    learn_state.rounds_since_wrong = 0;
    learn_state.time_last_answer = helper::get_current_unixtime_in_sec();
}

/// Loads the ./learninglearning.json file, or creates one if it doesn't exist.
///
/// This file contains a LearnStates JSON representation and represents the full information
/// of each question's current learning progress.
///
/// ### Return value
/// * The LearnStates, i.e. HashMap<String, LearnState>, of each question (with the
///   question identifiers as keys and the associated LearnState instances as values)
pub fn load_learning() -> LearnStates {
    let config_dir_path = Path::new("./learning");
    if !config_dir_path.exists() {
        helper::ensure_dir_existence("./learning");
    }

    let config_file_path = Path::new("./learning/learning.json");
    if !config_file_path.exists() {
        let default_learning: LearnStates = HashMap::new();
        let learning_json = serde_json::to_string_pretty(&default_learning).unwrap();
        helper::overwrite_file_str("./learning/learning.json", &learning_json);
    }

    let learning_text = helper::read_filetext("./learning/learning.json");
    serde_json::from_str(&learning_text).unwrap()
}

/// Saves the current LearnStates into the ./learning/learning.json
///
/// The JSON file is going to be overwritten in the process. That the "learning" dir
/// exists is ensured through this module's load_learning() beforehand.
///
/// ### Arguments
/// * learn_states: The current LearnStates for each question
pub fn save_learning(learn_states: &LearnStates) {
    let json_string = serde_json::to_string_pretty(learn_states).unwrap();
    helper::overwrite_file_str("./learning/learning.json", &json_string);
}

/// Out of the selected question categories and according to filters, select next question.
///
/// This function essentially implements the simple learning algorithm used by
/// Funkfragenhelfer; Out of the eligible questions (i.e., the ones which are a
/// member of the selected categories), the following is done:
///
/// 1. Start with a randomly chosen index for the list of eligible questions
/// 2. If filters are active: If the question at the current index fits to the
///    filters, return this question. If not, raise the index by 1 (or set to 0
///    if greater than the list length) until a filter-fitting question appears
///    and can be returned. If no question fits to the filters, proceed with step
///    3.
/// 3. Without filters: Look at the question at the current index. The higher its
///    bin, the lower the probability is chosen. If chosen, return the question.
///    If not chosen, proceed with the next question. This is repeated until a
///    question is chosen.
///
/// ### Arguments
/// * eligible_questions: The list of category-fitting questions from which one is chosen.
/// * learning: The current LearnStates, providing, e.g., the question bins.
/// * config: The current Funkfragenhelfer Config.
///
/// ### Return value
/// * A PrintQuestion, which includes the chosen question as well as randomly shuffled answers
///   (as in the original Bundesnetzagentur questions set, answer A is always correct).
pub fn get_next_print_question(
    eligible_questions: &Vec<Question>,
    learning: &mut LearnStates,
    config: &Config,
) -> PrintQuestion {
    // Setup randomness
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    // Question-choosing state variables
    // The current randomly initialized questions index
    let mut index = rng.gen_range(0..eligible_questions.len() - 1);
    // A counter counting how many questions were already looked up
    let mut counter = 0;
    // Start the question-choosing loop, which runs the algorithm as described
    // in this method's description.
    loop {
        // Get (or create if non-existent) the current LearnState
        let question = &eligible_questions[index];
        let learn_state = learning
            .entry(String::from(&question.identifier))
            .or_insert(LearnState::new());

        // Variable choosing whether or not a question is chosen for the return value
        let mut is_chosen = false;
        // If we didn't already look at all questions...
        if !(counter >= eligible_questions.len()) {
            // ...look up if any filters apply to the current question...
            let is_marked = (learn_state.marked) && config.prefer_marked;
            let is_wrong = (learn_state.wrong > 0) && config.prefer_wrong;
            let is_new = ((learn_state.wrong + learn_state.correct) == 0) && config.prefer_new;
            // ...if yes: Choose it
            if is_marked || is_wrong || is_new {
                is_chosen = true;
            }
        } else {
            // If no question applies to filters, look at its bin and get the
            // associated choosing probability (the higher the bin, the lower
            // probability)
            let threshold = match learn_state.current_bin {
                1 => 5,
                2 => 40,
                3 => 70,
                4 => 80,
                5 => 85,
                _ => 0,
            };
            // If the threshold is *lower* than a random value, choose the question
            is_chosen = rng.gen_range(0..=100) > threshold;
        }

        // If a question is chosen as return value...
        if is_chosen {
            // ...shuffle its answers randomly...
            let mut answer_shuffle = vec![Answer::A, Answer::B, Answer::C, Answer::D];
            answer_shuffle.shuffle(&mut rng);

            // ...and return the shuffle result together with the Question
            return PrintQuestion {
                question: question.clone(),
                answer_shuffle,
            };
        }

        // Raise the index or set to 0 if it is greater than the eligible questions length
        index = (index + 1) % eligible_questions.len();
        // Raise as we just looked at one more question
        counter += 1;
    }
}
