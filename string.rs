// Creating a String
fn main() {
    // let name:&str = "Abhishek";
    // println!("{}", name);

    // Creating a Empty String Object
    let mut name_2 = String::new();
    name_2.push_str("Abhishek");
    // name_2.pop();
    println!("Name_2: {}", name_2);

    // Strore a Value in the String Object
    let name_3 = String::from("Vaibhav_Sharma");
    println!("Name_3: {}", name_3);
}

// if you want to store the value in empty string object
// Then change variable in mutable object

