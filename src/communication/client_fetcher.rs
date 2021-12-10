use std::{ ops::Deref, thread, net, sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } }, str::FromStr };
use super::PlayerCommunication;

pub const SERVER_PORT: u16 = 7513;

pub struct ClientFetcher {
    server_socket: Arc<Mutex<net::TcpListener>>,
    clients: Arc<Mutex<Vec<PlayerCommunication>>>,
    stop_token: Arc<AtomicBool>
}

pub fn fetch_local_ip() -> Result<String, &'static str> {
    String::from_utf8_lossy(&std::process::Command::new("ipconfig")
        .output()
        .or(Err("no output from ipconfig"))?
        .stdout)
        .split_terminator('\n')
        .find(|line| line.contains("IPv4 Address")).ok_or("No ipv4 found in ipconfig")?
        .split_ascii_whitespace()
        .last()
        .ok_or("error finding adress in line")
        .map(|s| String::from(s.trim()))
}

impl ClientFetcher {
    pub fn new() -> Option<ClientFetcher> {
        match fetch_local_ip() {
            Ok(ip) => {
                println!("Found local ip {}", ip);
                let ip_adresses: Vec<net::SocketAddr> = 
                    (SERVER_PORT..SERVER_PORT + 10).into_iter().map(|p| net::SocketAddr::new(net::IpAddr::from_str(&ip).unwrap(), p)).collect();
                let socket = net::TcpListener::bind(&ip_adresses[..]);
                if socket.is_err() {
                    println!("Couldn't bind server socket");
                    return None;
                } else {
                    return Some(ClientFetcher {
                        server_socket: Arc::new(Mutex::new(socket.unwrap())),
                        clients: Arc::new(Mutex::new(Vec::new())),
                        stop_token: Arc::new(AtomicBool::new(false))
                    });
                }
            },
            Err(e) => {
                println!("Error starting Center: {}", e);
                None
            }
        }
    }

    pub fn stop_fetching(&mut self) -> Vec<PlayerCommunication> {
        self.stop_token.store(true, Ordering::SeqCst);
        self.clients.lock().unwrap().deref().clone()
    }

    pub fn start_fetching_players(&mut self) {
        let stop_sync = self.stop_token.clone();
        let clients = self.clients.clone();
        let server_socket = self.server_socket.clone();
        thread::spawn(move || {
            println!("Awaiting clients..");
            let server_socket = server_socket.lock().unwrap();
            server_socket.set_nonblocking(true).expect("Couldn't set non-blocking on server socket");
            for stream in server_socket.incoming() {
                match stream {
                    Ok(mut c) => {
                        println!("Connected client, awaiting name...");
                        if let Some(client) = PlayerCommunication::new(&mut c) {
                            let mut guard = clients.lock().unwrap();
                            guard.push(client);
                        }
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if stop_sync.load(Ordering::SeqCst) {
                            break;
                        } else {
                            thread::sleep(std::time::Duration::from_secs(1));
                        }
                    }
                    Err(e) => {
                        println!("Couldn't connect client, reason: {}", e);
                    }
                }
            }
            clients
        });
    }
}