pub trait Input {
    fn receive(&mut self) -> &str;
}

pub struct ManualInput(String);

impl Input for ManualInput {
    fn receive(&mut self) -> &str {
        std::io::stdin().read_line(&mut self.0).unwrap();
        &self.0.trim()
    }
}

impl Default for ManualInput {
    fn default() -> Self {
        ManualInput { 0: String::new() }
    }
}