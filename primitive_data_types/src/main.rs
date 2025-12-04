// Primitive data types
// --- int, float, bool, char

// Integer
// Rust has signed (+ and -) and usigned integer (only +) types of different sizes.
// --- Signed integers: i8, i16, i32, i64, i128
// --- Unsigned integers (only positive values) : u8, u16, u32, u64, u128

fn main() {
    let x: i32 = -42;
    let y: u32 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Difference: i32 (32bits) and i64 (64bits)
    // --- Range:
    // - i32 (-2^31 to +2^31) - 2147483647
    // - i64 (-2^63 to +2^63) - 9223372036854775807
    // - u32 (+2^32)
    // - u64 (+2^64)

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Max val of i32: {}", e);
    println!("Max val of i64: {}", i);

    // --- Note:
    // - if the number won't fit into the data type opt for larger options i128, etc...

    // Floats (nums with fractional parts) [Float Point Types]
    // --- f32, f64

    let pi: f64 = 3.14;
    println!("Value of PI: {}", pi);

    // Boolean Values
    // --- true, false

    let is_snowing: bool = true;
    println!("Is it snowing?: {}", is_snowing);

    // Character Type - unicode char
    let letter: char = 'a';
    println!("First alphabet letter is: {}", letter);
}
