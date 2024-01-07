fn main() {
    let x = 5;
    let y = x;
    // This will NOT move, since the data is fixed and known at compile time.
    println!("x = {}, y = {}", x, y);
}

/**
 * Types with the Copy trait
 * All the integer types, such as u32.
 * The Boolean type, bool, with values true and false.
 * All the floating-point types, such as f64.
 * The character type, char.
 * Tuples, if they only contain types that also implement Copy. 
 * For example, (i32, i32) implements Copy, but (i32, String) does not.
 */