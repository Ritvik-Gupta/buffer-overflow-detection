fn main() {
    println!(
        "{:#?}",
        buffer_overflow::LexicalAnalyser::perform_on(
            std::fs::read_to_string("./files/copy_into_buffer/dynamic_sized.cpp")
                .unwrap()
                .split('\n'),
        )
    );
}
