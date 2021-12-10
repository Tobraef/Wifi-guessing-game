// #[allow(unused_imports)]
// use crate::preparation::game_prepare;
// #[allow(unused_imports)]
// use crate::game;
// #[allow(unused_imports)]
// use crate::db::*;

// #[allow(unused_imports)]
// use super::framework;
// #[allow(unused_imports)]
// use crate::message::*;

// #[allow(unused_imports)]
// use std::{thread, time::Duration, rc::Rc, sync::{Arc, Mutex}};

// #[allow(dead_code)]
// const ROOT: &str = "C://Praca//music";
// #[allow(dead_code)]
// const P1_NAME: &str = "Player 1";
// #[allow(dead_code)]
// const P2_NAME: &str = "Player 2";

// #[test]
// fn game_awarding_points() {
//     let (p1n, p2n) = (String::from(P1_NAME), String::from(P2_NAME));
//     let answers = vec![(game::Answer::Correct, p1n, Duration::from_secs(3)), (game::Answer::Correct, p2n, Duration::from_secs(4))];
//     let points = game::award_points(answers, Difficulty::Easy);

//     assert_eq!(points.iter().find(|kvp| kvp.0 == P1_NAME).unwrap().1, 10 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == P2_NAME).unwrap().1, 9 * 100);

//     let answers = vec![(P1_NAME, game::Answer::Correct, Duration::from_secs(100)), (P2_NAME, game::Answer::Skip, Duration::from_secs(4))];
//     let points = game::award_points(&answers, 10);

//     assert_eq!(points.iter().find(|kvp| kvp.0 == P1_NAME).unwrap().1, 10 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == P2_NAME).unwrap().1, 0 * 100);

//     let answers = vec![(P1_NAME, game::Answer::Correct, Duration::from_secs(100)), (P2_NAME, game::Answer::Incorrect, Duration::from_secs(4))];
//     let points = game::award_points(&answers, 10);

//     assert_eq!(points.iter().find(|kvp| kvp.0 == P1_NAME).unwrap().1, 10 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == P2_NAME).unwrap().1, -10 * 100 / 5);

//     let answers = vec![
//         (P1_NAME, game::Answer::Correct, Duration::from_secs(100)), 
//         (P2_NAME, game::Answer::Correct, Duration::from_secs(20)), 
//         ("A", game::Answer::Correct, Duration::from_secs(50)), 
//         ("B", game::Answer::Correct, Duration::from_secs(30)),
//         ("C", game::Answer::Skip, Duration::from_secs(60)), 
//         ("D", game::Answer::Incorrect, Duration::from_secs(10))];
//     let points = game::award_points(&answers, 10);

//     assert_eq!(points.iter().find(|kvp| kvp.0 == P1_NAME).unwrap().1, 7 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == P2_NAME).unwrap().1, 10 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == "A").unwrap().1, 8 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == "B").unwrap().1, 9 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == "C").unwrap().1, 0 * 100);
//     assert_eq!(points.iter().find(|kvp| kvp.0 == "D").unwrap().1, -10 * 100 / 5);
// }

// #[test]
// fn game_score_board() {
//     let scores = vec![(P1_NAME, 9), (P2_NAME, 6)];
//     let mut score_board = game::ScoreObserver::new(10);
//     assert!(score_board.note_scores(scores.iter()).is_none());
    
//     let scores = vec![(P1_NAME, -1), (P2_NAME, 3)];
//     assert!(score_board.note_scores(scores.iter()).is_none());
    
//     let scores = vec![(P1_NAME, 1), (P2_NAME, -1)];
//     assert!(score_board.note_scores(scores.iter()).is_none());
    
//     let scores = vec![(P1_NAME, 100), (P2_NAME, 100)];
//     assert_eq!(score_board.note_scores(scores.iter()).unwrap(), P1_NAME);

//     let mut score_board = game::ScoreObserver::new(5);
    
//     let scores = vec![(P1_NAME, 4), (P2_NAME, -5)];
//     assert!(score_board.note_scores(scores.iter()).is_none());
    
//     let scores = vec![(P1_NAME, 0), (P2_NAME, 10)];
//     assert_eq!(score_board.note_scores(scores.iter()).unwrap(), P2_NAME);
// }

// #[allow(dead_code)]
// fn assert_points(msg: &Message, p1_points: i32, p2_points: i32) {
//     if let Message::Points(p, n) = msg {
//         let n = Message::text_from_bytes(n);
//         match &n[..] {
//             P1_NAME => assert_eq!(*p, p1_points),
//             P2_NAME => assert_eq!(*p, p2_points),
//             _ => panic!("Couldn't parse '{}' Needs either '{}' or '{}'", n, P1_NAME, P2_NAME)
//         }
//     } else {
//         panic!("Not a points msg, actual {:?}", msg)
//     }
// }

