fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello"); // s2 in scope.
    let s3 = takes_and_gives_back(s2);
    // s2 no longer valid, ownership transferred to function.
} // s3 and s1 are dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string;
} // some_string is dropped

fn takes_and_gives_back(a_string: String) -> String {
    // astring in scope
    a_string; // returned, and moved.
}
