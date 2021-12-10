use crate::db::{Category, Difficulty};

pub fn url_start() -> QuestionNumberBuilder {
    QuestionNumberBuilder::new(String::from("https://opentdb.com/api.php?type=multiple"))
}

pub struct QuestionNumberBuilder {
    url: String
}

pub struct CategoryBuilder {
    url: String
}

pub struct DifficultyBuilder {
    url: String
}

pub trait UrlBuilder {
    fn build(&self) -> &str;
}

macro_rules! impl_url {
    ($builder_name:ident) => {
        impl UrlBuilder for $builder_name { fn build(&self) -> &str { &self.url } }
    };
}

impl_url!(DifficultyBuilder);
impl_url!(CategoryBuilder);
impl_url!(QuestionNumberBuilder);

impl DifficultyBuilder {
    fn new(url: String) -> DifficultyBuilder {
        DifficultyBuilder {
            url: url
        }
    }

    pub fn difficulty(mut self, difficulty: Option<Difficulty>) -> Box<dyn UrlBuilder> {
        if let Some(d) = difficulty {
            self.url.push_str(&format!("&difficulty={}", d.to_string()));
        }
        Box::new(DifficultyBuilder::new(self.url))
    }
}

impl CategoryBuilder {
    fn new(url: String) -> CategoryBuilder {
        CategoryBuilder {
            url: url
        }
    }

    pub fn category(mut self, category: Option<Category>) -> DifficultyBuilder {
        if let Some(c) = category {
            if c != Category::Any {
                self.url.push_str(&format!("&category={}", c as u32));
            }
        }
        DifficultyBuilder::new(self.url)
    }
}

impl QuestionNumberBuilder {
    fn new(url: String) -> QuestionNumberBuilder {
        QuestionNumberBuilder {
            url: url
        }
    }

    pub fn number_of_questions(mut self, number: Option<u32>) -> CategoryBuilder {
        if let Some(n) = number {
            self.url.push_str(&format!("&amount={}", n));
        }
        CategoryBuilder::new(self.url)
    }
}