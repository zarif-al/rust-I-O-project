use std::{ fs, error::Error, env };

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the first value as it is the programs name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Did not find query string!");
            }
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Did not find file path string!");
            }
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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