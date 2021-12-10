use crate::communication::Center;
use crate::message::*;
use crate::db::{Question, Category};

use super::Answer;

use std::{sync::Mutex, time::{Instant, Duration}, ops::Sub};

pub trait Exchange {
    fn standard_question(&mut self, question: &Question) -> bool;

    fn available_categories(&mut self, categories: Vec<Category>) -> bool;

    fn collect_categories(&mut self) -> Vec<Category>;

    fn collect_answers(&mut self) -> Vec<(Answer, String, Duration)>;

    fn exchange_points(&mut self, pts: &Vec<(String, i32)>);

    fn announce_winner(&mut self, winner: &str);

    fn wait_for_ready(&mut self);
}

impl Exchange for Center {
    fn standard_question(&mut self, q: &Question) -> bool {
        let mut anwers_bytes = q.answers.iter().map(|t| {
            let mut to_ret = [0u8; ANSWER_LEN];
            to_ret[0..t.len()].clone_from_slice(t.as_bytes());
            to_ret
        });
        let mut question_bytes = [0u8; QUESTION_LEN];
        question_bytes[0..q.question.len()].clone_from_slice(q.question.as_bytes());
        //Question(Difficulty, Category, QuestionArray, AnswerArray, AnswerArray, AnswerArray, AnswerArray)
        self.send_to_players(Message::Question(
            q.difficulty.clone(), q.category.clone(), 
            question_bytes,
            anwers_bytes.next().unwrap(),
            anwers_bytes.next().unwrap(),
            anwers_bytes.next().unwrap(),
            anwers_bytes.next().unwrap()))
    }

    fn available_categories(&mut self, categories: Vec<Category>) -> bool {
        let mut iter = categories.into_iter();
        self.send_to_players(Message::Preferences(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()))
    }

    fn collect_categories(&mut self) -> Vec<Category> {
        let responses = Mutex::new(Vec::new());
        self.listen_to_players(&mut |m, n| {
           if let Message::Choice(c) = m {
               responses.lock().unwrap().push(c);
               true
           } else {
               println!("Received unexpected message from {}: {:?}", n, m);
               false
           }
        });
        responses.into_inner().unwrap()
    }

    fn collect_answers(&mut self) -> Vec<(Answer, String, Duration)> {
        let start_time = Instant::now();
        let responses = Mutex::new(Vec::new());
        self.listen_to_players(&mut |m, n| {
            if let Message::Answer(a) = m {
                let current_time = Instant::now();
                let mut guard = responses.lock().unwrap();
                guard.push((a, String::from(n), current_time.sub(start_time)));
                true
            } else {
                println!("Received unexpected message from {}: {:?}", n, m);
                false
            }
        });
        responses.into_inner().unwrap()
    }

    fn exchange_points(&mut self, pts: &Vec<(String, i32)>) {
        for data in pts.into_iter() {
            self.send_to_players(Message::Points(Message::i32_to_bytes(data.1), Message::bytes_to_name(&data.0)));
        }
        self.send_to_players(Message::Finalize);
    }

    fn announce_winner(&mut self, winner: &str) {
        self.send_to_players(Message::Winner(Message::bytes_to_name(winner)));
    }

    fn wait_for_ready(&mut self) {
        self.listen_to_players(&mut |m, _n| match m {
            Message::Ok => true,
            _ => false
        });
    }
}

