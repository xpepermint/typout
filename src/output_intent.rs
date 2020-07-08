/// The output is controlled through intents which are sent to the output
/// handler thread via a channel.
#[derive(Debug)]
pub enum OutputIntent {
    /// A request to append data to the output buffer.
    Write(String),

    /// A request to clear the buffered output data.
    Drain,

    /// A request to paint the buffered output data to the screen.
    Flush,

    /// A request to create or update a pinned message with.
    Pin(String, String),

    /// A request to remove pinned message.
    Unpin(String),

    /// A request to close the output thread.
    Exit,
}
