use std::{sync::{Arc, Mutex}};
use std::collections::VecDeque;

const TIMEOUT: &str = "3000";
const REPEAT: &str = "2";

fn append_all_possibles<'a>(curr_segments: String) -> impl Iterator<Item = String> {
    (0..255).map(move |i| format!("{}.{}", curr_segments, i))
}

fn ping_ip(ip: String) -> Option<String> {
    println!("Pinging {}", ip);
    if String::from_utf8_lossy(&std::process::Command::new("ping")
        .args(&[&ip, "-n", REPEAT, "-w", TIMEOUT])
        .output().expect("Couldn't run ping")
        .stdout)
            .contains("Destination host unreachable") {
                None
            } else {
                Some(ip)
            }
}

fn parallel_execution<'a, R, F, I>(iter: I) -> Vec<R>
    where 
        R: 'a + Send + Sync + Clone + std::fmt::Debug,
        F: 'static + Sync + Send + FnOnce() -> Option<R>,
        I: Iterator<Item = F> {
    let results: Arc<Mutex<Vec<R>>> = Arc::new(Mutex::new(Vec::new()));
    let functions: Arc<Mutex<VecDeque<F>>> = Arc::new(Mutex::new(iter.collect()));
    crossbeam::scope(|scope|  {
        let handles = (0..256).map(|_| scope.spawn(|| {
            loop {
                let mut f_guard = functions.lock().unwrap();
                if let Some(f) = f_guard.pop_front() {
                    drop(f_guard);
                    if let Some(f_result) = f() {
                        results.lock().unwrap().push(f_result);
                    }
                } else {
                    break;
                }
            }
        }))
        .collect::<Vec<_>>();
        handles.into_iter().for_each(|s| s.join());
    });
    return Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

pub fn fetch_local_ips() -> Vec<String> {
    let board_cast_ip = super::client_fetcher::fetch_local_ip()
        .expect("Couldn't fetch local ip")
        .split('.')
        .take(3)
        .collect::<Vec<_>>()
        .join(".");
    let functions = append_all_possibles(board_cast_ip)
        .map(|ip| { move || ping_ip(ip) });
    parallel_execution(functions)
}