use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number?");
    println!("Input your number");

    let secret_number = rand::thread_rng().gen_range(1, 101); //>= 1 && <=100
    println!("The secret number is: {}", secret_number);

    loop {
        
        // Inferred as a String type...
        let mut guess = String::new(); //UTF-8
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Explicit type needed for match cmp (below), default is i32 unless 
        // inferred.  We currently have a String type though so we need to convert it.

        // Also we reuse the same named variable.  Here we 'shadow' the previous value with a new 
        // one. This means we dont need another variable name.
        let guess : u32  = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {                
            Ordering::Less => println!("Too small!"),   // <- These are called 'arms'
            Ordering::Greater => println!("Too big!"),  // consist of pattern and invoke method if 
            Ordering::Equal => {                        // match comparisons check until match 
                println!("You win");                    // found.  Then associated code is run.
                break;
            }            
        }                                               
    }
}