// #[test]
// fn game_simple_two_players_game() {
//     let _client_thread = thread::spawn(|| {
//         let mut cl1 = framework::Client::new();
//         let mut cl2 = framework::Client::new();

//         cl1.send(Message::NewPlayer(Message::bytes_to_name(P1_NAME)));
//         cl2.send(Message::NewPlayer(Message::bytes_to_name(P2_NAME)));

//         assert_eq!(cl1.receive().join().unwrap(), Message::Preferences(Decade::Eightees, Decade::Eightees, Genre::Classical, Genre::Classical, Genre::Classical));
//         assert_eq!(cl2.receive().join().unwrap(), Message::Preferences(Decade::Eightees, Decade::Eightees, Genre::Classical, Genre::Classical, Genre::Classical));

//         cl1.send(Message::Preferences(Decade::Eightees, Decade::Milenium, Genre::Electronic, Genre::Classical, Genre::Metal));
//         cl2.send(Message::Preferences(Decade::Milenium, Decade::Eightees, Genre::Classical, Genre::Electronic, Genre::Metal));
//         //q1
//         let q1 = cl1.receive().join().unwrap();
//         let q2 = cl2.receive().join().unwrap();
//         use std::mem::discriminant;
//         assert_eq!(discriminant(&q1), discriminant(&q2));

//         cl2.send(Message::SongsAnswer(1));
//         let p1 = cl1.receive().join().unwrap();
        
//         cl1.send(Message::SongsAnswer(0));
//         let p2 = cl2.receive().join().unwrap();
//         assert_points(&p1, 1000, -200);
//         assert_points(&p2, 1000, -200);

//         let p3 = cl1.receive().join().unwrap();
//         let p4 = cl2.receive().join().unwrap();
//         assert_points(&p3, 1000, -200);
//         assert_points(&p4, 1000, -200);

//         let f = cl1.receive().join().unwrap();
//         assert_eq!(discriminant(&Message::Finalize), discriminant(&f));
//         let f = cl2.receive().join().unwrap();
//         assert_eq!(discriminant(&Message::Finalize), discriminant(&f));

//         cl1.send(Message::Ok);
//         cl2.send(Message::Ok);
//         //q2
//         let q1 = cl1.receive().join().unwrap();
//         let q2 = cl2.receive().join().unwrap();
//         assert_eq!(discriminant(&q1), discriminant(&q2));

//         cl2.send(Message::SongsAnswer(0));
//         let p2 = cl2.receive().join().unwrap();
        
//         let p1 = cl1.receive().join().unwrap();
//         cl1.send(Message::SongsAnswer(0));
//         assert_points(&p1, 900, 1000);
//         assert_points(&p2, 900, 1000);

//         let p3 = cl1.receive().join().unwrap();
//         let p4 = cl2.receive().join().unwrap();
//         assert_points(&p3, 900, 1000);
//         assert_points(&p4, 900, 1000);

//         let f = cl1.receive().join().unwrap();
//         assert_eq!(discriminant(&Message::Finalize), discriminant(&f));
//         let f = cl2.receive().join().unwrap();
//         assert_eq!(discriminant(&Message::Finalize), discriminant(&f));

//         cl1.send(Message::Ok);
//         cl2.send(Message::Ok);
//         //q3
//         let q1 = cl1.receive().join().unwrap();
//         let q2 = cl2.receive().join().unwrap();
//         assert_eq!(discriminant(&q1), discriminant(&q2));

//         cl2.send(Message::SongsAnswer(0));
//         let w2 = cl2.receive().join().unwrap();
        
//         cl1.send(Message::SongsAnswer(0));
//         let w1 = cl1.receive().join().unwrap();
//         assert_eq!(w1, Message::Winner(Message::bytes_to_name(P1_NAME)));
//         assert_eq!(w2, Message::Winner(Message::bytes_to_name(P2_NAME)));
//     });
//     let mut input = framework::MockInput::new();
//     input.mock_input("finish", 5);
//     let mut center = game_prepare::init(&mut input).unwrap();
//     let (g_choice, d_choice) = game_prepare::fetch_preferences(&mut center);
    
//     let context = Rc::new(FileContext::new(ROOT));
//     let track_chooser = TrackChooser::new(context.clone(), &g_choice, &d_choice);
//     let track_player = Player::new();
//     track_player.silent();

//     let mut game = game::GameLoop::new(center, track_player, track_chooser, 2000);
//     game.main_loop();
//     thread::sleep(Duration::from_secs(2));
// }