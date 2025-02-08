use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore: bool,
}

impl Config {
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args length less then 3")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore = env::var("RUST_IGNORE").is_ok();
        Ok(Config { query, file_path, ignore})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let res;
    if let ignore = config.ignore{
         res = search_insensitive(&config.query, &contents);
    } else {
         res = search(&config.query, &contents);
    }
    println!("find this contains {}", res[0]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let query = "duct";
        let file_path =  "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query,file_path));
    }
}

pub fn search<'a>(query: &str, contents: & 'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: & 'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}