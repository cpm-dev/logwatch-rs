use crate::parser::LogLine;

pub struct TransformedLine {
    pub level:   String,   // owned — we created this
    pub service: String,   // owned
    pub message: String,   // owned
}

pub fn transform(line: &LogLine<'_>) -> TransformedLine {
    TransformedLine {
        level:   line.level.to_lowercase(),
        service: line.service.trim().to_string(),
        message: line.message.to_string(),
    }
}
