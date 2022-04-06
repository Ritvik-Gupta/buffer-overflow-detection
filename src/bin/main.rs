use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DataType {
    CHAR,
    INT,
}

impl DataType {
    fn from(data_type: &str) -> Self {
        match data_type {
            "char" => Self::CHAR,
            "int" => Self::INT,
            _ => unimplemented!("No implementation of type"),
        }
    }
}

#[derive(Debug)]
struct BufferVar {
    data_type: DataType,
    var: String,
    buffer_size: usize,
}

lazy_static! {
    static ref DECLARE_BUFFER_PATTERN: Regex =
        Regex::new(r"(?P<type>char|int) (?P<var>.+)\[(?P<buffer_size>\d+)\]").unwrap();
    static ref STRCPY_BUFFER_PATTERN: Regex =
        Regex::new(r#"strcpy\((?P<var>.+), "(?P<written_to_buffer>.+)"\)"#).unwrap();
}

fn main() {
    let ref file_content =
        std::fs::read_to_string("./files/copy_into_buffer/static_sized.c").unwrap();

    let mut data_type_buffers: HashMap<DataType, HashMap<String, BufferVar>> = HashMap::new();
    data_type_buffers.insert(DataType::CHAR, HashMap::new());
    data_type_buffers.insert(DataType::INT, HashMap::new());

    for caps in DECLARE_BUFFER_PATTERN.captures_iter(file_content) {
        data_type_buffers
            .get_mut(&DataType::from(&caps["type"]))
            .unwrap()
            .insert(
                caps["var"].to_owned(),
                BufferVar {
                    data_type: DataType::from(&caps["type"]),
                    var: caps["var"].to_owned(),
                    buffer_size: caps["buffer_size"].parse().unwrap(),
                },
            );
    }

    data_type_buffers
        .iter()
        .for_each(|(_, buffer_vars)| println!("{:?}", buffer_vars));

    for (&data_type, buffer_vars) in &data_type_buffers {
        match data_type {
            DataType::CHAR => {
                for caps in STRCPY_BUFFER_PATTERN.captures_iter(file_content) {
                    println!("{}, {}", &caps["var"], &caps["written_to_buffer"]);
                    assert!(
                        buffer_vars[&caps["var"].to_owned()].buffer_size
                            >= caps["written_to_buffer"].len(),
                        "Buffer Overflow Erorr"
                    );
                }
            }
            DataType::INT => {}
        }
    }
}
