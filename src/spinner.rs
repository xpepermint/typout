/// Spinner provides animation frames which are used by the spinner animation
/// thread to animate a pinned message.
#[derive(Debug)]
pub struct Spinner {
    /// A sequence of characters which are used for animation. 
    frames: Vec<String>,
}

impl Spinner {
    /// Sets animation frame sequence.
    pub fn set_frames(&mut self, frames: Vec<String>) {
        self.frames = frames;
    }

    /// Returns next animation frame.
    pub fn take_frame(&mut self) -> String {
        let frame = self.frames.get(0).unwrap_or(&String::new()).to_string();
        self.frames.drain(0..1);
        self.frames.push(frame.clone());
        frame
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].iter().map(|i| i.to_string()).collect(),
        }
    }
}
