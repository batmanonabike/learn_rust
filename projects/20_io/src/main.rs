// cargo run the poem.txt
//           ^^^ to supply command line arguments.

use std::env; // For command line args.
use std::process;

use io::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); // ["target/debug/io", "the", "poem.txt"]

    // unwrap_or_else() is similar un unwrap(), it returns the inner value of Ok on success.  If
    // the result is an Err value, the method calls the closure, which is an anonymous function we
    // pass in.
    // Closures are described later.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with args: {}", err);
        process::exit(1);        
    });

    if let Err(e) = io::run(config) {
        println!("Applicate error: {}", e);
        process::exit(1);
    }
}

