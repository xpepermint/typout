use std::collections::HashMap;
use std::thread::{sleep, spawn};
use std::time::{Duration};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::{Output, OutputIntent, Spinner, SpinnerIntent};

/// Typout spawns a new thread for handling all output intents. When the
/// thread exits this variable is set to `false`. The flag is then used to
/// ensure all intents are processed before the Typout is dropped.
static OUTPUT_THREAD_ALIEVE: std::sync::atomic::AtomicBool = AtomicBool::new(false);

/// Similar to how the output intents are handled, the typout spawns a new
/// thread for each spinner. When a new thread is spawned the variable is
/// incremented by 1 and when the thread exits the variable is decremented by 1.
/// This flag is used to ensure all spinner threads complete before the object
/// is dropped.
static SPINNER_THREADS_COUNT: std::sync::atomic::AtomicUsize = AtomicUsize::new(0);

/// Typout represents a wrapper around the standard output of the current
/// process (stdout).
#[derive(Debug, Clone)]
pub struct Typout {
    /// The output is handled in a separate thread thus all messages are sent to
    /// the output through a channel.
    output: Sender<OutputIntent>,
    /// Spinner animations are handled in separate threads thus all messages are
    /// sent from the terminal to each thread through a channel.
    spinners: HashMap<String, Sender<SpinnerIntent>>,
}

impl Typout {
    /// Appends data to the output buffer.
    pub fn write(&mut self, data: &str) {
        self.output.send(OutputIntent::Write(data.to_string())).unwrap();
    }

    /// Clears buffered output data.
    pub fn drain(&self) {
        self.output.send(OutputIntent::Drain).unwrap();
    }

    /// Sends buffered output data to the standard output of the current
    /// process which displays the data in the terminal. The output buffer is
    /// cleared afterwards.
    pub fn flush(&self) {
        self.output.send(OutputIntent::Flush).unwrap();
    }

    /// Creates a new pinned message or updates an existing one. Pinned messages
    /// always stayed visible at the end of the output stream. An arbitrary
    /// number of pinned messages is allowed. Pins are uniquely identified by
    /// the provided `id` parameter.
    pub fn pin(&self, id: &str, data: &str) {
        self.output.send(OutputIntent::Pin(id.to_string(), data.to_string())).unwrap();
    }

    /// Creates a new animated pinned message or updates an existing one. It
    /// spawns the spinner animation thread for each new `id`. If the spinner
    /// with the provided `id` already exists, then only the message is updated.
    pub fn spin(&mut self, id: &str, data: &str) {
        if !self.spinners.contains_key(id) {
            let (spinner_tx, spinner_rx) = channel::<SpinnerIntent>();
            self.spinners.insert(id.to_string(), spinner_tx);
            start_spinner_thread(id, data, self.output.clone(), spinner_rx);
        }
        let spinner = self.spinners.get(id).unwrap();
        spinner.send(SpinnerIntent::Write(data.to_string())).unwrap();
    }

    /// Removes a pinned message with the provided `id`. This method works for
    /// all pinned messages including animated spinners.
    pub fn unpin(&mut self, id: &str) {
        if self.spinners.contains_key(id) {
            let spinner = self.spinners.get(id).unwrap();
            spinner.send(SpinnerIntent::Exit).unwrap();
            self.spinners.remove(id);
        } else {
            self.output.send(OutputIntent::Unpin(id.to_string())).unwrap();
        }
    }
}

/// When the terminal starts, the output handler is spawned in a new thread. The
/// messages and all other intents are sent the output through a channel.
impl Default for Typout {
    fn default() -> Self {
        let (output_tx, output_rx) = channel::<OutputIntent>();

        start_output_thread(output_rx);

        Self {
            output: output_tx,
            spinners: HashMap::new(),
        }
    }    
}

/// For the application to gracefully exits, we make sure that all the messages
/// are flushed and spawned threads exit before the object is dropped. 
impl Drop for Typout {
    fn drop(&mut self) {
        for (_, spinner) in self.spinners.iter() {
            spinner.send(SpinnerIntent::Exit).unwrap();
        }
        while SPINNER_THREADS_COUNT.load(Ordering::SeqCst) != 0 {
            sleep(Duration::from_millis(1)); 
        }

        self.output.send(OutputIntent::Exit).unwrap();
        while OUTPUT_THREAD_ALIEVE.load(Ordering::SeqCst) != false {
            sleep(Duration::from_millis(1)); 
        }
    }
}

/// Spawns the output handler in a new thread.
fn start_output_thread(receiver: Receiver<OutputIntent>) {
    OUTPUT_THREAD_ALIEVE.store(true, Ordering::SeqCst);

    spawn(move || { // start output handler in a new thread
        let mut output = Output::default();

        while let Ok(intent) = receiver.recv() {
            match intent {
                OutputIntent::Exit => break,
                _ => output.handle(intent),
            }
        }

        OUTPUT_THREAD_ALIEVE.store(false, Ordering::SeqCst);
    });    
}

/// Spawns a new spinner animation thread.
fn start_spinner_thread(id: &str, data: &str, output: Sender<OutputIntent>, receiver: Receiver<SpinnerIntent>) {
    SPINNER_THREADS_COUNT.fetch_add(1, Ordering::SeqCst);

    let id = id.to_string();
    let data = data.to_string();

    spawn(move || { // start spinner animation in a new thread
        let mut spinner = Spinner::default();
        let mut data = data.to_string();

        loop {
            if let Ok(intent) = receiver.try_recv() {
                match intent {
                    SpinnerIntent::Write(d) => data = d,
                    SpinnerIntent::Exit => break,
                }
            }
            let frame = spinner.take_frame();
            let message = format!("{} {}", frame, data);
            output.send(OutputIntent::Pin(id.clone(), message)).unwrap();
            sleep(Duration::from_millis(30));
        }
        output.send(OutputIntent::Unpin(id)).unwrap();
        SPINNER_THREADS_COUNT.fetch_sub(1, Ordering::SeqCst);
    });  
}
