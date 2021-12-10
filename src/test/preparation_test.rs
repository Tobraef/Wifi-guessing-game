#[allow(unused_imports)]
use crate::preparation::game_prepare;
#[allow(unused_imports)]
use crate::message::Message;
#[allow(unused_imports)]
use crate::db::{Category};
#[allow(unused_imports)]
use super::framework;
#[allow(unused_imports)]
use std::{thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};

#[test]
fn preparation_init_test() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        let cl1_n = "Przemek";
        let cl2_n = "Lukasz";
        let mut cl1 = framework::Client::new();
        let mut cl2 = framework::Client::new();

        cl1.send(Message::NewPlayer(Message::bytes_to_name(cl1_n)));
        cl2.send(Message::NewPlayer(Message::bytes_to_name(cl2_n)));
    });
    let mut input = framework::MockInput::new();
    input.mock_input("finish", 5);
    let r = game_prepare::init(&mut input);
    assert!(r.is_ok() && r.unwrap().players().count() == 2)
}