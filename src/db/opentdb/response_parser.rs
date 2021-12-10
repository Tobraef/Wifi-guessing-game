use crate::db::{Question, Category, Difficulty};
use serde::{Deserialize, Serialize};

pub struct ResponseParser {}

impl ResponseParser {
    pub fn new() -> ResponseParser {
        ResponseParser {}
    }

    pub fn parse(&self, resp: &str) -> Option<Vec<Question>> {
        let resp: TdbResponse = serde_json::from_str(resp).ok()?;
        if resp.response_code != 0 {
            None
        } else {
            Some(resp.results
                .into_iter()
                .map(|x| Question::from(x))
                .collect())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TdbQuestion {
    category: String,
    r#type: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TdbResponse {
    response_code: i32,
    results: Vec<TdbQuestion>
}

impl From<TdbQuestion> for Question {
    fn from(q: TdbQuestion) -> Question {
        println!("received question: {:?}", q);
        //fn new(c: Category, d: Difficulty, q: String, ans: Vec<String>) -> Question {
        Question::new(
            Category::from(q.category.as_str()), 
            Difficulty::from(q.difficulty.as_str()), 
            q.question, 
            std::iter::once(q.correct_answer).chain(q.incorrect_answers.into_iter()).collect())
    }
}