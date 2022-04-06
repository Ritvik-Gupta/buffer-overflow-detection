#[derive(Debug)]
pub struct AnalyserError {
    pub line_number: usize,
    pub error: ErrorType,
}

pub struct ErrorBuilder(usize);

#[derive(Debug)]
pub enum ErrorType {
    Warning(String),
    Fatal {
        reason: String,
        buffer_size: usize,
        content_size: usize,
    },
}

impl AnalyserError {
    pub fn builder(line_number: usize) -> ErrorBuilder {
        ErrorBuilder(line_number)
    }
}

impl ErrorBuilder {
    pub fn warning(&self, reason: &str) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            error: ErrorType::Warning(reason.to_owned()),
        }
    }

    pub fn fatal(&self, reason: &str, buffer_size: usize, content_size: usize) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            error: ErrorType::Fatal {
                reason: reason.to_owned(),
                buffer_size,
                content_size,
            },
        }
    }
}
