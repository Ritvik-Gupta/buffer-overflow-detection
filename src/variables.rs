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

lazy_static! {
    pub static ref ASSIGN_STATIC_BUFFER: Regex =
        Regex::new(r"char (?P<var>[a-zA-Z0-9]+)\[(?P<buffer_size>\d+)\];").unwrap();
    pub static ref ASSIGN_DYNAMIC_BUFFER: Regex =
        Regex::new(r"char\* (?P<var>[a-zA-Z0-9]+) = new char\[(?P<buffer_size>\d+)\];").unwrap();
    pub static ref INDEXING_BUFFER: Regex =
        Regex::new(r"(?P<var>[a-zA-Z0-9]+)\[(?P<indexing_at>\d+)\] = '\w';").unwrap();
    pub static ref STRCPY_BUFFER: Regex = Regex::new(
        r#"strcpy\((?P<var>[a-zA-Z0-9]+), (?:"(?P<written_to_buffer>[a-zA-Z0-9]+)"|(?P<written_from_var>[a-zA-Z0-9]+))\);"#
    )
    .unwrap();
    pub static ref STRCAT_BUFFER: Regex = Regex::new(
        r#"strcat\((?P<var>[a-zA-Z0-9]+), (?:"(?P<written_to_buffer>[a-zA-Z0-9]+)"|(?P<written_from_var>[a-zA-Z0-9]+))\);"#
    )
    .unwrap();
    pub static ref STD_CIN_BUFFER: Regex = Regex::new(r"std::cin >> (?P<var>[a-zA-Z0-9]+);").unwrap();
}
