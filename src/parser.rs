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
    		let mut iter = raw.splitn(3, ' ');
    		let level   = iter.next()?.trim();
    		let service = iter.next()?.trim();
    		let message = iter.next()?.trim();

    		if level.is_empty() || service.is_empty() || message.is_empty() {
        		return None;
    		}

    		Some(LogLine { level, service, message, raw })
	}	
}
