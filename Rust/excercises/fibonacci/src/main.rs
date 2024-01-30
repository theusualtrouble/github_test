use std::io;

fn main() {
    println!("Type the nth number you want in the fibonacci sequence.");

    let mut n = String::new();

    io::stdin() 
        .read_line(&mut n)
        .expect("Failed");

    let n: i32 = n.trim().parse().expect("Please input a number");

    println!("The {}. in the fibonnaci sequence is {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fib(n-1) + fib(n-2);
    }
}

