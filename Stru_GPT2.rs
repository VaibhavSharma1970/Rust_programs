// Structure in Rust. 
struct Employee {
    name: String,
    age: i32,
    profile: String,
    active: bool
}

// Making Function
fn struct_func(x: &Employee) {
    println!("{}", x.name);
    println!("{}", x.age);
    println!("{}", x.profile);
    println!("{}", x.active);
}

fn main() {
    let employee_1 = Employee {
        name: String::from("Vaibhav"),
        age: 22,
        profile: String::from("Software Engineer"),
        active: true
    };
    println!("Name: {}", employee_1.name);
    println!("Age: {}", employee_1.age);
    println!("Profile: {}", employee_1.profile);
    println!("Active_Status: {}", employee_1.active);

    struct_func(&employee_1); // Pass a reference to the struct to struct_func
}
