use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();        
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // Using env variables.

        Ok(Config { query, filename, case_sensitive })
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)  
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

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

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) ->  Vec<&'a str> {

    let query = query.to_lowercase(); // query is now a string.
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // passing string reference.
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
