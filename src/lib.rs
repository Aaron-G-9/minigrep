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
        println!("{:?}", line.1);
        let v: Vec<&str> = line.1.split(&config.query).collect();
        print!("{0}:  ", line.0.to_string().trim().green());
        for word in v {
            if word == "" {
                print!("{}", &config.query.bold().red());
            } else {
                print!("{}", word);
            }
        }
        print!("\n");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|&(_, line)| line.contains(query))
        .collect()
}
