use std::collections::{HashMap};
use std::time::Duration;

use super::Answer;
use crate::db::Difficulty;

fn map_to_pts(d: Difficulty) -> i32 { 
    match d {
        Difficulty::Easy => 10,
        Difficulty::Medium => 20,
        Difficulty::Hard => 30,
    }
}

pub fn award_points<'a>(players_answers: Vec<(Answer, String, Duration)>, difficulty: Difficulty) -> Vec<(String, i32)> {
    let pts = map_to_pts(difficulty);
    let mut sorted_times: Vec<_> = players_answers.iter()
        .filter(|d| d.0 == Answer::Correct)
        .map(|d| d.2)
        .collect();
    sorted_times.sort();
    let time_to_reward: HashMap<_,_> = sorted_times.iter()
        .zip(
            (0..=(pts.max(players_answers.len() as i32)))
            .rev()
            .map(|n| n * 100))
        .collect();
    players_answers.into_iter()
    .map(|d| 
        match d.0 {
            Answer::Correct => (d.1, *time_to_reward.get(&d.2).unwrap()),
            Answer::Incorrect => (d.1, -20 * pts),
            Answer::Skip => (d.1, 0),
        })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1: &str = "A";
    const P2: &str = "B";
    const P3: &str = "C";

    #[test]
    fn reward_mapping() {
        let answers = vec![
            (Answer::Correct, String::from(P1), Duration::from_secs(5)),
            (Answer::Correct, String::from(P2), Duration::from_secs(4)),
            (Answer::Incorrect, String::from(P3), Duration::from_secs(1))];

        let awarded = award_points(answers, Difficulty::Hard);

        assert_eq!(awarded.iter().find(|x| x.0 == P2).unwrap().1, 30 * 100);
        assert_eq!(awarded.iter().find(|x| x.0 == P1).unwrap().1, 29 * 100);
        assert_eq!(awarded.iter().find(|x| x.0 == P3).unwrap().1, 30 * -20);

        let answers = vec![
            (Answer::Skip, String::from(P1), Duration::from_secs(5)),
            (Answer::Correct, String::from(P2), Duration::from_secs(4)),
            (Answer::Incorrect, String::from(P3), Duration::from_secs(1))];

            let awarded = award_points(answers, Difficulty::Hard);

            assert_eq!(awarded.iter().find(|x| x.0 == P1).unwrap().1, 0);
            assert_eq!(awarded.iter().find(|x| x.0 == P2).unwrap().1, 30 * 100);
            assert_eq!(awarded.iter().find(|x| x.0 == P3).unwrap().1, 30 * -20);
    }
}