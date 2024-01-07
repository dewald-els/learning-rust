fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // This will COPY the Heap data, not just the pointer. This does NOT move the variable.
    println!("s1 = {}, s2 = {}", s1, s2);
}
