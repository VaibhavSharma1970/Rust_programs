fn factorial(x:i32) -> i32 {
    if x>1{
        x*factorial(x-1)
    }
    else{
        x
    }
}

fn main() {
    println!("Factorial is {}" factorial(5));
}