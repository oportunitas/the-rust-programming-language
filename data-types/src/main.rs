use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; //panics at index > 4

    println!("The value of the element at index {index} is: {element}");
}

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let b = [3; 5];
// }

// fn main() {
//     let mut x: (i32, i32) = (1, 2);
//     println!("x is ({}, {})", x.0, x.1);

//     x.0 = 0;
//     x.1 += 5;
//     println!("z is ({}, {})", x.0, x.1);
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup;

//     println!("tup is ({x}, {y}, {z})");
//     println!("tup is ({}, {}, {})", tup.0, tup.1, tup.2);
// }

// fn main() {
//     let c = 'z';
//     println!("c is {c}");
//     let z: char = 'ℤ';
//     println!("z is {z}");
//     let heart_eyed_cat = '😻';
//     println!("heart_eyed_cat is {heart_eyed_cat}");
// }

// fn main() {
//     let t = true;
//     println!("t is {t}");

//     let f: bool = false; // with explicit type annotation
//     println!("f is {f}");
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("sum is {sum}");

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("difference is {difference}");

//     // multiplication
//     let product = 4 * 30;
//     println!("product is {product}");

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("quotient is {quotient}");

//     let truncated = -5 / 3; // Results in -1
//     println!("truncated is {truncated}");

//     // remainder
//     let remainder = 43 % 5;
//     println!("remainder is {remainder}");
// }

// fn main() {
//     let x = 2.0; // f64
//     println!("x is {x}");
//     let y: f32 = 3.0; // f32
//     println!("y is {y}");
// }
