use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();        
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Box<dyn Error> is a trait object (covered later).
// Box<dyn Error> here means the function will return a type that implements the Error trait but
// we don't have to specify what particular type the return value will be.  This gives us the
// flexibility to return error values that may be of different error cases.  
// The 'dyn' keyword is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    println!("Searching for: \"{}\"", config.query);
    println!("In file: \"{}\"", config.filename);

    let contents = fs::read_to_string(config.filename)?;
        //.expect("Failed to read input file!");

    println!("With text:\n{}", contents);

    // use () inside Ok is to indicate we are using this function for its side effects only, it
    // doesn't return a value we need.
    Ok(())
}

