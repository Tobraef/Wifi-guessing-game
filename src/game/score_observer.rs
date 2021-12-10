use std::{collections::HashMap, iter::Iterator};

pub struct ScoreObserver {
    scores: HashMap<String, i32>,
    limit: i32
}

impl ScoreObserver {
    pub fn new(pts_to_win: i32) -> ScoreObserver {
        ScoreObserver {
            scores: HashMap::new(),
            limit: pts_to_win
        }
    }

    pub fn note_scores<'a, I: Iterator<Item = &'a(String, i32)>>(&mut self, scores: I) -> Option<String> {
        for score in scores {
            if let Some(v) = self.scores.get_mut(&score.0) {
                *v += score.1;
            } else {
                self.scores.insert(score.0.clone(), score.1);
            }
        }
        let max = self.scores.iter().max_by(|l,r| l.1.cmp(&r.1)).unwrap().1;
        return if *max >= self.limit {
            Some(self.scores.iter().find(|kvp| *kvp.1 == *max).unwrap().0.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIN_PTS: i32 = 100;
    const P1: &str = "A";
    const P2: &str = "B";
    const P3: &str = "C";

    #[test]
    fn score_noting() {
        let mut sut = ScoreObserver::new(WIN_PTS);
        
        let score = vec![(String::from(P1), 30), (String::from(P2), 20), (String::from(P3), -10)];
        assert!(sut.note_scores(score.iter()).is_none());
        // 30 20 -10
        let score = vec![(String::from(P1), 50), (String::from(P2), 40), (String::from(P3), -10)];
        assert!(sut.note_scores(score.iter()).is_none());
        // 80 60 -20
        let score = vec![(String::from(P1), 19), (String::from(P2), 39), (String::from(P3), -10)];
        assert!(sut.note_scores(score.iter()).is_none());
        // 99 99 -30
        let score = vec![(String::from(P1), 1), (String::from(P2), 0), (String::from(P3), 132)];
        assert_eq!(sut.note_scores(score.iter()).unwrap(), P3);
        // 101 99 102
        let score = vec![(String::from(P1), 1), (String::from(P2), 4), (String::from(P3), 0)];
        assert_eq!(sut.note_scores(score.iter()).unwrap(), P2);
        // 102 103 102       
    }
}