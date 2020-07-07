use ansistr::clean_str;
use crate::{Spinner, Verbosity};

#[derive(Debug)]
pub struct Console {
    spinner: Spinner,
    verbosity: Verbosity,
    stripansi: bool,
    speed: u64,
    chars: Vec<String>,
    message: String,
}

impl Console {
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    pub fn stripansi(&self) -> &bool {
        &self.stripansi
    }

    pub fn speed(&self) -> &u64 {
        &self.speed
    }

    pub fn chars(&self) -> &Vec<String> {
        &self.chars
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    pub fn set_stripansi(&mut self, strip: bool) {
        self.stripansi = strip;
    }

    pub fn set_spinner_speed(&mut self, speed: u64) {
        self.speed = speed;
        self.spinner.set_speed(speed);
    }

    pub fn set_spinner_chars<C, T>(&mut self, chars: C)
        where
        C: IntoIterator<Item = T>,
        T: Into<String>,    
    {
        self.chars = chars.into_iter().map(Into::into).collect();
        self.spinner.set_chars(&self.chars);
    }

    pub fn write<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        self.message.push_str(&txt.into());
        self
    }

    pub fn debug<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        if self.can_write(Verbosity::Debug) {
            self.write(self.normalize(txt.into()));
        }
        self
    }

    pub fn info<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        if self.can_write(Verbosity::Info) {
            self.write(self.normalize(txt.into()));
        }
        self
    }

    pub fn warn<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        if self.can_write(Verbosity::Warning) {
            self.write(self.normalize(txt.into()));
        }
        self
    }

    pub fn error<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        if self.can_write(Verbosity::Error) {
            self.write(self.normalize(txt.into()));
        }
        self
    }

    pub fn flush(&mut self) -> String {
        self.spinner.stop();

        let mut output = self.message.to_string();
        if self.stripansi {
            output = clean_str(output);
        }
        self.message.clear();

        if let Some(term) = term::stdout().as_mut() {
            write!(term, "{}", output).unwrap();
        }
        output
    }

    pub fn drain(&mut self) -> &mut Self {
        self.spinner.stop();
        self.message.clear();
        self
    }

    pub fn spin<S>(&mut self, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        self.spinner.start();
        self.spinner.set_message(txt.into());
        self
    }

    pub fn progress<S>(&mut self, total: u64, size: u64, txt: S) -> &mut Self
        where
        S: Into<String>,
    {
        let perc = if total == 0 {
            0
        } else {
            size / total * 100
        };
        self.spinner.start();
        self.spinner.set_message(format!("{:.1}% {}", perc, txt.into()));
        self
    }

    fn can_write(&self, verbosity: Verbosity) -> bool {
        self.verbosity.clone() as usize >= verbosity as usize
    }

    fn normalize(&self, txt: String) -> String {
        match self.stripansi {
            true => clean_str(txt),
            false => txt,
        }
    }
}

impl Default for Console {
    fn default() -> Self {
        let chars = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].iter().map(|i| i.to_string()).collect();
        let speed = 30;
        Self {
            spinner: Spinner::new(speed, &chars, ""),
            verbosity: Verbosity::Debug,
            stripansi: false,
            speed,
            chars,
            message: String::new(),
        }
    }
}
