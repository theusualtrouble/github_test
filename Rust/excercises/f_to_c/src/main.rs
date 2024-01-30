use std::io;

fn main() {

    println!("Enter a temperature in fahrenheit you would like to convert to celsius!");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: i32 = fahrenheit
        .trim()
        .parse()
        .expect("Please enter a number.");

    println!("Your input was: {fahrenheit}");

    let celsius = ( fahrenheit - 32 ) * 5 / 9;

    println!("{fahrenheit} degrees fahrenheit is {celsius} degrees celsius.");

}