use std::time::Duration;
use std::thread::sleep;

/// Spinner provides animation frames which are used by the spinner animation
/// thread to animate a pinned message.
#[derive(Debug)]
pub struct Spinner {
    /// A sequence of characters which are used for animation. 
    frames: Vec<String>,
    /// Animated speed is provided as FPS.
    speed: u32,
    /// Spinner can format pinned message based on a template string. Use
    /// `#{frame}` and  `#{message}` placeholders to define the output style.
    tpl: String,
}

impl Spinner {
    /// Sets animation frame sequence.
    pub fn set_frames(&mut self, frames: Vec<String>) {
        self.frames = frames;
    }

    /// Sets animation speed. This attribute is used by the `sleep()` method.
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
    
    /// Returns the next animation frame.
    pub fn next_frame(&mut self) -> String {
        let frame = self.frames.get(0).unwrap_or(&String::new()).to_string();
        self.frames.drain(0..1);
        self.frames.push(frame.clone());
        frame
    }

    /// Returns a formated message of the next animation frame.
    pub fn next_message(&mut self, data: &str) -> String {
        self.tpl.clone()
            .replace("#{frame}", &self.next_frame())
            .replace("#{message}", data)
    }

    /// Blocks the thread for the number of ms based on the speed variable.
    pub fn sleep(&mut self) {
        sleep(Duration::from_millis(self.speed.into()));
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].iter().map(|i| i.to_string()).collect(),
            speed: 30,
            tpl: String::from("#{frame} #{message}"),
        }
    }
}
