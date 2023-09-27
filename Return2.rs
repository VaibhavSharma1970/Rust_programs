use std::io;

fn sum(num1:i32, num2:i32) -> i32 {
    let result = num1 + num2;
    return result;
}

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    println!("Enter a First Value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("Enter a Second Value: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input2");

    let number1:i32 = input.trim().parse().expect("Please enter a Valid Number");
    let number2:i32 = input2.trim().parse().expect("Please enter a Valid Number");

    println!("You Enter: {}", number1);
    println!("You Enter: {}", number2);
    // let sum: i32 = number1 + number2;
    // println!("The Sum is:{}", sum);

    // Call the function and store the result
    let sum_1 = sum(number1, number2);
    println!("The Sum of {} and {} is: {}",number1,number2, sum_1);
}


