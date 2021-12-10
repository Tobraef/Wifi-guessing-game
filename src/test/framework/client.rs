use std::{ time::Duration, thread, sync::{Arc, Mutex}, net, str::FromStr, io::{Write, Read} };
use crate::communication::client_fetcher::{ SERVER_PORT, fetch_local_ip };
use crate::message::Message;

pub struct Client {
    socket: Arc<Mutex<net::TcpStream>>
}

impl Client {
    pub fn new() -> Client {
        let ip = fetch_local_ip().unwrap();
        let ip_adresses: Vec<net::SocketAddr> = 
            (SERVER_PORT..SERVER_PORT + 10).into_iter().map(|p| net::SocketAddr::new(net::IpAddr::from_str(&ip).unwrap(), p)).collect();
        let socket = net::TcpStream::connect(&ip_adresses[..]).unwrap();
        socket.set_nonblocking(true).unwrap();
        Client {
            socket: Arc::new(Mutex::new(socket))
        }
    }

    pub fn send(&mut self, msg: Message) -> thread::JoinHandle<()> {
        let socket = self.socket.clone();
        thread::spawn(move || {
            let mut socket = socket.lock().unwrap();
            loop {
                if let Err(e) = socket.write(msg.as_bytes()) {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_secs(1));
                    } else {
                        panic!("Error sending from client fake: {}", e)
                    }
                } else {
                    println!("TEST: Sent {:?} successfully", msg);
                    break;
                }
            }
        })
    }

    #[allow(dead_code)]
    pub fn receive(&mut self) -> thread::JoinHandle<Message> {
        let socket = self.socket.clone();
        thread::spawn(move || {
            let mut socket = socket.lock().unwrap();
            let mut buffer: [u8; std::mem::size_of::<Message>()] = [0;std::mem::size_of::<Message>()];
            loop {
                if let Err(e) = socket.read(&mut buffer) {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_secs(1));
                    } else {
                        panic!("Error receiving from client fake: {}", e)
                    }
                } else {
                    println!("TEST: Received {:?}", Message::from_bytes(&buffer));
                    break;
                }
            }
            Message::from_bytes(&buffer).clone()
        })
    }
}

