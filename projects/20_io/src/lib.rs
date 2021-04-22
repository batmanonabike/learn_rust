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
#[allow(unused_variables)]
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.filename)?;        

    // use () inside Ok is to indicate we are using this function for its side effects only, it
    // doesn't return a value we need.
    Ok(())
}

// Regarding this functions lifetime annotations...
// Indicate that the returned vector should contain string slices that reference slices of the
// argument 'contents' (rather than the argument 'query')... We tell Rust that the data returned by 
// this function will live as long as the data passed into the search function in the 'contents'
// argument.
// IMPORTANT:
// The data references by a slice needs to be valid for the reference to be valid.
pub fn search<'a>(query: &str, contents: &'a str) ->  Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}