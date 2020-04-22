use crate::streamlets::Streamlet;

pub struct Console {
    chunks: Vec<String>,
}

impl Console {

    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
        }
    }
}

impl Streamlet for Console {

    fn write(&mut self, txt: String) {
        self.chunks.push(txt);
    }

    fn flush(&mut self) -> String {
        let output = self.chunks.join("");
        self.chunks.clear();
        print!("{}", output);
        output
    }
}
