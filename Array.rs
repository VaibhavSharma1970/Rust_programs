// // fn main() {
// //     // declare an Array
// //     // let a = [1, 200,3,400,5];
// //     let a1:[i32;5] = [1, 200,3,400,5];
// //     println!("{:?}", a1);
// //     let
// // }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     println!("{:?}", arr);
//     println!("{:?}", arr.len());

//     let arr2 = [1.0, 2.0, 3.0, 4.0];
//     println!("{:?}", arr2);
//     println!("{:?}", arr2.len());

//     // data size
//     let arr3:[i32;5] = [1, 2, 3, 4, 5];
//     println!("{:?}", arr3);
//     println!("{:?}", arr3.len());
//     println!("{:?}", arr3[2]);

//     // default value
//     let arr4:[f32;20] = [100.0;20];
//     println!("{:?}", arr4);

//     // Update Element in an Array
//     let mut arr5:[i32;10] = [100;10];
//     arr5[5] = 12;
//     println!("{:?}", arr5);
// }

// fn main() {
//     let a:[i32;4]=[10,20,30,40];
//     println!("{:?}", a);
//     println!("Length of Array: {:?}", a.len());
//     // for i in a{
//     //     println!("{:?}", i);
//     // }
//     for i in 0..=3{
//         println!("Index is: {:?} and Value is: {:?}", i,a[i]);
//     }
// }

fn main() {
    let a = [[10,20],[100,200],[30,40],[40,50]];
    println!("{:?}", a);
}