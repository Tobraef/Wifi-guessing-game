use std::{ net, io::{Read, Write}, time::Duration };
use crate::message::Message;

const BUFFER_SIZE: usize = 1024;

pub struct PlayerCommunication {
    socket: net::TcpStream,
    name: String,
    buffer: [u8;BUFFER_SIZE],
}

fn new_buffer() -> [u8;BUFFER_SIZE] {
    [0;BUFFER_SIZE]
}

impl Clone for PlayerCommunication {
    fn clone(&self) -> PlayerCommunication {
        PlayerCommunication {
            socket: self.socket.try_clone().unwrap(),
            name: self.name.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl PlayerCommunication {
    pub fn new(socket: &mut net::TcpStream) -> Option<PlayerCommunication> {
        socket.set_nonblocking(false).unwrap();
        let mut buffer = new_buffer();
        match socket.read(&mut buffer) {
            Ok(size) => {
                let name_len = if let Some(indx) = buffer[1..].iter().position(|b| *b == b'\0') { indx + 1 } else { size };
                let name = String::from_utf8_lossy(&buffer[1..name_len]);
                println!("New player! Name: {}", name);
                Some(PlayerCommunication {
                    socket: socket.try_clone().unwrap(),
                    name: name.into_owned(),
                    buffer: buffer,
                })
            },
            Err(e) => { 
                println!("Couldn't receive name from the player, because: {}", e);
                None
            }
        }        
    }

    pub fn receive(&mut self, timeout: &Option<Duration>) -> Option<Message> {
        self.socket.set_read_timeout(*timeout).unwrap();
        match self.socket.read(&mut self.buffer) {
            Ok(_) => {
                let message = Message::from_bytes(&self.buffer);
                Some(message.clone())
            },
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                None
            },
            Err(e) => { 
                panic!("Failed to receive a message, reason: {}", e)
            }
        }
    }

    pub fn send(&mut self, msg: &Message) -> bool {
        let encoded = msg.as_bytes();
        self.buffer[0..encoded.len()].copy_from_slice(&encoded);
        match self.socket.write(&self.buffer) {
            Err(e) => {
                println!("Failed to send a message, reason: {}", e);
                false
            }
            _ => true
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}