use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    println!(
        "\n---searching for keyword '{0}' in file '{1}'---\n",
        config.query, config.file_path
    );

    let results;
    if config.ignore_case {
        results = search_case_insensitive(&config.query, &contents);
    } else {
        results = search(&config.query, &contents);
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            v.push(line.trim());
        }
    }
    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = vec![];

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            v.push(line.trim())
        }
    }
    v
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments... needs 3");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        if args.len() == 4 && args[3].contains("ignore") {
            ignore_case = true;
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
    fn one_result() {
        let query = "duct";
        let contents = "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
