fn main() {
    buffer_overflow::LexicalAnalyser::perform_on(
        std::fs::read_to_string("./files/copy_into_buffer/dynamic_sized.cpp").unwrap(),
    )
    .iter()
    .for_each(|error| println!("{:?}", error));
}
