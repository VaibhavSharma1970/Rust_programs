fn main() {
    let mut x = 1;
    while x <= 100 {
        if x % 2 != 0 {
            println!("{}", x);
        }
        x += 1;
    }
}
