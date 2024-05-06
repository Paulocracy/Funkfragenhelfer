//! This module includes the representation of questions
//! themselves, as well as the function for getting all
//! questions that are members of the chosen category.

// IMPORTS SECTION //
use crate::config::Config;
use serde::{Deserialize, Serialize};

// ENUM SECTION //
/// Representation of main questions categories, i.e.:
/// * V - Vorschriften
/// * B - Betriebliches
/// * N - Technik Klasse N
/// * E - Technik Klasse E
/// * A - Technik Klasse A
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Category {
    V,
    B,
    N,
    E,
    A,
}

// STRUCT SECTION //
/// Representation of the full data for a question, including
/// its question category, question and answer texts.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Question {
    /// The question's main category
    pub category: Category,
    /// The question's identifier or "number"
    pub identifier: String,
    /// The actual question text
    pub question: String,

    /// Text of answer A (if existent, otherwise "")
    pub answer_a: String,
    /// Text of answer B (if existent, otherwise "")
    pub answer_b: String,
    /// Text of answer C (if existent, otherwise "")
    pub answer_c: String,
    /// Text of answer D (if existent, otherwise "")
    pub answer_d: String,

    /// Text of picture question (if existent, otherwise "")
    pub picture_question: String,
    /// Text of picture answer A (if existent, otherwise "")
    pub picture_a: String,
    /// Text of picture answer B (if existent, otherwise "")
    pub picture_b: String,
    /// Text of picture answer C (if existent, otherwise "")
    pub picture_c: String,
    /// Text of picture answer D (if existent, otherwise "")
    pub picture_d: String,
}

// PUBLIC FUNCTION SECTION //
/// Out of the given set of questions, a new set of questions is returned
/// which includes only the questions that are a part of the config-allowed
/// categories.
pub fn get_eligible_questions(questions: &Vec<Question>, config: &Config) -> Vec<Question> {
    let mut eligible_questions = Vec::new();
    for question in questions {
        match question.category {
            Category::V => {
                if config.include_v {
                    eligible_questions.push(question.clone());
                }
            }
            Category::B => {
                if config.include_b {
                    eligible_questions.push(question.clone());
                }
            }
            Category::N => {
                if config.include_n {
                    eligible_questions.push(question.clone());
                }
            }
            Category::E => {
                if config.include_e {
                    eligible_questions.push(question.clone());
                }
            }
            Category::A => {
                if config.include_a {
                    eligible_questions.push(question.clone());
                }
            }
        }
    }
    eligible_questions
}
