use super::{Category,Difficulty,Question};

pub trait Context {
    fn load_questions(&self, d: Option<Difficulty>, c: Option<Category>, count: Option<u32>) -> Vec<Question>;
}