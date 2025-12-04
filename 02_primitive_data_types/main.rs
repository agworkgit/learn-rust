// Primitive data types - int, float, bool, char

fn main() {
    // Integer
    // Rust has signed (+ and -) and unsigned integer (only +) types of different sizes.
    // --- i8, i16, i32, i64, i128: Signed integers.
    // --- u8, u16, u32, u64, u128: Unsigned integers.
    // Generally, the smaller the number the smaller the amount of numbers and range.

    let x: i32 = -42;
    let y: u64 = 100; // only positive values
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Differences --- i32 (32bits), i64 (64bits)
    // Range : i32 can vary from -2^31 to +2^31 while i64 can vary from -2^63 to +2^63
    // Examples:
    // --- i32 - 2477483647
    // --- i64 - 9223372036854775807

    let e: i32 = 2147483647;
    let i: i64 = 922337203684775807;
    println!("Max value of i32: {}", e);
    println!("Max value of i64: {}", i);

    // Always add semicolons at the end of your statements otherwise rustc will complain

    // --- u32 and u64 have a larger range than i32 and i64 but not by much, in any case the compiler will make suggestions

    // Floats [Floating Point Types]
    // --- f32, f64

    let pi: f64 = 3.14;
    println!("Value of PI: {}", pi);

    // Boolean
    // --- true, false

    let is_snowing: bool = true;
    println!("Is it snowing?: {}", is_snowing);

    // Character Type - char (single unicode value)

    let letter: char = 'a';
    println!("The first alphabet letter is: {}", letter);
}
