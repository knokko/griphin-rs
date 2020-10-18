use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel {

    Minimal,
    Low,
    Basic,
    High,
    All
}

pub fn log(writer: &mut Option<&mut dyn Write>, source: &str, message: &str) {
    if writer.is_some() {
        writeln!(writer.as_mut().unwrap(), "[Griphin] [{}]: {}", source, message).unwrap();
    } else {
        println!("[Griphin] [{}]: {}", source, message);
    }
}