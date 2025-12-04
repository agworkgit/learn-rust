// Functions

// Entry Point
// --- Any new functions are expected by Rust to be inside the main() function
// --- Any function and variable is written in snake case
// +++ snake case: hello_world (Rust convention)
// +++ kebab case: hello-world

fn main() {
    hello_world();
    tell_height(185);
    human_id("Alex", 31, 185.3);

    let _x = {
        let price = 5;
        let qty = 10;
        price * qty // Evaluations don't need semicolons
    };

    println!("Result is: {}", _x);

    // add(4, 6);
    let y = add(4, 6);
    println!("Value of y is: {}", y);
    println!("Value from function 'add' is: {}.", add(4, 6));

    // Calling BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi); // :.2 display to 2 decimal points
}

// Hoisting
// --- Functions will be hoisted to the top, making them available to call from anywhere

fn hello_world() {
    println!("Hello, Rust 🦀!");
}

// --- You can insert parameters

fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

// --- More than one parameter can be specified

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {}, years old, and my height is {} cm.",
        name, age, height
    );
}

// --- Functions returning values

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// --- Expression : Anything that returns a value.

// +++ Expression Examples:
// 5
// true or false
// add(3, 4)
// if conditions {value1} else {value 2}
// code blocks ({code})

// --- Statement : Anything that does not return a value.
// --- Almost all statements in Rust end with semicolon ;

// +++ Statement Examples:
// Variable declarations --- let x = 5;
// Function definitions --- fn foo() {}
// Control flow statements --- if condition { code } else { code }, while condition { code }, etc.

// Final example on functions
// BMI = height(kg) / height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
