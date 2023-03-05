use std::io;

fn main() {

    let mut input = String::new();

    println!("input a temperature");

    io::stdin()
        .read_line(&mut input)
        .expect("give an input!");

    let input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("whoops"),
    };

    // (faren - 32) * (5/9) = celsius
    let celsius: f32 = (input - 32.0) * (5.0 / 9.0);
    
    // (cels * (9/5)) + 32 = faren
    let fahrenheit: f32 = (input * (9.0 / 5.0)) + 32.0;

    println!("{input}F is {celsius}C");
    println!("{input}C is {fahrenheit}F");
}
