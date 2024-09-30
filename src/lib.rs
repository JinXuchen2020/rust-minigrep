use std::{env, fs};
use std::error::Error;

pub struct Config {
  query: String,
  file_path: String,
  ignore_case: bool,
}

impl Config {
  pub fn build(
    mut args: impl Iterator<Item = String>
  ) -> Result<Config, &'static str> {
    args.next(); // skip program name

    let query = args.next().ok_or("Missing query argument")?;
    let file_path = args.next().ok_or("Missing file argument")?;

    let ignore_case = env::var("IGNORE_CASE").map_or_else(
      |_| match args.next() {
        Some(arg) => arg == "-i",
        None => false,
      },
      |val| val == "1"
    );

    Ok(Config { query, file_path, ignore_case })
  }

  pub fn run(&self) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&self.file_path)?;

    let results = self.search(&contents);
    for line in results {
      println!("{}", line);
    }

    Ok(())
  }

  fn search<'a>(&'a self, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
      .filter(|line| {
        if self.ignore_case {
          line.to_lowercase().contains(&self.query.to_lowercase())
        }
        else {
          line.contains(&self.query)
        }
      })
      .map(|line| line.trim())
      .collect()
  }

  pub fn print(&self){
    println!("Searching for {}", self.query);
    println!("In file {}", self.file_path);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build() {
    let args = ["minigrep".to_string(),"test".to_string(), "test.txt".to_string()].into_iter();
    let config = Config::build(args).unwrap();
    assert_eq!(config.query, "test");
    assert_eq!(config.file_path, "test.txt");
  }

  #[test]
  fn test_run() {
    let args = vec!["minigrep".to_string(),"to".to_string(), "poem.txt".to_string()].into_iter();
    let config = Config::build(args).unwrap();
    config.run().unwrap();
  }

  #[test]
  fn test_run_insensitive() {
    let args = vec!["minigrep".to_string(), "to".to_string(), "poem.txt".to_string(), "-i".to_string()].into_iter();
    let config = Config::build(args).unwrap();
    config.run().unwrap();
  }

  #[test]
  fn test_print() {
    let args = vec!["minigrep".to_string(), "test".to_string(), "test.txt".to_string()].into_iter();
    let config = Config::build(args).unwrap();
    config.print();
  }
}