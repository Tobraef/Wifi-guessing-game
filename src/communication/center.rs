use super::{PlayerCommunication, SendOptions};
use crate::message::Message;

pub struct Center {
    clients: Vec<PlayerCommunication>,
    retry_options: SendOptions
}

impl Center {
    pub fn new(players: Vec<PlayerCommunication>) -> Center {
        Center {
            clients: players,
            retry_options: SendOptions::WaitIndefinetly
        }
    }

    pub fn players(&self) -> impl std::iter::Iterator<Item= &str> {
        self.clients.iter().map(|p| p.name())
    }

    pub fn setup_options(&mut self, opts: SendOptions) {
        self.retry_options = opts;
    }

    pub fn send_to_players(&mut self, msg: Message) -> bool {
        println!("Sending msg {:?}", msg);
        let _opts = self.retry_options.clone();
        for_each_client(&mut self.clients, |player| {
            player.send(&msg)
        })
    }

    pub fn listen_to_players<F: FnMut(Message, &str) -> bool + Send>(&mut self, f: &mut F) {
        for_each_client(&mut self.clients, |player| {
            loop {
                println!("Waiting for msg from {}", player.name());
                match player.receive(&None) {
                    Some(msg) => { 
                        println!("Received {:?}", &msg); 
                        if f(msg, player.name()) {
                            break true;
                        }
                    },
                    None => {}
                } 
            }
        });
    }
}

fn for_each_client<F: FnMut(&mut PlayerCommunication) -> bool + Send>(clients: &mut Vec<PlayerCommunication>, mut f: F) -> bool {
    clients.iter_mut().map(|mut c| crossbeam::scope(|s| s.spawn(|| {
        f(&mut c)
    }))).all(|s| s.join())
}