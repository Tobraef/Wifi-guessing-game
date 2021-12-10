#[derive(Clone)]
pub enum SendOptions {
    Retry(u8, std::time::Duration), // retry count, timeout duration
    WaitIndefinetly
}