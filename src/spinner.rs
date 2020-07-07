use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::{RwLock, Arc};
use crate::SpinnerState;

#[derive(Debug)]
pub struct Spinner {
    state: Arc<RwLock<SpinnerState>>,
    handle: Option<JoinHandle<()>>,
}

impl Spinner {
    pub fn new<C, T, M>(speed: u64, chars: C, message: M) -> Self
        where
        C: IntoIterator<Item = T>,
        T: Into<String>,    
        M: Into<String>,    
    {
        Self {
            state: Arc::new(RwLock::new(SpinnerState {
                chars: chars.into_iter().map(Into::into).collect(),
                message: message.into(),
                speed,
                started: false,
            })),
            handle: None,
        }
    }

    pub fn set_speed(&mut self, speed: u64) {
        let mut state = self.state.write().unwrap();
        state.speed = speed;
    }

    pub fn set_message<T>(&mut self, text: T)
        where
        T: Into<String>,
    {
        let mut state = self.state.write().unwrap();
        state.message = text.into();
    }

    pub fn set_chars<C, T>(&mut self, chars: C)
        where
        C: IntoIterator<Item = T>,
        T: Into<String>,    
    {
        let mut state = self.state.write().unwrap();
        state.chars = chars.into_iter().map(Into::into).collect();
    }

    pub fn start(&mut self) {
        let mut state = self.state.write().unwrap();
        if state.started {
            return;
        } else {
            state.started = true;
        }

		self.handle = Some(start_thread(self.state.clone()));
    }

    pub fn stop(&mut self) {
        let mut state = self.state.write().unwrap();

        if let Some(handle) = &self.handle {
            drop(handle);
            self.handle = None;

            if let Some(term) = term::stdout().as_mut() {
                term.carriage_return().unwrap();
                term.delete_line().unwrap();
            }
        }

        state.started = false;
        state.message = String::new();
    }
}

fn take_chr(chars: &mut Vec<String>) -> String {
    let chr = chars[0].to_string();
    chars.drain(0..1);
    chars.push(chr.clone());
    chr
}

fn start_thread(state: Arc<RwLock<SpinnerState>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut term = term::stdout();

        loop {
            let mut state = state.write().unwrap();
            if !state.started {
                break;
            }

            let chr = take_chr(&mut state.chars);
            let speed = state.speed;
            let message = &state.message;

            if let Some(term) = term.as_mut() {
                term.flush().unwrap();
                term.carriage_return().unwrap();
                term.delete_line().unwrap();
                write!(term, "{} {}", chr, message).unwrap();
            }

            drop(state); // release lock
            thread::sleep(Duration::from_millis(speed));
        }
    })
}
