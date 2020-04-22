pub mod streamlets;

use crate::streamlets::Streamlet;
use crate::streamlets::console::Console;
use crate::streamlets::memory::Memory;

pub struct Typout {
    streamlet: Box<dyn Streamlet>,
}

impl Typout {
    
    pub fn with_streamlet<'a>(streamlet: Box<dyn Streamlet>) -> Self {
        Self {
            streamlet,
        }
    }

    pub fn console() -> Self {
        Self {
            streamlet: Box::new(Console::new()),
        }
    }

    pub fn memory() -> Self {
        Self {
            streamlet: Box::new(Memory::new()),
        }
    }

    pub fn write<S: Into<String>>(&mut self, txt: S) {
        self.streamlet.write(txt.into());
    }

    pub fn flush(&mut self) -> String {
        self.streamlet.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_buffer() {
        let mut out = Typout::memory();
        out.write("11");
        out.write("22");
        out.write("33");
        assert_eq!(out.flush(), "112233");
    }
}
