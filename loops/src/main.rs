fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// fn main() {
//     let a: [i32; 5] = [10, 20, 30, 40, 50];
//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// fn main() {
//     let a: [i32; 5] = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);
//         index += 1;
//     }
// }

// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let mut count: i32 = 0;
//     'counting_up: loop {
//         println!("count: {count}");
//         let mut remainder = 10;
//         loop {
//             println!("remainder: {remainder}");
//             if remainder == 9 {
//                 break
//             }
//             if count == 2 {
//                 break 'counting_up
//             }
//             remainder -= 1;
//         }
//         count += 1;
//     }
//     println!("end count: {count}");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2
//         }
//     };

//     println!("the result is {result}");
// }
