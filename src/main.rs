use logwatch_rs::parser::LogLine;
use logwatch_rs::processors::transform::transform;

fn main() {
    let raw_lines = vec![
        String::from("ERROR auth login failed"),
        String::from("WARN  gateway timeout"),
        String::from("INFO  api request received"),
    ];

    for raw in &raw_lines {
        if let Some(line) = LogLine::parse(raw) {
            let transformed = transform(&line);
            println!(
                "level: {} | service: {} | message: {}",
                transformed.level,
                transformed.service,
                transformed.message
            );
        }
    }
}
