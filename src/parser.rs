///LogLine<'a>  ← lifetime 'a lives her///
/// LogLine borrows directly from the raw input string.
/// 'a is the lifetime of that raw string — no allocation.
#[derive(Debug)]
pub struct LogLine<'a> {
    pub level:   &'a str,   // e.g. "ERROR"
    pub service: &'a str,   // e.g. "auth"
    pub message: &'a str,   // rest of the line
    pub raw:     &'a str,   // the whole original line
}

impl<'a> LogLine<'a> {
    /// Parse a raw &str into a LogLine that borrows from it.
    /// The caller must ensure the raw string outlives the LogLine.

	pub fn parse(raw: &'a str) -> Option<LogLine<'a>> {
    		let mut parts = raw.split_whitespace();
    		let level   = parts.next()?;
    		let service = parts.next()?;
    		// collect the rest as message
		let message = parts.collect::<Vec<&str>>().join(" ");

    		Some(LogLine { level, service, message: &raw[raw.find(&message)?..], raw })		
    		
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_space() {
        let raw = "ERROR auth login failed";
        let line = LogLine::parse(raw).unwrap();
        assert_eq!(line.level, "ERROR");
        assert_eq!(line.service, "auth");
        assert_eq!(line.message, "login failed");
    }

    #[test]
    fn test_parse_double_space() {
        let raw = "WARN  gateway timeout";
        let line = LogLine::parse(raw).unwrap();
        assert_eq!(line.level, "WARN");
        assert_eq!(line.service, "gateway");
        assert_eq!(line.message, "timeout");
    }

    #[test]
    fn test_parse_returns_none_on_invalid() {
        let raw = "ERRORONLY";
        assert!(LogLine::parse(raw).is_none());
    }
}
