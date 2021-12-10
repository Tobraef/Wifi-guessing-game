#[derive(Clone, PartialEq, Debug, Hash, Eq)]
#[repr(u8)]
pub enum Category {
    Any = 0,
    GeneralKnowledge = 9,
    Books = 10,
    Film = 11,
    Music = 12,
    Television = 14,
    VideoGames = 15,
    ScienceAndNature = 17,
    Computers = 18,
    Mathematics = 19,
    Mythology = 20,
    Sports = 21,
    Geography = 22,
    History = 23,
    Animals = 27,
    Vehicles = 28,
    Gadgets = 30,
    Cartoon = 32,
}

impl From<&str> for Category {
    fn from(c: &str) -> Category {
        match c {
            "Any" => Category::Any,
            "General Knowledge" => Category::GeneralKnowledge,
            "Entertainment: Books" => Category::Books,
            "Entertainment: Film" => Category::Film,
            "Entertainment: Music" => Category::Music,
            "Entertainment: Television" => Category::Television,
            "Entertainment: Video Games" => Category::VideoGames,
            "Science & Nature" => Category::ScienceAndNature,
            "Science: Computers" => Category::Computers,
            "Science: Mathematics" => Category::Mathematics,
            "Mythology" => Category::Mythology,
            "Sports" => Category::Sports,
            "Geography" => Category::Geography,
            "History" => Category::History,
            "Animals" => Category::Animals,
            "Vehicles" => Category::Vehicles,
            "Science: Gadgets" => Category::Gadgets,
            "Entertainment: Cartoon & Animations" => Category::Cartoon,
            _ => panic!("Unrecognized category: {}", c),
        }
    }
}

pub fn all_categories() -> Vec<Category> {
    vec![
        Category::GeneralKnowledge,
        Category::Books,
        Category::Film,
        Category::Music,
        Category::Television,
        Category::VideoGames,
        Category::ScienceAndNature,
        Category::Computers,
        Category::Mathematics,
        Category::Mythology,
        Category::Sports,
        Category::Geography,
        Category::History,
        Category::Animals,
        Category::Vehicles,
        Category::Gadgets,
        Category::Cartoon]
}