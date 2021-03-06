fn main() {
    buffer_overflow::LexicalAnalyser::perform_on(
        std::fs::read_to_string("./files/copy_into_buffer/dynamic_sized.cpp").unwrap(),
        // Choose the file to perform Lexical Analysis on
    )
    .iter()
    .for_each(|error| println!("{:?}", error));
}
