fn main() {
    let a = 20;
    println!("a = {}", a);
    {
        // Shadowing allow you to re-declare a variable in a same scope Using the same name
        let a = 100;
        let b = 1000;
        println!("a = {}", a);
        println!("b = {}", b);
    }
    println!("a = {}", a);
}
