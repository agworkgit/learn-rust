// Ownership, Borrowing and References in Rust

// Ownership

// --- C, C++ -> Memory Management Control Issue.
// --- Garbage Collector solved this issue, but created a new issue -> Show Performance: [Stopping/Resuming the program].
// --- Ownership introduced in Rust to solve memeory safety issues and high performance at the same time.

// What is ownership?
// --- Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
// --- 1. Each value in Rust has a variable that's its owner.
// --- 2. There can only be one owner at at time.
// --- 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    ex1();
    ex2();
    ex3();
}

// Example 1: Each value in Rust has a variable that's its owner.
fn ex1() {
    let s1 = String::from("RUST"); // original owner
    let len = calculate_length(&s1); // passed reference to s1 string
    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Example 2: There can only be one owner at a time.

fn ex2() {
    let s2 = String::from("RUSTLANG");
    let s3 = s2;
    println!("{}", s3); // error!!! : s2 no longer owns the string, assigned to s3
}

// Example 3: Owner goes out of scope, value will be dropped

fn ex3() {
    let s4 = String::from("CRAB");
    let size = calculate_size(&s4);
    println!("Length of '{}' is {}.", s4, size);
}
// s4 goes out of scope and its value will be dropped

/*
    fn print_lost(s: &string) {
        println!("{}", &s4); // cannot find value 's4' in this scope
    }
*/

fn calculate_size(s: &String) -> usize {
    s.len()
}
