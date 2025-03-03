use std::io;
fn main() {
    // TODO: Change the line below to fix the compiler error.
    println!("Enter a number:");

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: i32 = x.trim().parse().expect("Please type a number!");

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
