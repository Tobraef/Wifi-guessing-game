use super::{Exchange, rewards, ScoreObserver};
use crate::db::{context::Context,Question,Category,Difficulty,category};
use crate::randomizer::EnumRandomizer;

use std::{thread, time::Duration};

pub struct GameLoop<E: Exchange, Ctx: Context> {
    exchange: E,
    ctx: Ctx,
    scores: ScoreObserver,
    category_randomizer: EnumRandomizer<Category>,
    difficulty_randomizer: EnumRandomizer<Difficulty>
}

impl<E: Exchange, Ctx: Context> GameLoop<E, Ctx> {
    pub fn new(exchange: E, ctx: Ctx, pts_to_win: i32) -> GameLoop<E, Ctx> {
        GameLoop {
            exchange: exchange,
            ctx: ctx,
            scores: ScoreObserver::new(pts_to_win),
            category_randomizer: EnumRandomizer::new(&category::all_categories()),
            difficulty_randomizer: EnumRandomizer::new(&vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard])
        }
    }

    fn break_between_questions(&self) {
        println!("Next task in...");
        for i in (1..=5).rev() {
            println!("{}", i);
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn roll_question(&mut self) -> Question {
        let _success = self.exchange.available_categories(self.category_randomizer.random_group(4));
        let chosen_categories = self.exchange.collect_categories();
        let random_category = EnumRandomizer::new(&chosen_categories).random();
        self.ctx.load_questions(
            Some(self.difficulty_randomizer.random()), Some(random_category), Some(1))
        .into_iter()
        .next()
        .unwrap()
    }

    fn ask_question(&mut self) -> Difficulty {
        let question = self.roll_question();
        self.exchange.standard_question(&question);
        question.difficulty.clone()
    }

    pub fn main_loop(&mut self) {
        loop {
            self.break_between_questions();
            let question_difficulty = self.ask_question();
            let answers = self.exchange.collect_answers();
            let rewards = rewards::award_points(answers, question_difficulty);
            if let Some(name) = self.scores.note_scores(rewards.iter()) {
                self.exchange.announce_winner(&name);
                break;
            } else {
                self.exchange.exchange_points(&rewards);
                self.exchange.wait_for_ready();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Answer;

    const Q1: &str = "Q1";
    const A1: &str = "A1";
    const A2: &str = "A2";
    const A3: &str = "A3";
    const A4: &str = "A4";
    const P1: &str = "P1";
    const P2: &str = "P2";

    struct TestCtx();
    impl Context for TestCtx {
        fn load_questions(&self, d: Option<Difficulty>, c: Option<Category>, count: Option<u32>) -> Vec<Question> {
            let dif = d.unwrap().clone();
            let cat = c.unwrap().clone();
            assert_eq!(cat, Category::Books);
            std::iter::repeat_with(move || 
                    Question::new(cat.clone(), dif.clone(), String::from(Q1), vec![A1.to_owned(), A2.to_owned(), A3.to_owned(), A4.to_owned()]))
                .take(count.unwrap() as usize)
                .collect()
        }
    }

    struct TestExchange {
        categories_asked: bool,
        categories_pulled: bool,
        question_asked: bool,
        answers_pulled: bool,
        pts_sent: bool,
    }

    impl TestExchange {
        fn new() -> TestExchange {
            TestExchange {
                categories_asked: false,
                categories_pulled: false,
                question_asked: false,
                answers_pulled: false,
                pts_sent: true,
            }
        }
    }

    fn check_and_false(v: &mut bool) {
        assert!(*v);
        *v = false;
    }

    impl Exchange for TestExchange {
        fn standard_question(&mut self, question: &Question) -> bool {
            check_and_false(&mut self.categories_pulled);
            assert_eq!(question.question, Q1);
            for (a1, a2) in [A1, A2, A3, A4].iter().zip(question.answers.iter()) {
                assert_eq!(a1, a2);
            }
            self.question_asked = true;
            true
        }

        fn available_categories(&mut self, _categories: Vec<Category>) -> bool {
            check_and_false(&mut self.pts_sent);
            self.categories_asked = true;
            true
        }

        fn collect_categories(&mut self) -> Vec<Category> {
            check_and_false(&mut self.categories_asked);
            self.categories_pulled = true;
            vec![Category::Books, Category::Books]
        }

        fn collect_answers(&mut self) -> Vec<(Answer, String, Duration)> {
            check_and_false(&mut self.question_asked);
            self.answers_pulled = true;
            vec![
                (Answer::Correct, P1.to_owned(), Duration::from_secs(3)),
                (Answer::Correct, P2.to_owned(), Duration::from_secs(4)),]
        }

        fn exchange_points(&mut self, pts: &Vec<(String, i32)>) {
            check_and_false(&mut self.answers_pulled);
            assert!(pts.iter().find(|x| x.0 == P1).unwrap().1 > pts.iter().find(|x| x.0 == P2).unwrap().1);
            self.pts_sent = true;
        }

        fn announce_winner(&mut self, winner: &str) {
            check_and_false(&mut self.answers_pulled);
            assert_eq!(winner, P1);
        }

        fn wait_for_ready(&mut self) {}
    }

    #[test]
    fn quick_loop() {
        let mut sut = GameLoop::new(TestExchange::new(), TestCtx(), 30 * 100 * 3);
        sut.main_loop();
    }
}