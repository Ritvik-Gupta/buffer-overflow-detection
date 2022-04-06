use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum ContentType {
    UNKNOWN,
    Value(String),
}

#[derive(Debug)]
pub struct BufferVariable {
    pub var_name: String,
    pub buffer_size: usize,
    pub content: ContentType,
}

static VARIABLE: &'static str = "[a-zA-Z][_a-zA-Z0-9]*";

lazy_static! {
    pub static ref ASSIGN_STATIC_BUFFER: Regex =
        Regex::new(&format!(
            r"char (?P<var>{VARIABLE})\[(?P<buffer_size>\d+)\];"
        )).unwrap();

    pub static ref ASSIGN_DYNAMIC_BUFFER: Regex =
        Regex::new(&format!(
            r"char\* (?P<var>{VARIABLE}) = new char\[(?P<buffer_size>\d+)\];"
        )).unwrap();

    pub static ref INDEXING_BUFFER: Regex =
        Regex::new(&format!(
            r"(?P<var>{VARIABLE})\[(?P<indexing_at>\d+)\] = '.';"
        )).unwrap();

    pub static ref STRCPY_BUFFER: Regex =
        Regex::new(&format!(
            r#"strcpy\((?P<var>{VARIABLE}), (?:"(?P<written_to_buffer>.*)"|(?P<written_from_var>{VARIABLE}))\);"#
        )).unwrap();

    pub static ref STRCAT_BUFFER: Regex =
        Regex::new(&format!(
            r#"strcat\((?P<var>{VARIABLE}), (?:"(?P<written_to_buffer>.*)"|(?P<written_from_var>{VARIABLE}))\);"#
        )).unwrap();

    pub static ref GETS_BUFFER: Regex =
        Regex::new(&format!(
            r"gets\((?P<var>{VARIABLE})\);"
        )).unwrap();
}
