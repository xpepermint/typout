#[derive(Debug, Clone, PartialEq)]
pub struct SpinnerState {
    pub chars: Vec<String>,
    pub message: String,
    pub speed: u64,
    pub started: bool,
}
