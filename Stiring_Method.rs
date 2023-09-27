fn  main() {
    let mut name = "My name is Vaibhav";
    println!("{}", name);
    let l = name.len();
    println!("{}", l);
    let _rep = name.replace("is","are");
    println!("{}", _rep);
    println!("{}", _rep.len());
    let mut name2 =  "My name is Shivam".to_string();
    // name.push('s');
    name2.push_str(" boy");
    println!("{}", name2);

}

// fn main() {
//     let mut company = "Tutorial".to_string();
//     company.push('s');
//     println!("{}", company);
// }
