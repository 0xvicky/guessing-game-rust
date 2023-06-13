use std::io;//to use input output library

fn main() {
    println!("Hello, world!");

    println!("Input variables:");

    let mut guess = String::new()://In rust, by default variables are immutable so we use "mut" keyword

    io::stdin().read_line(buf:&mut guess)
}

