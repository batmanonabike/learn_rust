use std::io;

fn main() {
    println!("Guess the number?");
    println!("Input your number");

    let mut guess = String::new(); //UTF-8
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
