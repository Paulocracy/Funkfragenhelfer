use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Category {
    V,
    B,
    N,
    E,
    A,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Question {
    pub category: Category,
    pub identifier: String,
    pub question: String,

    pub answer_a: String,
    pub answer_b: String,
    pub answer_c: String,
    pub answer_d: String,

    pub picture_question: String,
    pub picture_a: String,
    pub picture_b: String,
    pub picture_c: String,
    pub picture_d: String,
}

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
    return eligible_questions;
}
