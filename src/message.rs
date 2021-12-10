use crate::error_type::ErrorType;
use crate::db::*;

use crate::game::Answer;

use std::{mem::discriminant};

pub const NAME_LEN: usize = 31;
pub const QUESTION_LEN: usize = 200;
pub const ANSWER_LEN: usize = 50;

type NameArray = [u8; NAME_LEN];
type QuestionArray = [u8; QUESTION_LEN];
type AnswerArray = [u8; ANSWER_LEN];
type I32Array = [u8; std::mem::size_of::<i32>()];

#[derive(Clone)]
pub enum Message {
    NewPlayer(NameArray),
    Ok,
    Finalize,
    Preferences(Category, Category, Category, Category),
    Choice(Category),
    Nok(ErrorType),
    Points(I32Array, NameArray),
    Question(Difficulty, Category, QuestionArray, AnswerArray, AnswerArray, AnswerArray, AnswerArray),
    Winner(NameArray),
    Answer(Answer)
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        discriminant(self) == discriminant(other) 
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Message::NewPlayer(_) => write!(f, "NewPlayer"),
            Message::Ok => write!(f, "Ok"),
            Message::Nok(_) => write!(f, "Nok"),
            Message::Question(_,_,_,_,_,_,_) => write!(f, "SongsQuestion"),
            Message::Answer(_) => write!(f, "SongsAnswer"),
            Message::Finalize => write!(f, "Finalize"),
            Message::Points(_,_) => write!(f, "Points"),
            Message::Winner(_) => write!(f, "Winner"),
            Message::Preferences(_,_,_,_) => write!(f, "Preferences"),
            Message::Choice(_) => write!(f, "Choice"),
        }
    }
}

impl Message {
    pub fn text_from_bytes(bytes: &[u8]) -> &str {
        let last = if let Some(indx) = bytes[0..].iter().position(|b| *b == b'\0') { indx } else { bytes.len() };
        std::str::from_utf8(&bytes[0..last]).unwrap()
    }

    pub fn bytes_to_name(name: &str) -> NameArray {
        let mut to_ret: NameArray = [0;NAME_LEN];
        &to_ret[0..name.len()].copy_from_slice(name.as_bytes());
        to_ret
    }

    pub fn i32_to_bytes(num: i32) -> I32Array {
        num.to_ne_bytes()
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*(self as *const Message as *const [u8;std::mem::size_of::<Message>()]) }
    }

    pub fn from_bytes(bytes: &[u8]) -> &Message {
        unsafe { &*(bytes as *const [u8] as *const Message) }
    }

    pub const fn size() -> usize {
        std::mem::size_of::<Message>()
    }
}