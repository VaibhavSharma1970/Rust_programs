// Structure in Rust.
struct Employee {
    name: String,
    age: i32,
    profile: String,
    active: bool,
}

fn main() {
    // Create a mutable instance of the Employee struct.
    let mut employee_1 = Employee {
        name: String::from("Vaibhav"),
        age: 22,
        profile: String::from("Software Engineer"),
        active: true,
    };

    // Access and print the fields of the employee_1 struct.
    println!("Name: {}", employee_1.name);
    println!("Age: {}", employee_1.age);
    println!("Profile: {}", employee_1.profile);
    println!("Active: {}", employee_1.active);

    // You can modify the fields if needed.
    employee_1.age = 23;
    println!("Updated Age: {}", employee_1.age);

    // You can also create additional Employee instances.
    let employee_2 = Employee {
        name: String::from("Alice"),
        age: 30,
        profile: String::from("Product Manager"),
        active: false,
    };
    
    // Access and print the fields of employee_2.
    println!("Name: {}", employee_2.name);
    println!("Age: {}", employee_2.age);
    println!("Profile: {}", employee_2.profile);
    println!("Active: {}", employee_2.active);
}
