use crate::streamlets::Streamlet;

pub struct Memory {
    chunks: Vec<String>,
    output: String,
}

impl Memory {

    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            output: String::new(),
        }
    }

    pub fn output(&self) -> &String {
        &self.output
    }
}

impl Streamlet for Memory {

    fn write(&mut self, txt: String) {
        self.chunks.push(txt);
    }

    fn flush(&mut self) -> String {
        let output = self.chunks.join("");
        self.chunks.clear();
        output
    }
}
