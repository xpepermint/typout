mod streamlets;
mod verbosity;

use ansistr::{clean_str};
pub use crate::streamlets::Streamlet;
pub use crate::streamlets::console::Console;
pub use crate::streamlets::memory::Memory;
pub use crate::verbosity::Verbosity;

pub struct Typout {
    streamlet: Box<dyn Streamlet>,
    verbosity: Verbosity,
    stripansi: bool,
}

impl Typout {
    
    pub fn with_streamlet<'a>(streamlet: Box<dyn Streamlet>) -> Self {
        Self {
            streamlet,
            verbosity: Verbosity::Debug,
            stripansi: false,
        }
    }

    pub fn console() -> Self {
        Self {
            streamlet: Box::new(Console::new()),
            verbosity: Verbosity::Debug,
            stripansi: false,
        }
    }

    pub fn memory() -> Self {
        Self {
            streamlet: Box::new(Memory::new()),
            verbosity: Verbosity::Debug,
            stripansi: false,
        }
    }

    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    pub fn stripansi(&self) -> &bool {
        &self.stripansi
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    pub fn set_stripansi(&mut self, strip: bool) {
        self.stripansi = strip;
    }

    pub fn write<S: Into<String>>(&mut self, txt: S) {
        self.streamlet.write(self.normalize(txt.into()));
    }

    pub fn debug<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Debug) {
            self.streamlet.write(self.normalize(txt.into()));
        }
    }

    pub fn info<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Info) {
            self.streamlet.write(self.normalize(txt.into()));
        }
    }

    pub fn warn<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Warning) {
            self.streamlet.write(self.normalize(txt.into()));
        }
    }

    pub fn error<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Error) {
            self.streamlet.write(self.normalize(txt.into()));
        }
    }

    pub fn flush(&mut self) -> String {
        self.streamlet.flush()
    }

    pub fn drain(&mut self) {
        self.streamlet.drain();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flushes_buffer() {
        let mut out = Typout::memory();
        out.write("11");
        out.write("22");
        out.write("33");
        assert_eq!(out.flush(), "112233");
    }

    #[test]
    fn drains_buffer() {
        let mut out = Typout::memory();
        out.write("11");
        out.write("22");
        out.drain();
        assert_eq!(out.flush(), "");
    }

    #[test]
    fn strips_ansi() {
        let mut out = Typout::memory();
        out.set_stripansi(false);
        out.write("1\x1B[1m2");
        assert_eq!(out.flush(), "1\x1B[1m2");
        out.set_stripansi(true);
        out.write("1\x1B[1m2");
        assert_eq!(out.flush(), "12");
    }

    #[test]
    fn honors_debug_verbosity() {
        let mut out = Typout::memory();
        out.set_verbosity(Verbosity::Debug);
        out.error("1");
        out.warn("2");
        out.info("3");
        out.debug("4");
        assert_eq!(out.flush(), "1234");
    }

    #[test]
    fn honors_info_verbosity() {
        let mut out = Typout::memory();
        out.set_verbosity(Verbosity::Info);
        out.error("1");
        out.warn("2");
        out.info("3");
        out.debug("4");
        assert_eq!(out.flush(), "123");
    }

    #[test]
    fn honors_warning_verbosity() {
        let mut out = Typout::memory();
        out.set_verbosity(Verbosity::Warning);
        out.error("1");
        out.warn("2");
        out.info("3");
        out.debug("4");
        assert_eq!(out.flush(), "12");
    }

    #[test]
    fn honors_error_verbosity() {
        let mut out = Typout::memory();
        out.set_verbosity(Verbosity::Error);
        out.error("1");
        out.warn("2");
        out.info("3");
        out.debug("4");
        assert_eq!(out.flush(), "1");
    }

    #[test]
    fn honors_none_verbosity() {
        let mut out = Typout::memory();
        out.set_verbosity(Verbosity::None);
        out.error("1");
        out.warn("2");
        out.info("3");
        out.debug("4");
        assert_eq!(out.flush(), "");
    }
}
