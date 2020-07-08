/// The spinner animation is controlled through intents which are sent to each
/// spinner thread via a channel.
#[derive(Debug)]
pub enum SpinnerIntent {
    /// A request to set spinner message.
    Write(String),

    /// A request to close the spinner thread.
    Exit,
}
