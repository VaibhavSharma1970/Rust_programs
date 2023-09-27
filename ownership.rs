fn square_elements(input: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for num in input {
        let squared = num * num;
        result.push(squared);
    }

    result
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("Vec: {:?}", v);

    let sq = square_elements(v);
    println!("Sq: {:?}", sq);
    // let u = v;
    // println!("{:?}",v);  // error through

}