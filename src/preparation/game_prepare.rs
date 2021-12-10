use crate::communication::{Center, SendOptions, Input};
use crate::communication::client_fetcher;
use crate::Message;

use std::{time::Duration};

pub const DECADE_1_VAL: i32 = 4;
pub const DECADE_2_VAL: i32 = 3;

pub const GENRE_1_VAL: i32 = 4;
pub const GENRE_2_VAL: i32 = 3;
pub const GENRE_3_VAL: i32 = 2;

fn parse_retry(retry: &str) -> Option<SendOptions> {
    let mut splt = retry.split(' ');
    let times = splt
        .next()?
        .parse::<u8>()
        .ok()?;
    let duration = splt
        .next()?
        .parse::<u64>()
        .ok()?;
    Some(SendOptions::Retry(times, Duration::from_secs(duration)))
}

fn players_fetching_mode<I: Input>(host_input: &mut I) -> SendOptions {
    println!("Waiting for players, options: 'finish', 'retry |X| |Y|', 'noretry'");
    let mut opts = SendOptions::WaitIndefinetly;
    loop {
        let line = host_input.receive();
        match line {
            "finish" => {
                println!("Finished waiting for clients, beginning the game");
                break;
            },
            noretry if noretry == "noretry" => {
                println!("Set option: wait indefinetly untill message reaches its destination");
                opts = SendOptions::WaitIndefinetly;
            },
            retry if retry.starts_with("retry") => {
                if let Some(opt) = parse_retry(retry) {
                    opts = opt
                } else {
                    println!("received 'retry', but couldn't parse following args into numbers, expected format 'retry |X| |Y|'");
                }
            }
            _ => println!("Incorrect input, options: 'finish', 'retry X Y', 'noretry'")
        }
    }
    opts
}

pub fn init<'a, I: Input>(host_input: &mut I) -> Result<Center, &'static str> {
    let mut fetcher = client_fetcher::ClientFetcher::new().ok_or("Couldn't fetch clients")?;
    fetcher.start_fetching_players();
    let opts = players_fetching_mode(host_input);
    let players = fetcher.stop_fetching();
    let names: Vec<_> = players.iter().map(|p| p.name().to_owned()).collect();
    let mut center = Center::new(players);
    for name in names {
        center.send_to_players(Message::NewPlayer(Message::bytes_to_name(&name)));
    }
    center.send_to_players(Message::Ok);
    center.setup_options(opts);
    center.listen_to_players(&mut |m, _| match m {
        Message::Ok => true,
        _ => false,
    });
    Ok(center)
}