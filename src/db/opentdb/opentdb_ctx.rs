use super::{response_parser::ResponseParser, url_builder};
use crate::db::{Context,Question,Category,Difficulty};

pub struct OpentdbCtx {
    parser: ResponseParser
}

impl OpentdbCtx {
    pub fn new() -> OpentdbCtx {
        OpentdbCtx {
            parser: ResponseParser::new()
        }
    }
}

impl Context for OpentdbCtx {
    fn load_questions(&self, d: Option<Difficulty>, c: Option<Category>, count: Option<u32>) -> Vec<Question> {
        let builder = url_builder::url_start()
            .number_of_questions(count)
            .category(c)
            .difficulty(d);
        let url = builder.build();
        println!("Url: {}", url);
        let resp = reqwest::blocking::get(url).expect(&format!("Didn't receive response from url: {}", url))
            .text().expect("Couldn't read text in response");
        self.parser.parse(&resp).expect("Error parsing json input")        
    }
}