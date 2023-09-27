fn add_with_return(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result; // Explicit return statement
}

fn add_without_return(x: i32, y: i32) -> i32 {
    x + y // Implicit return
}

fn main() {
    let sum1 = add_with_return(10, 20);
    let sum2 = add_without_return(5, 15);

    println!("Sum with return: {}", sum1);
    println!("Sum without return: {}", sum2);
}






