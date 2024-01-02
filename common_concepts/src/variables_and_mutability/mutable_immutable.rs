fn immutable_by_default() {
    let x = 5;
    println!("The value of x is {x}");
    x = 6; // Not allowed since x is not immutable.
    println!("The value of x is {x}");
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; // Allowed, since x is marked as mutable.
    println!("The value of x is {x}");
}

fn constant_variables() {
    const I_CANT_CHANGE: u32 = 42;
    // Can NOT use mut with const variables.
}
