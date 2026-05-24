use crate::parser::LogLine;

/// Returns a closure that filters lines by level.
/// YOUR TASK: add the correct lifetime annotations.
pub fn level_filter<'a>(level: &'a str) -> impl Fn(&LogLine<'_>) + 'a {
    move |line| {
        if line.level == level {
            println!("[FILTER PASS] {}", line.raw);
        }
    }
}
