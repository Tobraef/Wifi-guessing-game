#[derive(Clone, PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl From<&str> for Difficulty {
    fn from(d: &str) -> Difficulty {
        match d {
            "hard" => Difficulty::Hard,
            "medium" => Difficulty::Medium,
            "easy" => Difficulty::Easy,
            _ => panic!("Couldn't recognize difficulty: {}", d)
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Difficulty::Hard => "hard",
            Difficulty::Medium => "medium",
            Difficulty::Easy => "easy",
        })
    }
}