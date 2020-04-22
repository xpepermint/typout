pub mod console;
pub mod memory;

pub trait Streamlet {
    fn write(&mut self, txt: String);
    fn flush(&mut self) -> String;
}
