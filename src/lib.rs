extern crate colored;

use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

//***************************************************
#[cfg(test)]
mod test {
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
}

//***************************************************

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not provided"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename not provided"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        "Program".chars().position(|c| c == 'g').unwrap();
        let v: Vec<&str> = line.split(&config.query).collect();
        for word in v{
            if word == ""{
                print!("{}", &config.query.bold().red());
            }else{
                print!("{}", word);
            }
        }
        print!("\n");
        //println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
