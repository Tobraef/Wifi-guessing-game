use super::{Category, Difficulty};

pub struct Question {
    pub category: Category,
    pub difficulty: Difficulty,
    pub answers: [String; 4],
    pub question: String,
}

impl Question {
    pub fn new(c: Category, d: Difficulty, q: String, ans: Vec<String>) -> Question {
        let mut ans_iter = ans.into_iter();
        Question {
            category: c,
            difficulty: d,
            answers: [
                ans_iter.next().unwrap(), 
                ans_iter.next().unwrap(), 
                ans_iter.next().unwrap(), 
                ans_iter.next().unwrap()],
            question: q,
        }
    }
}