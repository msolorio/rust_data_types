// Numbers
// fn main() {
//     // let guess: u32 = "42".parse().expect("Not a number");

//     // let my_num: u8 = 256;
//     // let my_num: u8 = 255;

//     // println!("{}", guess);
//     // println!("{}", my_num);

//     let x = 2.0;
//     let y: f32 = 3.0;

//     println!("{}", x);
//     println!("{}", y);
// }

// Numeric Operations
// fn main() {
//     let sum = 5 + 10;

//     let difference = 95.5 - 4.3;

//     let product = 4 * 30;

//     let quotient = 56.7 / 32.2;

//     let remainder = 43 % 5;

//     println!(
//         "{} {} {} {} {}",
//         sum, difference, product, quotient, remainder
//     );
// }

// Boolean type
// fn main() {
//     let t = true;

//     let f: bool = false;

//     println!("{} {}", t, f);
// }

// Character Type
// fn main() {
//     let c = 'z';
//     let z = 'Z';
//     let fav_emoji = '👟';

//     println!("{} {} {}", c, z, fav_emoji);
// }

//////////////////////////////////////////////////
// Compound Types

// Tuple type
// Tuples have a fixed length - once they are declared, they cannot shrink or grow.
// fn main() {
//     let tup: (i32, f64, u8) = (-500, 4.3, 255);

//     let (x, y, z) = tup;
//     println!("{} {} {}", x, y, z);

//     let first_val = tup.0;
//     let second_val = tup.1;
//     println!("{} {}", first_val, second_val);
// }

// Array Type
// Every element in the array must have the same type
// Arrays have a fixed length
// Arrays are useful when you want your data allocated on the stack rather than on the heap
// fn main() {
//     let my_array: [&str; 5] = ["charles", "Zane", "Lane", "Blain", "Raymond"];

//     let my_nums: [i32; 5] = [42, 11, 67, 45, 10];

//     let a: [i32; 5] = [0; 5];

//     // my_array[1] = "Jay"; // my_array is not declared as mutable

//     println!("{}", a[0]);
//     println!("{}", my_nums[0]);

//     println!("{}", my_array[0]);
// }

// Invalid array access

use std::io;

fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is {}",
        index, element
    );
}
