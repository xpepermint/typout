use std::collections::HashMap;
use std::io::{stdout, Write, Stdout};
use crossterm::style::{Print};
use crossterm::execute;
use crossterm::cursor::{MoveToColumn, MoveUp, position};
use crossterm::terminal::{Clear, ClearType};
use crate::{OutputIntent};

/// Output represents a wrapper around the standard output of the current
/// process and is responsible for painting massages to the terminal.
pub struct Output {
    /// Line messages are stored in this internal buffer. The buffered data is
    /// painted to the screen on flush, which also clears the buffer. 
    buffer: String,

    /// One or more messages can be pinned to the end of the output stream.
    /// These messages are inserted to the hash map with a unique id where each
    /// entry represents one pinned message.
    pins: HashMap<String, String>,

    /// All messages are streamed to the standard output of the current process.
    stdout: Stdout,

    /// The handler needs to be aware of the last permanent character position.
    /// This are the (left, bottom) coordinates in the terminal where the last
    /// character, which will not be cleared from the screen anymore, is
    /// located. The position is dynamically updated when data are painted on
    /// the screen.
    position: (u16, u16),
}

impl Output {
    /// Handles the received intent.
    pub fn handle(&mut self, intent: OutputIntent) {
        match intent {
            OutputIntent::Write(data) => self.write(&data),
            OutputIntent::Drain => self.drain(),
            OutputIntent::Pin(id, data) => self.pin(&id, &data),
            OutputIntent::Unpin(id) => self.unpin(&id),
            OutputIntent::Flush => self.flush(),
            OutputIntent::Exit => {}, // handled by the Term struct
        }
    }

    /// Writes the received data to the internal message buffer. The buffer is
    /// sent to the output stream when calling the `flush()` method.
    fn write(&mut self, data: &str) {
        self.buffer.push_str(&data);
    }

    /// Removes data from the message buffer.
    fn drain(&mut self) {
        self.buffer.clear();
    }

    /// Sends the buffered message to the standard output of the current process
    /// which displays the message in the terminal. The message buffer is
    /// cleared afterwards.
    fn flush(&mut self) {
        self.clear_pins();

        execute!(self.stdout, Print(&self.buffer)).unwrap();
        self.buffer.clear();

        let (left, _) = position().unwrap();
        self.position = (left, 0);

        self.paint_pins();
    }

    /// Creates a new pinned message or updates an existing one. Pinned messages
    /// always stayed visible at the end of the output stream. An arbitrary
    /// number of pinned messages is allowed. Pins are uniquely identified by
    /// the received `id` parameter.
    fn pin(&mut self, id: &str, data: &str) {
        self.pins.insert(id.to_string(), data.to_string());
        self.clear_pins();
        self.paint_pins();
    }

    /// Removes a pinned message with the provided `id`.
    fn unpin(&mut self, id: &str) {
        self.pins.remove(id);
        self.clear_pins();
        self.paint_pins();
    }

    /// Displays all pinned messages in the terminal.
    fn paint_pins(&mut self) {
        let stdout = &mut self.stdout;
        for (_, value) in self.pins.iter() {
            self.position.1 += value.matches("\n").count() as u16;
            execute!(stdout, Print(value)).unwrap();
        }
    }

    /// Clears all pinned messages from the terminal.
    fn clear_pins(&mut self) {
        if self.position.1 > 0 {
            execute!(self.stdout, MoveUp(self.position.1)).unwrap();
        }
        execute!(self.stdout, MoveToColumn(self.position.0)).unwrap();
        self.position.1 = 0;

        execute!(self.stdout, Clear(ClearType::UntilNewLine)).unwrap();
        execute!(self.stdout, Clear(ClearType::FromCursorDown)).unwrap();
    }
}

impl Default for Output {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            pins: HashMap::new(),
            stdout: stdout(),
            position: (0, 0),
        }
    }
}
