#[derive(Debug)]
enum H{
    Name(String),
    Age(i32),
}
#[derive(Debug)]
enum H{
    Name(String),
    Age(i32),
}
fn main() {
    let set_name = H::Name(String::from("Vaibhava"));
    let set_age = H::Age(22);
    println!("{:?}", set_name);
    println!("{:?}", set_age);

}
