fn main() {
    // Simple, can be added to the stack, Sizes are known.
    let x = 5;
    let y = x;

    // s1 and s2 share the same reference in memory (Similar to JS)
    let s1 = String::from("hello");
    // s2 does NOT copy the data on the heap, simply the pointer in stack referencing s1.
    let s2 = s1;
    // s1 is now considered invalid.
    //This is to prevent the double free error when
    // freeing memory that shares the same reference
    // This is called "move" in Rust.
}
