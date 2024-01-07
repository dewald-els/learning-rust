fn main() {
    let s = String::from("hello"); // Heap, since the size can be unknown.

    s.push_str(", world"); // append a literal to a String.

    println!("{}", s); // Print "hello, world".
}
