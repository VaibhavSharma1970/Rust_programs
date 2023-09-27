use std::io;
// fn main() {
//     let mut input = String::new();
//     println!("Enter something:");
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     println!("You Entered: {}", input);
// }

fn main() {
    let mut input = String::new();
    println!("Enter a integer Value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number:i32 = input.trim().parse().expect("Please enter a Valid Number");
    println!("You Enter: {}", number);
}