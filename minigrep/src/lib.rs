use std::{ error::Error, fs, env };

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(Config { 
            query: args[1].clone(), 
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_insensitive(&config.query, &text)
    } else {
        search(&config.query, &text)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in text.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
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
Duct tape.";

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
            search_insensitive(query, contents)
        );
    }
}