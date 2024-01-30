fn main() {
    let number = 7;

    //if number < 5 {
    //    println!("condition was true");
    //} else {
    //    println!("condition was false");
    //}

    //if number == 3 {
    //    println!("number was 3");
    //} else {
    //    println!("number was not 3");
    //}

    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}