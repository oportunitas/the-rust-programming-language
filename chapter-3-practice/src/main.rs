use std::io;

fn fahrenheit_to_celcius(f: f64) -> f64 { (f - 32.0) / 1.8 }

fn find_nth_fibonacci(n: i64) -> i64 { 
    match n {
        n if n >= 2 => find_nth_fibonacci(n - 1) + find_nth_fibonacci(n - 2),
        1           => 1,
        _           => 0
    }
}

fn main() {
    // get selection
    let mut selection = String::new();
    println!(
        "select mode:\n    [c]onvert fahrenheit to celcius\n    [f]ind nth fibonacci number");
    io::stdin()
        .read_line(&mut selection)
        .expect("failed to read line");
    println!("your selection: {}", selection);

    // get input
    let mut input = String::new();
    println!("please type your input below");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("your input: {}", input);
    let input: f64 = input.trim().parse().expect("input is not a number");

    let output: f64 = match selection.as_str().trim() {
        "c" => fahrenheit_to_celcius(input),
        "f" => find_nth_fibonacci(input as i64) as f64,
        _ => -3.1415
    };
    
    println!("your output is: {}", output);
}
