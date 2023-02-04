use std::{ fs, error::Error, env };

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("There are not enough arguments.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insentive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        let contains_query = line.contains(query);

        if contains_query {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        let contains_query = line.to_lowercase().contains(&query.to_lowercase());

        if contains_query {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust Me.";

    #[test]
    fn case_sensitive_search() {
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(query, SAMPLE_CONTENT))
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUst";

        assert_eq!(vec!["Rust:", "Trust Me."], search_case_insentive(query, SAMPLE_CONTENT))
    }
}