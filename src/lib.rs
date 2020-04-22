mod streamlets;
mod verbosity;

pub use crate::streamlets::Streamlet;
pub use crate::streamlets::console::Console;
pub use crate::streamlets::memory::Memory;
pub use crate::verbosity::Verbosity;

pub struct Typout {
    streamlet: Box<dyn Streamlet>,
    verbosity: Verbosity,
}

impl Typout {
    
    pub fn with_streamlet<'a>(streamlet: Box<dyn Streamlet>) -> Self {
        Self {
            streamlet,
            verbosity: Verbosity::Debug,
        }
    }

    pub fn console() -> Self {
        Self {
            streamlet: Box::new(Console::new()),
            verbosity: Verbosity::Debug,
        }
    }

    pub fn memory() -> Self {
        Self {
            streamlet: Box::new(Memory::new()),
            verbosity: Verbosity::Debug,
        }
    }

    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    pub fn write<S: Into<String>>(&mut self, txt: S) {
        self.streamlet.write(txt.into());
    }

    pub fn debug<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Debug) {
            self.streamlet.write(txt.into());
        }
    }

    pub fn info<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Info) {
            self.streamlet.write(txt.into());
        }
    }

    pub fn warn<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Warning) {
            self.streamlet.write(txt.into());
        }
    }

    pub fn error<S: Into<String>>(&mut self, txt: S) {
        if self.can_write(Verbosity::Error) {
            self.streamlet.write(txt.into());
        }
    }

    pub fn flush(&mut self) -> String {
        self.streamlet.flush()
    }

    fn can_write(&self, verbosity: Verbosity) -> bool {
        self.verbosity.clone() as usize >= verbosity as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flushes_buffered_data() {
        let mut out = Typout::memory();
        out.write("11");
        out.write("22");
        out.write("33");
        assert_eq!(out.flush(), "112233");
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
