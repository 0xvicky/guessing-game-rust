use std::io;//to use input output library
use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Enter your guess:");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut guess = String::new(); //In rust, by default variables are immutable so we use "mut" keyword

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read msg.");//Here stdin give us handle the input, Also "read_line" takes the input and append it to the buffer, buffer here in this case is guess variable, ultimately the guess is assigned a value

    println!("You guess: {}", guess);

    
    //Now we need to generate random numbers, so we need dependency

}



