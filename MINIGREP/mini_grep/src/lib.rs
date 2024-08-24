use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //let file_open_result = fs::read_to_string(config.file_path); 

    //let contents = match file_open_result { 
    //    Ok(_) => file_open_result,
    //    Err(e) => return Err(e.into()),
    //}; you can do this, or you can use the ? operator to shorthand it...

    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
         for line in search_ignore_case(&config.query, &contents){
             println!("{line}");
         }
    } else {
        for line in search(&config.query, &contents){
            println!("{line}");
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results:  Vec<&str> = vec![];
    for line in contents.lines(){
        if line.contains(query){
            results.push(line.trim());
        }
    }
    results 
}

pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line.trim());
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub file_path: String, 
    pub ignore_case: bool,
}

impl Config{ 
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut ignore_case; 

        if args.len() < 3 {
            return Err("need 3 CLI args!! !! !!!!! !! !") 
        }
        let query = args[1].clone(); 
        let file_path = args[2].clone();

        ignore_case = match env::var("IGNORE_CASE"){
            Ok(val) => match val.as_str(){
                "true" | "1" => true,
                "false" | "0" => false,
                _ => false,
            }
            Err(_) => false,
        };

        if args.len() > 3 {                                    
            ignore_case = match args[3].as_str(){
                "true" | "1" => true,
                "false" | "0" => false,
                _ => return Err("Bad CLI arg for ignore_case")
            };
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
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
                        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
     
    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";
        assert_eq!(vec!["Rust:"], search_ignore_case(query, contents)); 

    }
}
