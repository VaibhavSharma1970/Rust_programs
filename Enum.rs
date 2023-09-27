// Enum is a custom data type which contains some definite values.
// Enumeration also referred to as enum.
// Use PascalCase.

#[derive(Debug)]
enum Genders{
    Male,
    Female,
}

// Enum with Struct

#[derive(Debug)]
struct People{
    name : String,
    gender : Genders,
}


fn main() {
    let x = Genders::Male;
    let y = Genders::Female;
    println!("{:?}", x);
    println!("{:?}", y);

    let z = People{
        name: String::from("John"),
        gender: Genders::Male,
        // gender2: Genders::Female,
    };
    println!("{:?}", z.name);
    println!("{:?}", z.gender);
    println!("{:?}", z);
}

