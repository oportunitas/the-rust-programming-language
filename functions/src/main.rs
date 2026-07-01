fn plus_one(x: i32) -> i32 {x + 1}

fn main() {
    let x = plus_one(5);
    println!("the value of x is {x}");
}

// fn five() -> i32 {5}

// fn main() {
//     let x = five();
//     println!("The value of x is {x}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn f(x: i32) {
//     println!("{x}");
// }

// fn main() {
//     f(0);
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// parameter: variables in a function definition
// argument: concrete valule passed in when a function is called

// fn another_function(num: i32) {
//     println!("number is {num}");
// }

// fn main() {
//     another_function(5);
// }

// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }
