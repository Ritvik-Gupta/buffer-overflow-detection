use analyser_error::AnalyserError;
use std::collections::HashMap;
use variables::{
    BufferVariable,
    ContentType::{Value, UNKNOWN},
    ASSIGN_DYNAMIC_BUFFER, ASSIGN_STATIC_BUFFER, INDEXING_BUFFER, STD_CIN_BUFFER, STRCAT_BUFFER,
    STRCPY_BUFFER,
};

pub mod analyser_error;
pub mod variables;

pub struct LexicalAnalyser {
    buffers: HashMap<String, BufferVariable>,
}

impl LexicalAnalyser {
    pub fn perform_on<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<AnalyserError> {
        let mut analyser = LexicalAnalyser {
            buffers: HashMap::new(),
        };
        let mut errors = Vec::new();

        for (err_builder, line) in lines
            .enumerate()
            .map(|(idx, line)| (AnalyserError::builder(idx + 1), line))
        {
            match (
                ASSIGN_STATIC_BUFFER.captures(line),
                ASSIGN_DYNAMIC_BUFFER.captures(line),
            ) {
                (Some(cap), _) | (_, Some(cap)) => {
                    let var_name = cap["var"].to_owned();
                    analyser.buffers.insert(
                        var_name.clone(),
                        BufferVariable {
                            var_name,
                            buffer_size: cap["buffer_size"].parse().unwrap(),
                            content: UNKNOWN,
                        },
                    );
                    continue;
                }
                _ => {}
            }

            if let Some(cap) = INDEXING_BUFFER.captures(line) {
                let var_name = cap["var"].to_owned();

                let ref buffer = analyser.buffers[&var_name];
                let indexing_at = cap["indexing_at"].parse().unwrap();
                if indexing_at >= buffer.buffer_size {
                    errors.push(err_builder.fatal(
                        "Buffer Index Overflow Detected",
                        buffer.buffer_size,
                        indexing_at,
                    ));
                }

                println!("{:?}", cap);
            } else if let Some(cap) = STRCPY_BUFFER.captures(line) {
                let var_name = cap["var"].to_owned();

                let written_content = if let Some(to_buffer) = cap.name("written_to_buffer") {
                    Value(to_buffer.as_str().to_owned())
                } else if let Some(from_var) = cap.name("written_from_var") {
                    analyser.buffers[&from_var.as_str().to_owned()]
                        .content
                        .clone()
                } else {
                    unreachable!();
                };

                let ref buffer = analyser.buffers[&var_name];
                match &written_content {
                    UNKNOWN => {
                        errors.push(err_builder.warning("Trying to add content set at runtime"));
                    }
                    Value(written_content) => {
                        if buffer.buffer_size < written_content.len() {
                            errors.push(err_builder.fatal(
                                "Buffer Overflow Detected",
                                buffer.buffer_size,
                                written_content.len(),
                            ));
                        }
                        analyser.buffers.get_mut(&var_name).unwrap().content =
                            Value(written_content.clone());
                    }
                }

                println!("{:?}", cap);
            } else if let Some(cap) = STRCAT_BUFFER.captures(line) {
                let var_name = cap["var"].to_owned();

                let written_content = if let Some(to_buffer) = cap.name("written_to_buffer") {
                    Value(to_buffer.as_str().to_owned())
                } else if let Some(from_var) = cap.name("written_from_var") {
                    analyser.buffers[&from_var.as_str().to_owned()]
                        .content
                        .clone()
                } else {
                    unreachable!();
                };

                let ref buffer = analyser.buffers[&var_name];
                match (&buffer.content, &written_content) {
                    (_, UNKNOWN) => {
                        errors.push(err_builder.warning("Trying to append unkown value to buffer"));
                    }
                    (UNKNOWN, _) => {
                        errors.push(err_builder.warning("Buffer stores unknown value"));
                    }
                    (Value(buffer_content), Value(written_content)) => {
                        let content_size = buffer_content.len() + written_content.len();
                        if buffer.buffer_size < content_size {
                            errors.push(err_builder.fatal(
                                "Buffer Overflow when concatenating to Buffer content",
                                buffer.buffer_size,
                                content_size,
                            ));
                        }
                    }
                }

                println!("{:?}", cap);
            } else if let Some(cap) = STD_CIN_BUFFER.captures(line) {
                let var_name = cap["var"].to_owned();

                errors.push(err_builder.warning("Cannot determine the content of Buffer"));
                analyser.buffers.get_mut(&var_name).unwrap().content = UNKNOWN;

                println!("{:?}", cap);
            }
        }

        analyser
            .buffers
            .iter()
            .for_each(|(_, buffer)| println!("{:?}", buffer));

        errors
    }
}
