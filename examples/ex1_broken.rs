use logwatch_rs::parser::LogLine;

fn make_filter<'k>(keyword: &'k str) -> impl Fn(&LogLine<'_>) + 'k {
    move |line| {
        if line.message.contains(keyword) {
            println!("matched: {}", line.raw);
        }
    }
}

fn main() {
    let line_str = String::from("ERROR auth login failed");
    let line = LogLine::parse(&line_str).unwrap();
    let f = make_filter("failed");
    f(&line);
}
