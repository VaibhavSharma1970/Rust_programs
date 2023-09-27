fn cust(s:&Vec<i32>){
    println!("{:?}", s);
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", v);

    cust(&v);
    println!("{:?}", v);

    let a = 21;
    let b = a;
    println!("{}",b);
    println!("{}",a);
}