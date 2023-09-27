use std::io;

fn main() {
    // Create mutable Strings to store the user inputs
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Read the first input from the user
    println!("Please enter the first integer:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    // Read the second input from the user
    println!("Please enter the second integer:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Parse the input strings into integers
    let number1: i32 = input1.trim().parse().expect("Please enter a valid integer");
    let number2: i32 = input2.trim().parse().expect("Please enter a valid integer");

    // Print the user inputs
    let c:i32 = number1+number2;
    // println!("You entered: {} and {}", number1, number2);
    println!("The Sum of Two numbers is {}", c);
}





// /* Get Float user input */

// use std::io;

// fn main() {
//     // Create a mutable String to store the user input
//     let mut input = String::new();

//     // Read a line of text from the user
//     println!("Please enter a floating-point number:");
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     // Parse the input string into a floating-point number
//     let number: f64 = input.trim().parse().expect("Please enter a valid number");

//     // Print the user input as a floating-point number
//     println!("You entered: {}", number);
// }
