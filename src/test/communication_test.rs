#[allow(unused_imports)]
use crate::communication::{ send_options::SendOptions, center::Center, client_fetcher::ClientFetcher };
#[allow(unused_imports)]
use crate::{message::{Message, NAME_LEN}, error_type::ErrorType};
use super::framework::client::Client;
#[allow(unused_imports)]
use std::{ time::Duration, thread, sync::{Arc, atomic::{AtomicBool, Ordering}} };

#[allow(dead_code)]
fn estabilish_connection(num_of_clients: u32) -> (Center, Vec<Client>) {
    thread::sleep(Duration::from_secs(1));
    let mut init = ClientFetcher::new().unwrap();
    init.start_fetching_players();
    
    let mut clients = Vec::new();
    for i in 0..num_of_clients {
        let mut client = Client::new();
        let player_name = format!("{}-{}", "Przemek", i);
        let mut player_name_bytes: [u8;NAME_LEN] = [0;NAME_LEN];
        player_name_bytes[0..player_name.len()].copy_from_slice(player_name.as_bytes());
        client.send(Message::NewPlayer(player_name_bytes)).join().unwrap();
        clients.push(client);
    }

    thread::sleep(Duration::from_secs(1));
    let players = init.stop_fetching();
    
    let mut przemek = 0;
    for player in &players {
        assert_eq!(format!("{}-{}", "Przemek", przemek), player.name());
        przemek += 1;
    }
    (Center::new(players), clients)
}

#[test]
fn communication_connection_estabilishment() {
    estabilish_connection(1);
}

#[test]
fn communication_send_options_wait_indefinetly() {
    let (mut center, mut client) = estabilish_connection(1);
    let client = client.first_mut().unwrap();
    let options = SendOptions::WaitIndefinetly;
    center.setup_options(options);
    
    client.receive();
    thread::sleep(Duration::from_secs(1));
    assert_eq!(center.send_to_players(Message::Ok), true);
    
    client.send(Message::Ok);
    thread::sleep(Duration::from_secs(1));
    center.listen_to_players(&mut |msg, _name| {assert_eq!(msg, Message::Ok); true});
}

#[allow(dead_code)]
fn receive_from_all(center: &mut Center, clients: &mut Vec<Client>, msg: Message) {
    let received_msgs: Vec<thread::JoinHandle<Message>> = clients.into_iter().map(|c| c.receive()).collect();
    assert_eq!(center.send_to_players(msg.clone()), true);
    for message in received_msgs.into_iter().map(|m| m.join()) {
        assert_eq!(message.unwrap(), msg);
    }
}