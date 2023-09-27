// Using Vec::new()
// let mut my_vector: Vec<i32> = Vec::new();

// Using the vec! macro
// let mut another_vector = vec![1, 2, 3, 4, 5];

fn main() {
    // let mut vec = Vec::<i32>::new();
    let mut vec:Vec<i32>=Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    println!("{:?}", vec)
}
