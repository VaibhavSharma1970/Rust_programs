fn main() {
    let a = 9;
    match a {
        1 => println!("its one"),
        2 => println!("its two"),
        3 | 4 | 5 => println!("its three, four, and five"),
        6..=10 => println!("it is between 6 to 10"),
        _ => println!("Its something"),
    }
}
