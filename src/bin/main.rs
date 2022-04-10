fn main() {
    use regex::Regex;

    println!(
        "{:?}",
        Regex::new(r"(\d)+").unwrap().captures("abc19373fp3294")
    );
}
