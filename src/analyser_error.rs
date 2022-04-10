use colored::Colorize;

pub struct AnalyserError {
    pub line_number: usize,
    pub line: String,
    pub reason: String,
    pub error: ErrorType,
}

pub struct ErrorBuilder(usize, String);

pub enum ErrorType {
    Warning(),
    Fatal(Vec<String>),
}

impl AnalyserError {
    pub fn builder(line_number: usize, line: &str) -> ErrorBuilder {
        ErrorBuilder(line_number, line.trim().to_owned())
    }
}

impl ErrorBuilder {
    pub fn warning(self, reason: &str) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            line: self.1,
            reason: reason.to_owned(),
            error: ErrorType::Warning(),
        }
    }

    pub fn index_buffer_overflow(self, indexed_at: usize, buffer_size: usize) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            line: self.1,
            reason: "Buffer Index Overflow detected".to_owned(),
            error: ErrorType::Fatal(vec![
                format!("Buffer Size is {} chars", buffer_size),
                format!("Tried to Index at {}", indexed_at),
                format!("Indexing allowed for indices {} - {}", 0, buffer_size - 1),
            ]),
        }
    }

    pub fn copy_buffer_overflow(
        self,
        written_content: &String,
        buffer_size: usize,
    ) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            line: self.1,
            reason: "Buffer Overflow detected while Copying".to_owned(),
            error: ErrorType::Fatal(vec![
                format!("Buffer Size is {} chars", buffer_size),
                format!("Content being updated to = '{}'", written_content),
                format!(
                    "Tried to Write total {} extra chars",
                    written_content.len() - buffer_size
                ),
            ]),
        }
    }

    pub fn concat_buffer_overflow(
        self,
        previous_content: &String,
        additional_content: &String,
        buffer_size: usize,
    ) -> AnalyserError {
        AnalyserError {
            line_number: self.0,
            line: self.1,
            reason: "Buffer Overflow detected while Concatenating".to_owned(),
            error: ErrorType::Fatal(vec![
                format!("Buffer Size is {} chars", buffer_size),
                format!(
                    "Content being updated to = '{}' + '{}'",
                    previous_content, additional_content
                ),
                format!(
                    "Tried to Write total {} extra chars",
                    previous_content.len() + additional_content.len() - buffer_size
                ),
            ]),
        }
    }
}

impl std::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fatal(additional_info) => {
                for info in additional_info {
                    write!(f, "{} {}\n", "*".red().underline(), info.red().bold())?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Debug for AnalyserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_number = self.line_number.to_string();

        write!(
            f,
            "{}  {}\n",
            " ".repeat(line_number.len()),
            "|".bright_cyan().bold()
        )?;
        write!(
            f,
            "{}  {}\t{}\n",
            line_number.bright_cyan().bold(),
            "|".bright_cyan().bold(),
            self.line.white()
        )?;
        write!(
            f,
            "{}  {}\n",
            " ".repeat(line_number.len()),
            "|".bright_cyan().bold()
        )?;

        match self.error {
            ErrorType::Warning() => write!(f, "{}", "warning : ".yellow().italic())?,
            ErrorType::Fatal(_) => write!(f, "{}", "error : ".red().italic())?,
        }

        write!(f, "{}\n", self.reason.white().bold())?;
        write!(f, "\n{:?}\n", self.error)?;

        match self.error {
            ErrorType::Warning() => write!(f, "{}\n", "_".repeat(50).yellow())?,
            ErrorType::Fatal(_) => write!(f, "{}\n", "_".repeat(50).red())?,
        }

        Ok(())
    }
}
