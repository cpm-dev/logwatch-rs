///ProcessorRegistry  ← stores closures
///
use crate::parser::LogLine;

/// A callback that processes one log line.
/// The 'a on the closure input must match the 'a on LogLine.
pub type Processor<'a> = Box<dyn Fn(&LogLine<'a>) + 'a>;

pub struct ProcessorRegistry<'a> {
    processors: Vec<Processor<'a>>,
}

impl<'a> ProcessorRegistry<'a> {
    pub fn new() -> Self {
        Self { processors: vec![] }
    }

    /// Register a callback. The closure borrows for 'a.
    pub fn register(&mut self, f: impl Fn(&LogLine<'a>) + 'a) {
        self.processors.push(Box::new(f));
    }

    /// Run all processors against one log line.
    pub fn run(&self, line: &LogLine<'a>) {
        for processor in &self.processors {
            processor(line);
        }
    }
}
