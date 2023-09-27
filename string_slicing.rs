// SYNTAX of STRING SLICING
// let sliced_value = &data_structure[start_index..end_index]

// fn main() {
//     let name:&str = "Vaibhav";
//     println!("Length of Name:{}", name.len());
//     let sli = &name[2..6];
//     println!("{}", sli);
// }

fn main(){
    let name:&str = "Rust Tutorial";
    println!("Name:{}", name);
    println!("&name: {}", &name[0..]);
    println!("Length of Name: {}", name.len());
}
