// Compound Data Types
// --- arrays, tuples, slices, and strings (slice string)

fn main() {
    // Arrays - Fixed size collection of elements of homogeneous type (same type)
    // --- We then need to specify the type and size in square brackets

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    // let mix = [1, 2, "apple", true];
    // println!("Mix Array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {:?}", fruits[0]);
    println!("Fruits Array 2nd element: {:?}", fruits[1]);
    println!("Fruits Array 3rd element: {:?}", fruits[2]);

    // Tuples - Tuples contain heterogeneous collections of elements of fixed size

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false); // .to_string() converts from string slice to string data type
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // Slices - Dynamically sized view into a contigeous sequence of elements
    // --- "contigeous" = data or memory that is stored in an unbroken, sequential block
    // [1, 2, 3, 4, 5] - "contigeous = uninterrupted, meaning all elements are adjacent to one another"

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    // String VS String Slices (&str)
    // --- Differences: Strings are expandable (you can increase them or decrease them, hence they are 'mutable') and 'owned' string types
    // --- 'owned' - meaning they are not 'borrowed'
    // * Look into heap vs stack memory

    // String - stored in heap memory

    let mut stone_cold: String = String::from("Hell, "); // 'mut' converts to mutable data type
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // String Slice (&str) - stored in stack memory, a reference to a portion of a string that's stored somewhere in your code, 'immutable'
    // --- Used to reference string literals, or substrings, or string objects without needing to copy or own the data
    // --- Stack is quicker, Heap is slower, Stack data types are 'immutable', Heap data types are 'dynamic, or mutable'

    let string: String = String::from("Hello, World!");
    // --- '&string' refers to the value of the variable above, it's a copy of it
    let slice: &str = &string[0..5]; // [0..5] get only portions of a string
    println!("Slice Value: {}", slice);
}

/*
    fn print() {
        println!("SLICE: {}", slice);
        // ------------------ ^^^^^ not found in this scope
}
*/
