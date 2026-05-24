use logwatch_rs::parser::LogLine;

fn parse_and_process<'a>(raw: &'a str) -> Option<LogLine<'a>> {
    LogLine::parse(raw)
}

fn main() {
    let raw = String::from("WARN gateway timeout"); // caller owns this
    let line = parse_and_process(&raw).unwrap();
    println!("{}", line.raw);
}
