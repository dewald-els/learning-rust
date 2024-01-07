fn main() {
    let s1 = String::from("hello");
    let (s2, length) = calculate_length(s1);

    // s1 no longer valid.
}

// Return multiple values as Tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length);
}
