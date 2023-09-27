// Method in Structure
// Methods are like functions. They are a logical group of programming instructions. Methods are declared with the fn keyword. The scope of a method is within the structure block.

// Methods are declared outside the structure block. The impl keyword is used to define a method within the context of a structure. The first parameter of a method will be always self, which represents the calling instance of the structure. Methods operate on the data members of a structure.

// To invoke a method, we need to first instantiate the structure. The method can be called using the structure's instance.


// struct Rectangle{
//     length:i32,
//     width:i32
// }

// impl Rectangle{
//     fn area(&self)->i32{
//         (self.length)*(self.width)
//     }
// }

// fn main() {
//     let rec_1 = Rectangle{
//         length: 100,
//         width: 200
//     };
//     println!("{}", rec_1.area());
// }

struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 { 
        self.length * self.width
    } 
    fn display(&self) {
        println!("Length: {}", self.length);
        println!("Width: {}", self.width);
    }
}

fn main() {
    let rec_1 = Rectangle {
        length: 100,
        width: 200,
    };

    println!("Area: {}", rec_1.area());
    rec_1.display();
}

