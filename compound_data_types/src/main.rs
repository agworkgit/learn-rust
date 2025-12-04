// Compound Data Types
// --- array, tuples, slices, and slice string (strings)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Num Array: {:?}", numbers);

    /*
    let mix = [1, 2, "Apple", true];
    // ------------> ^^^^^^^ expected integer, found `&str`
    println!("Mix Array: {:?}", mix);
    */

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st Index: {:?}", fruits[0]);
    println!("Fruits Array 2nd Index: {:?}", fruits[1]);
    println!("Fruits Array 3rd Index: {:?}", fruits[2]);

    // Tuples
    // --- Contain heterogeneous elements of fixed size
    // --- We can mix types with tuples

    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices
    // --- Dynamically sized view into a contigeous squence of elements
    // --- Contigeous sequence means elements are adjacent to one another
    // --- e.g. [1, 2, 3, 4, 5]

    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    // Strings VS String Slices (&str)
    // --- Strings [growable, mutable, owned string type] --- allocated on the Heap

    let mut stone_cold: String = String::from("Hell, "); // --- 'mut' converts from immutable to mutable type
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // --- String Slice (&str) [immutable, reference to some string literal, no copy, or data ownership]
    // --- The Stack remembers the size and known number of bytes, has very quick access
    // --- The Heap can store dynamic types, The Stack can only hold immutable types

    let string: String = String::from("Hello, Rust!"); // String data type
    let slice: &str = &string[0..5];
    // slice referes to the value inside the string variable, [0..5] definition of number of characters we want
    println!("Slice Value: {}", slice);
}

/*
    fn print() {
        println!("SLICE: {}", slice);
        // ------------------ ^^^^^ not found in this scope
    }
*/
