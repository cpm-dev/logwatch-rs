use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use logwatch_rs::parser::LogLine;
use logwatch_rs::registry::ProcessorRegistry;

fn main() {
    let raw_lines = vec![
        String::from("ERROR auth failed"),
        String::from("WARN  auth slow"),
        String::from("ERROR auth failed"),
    ];

    let counts = Arc::new(Mutex::new(HashMap::new()));
    let counts_clone = Arc::clone(&counts);
    let mut registry = ProcessorRegistry::new();

    registry.register(move |line| {
        let mut map = match counts_clone.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("mutex poisoned, recovering...");
                poisoned.into_inner()
            }
        };
        *map.entry(line.level.to_string()).or_insert(0) += 1;
    });

    for raw in &raw_lines {
        if let Some(line) = LogLine::parse(raw) {
            registry.run(&line);
        }
    }

    println!("{:?}", counts.lock().unwrap());
}
